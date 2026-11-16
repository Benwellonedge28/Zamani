
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Workspace Metadata
//!
//! Path: src/toolchain/workspace/metadata.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Produces immutable, deterministic metadata snapshots for explicitly
//! discovered Zamani workspaces.
//!
//! A snapshot can describe:
//!
//! - Workspace identity.
//! - Canonical workspace location.
//! - Workspace and package manifests.
//! - Resolved workspace members.
//! - Project identities.
//! - Project layout inputs.
//! - Source-file inventories.
//! - Content fingerprints.
//!
//! ## Ownership
//!
//! This module owns metadata aggregation and content fingerprinting.
//!
//! It does not own:
//!
//! - Workspace discovery.
//! - Workspace or project identity allocation.
//! - Manifest parsing.
//! - Workspace member resolution.
//! - Project layout discovery.
//! - Dependency resolution.
//! - Compiler or IR versioning.
//! - Hardware discovery.
//! - Target selection.
//! - Runtime execution.
//!
//! ## Integration
//!
//! Consumes:
//!
//! - super::discovery::DiscoveryResult
//! - super::discovery::DiscoveredManifest
//! - super::layout::ProjectLayout
//! - super::members::WorkspaceMembers
//! - crate::toolchain::ids::{WorkspaceId, ProjectId}
//! - crate::toolchain::errors::ToolchainError
//! - crate::toolchain::result::Result
//!
//! The canonical manifest parser remains:
//!
//! - crate::zamani_project_config
//!
//! The caller supplies validated discovery, layout and member results.
//! This module does not parse Zamani.toml independently.
//!
//! ## Fingerprint contract
//!
//! The workspace fingerprint is SHA-256 over a versioned, domain-separated,
//! deterministic stream of:
//!
//! - Metadata schema identity.
//! - Workspace manifest content, when present.
//! - Root package manifest content, when present.
//! - Resolved member identities and relative paths.
//! - Member package manifest contents.
//! - Project layout source-file inventories.
//! - Source-file contents.
//!
//! Absolute filesystem paths, filesystem timestamps and machine-specific
//! properties are excluded.
//!
//! File paths are represented relative to the workspace root.
//!
//! A relocation that preserves the relative layout and contents therefore
//! does not change the fingerprint.
//!
//! Fingerprints are content identities, not persistent workspace IDs.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary limits on:
//!
//! - Workspace members.
//! - Project count.
//! - Source-file count.
//! - Directory depth.
//! - Computational domains.
//! - Targets.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Source files are hashed incrementally. Their contents are not loaded
//! entirely into memory.
//!
//! Memory consumption remains proportional to the metadata and file-path
//! inventories being collected, subject to available process resources.
//!
//! No implementation can promise literal infinite storage or execution.
//!
//! ## Determinism
//!
//! - Members are consumed in their existing deterministic order.
//! - Layouts are sorted by workspace-relative project path.
//! - Source files are sorted by workspace-relative path.
//! - Manifest and source content are hashed in deterministic order.
//! - Timestamps and absolute paths are excluded.
//! - Non-UTF-8 relative paths are rejected rather than ambiguously encoded.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No shell execution.
//! - No external process execution.
//! - No manifest content execution.
//! - Symlinked metadata inputs are rejected.
//! - Canonical paths must remain within the workspace.
//! - File changes during fingerprinting are detected where observable.
//! - I/O failures preserve their original causal errors.
//! - Incomplete fingerprints are never returned as successful results.
//!
//! Portable filesystem APIs cannot eliminate all adversarial
//! time-of-check/time-of-use races. Filesystem permissions and isolation
//! remain necessary when processing hostile shared directories.

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::{ProjectId, WorkspaceId};
use crate::toolchain::result::Result;

use super::discovery::{
    DiscoveredManifest,
    DiscoveryResult,
};
use super::layout::ProjectLayout;
use super::members::{
    WorkspaceMember,
    WorkspaceMembers,
};

use sha2::{Digest, Sha256};

use std::collections::BTreeSet;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

// ============================================================================
// FORMAT CONTRACT
// ============================================================================

/// Stable identifier for the workspace metadata fingerprint format.
///
/// Changing the meaning or ordering of hashed fields requires a new format
/// identifier. Existing fingerprints must never silently change meaning.
pub const METADATA_FINGERPRINT_FORMAT: &str =
    "zamani.workspace.metadata.sha256.v1";

/// Name of the fingerprint algorithm.
pub const METADATA_FINGERPRINT_ALGORITHM: &str = "sha256";

/// Streaming read buffer size.
///
/// This is an internal I/O batching choice, not a file-size limit.
/// Files of any size supported by the operating system are processed
/// incrementally.
const READ_BUFFER_SIZE: usize = 64 * 1024;

/// Domain-separation prefix for workspace metadata fingerprints.
const FINGERPRINT_DOMAIN: &[u8] =
    b"ZAMANI-WORKSPACE-METADATA\0";

// ============================================================================
// FINGERPRINT
// ============================================================================

/// A SHA-256 workspace metadata fingerprint.
///
/// The digest is independent of the workspace's absolute location and
/// filesystem timestamps.
///
/// The fingerprint format identifier is exposed separately so that
/// consumers never need to infer the encoding or hashing contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkspaceFingerprint {
    digest: [u8; 32],
}

impl WorkspaceFingerprint {
    /// Construct a fingerprint from a SHA-256 digest.
    #[must_use]
    pub const fn from_digest(digest: [u8; 32]) -> Self {
        Self { digest }
    }

    /// Borrow the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.digest
    }

    /// Return the canonical lowercase hexadecimal digest.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(64);

        for byte in self.digest {
            use fmt::Write as _;

            // Writing into String is infallible.
            let _ = write!(&mut output, "{byte:02x}");
        }

        output
    }

    /// Return the fingerprint algorithm identifier.
    #[must_use]
    pub const fn algorithm(&self) -> &'static str {
        METADATA_FINGERPRINT_ALGORITHM
    }

    /// Return the fingerprint format identifier.
    #[must_use]
    pub const fn format(&self) -> &'static str {
        METADATA_FINGERPRINT_FORMAT
    }
}

impl fmt::Display for WorkspaceFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.format(),
            self.to_hex()
        )
    }
}

// ============================================================================
// MANIFEST METADATA
// ============================================================================

/// Classification of a manifest included in workspace metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestRole {
    /// Explicit workspace manifest.
    Workspace,

    /// Package manifest belonging to a project.
    Package,
}

/// Metadata describing one manifest without duplicating its parser.
///
/// `relative_path` is relative to the workspace root.
/// `content_fingerprint` is the SHA-256 digest of the original manifest
/// bytes, not a normalized TOML representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestMetadata {
    role: ManifestRole,
    relative_path: PathBuf,
    content_fingerprint: WorkspaceFingerprint,
    size_bytes: u64,
}

impl ManifestMetadata {
    /// Return the manifest role.
    #[must_use]
    pub const fn role(&self) -> ManifestRole {
        self.role
    }

    /// Return the workspace-relative manifest path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Return the manifest content fingerprint.
    #[must_use]
    pub const fn content_fingerprint(
        &self,
    ) -> WorkspaceFingerprint {
        self.content_fingerprint
    }

    /// Return the manifest size in bytes.
    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }
}

// ============================================================================
// SOURCE METADATA
// ============================================================================

/// Metadata describing one inspected source file.
///
/// Paths are workspace-relative, not absolute.
///
/// The digest represents the file bytes observed during snapshot
/// construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFileMetadata {
    relative_path: PathBuf,
    content_fingerprint: WorkspaceFingerprint,
    size_bytes: u64,
}

impl SourceFileMetadata {
    /// Return the workspace-relative source path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Return the source-file content fingerprint.
    #[must_use]
    pub const fn content_fingerprint(
        &self,
    ) -> WorkspaceFingerprint {
        self.content_fingerprint
    }

    /// Return the observed file size.
    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }
}

// ============================================================================
// PROJECT METADATA
// ============================================================================

/// Metadata describing one resolved project in a workspace.
///
/// The project identity is supplied by the existing identity authority.
/// This module never generates or guesses project IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceProjectMetadata {
    project_id: Option<ProjectId>,
    relative_root: PathBuf,
    package_manifest: Option<ManifestMetadata>,
    source_files: Vec<SourceFileMetadata>,
    source_fingerprint: WorkspaceFingerprint,
}

impl WorkspaceProjectMetadata {
    /// Return the supplied persistent project identity, if available.
    #[must_use]
    pub fn project_id(&self) -> Option<&ProjectId> {
        self.project_id.as_ref()
    }

    /// Return the project root relative to the workspace.
    #[must_use]
    pub fn relative_root(&self) -> &Path {
        &self.relative_root
    }

    /// Return package-manifest metadata, when available.
    #[must_use]
    pub fn package_manifest(&self) -> Option<&ManifestMetadata> {
        self.package_manifest.as_ref()
    }

    /// Borrow the deterministically ordered source-file metadata.
    #[must_use]
    pub fn source_files(&self) -> &[SourceFileMetadata] {
        &self.source_files
    }

    /// Return the number of inspected source files.
    #[must_use]
    pub fn source_file_count(&self) -> usize {
        self.source_files.len()
    }

    /// Return the source inventory and content fingerprint.
    ///
    /// This is distinct from the complete workspace fingerprint.
    #[must_use]
    pub const fn source_fingerprint(
        &self,
    ) -> WorkspaceFingerprint {
        self.source_fingerprint
    }
}

// ============================================================================
// WORKSPACE METADATA
// ============================================================================

/// Immutable metadata snapshot of a validated workspace.
///
/// The snapshot does not watch the filesystem. Callers must build another
/// snapshot to observe changes made after construction.
///
/// The workspace ID is persistent identity supplied by the caller.
/// The fingerprint describes the observed content and structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMetadata {
    workspace_id: WorkspaceId,
    canonical_root: PathBuf,
    explicit_workspace: bool,
    workspace_manifest: Option<ManifestMetadata>,
    root_package_manifest: Option<ManifestMetadata>,
    projects: Vec<WorkspaceProjectMetadata>,
    fingerprint: WorkspaceFingerprint,
    total_source_bytes: u64,
    total_manifest_bytes: u64,
}

impl WorkspaceMetadata {
    /// Build a complete metadata snapshot from validated upstream results.
    ///
    /// `workspace_id` must come from the persistent identity authority.
    ///
    /// `discovery` must be a successful result from workspace discovery.
    ///
    /// `layouts` contains the already inspected layouts for the projects
    /// whose source contents should participate in the fingerprint.
    ///
    /// `members` supplies validated member identities and paths for an
    /// explicit workspace. It must refer to the same workspace root.
    ///
    /// A standalone project may pass `None` for `members`.
    ///
    /// For a workspace with multiple package projects, provide a layout
    /// for every member whose source contents must be covered by the
    /// fingerprint. A missing layout is not interpreted as an empty source
    /// project: it is simply not included in the source inventory.
    ///
    /// The workspace and package manifests are always fingerprinted when
    /// present in discovery.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Discovery did not identify a project or workspace.
    /// - The workspace root cannot be established.
    /// - Supplied members belong to another workspace.
    /// - A layout is outside the workspace or is not a known project root.
    /// - A manifest or source file is missing or inaccessible.
    /// - A file is a symbolic link or not a regular file.
    /// - A relative path cannot be represented portably as UTF-8.
    /// - A file changes while it is being fingerprinted.
    /// - An integer accounting operation overflows.
    /// - Any filesystem operation fails.
    pub fn build(
        workspace_id: WorkspaceId,
        discovery: &DiscoveryResult,
        layouts: &[ProjectLayout],
        members: Option<&WorkspaceMembers>,
    ) -> Result<Self> {
        if !discovery.is_successful() {
            return Err(
                ToolchainError::invalid_input(
                    "workspace metadata requires successful project or workspace discovery",
                )
                .with_operation("workspace.metadata.build"),
            );
        }

        let root = discovery
            .effective_workspace_root()
            .ok_or_else(|| {
                ToolchainError::internal(
                    "successful discovery has no effective workspace root",
                )
                .with_operation("workspace.metadata.build")
            })?;

        let canonical_root = fs::canonicalize(root).map_err(|error| {
            io_error(
                "workspace.metadata.canonicalize_root",
                root,
                error,
            )
        })?;

        validate_directory(&canonical_root)?;

        let explicit_workspace = discovery.has_explicit_workspace();

        if let Some(members) = members {
            if !explicit_workspace {
                return Err(
                    ToolchainError::invalid_input(
                        "resolved workspace members require an explicit workspace manifest",
                    )
                    .with_operation("workspace.metadata.build"),
                );
            }

            let member_root = fs::canonicalize(
                members.workspace_root(),
            )
            .map_err(|error| {
                io_error(
                    "workspace.metadata.canonicalize_members_root",
                    members.workspace_root(),
                    error,
                )
            })?;

            if member_root != canonical_root
                || members.workspace_id() != &workspace_id
            {
                return Err(
                    ToolchainError::validation_failed(
                        "workspace identity or root does not match the supplied member collection",
                    )
                    .with_operation("workspace.metadata.build"),
                );
            }
        }

        let mut projects = collect_project_descriptors(
            &canonical_root,
            discovery,
            members,
        )?;

        validate_layouts(
            &canonical_root,
            &projects,
            layouts,
        )?;

        // Layouts are mapped to their canonical project roots.
        // Sorting is based on portable workspace-relative paths.
        let mut ordered_layouts: Vec<&ProjectLayout> =
            layouts.iter().collect();

        ordered_layouts.sort_by(|left, right| {
            relative_path_key(
                &canonical_root,
                left.project_root(),
            )
            .cmp(
                &relative_path_key(
                    &canonical_root,
                    right.project_root(),
                ),
            )
        });

        let mut hasher = Sha256::new();
        begin_fingerprint(&mut hasher);

        hash_field(
            &mut hasher,
            b"workspace-root",
            &portable_relative_path(
                &canonical_root,
                &canonical_root,
            )?,
        );

        let mut manifests = Vec::new();
        let mut total_manifest_bytes = 0_u64;

        if let Some(manifest) = discovery.workspace_manifest() {
            let metadata = collect_manifest(
                &canonical_root,
                manifest,
                ManifestRole::Workspace,
            )?;

            hash_manifest(&mut hasher, &metadata);

            total_manifest_bytes = checked_add(
                total_manifest_bytes,
                metadata.size_bytes,
                "manifest byte accounting",
            )?;

            manifests.push(metadata);
        }

        let root_package_manifest = if let Some(manifest) =
            discovery.project_manifest()
        {
            let metadata = collect_manifest(
                &canonical_root,
                manifest,
                ManifestRole::Package,
            )?;

            hash_manifest(&mut hasher, &metadata);

            total_manifest_bytes = checked_add(
                total_manifest_bytes,
                metadata.size_bytes,
                "manifest byte accounting",
            )?;

            Some(metadata)
        } else {
            None
        };

        // Every resolved member manifest is included, even when its
        // source layout has not yet been inspected.
        for descriptor in &projects {
            if let Some(manifest_path) =
                descriptor.package_manifest_path.as_deref()
            {
                let already_included = root_package_manifest
                    .as_ref()
                    .is_some_and(|manifest| {
                        canonical_root
                            .join(&manifest.relative_path)
                            == manifest_path
                    });

                if already_included {
                    continue;
                }

                let metadata = collect_manifest_path(
                    &canonical_root,
                    manifest_path,
                    ManifestRole::Package,
                )?;

                hash_manifest(&mut hasher, &metadata);

                total_manifest_bytes = checked_add(
                    total_manifest_bytes,
                    metadata.size_bytes,
                    "manifest byte accounting",
                )?;

                descriptor.manifest_metadata = Some(metadata);
            }
        }

        // Associate the discovered root package manifest with its
        // corresponding project descriptor.
        if let Some(manifest) = &root_package_manifest {
            for descriptor in &mut projects {
                if descriptor.relative_root == PathBuf::from(".") {
                    descriptor.manifest_metadata =
                        Some(manifest.clone());
                }
            }
        }

        let mut total_source_bytes = 0_u64;
        let mut all_sources = BTreeSet::new();

        for layout in ordered_layouts {
            let relative_root = portable_relative_path(
                &canonical_root,
                layout.project_root(),
            )?;

            hash_field(
                &mut hasher,
                b"project-layout-root",
                relative_root.as_bytes(),
            );

            let descriptor_index = projects
                .iter()
                .position(|project| {
                    project.relative_root == relative_root
                })
                .ok_or_else(|| {
                    ToolchainError::internal(
                        "validated project layout has no project metadata descriptor",
                    )
                    .with_operation(
                        "workspace.metadata.associate_layout",
                    )
                })?;

            let mut source_paths = Vec::new();

            layout.visit_source_files(|path| {
                source_paths.push(path.to_path_buf());
                Ok(())
            })?;

            source_paths.sort_by(|left, right| {
                relative_path_key(&canonical_root, left)
                    .cmp(&relative_path_key(&canonical_root, right))
            });

            let mut project_sources = Vec::new();

            for source_path in source_paths {
                let relative_path = portable_relative_path(
                    &canonical_root,
                    &source_path,
                )?;

                if !all_sources.insert(relative_path.clone()) {
                    return Err(
                        ToolchainError::validation_failed(
                            "a source file was reported by more than one project layout",
                        )
                        .with_operation(
                            "workspace.metadata.collect_sources",
                        )
                        .with_context(relative_path),
                    );
                }

                let source = collect_file(
                    &canonical_root,
                    &source_path,
                    b"source-file",
                )?;

                hash_source(&mut hasher, &source);

                total_source_bytes = checked_add(
                    total_source_bytes,
                    source.size_bytes,
                    "source byte accounting",
                )?;

                project_sources.push(source);
            }

            project_sources.sort_by(|left, right| {
                left.relative_path.cmp(&right.relative_path)
            });

            let source_fingerprint =
                fingerprint_project_sources(
                    &relative_root,
                    &project_sources,
                );

            projects[descriptor_index].source_files =
                project_sources;

            projects[descriptor_index].source_fingerprint =
                source_fingerprint;
        }

        // Hash project identities and relative membership independently
        // of the source inventory.
        projects.sort_by(|left, right| {
            left.relative_root.cmp(&right.relative_root)
        });

        for project in &projects {
            hash_field(
                &mut hasher,
                b"project-relative-root",
                &portable_path(&project.relative_root)?,
            );

            if let Some(project_id) = &project.project_id {
                hash_field(
                    &mut hasher,
                    b"project-id",
                    project_id.to_string().as_bytes(),
                );
            } else {
                hash_field(
                    &mut hasher,
                    b"project-id",
                    b"unassigned",
                );
            }

            if let Some(manifest) = &project.manifest_metadata {
                hash_manifest(&mut hasher, manifest);
            }

            for source in &project.source_files {
                hash_source(&mut hasher, source);
            }
        }

        hash_field(
            &mut hasher,
            b"project-count",
            projects.len().to_string().as_bytes(),
        );

        hash_field(
            &mut hasher,
            b"source-count",
            all_sources.len().to_string().as_bytes(),
        );

        let digest: [u8; 32] = hasher.finalize().into();

        let fingerprint =
            WorkspaceFingerprint::from_digest(digest);

        let mut finalized_projects = Vec::with_capacity(
            projects.len(),
        );

        for project in projects {
            let source_fingerprint =
                if project.source_files.is_empty() {
                    fingerprint_project_sources(
                        &project.relative_root,
                        &[],
                    )
                } else {
                    project.source_fingerprint
                };

            finalized_projects.push(
                WorkspaceProjectMetadata {
                    project_id: project.project_id,
                    relative_root: project.relative_root,
                    package_manifest: project.manifest_metadata,
                    source_files: project.source_files,
                    source_fingerprint,
                },
            );
        }

        Ok(Self {
            workspace_id,
            canonical_root,
            explicit_workspace,
            workspace_manifest: manifests
                .into_iter()
                .next(),
            root_package_manifest,
            projects: finalized_projects,
            fingerprint,
            total_source_bytes,
            total_manifest_bytes,
        })
    }

    /// Return the stable workspace identity.
    #[must_use]
    pub fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    /// Return the canonical absolute workspace location.
    ///
    /// This location is informational and is deliberately excluded from
    /// the content fingerprint.
    #[must_use]
    pub fn canonical_root(&self) -> &Path {
        &self.canonical_root
    }

    /// Whether discovery identified an explicit workspace manifest.
    #[must_use]
    pub const fn has_explicit_workspace(&self) -> bool {
        self.explicit_workspace
    }

    /// Return explicit workspace-manifest metadata, when present.
    #[must_use]
    pub fn workspace_manifest(&self) -> Option<&ManifestMetadata> {
        self.workspace_manifest.as_ref()
    }

    /// Return root package-manifest metadata, when present.
    #[must_use]
    pub fn root_package_manifest(&self) -> Option<&ManifestMetadata> {
        self.root_package_manifest.as_ref()
    }

    /// Borrow projects in deterministic relative-path order.
    #[must_use]
    pub fn projects(&self) -> &[WorkspaceProjectMetadata] {
        &self.projects
    }

    /// Return the number of represented projects.
    #[must_use]
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    /// Return the total number of indexed source files.
    #[must_use]
    pub fn source_file_count(&self) -> usize {
        self.projects
            .iter()
            .map(WorkspaceProjectMetadata::source_file_count)
            .sum()
    }

    /// Return total source bytes read during snapshot construction.
    #[must_use]
    pub const fn total_source_bytes(&self) -> u64 {
        self.total_source_bytes
    }

    /// Return total manifest bytes read during snapshot construction.
    #[must_use]
    pub const fn total_manifest_bytes(&self) -> u64 {
        self.total_manifest_bytes
    }

    /// Return the complete workspace content fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> WorkspaceFingerprint {
        self.fingerprint
    }

    /// Return the canonical fingerprint string.
    #[must_use]
    pub fn fingerprint_string(&self) -> String {
        self.fingerprint.to_string()
    }

    /// Return the metadata fingerprint format.
    #[must_use]
    pub const fn fingerprint_format(&self) -> &'static str {
        METADATA_FINGERPRINT_FORMAT
    }

    /// Find project metadata by persistent project identity.
    #[must_use]
    pub fn project_by_id(
        &self,
        project_id: &ProjectId,
    ) -> Option<&WorkspaceProjectMetadata> {
        self.projects.iter().find(|project| {
            project.project_id.as_ref() == Some(project_id)
        })
    }

    /// Find project metadata by workspace-relative root.
    #[must_use]
    pub fn project_by_relative_root(
        &self,
        path: impl AsRef<Path>,
    ) -> Option<&WorkspaceProjectMetadata> {
        self.projects.iter().find(|project| {
            project.relative_root == path.as_ref()
        })
    }
}

// ============================================================================
// INTERNAL PROJECT DESCRIPTORS
// ============================================================================

struct ProjectDescriptor {
    project_id: Option<ProjectId>,
    relative_root: PathBuf,
    package_manifest_path: Option<PathBuf>,
    manifest_metadata: Option<ManifestMetadata>,
    source_files: Vec<SourceFileMetadata>,
    source_fingerprint: WorkspaceFingerprint,
}

fn collect_project_descriptors(
    workspace_root: &Path,
    discovery: &DiscoveryResult,
    members: Option<&WorkspaceMembers>,
) -> Result<Vec<ProjectDescriptor>> {
    let mut projects = Vec::new();

    if let Some(members) = members {
        for member in members.iter() {
            projects.push(project_descriptor(
                workspace_root,
                Some(member),
            )?);
        }
    }

    if let Some(root_manifest) = discovery.project_manifest() {
        let already_present = projects.iter().any(|project| {
            workspace_root.join(&project.relative_root)
                == root_manifest.root()
        });

        if !already_present {
            projects.push(project_descriptor_from_root(
                workspace_root,
                None,
                root_manifest,
            )?);
        }
    }

    projects.sort_by(|left, right| {
        left.relative_root.cmp(&right.relative_root)
    });

    Ok(projects)
}

fn project_descriptor(
    workspace_root: &Path,
    member: &WorkspaceMember,
) -> Result<ProjectDescriptor> {
    let relative_root = portable_relative_path(
        workspace_root,
        member.canonical_root(),
    )?;

    Ok(ProjectDescriptor {
        project_id: Some(member.project_id().clone()),
        relative_root: PathBuf::from(relative_root),
        package_manifest_path: Some(
            member.manifest_path().to_path_buf(),
        ),
        manifest_metadata: None,
        source_files: Vec::new(),
        source_fingerprint: empty_fingerprint(),
    })
}

fn project_descriptor_from_root(
    workspace_root: &Path,
    project_id: Option<ProjectId>,
    manifest: &DiscoveredManifest,
) -> Result<ProjectDescriptor> {
    let relative_root = portable_relative_path(
        workspace_root,
        manifest.root(),
    )?;

    Ok(ProjectDescriptor {
        project_id,
        relative_root: PathBuf::from(relative_root),
        package_manifest_path: Some(
            manifest.path().to_path_buf(),
        ),
        manifest_metadata: None,
        source_files: Vec::new(),
        source_fingerprint: empty_fingerprint(),
    })
}

// ============================================================================
// LAYOUT VALIDATION
// ============================================================================

fn validate_layouts(
    workspace_root: &Path,
    projects: &[ProjectDescriptor],
    layouts: &[ProjectLayout],
) -> Result<()> {
    let mut seen_roots = BTreeSet::new();

    for layout in layouts {
        let canonical = fs::canonicalize(
            layout.project_root(),
        )
        .map_err(|error| {
            io_error(
                "workspace.metadata.validate_layout",
                layout.project_root(),
                error,
            )
        })?;

        if canonical != layout.project_root() {
            return Err(
                ToolchainError::validation_failed(
                    "project layout root is not canonical",
                )
                .with_operation(
                    "workspace.metadata.validate_layout",
                ),
            );
        }

        let relative = portable_relative_path(
            workspace_root,
            &canonical,
        )?;

        if !projects.iter().any(|project| {
            project.relative_root == relative
        }) {
            return Err(
                ToolchainError::validation_failed(
                    "project layout does not belong to the supplied workspace project set",
                )
                .with_operation(
                    "workspace.metadata.validate_layout",
                )
                .with_context(relative),
            );
        }

        if !seen_roots.insert(relative.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    "multiple project layouts describe the same project root",
                )
                .with_operation(
                    "workspace.metadata.validate_layout",
                )
                .with_context(relative),
            );
        }
    }

    Ok(())
}

// ============================================================================
// MANIFEST COLLECTION
// ============================================================================

fn collect_manifest(
    workspace_root: &Path,
    manifest: &DiscoveredManifest,
    role: ManifestRole,
) -> Result<ManifestMetadata> {
    collect_manifest_path(
        workspace_root,
        manifest.path(),
        role,
    )
}

fn collect_manifest_path(
    workspace_root: &Path,
    manifest_path: &Path,
    role: ManifestRole,
) -> Result<ManifestMetadata> {
    let file = collect_file(
        workspace_root,
        manifest_path,
        b"manifest-file",
    )?;

    Ok(ManifestMetadata {
        role,
        relative_path: file.relative_path,
        content_fingerprint: file.content_fingerprint,
        size_bytes: file.size_bytes,
    })
}

// ============================================================================
// FILE COLLECTION
// ============================================================================

struct CollectedFile {
    relative_path: PathBuf,
    content_fingerprint: WorkspaceFingerprint,
    size_bytes: u64,
}

fn collect_file(
    workspace_root: &Path,
    supplied_path: &Path,
    domain: &[u8],
) -> Result<CollectedFile> {
    let canonical = fs::canonicalize(supplied_path)
        .map_err(|error| {
            io_error(
                "workspace.metadata.canonicalize_file",
                supplied_path,
                error,
            )
        })?;

    let relative = portable_relative_path(
        workspace_root,
        &canonical,
    )?;

    let path_metadata = fs::symlink_metadata(&canonical)
        .map_err(|error| {
            io_error(
                "workspace.metadata.inspect_file",
                &canonical,
                error,
            )
        })?;

    if path_metadata.file_type().is_symlink()
        || !path_metadata.is_file()
    {
        return Err(
            ToolchainError::validation_failed(
                "metadata input must be a regular non-symlink file",
            )
            .with_operation("workspace.metadata.collect_file")
            .with_context(relative),
        );
    }

    let mut file = File::open(&canonical).map_err(|error| {
        io_error(
            "workspace.metadata.open_file",
            &canonical,
            error,
        )
    })?;

    let before = file.metadata().map_err(|error| {
        io_error(
            "workspace.metadata.metadata_before_read",
            &canonical,
            error,
        )
    })?;

    if !before.is_file() {
        return Err(
            ToolchainError::validation_failed(
                "opened metadata input is not a regular file",
            )
            .with_operation("workspace.metadata.collect_file"),
        );
    }

    let expected_length = before.len();

    let mut hasher = Sha256::new();
    hash_field(&mut hasher, b"file-domain", domain);
    hash_field(
        &mut hasher,
        b"file-relative-path",
        relative.as_bytes(),
    );

    let mut buffer = [0_u8; READ_BUFFER_SIZE];
    let mut observed_length = 0_u64;

    loop {
        let count = file.read(&mut buffer).map_err(|error| {
            io_error(
                "workspace.metadata.read_file",
                &canonical,
                error,
            )
        })?;

        if count == 0 {
            break;
        }

        observed_length = checked_add(
            observed_length,
            count as u64,
            "file byte accounting",
        )?;

        hasher.update(&buffer[..count]);
    }

    let after = file.metadata().map_err(|error| {
        io_error(
            "workspace.metadata.metadata_after_read",
            &canonical,
            error,
        )
    })?;

    if observed_length != expected_length
        || after.len() != expected_length
    {
        return Err(
            ToolchainError::validation_failed(
                "file size changed while metadata was being collected",
            )
            .with_operation(
                "workspace.metadata.verify_file_stability",
            )
            .with_context(relative),
        );
    }

    if before.modified().ok() != after.modified().ok() {
        return Err(
            ToolchainError::validation_failed(
                "file modification time changed while metadata was being collected",
            )
            .with_operation(
                "workspace.metadata.verify_file_stability",
            )
            .with_context(relative),
        );
    }

    let digest: [u8; 32] = hasher.finalize().into();

    Ok(CollectedFile {
        relative_path: PathBuf::from(relative),
        content_fingerprint:
            WorkspaceFingerprint::from_digest(digest),
        size_bytes: observed_length,
    })
}

// ============================================================================
// HASHING
// ============================================================================

fn begin_fingerprint(hasher: &mut Sha256) {
    hash_field(
        hasher,
        b"fingerprint-domain",
        FINGERPRINT_DOMAIN,
    );

    hash_field(
        hasher,
        b"fingerprint-format",
        METADATA_FINGERPRINT_FORMAT.as_bytes(),
    );

    hash_field(
        hasher,
        b"fingerprint-algorithm",
        METADATA_FINGERPRINT_ALGORITHM.as_bytes(),
    );
}

fn hash_field(
    hasher: &mut Sha256,
    name: &[u8],
    value: &[u8],
) {
    // Length-prefixing prevents ambiguous concatenations.
    hasher.update((name.len() as u64).to_be_bytes());
    hasher.update(name);
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn hash_manifest(
    hasher: &mut Sha256,
    manifest: &ManifestMetadata,
) {
    hash_field(
        hasher,
        b"manifest-role",
        match manifest.role {
            ManifestRole::Workspace => b"workspace",
            ManifestRole::Package => b"package",
        },
    );

    hash_field(
        hasher,
        b"manifest-path",
        manifest
            .relative_path
            .to_string_lossy()
            .as_bytes(),
    );

    hash_field(
        hasher,
        b"manifest-digest",
        manifest.content_fingerprint.as_bytes(),
    );
}

fn hash_source(
    hasher: &mut Sha256,
    source: &SourceFileMetadata,
) {
    hash_field(
        hasher,
        b"source-path",
        source
            .relative_path
            .to_string_lossy()
            .as_bytes(),
    );

    hash_field(
        hasher,
        b"source-digest",
        source.content_fingerprint.as_bytes(),
    );
}

fn fingerprint_project_sources(
    relative_root: &str,
    sources: &[SourceFileMetadata],
) -> WorkspaceFingerprint {
    let mut hasher = Sha256::new();

    begin_fingerprint(&mut hasher);

    hash_field(
        &mut hasher,
        b"project-source-root",
        relative_root.as_bytes(),
    );

    for source in sources {
        hash_source(&mut hasher, source);
    }

    let digest: [u8; 32] = hasher.finalize().into();

    WorkspaceFingerprint::from_digest(digest)
}

fn empty_fingerprint() -> WorkspaceFingerprint {
    fingerprint_project_sources(".", &[])
}

// ============================================================================
// PORTABLE PATH REPRESENTATION
// ============================================================================

fn portable_relative_path(
    root: &Path,
    path: &Path,
) -> Result<String> {
    let relative = path.strip_prefix(root).map_err(|_| {
        ToolchainError::validation_failed(
            "metadata input path escapes the canonical workspace root",
        )
        .with_operation(
            "workspace.metadata.validate_path_containment",
        )
        .with_context(path.display().to_string())
    })?;

    portable_path(relative)
}

fn portable_path(path: &Path) -> Result<String> {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            Component::Normal(value) => {
                let value = value.to_str().ok_or_else(|| {
                    ToolchainError::unsupported_operation(
                        "workspace metadata requires portable UTF-8 relative paths",
                    )
                    .with_operation(
                        "workspace.metadata.portable_path",
                    )
                })?;

                components.push(value);
            }

            Component::CurDir => {}

            Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return Err(
                    ToolchainError::validation_failed(
                        "workspace-relative path contains a forbidden path component",
                    )
                    .with_operation(
                        "workspace.metadata.portable_path",
                    ),
                );
            }
        }
    }

    if components.is_empty() {
        return Ok(".".to_owned());
    }

    Ok(components.join("/"))
}

fn relative_path_key(
    root: &Path,
    path: &Path,
) -> String {
    portable_relative_path(root, path)
        .unwrap_or_else(|_| path.display().to_string())
}

// ============================================================================
// FILESYSTEM AND ACCOUNTING HELPERS
// ============================================================================

fn validate_directory(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        io_error(
            "workspace.metadata.inspect_directory",
            path,
            error,
        )
    })?;

    if metadata.file_type().is_symlink()
        || !metadata.is_dir()
    {
        return Err(
            ToolchainError::validation_failed(
                "workspace root must be a regular non-symlink directory",
            )
            .with_operation(
                "workspace.metadata.validate_directory",
            )
            .with_context(path.display().to_string()),
        );
    }

    Ok(())
}

fn checked_add(
    left: u64,
    right: u64,
    description: &str,
) -> Result<u64> {
    left.checked_add(right).ok_or_else(|| {
        ToolchainError::resource_exhausted(
            format!("{description} exceeded the supported integer representation"),
        )
        .with_operation("workspace.metadata.accounting")
    })
}

fn io_error(
    operation: &'static str,
    path: &Path,
    source: io::Error,
) -> ToolchainError {
    ToolchainError::from_io(operation, source)
        .with_context(path.display().to_string())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_hex_is_canonical() {
        let fingerprint =
            WorkspaceFingerprint::from_digest([0xab; 32]);

        assert_eq!(fingerprint.to_hex().len(), 64);
        assert_eq!(
            fingerprint.to_hex(),
            "ab".repeat(32)
        );
    }

    #[test]
    fn fingerprint_display_contains_format_and_digest() {
        let fingerprint =
            WorkspaceFingerprint::from_digest([0_u8; 32]);

        assert_eq!(
            fingerprint.to_string(),
            format!(
                "{}:{}",
                METADATA_FINGERPRINT_FORMAT,
                "00".repeat(32)
            )
        );
    }

    #[test]
    fn fingerprint_algorithms_are_explicit() {
        let fingerprint =
            WorkspaceFingerprint::from_digest([1_u8; 32]);

        assert_eq!(
            fingerprint.algorithm(),
            "sha256"
        );

        assert_eq!(
            fingerprint.format(),
            METADATA_FINGERPRINT_FORMAT
        );
    }

    #[test]
    fn portable_paths_use_forward_slashes() {
        let path = Path::new("modules")
            .join("classical")
            .join("source");

        let portable = portable_path(&path)
            .expect("normal relative path");

        assert_eq!(
            portable,
            "modules/classical/source"
        );
    }

    #[test]
    fn portable_current_directory_is_canonical() {
        assert_eq!(
            portable_path(Path::new("."))
                .expect("current directory"),
            "."
        );
    }

    #[test]
    fn parent_components_are_rejected() {
        let error = portable_path(
            Path::new("../outside"),
        )
        .expect_err("parent traversal must be rejected");

        assert_eq!(
            error.kind(),
            crate::toolchain::errors::ToolchainErrorKind::ValidationFailed
        );
    }

    #[test]
    fn source_fingerprints_are_deterministic() {
        let first = fingerprint_project_sources(
            ".",
            &[],
        );

        let second = fingerprint_project_sources(
            ".",
            &[],
        );

        assert_eq!(first, second);
    }

    #[test]
    fn different_project_roots_have_distinct_source_fingerprints() {
        let first = fingerprint_project_sources(
            ".",
            &[],
        );

        let second = fingerprint_project_sources(
            "modules/example",
            &[],
        );

        assert_ne!(first, second);
    }

    #[test]
    fn hash_field_encoding_distinguishes_field_boundaries() {
        let mut first = Sha256::new();

        hash_field(
            &mut first,
            b"a",
            b"bc",
        );

        let mut second = Sha256::new();

        hash_field(
            &mut second,
            b"ab",
            b"c",
        );

        assert_ne!(
            first.finalize(),
            second.finalize()
        );
    }

    #[test]
    fn checked_accounting_detects_overflow() {
        let error = checked_add(
            u64::MAX,
            1,
            "test accounting",
        )
        .expect_err("overflow must be reported");

        assert_eq!(
            error.kind(),
            crate::toolchain::errors::ToolchainErrorKind::ResourceExhausted
        );
    }
}
