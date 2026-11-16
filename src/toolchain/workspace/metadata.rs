
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Workspace Metadata
//!
//! Path: `src/toolchain/workspace/metadata.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Constructs deterministic workspace and project metadata from validated
//! workspace membership, project layouts, manifests and source contents.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Workspace metadata snapshots.
//! - Project metadata snapshots.
//! - Source inventory completeness.
//! - Manifest and source content fingerprints.
//! - Workspace and project fingerprints.
//! - Deterministic metadata aggregation.
//! - Detection of overlapping project source inventories.
//!
//! This module does not own:
//!
//! - Project or workspace identity allocation.
//! - Workspace discovery.
//! - Workspace member resolution.
//! - Manifest parsing or interpretation.
//! - Source-layout discovery.
//! - Dependency resolution.
//! - Compiler or IR fingerprints.
//! - Backend or hardware fingerprints.
//! - Build execution or caching.
//! - Cryptographic signing or trust decisions.
//!
//! ## Integration
//!
//! - `super::discovery` owns project/workspace discovery.
//! - `super::layout` owns project-layout inspection and source traversal.
//! - `super::members` owns validated workspace membership.
//! - `crate::toolchain::ids` owns project/workspace identities.
//! - `crate::toolchain::errors` owns error classification.
//! - `crate::toolchain::result` owns result aliases.
//! - `sha2` provides SHA-256.
//!
//! Future consumers:
//!
//! - `build::inputs`.
//! - `build::incremental`.
//! - `dependencies::lockfile`.
//! - `environments::fingerprints`.
//! - `security::provenance`.
//!
//! ## Fingerprint semantics
//!
//! Content fingerprints identify file contents and file roles.
//! They deliberately do not include filesystem paths.
//!
//! Project fingerprints additionally identify project-relative paths.
//!
//! Workspace fingerprints identify the ordered workspace composition,
//! project identities, relative paths, manifest contents, source contents
//! and source-inventory completeness.
//!
//! These fingerprints are not cryptographic signatures.
//!
//! ## Scalability
//!
//! No arbitrary maximum workspace size, project count, member count,
//! source-file count, file size, directory depth, computational domain,
//! target count or hardware-resource count is imposed.
//!
//! File contents are hashed incrementally. Entire files are never loaded
//! into memory merely to calculate their fingerprints.
//!
//! Memory use is proportional to metadata records, path representations,
//! traversal state and the configured filesystem implementation.
//!
//! All operations remain subject to available memory, storage, address
//! space, operating-system permissions and filesystem capabilities.
//!
//! Literal infinite storage or execution is not promised.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No shell execution.
//! - No external process execution.
//! - No manifest execution.
//! - No automatic modification of project files.
//! - No following source-directory symlinks through layout traversal.
//! - No silent treatment of unreadable files as empty files.
//! - No acceptance of paths outside their owning project.
//! - File changes observed during hashing are reported.
//!
//! Standard portable filesystem APIs cannot eliminate every hostile
//! time-of-check/time-of-use race. Adversarial shared filesystems require
//! appropriate filesystem permissions or an OS-specific handle-relative
//! filesystem implementation.

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::ids::{
    ProjectId,
    WorkspaceId,
};
use crate::toolchain::result::Result;

use super::layout::ProjectLayout;
use super::members::WorkspaceMembers;

use sha2::{Digest, Sha256};

use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fs::{self, File, Metadata};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// ============================================================================
// FINGERPRINT FORMAT
// ============================================================================

/// Algorithm used for metadata content fingerprints.
///
/// The identifier is part of the public fingerprint format contract.
pub const FINGERPRINT_ALGORITHM: &str = "sha256";

/// Version of the workspace metadata fingerprint format.
///
/// Increment this when the canonical fingerprint serialization changes
/// incompatibly.
///
/// This version is independent of the Zamani language, compiler,
/// manifest and package-format versions.
pub const FINGERPRINT_FORMAT_VERSION: u32 = 1;

// ============================================================================
// FILE ROLES
// ============================================================================

/// Semantic role of a file included in a metadata fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetadataFileRole {
    /// Explicit workspace manifest.
    WorkspaceManifest,

    /// Individual project manifest.
    ProjectManifest,

    /// Zamani source file.
    Source,
}

impl MetadataFileRole {
    /// Return the stable textual role identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceManifest => "workspace-manifest",
            Self::ProjectManifest => "project-manifest",
            Self::Source => "source",
        }
    }
}

// ============================================================================
// FILE METADATA
// ============================================================================

/// Content fingerprint and identity of one inspected file.
///
/// `content_fingerprint` represents the file's bytes and semantic role.
/// It deliberately excludes its filesystem path.
///
/// `relative_path` identifies its location within the owning workspace.
///
/// Paths use `/` as their portable separator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataFile {
    relative_path: String,
    role: MetadataFileRole,
    size_bytes: u64,
    content_fingerprint: String,
}

impl MetadataFile {
    fn new(
        relative_path: String,
        role: MetadataFileRole,
        size_bytes: u64,
        content_fingerprint: String,
    ) -> Self {
        Self {
            relative_path,
            role,
            size_bytes,
            content_fingerprint,
        }
    }

    /// Return the normalized workspace-relative path.
    #[must_use]
    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    /// Return the semantic file role.
    #[must_use]
    pub const fn role(&self) -> MetadataFileRole {
        self.role
    }

    /// Return the number of bytes read.
    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Return the SHA-256 content fingerprint.
    #[must_use]
    pub fn content_fingerprint(&self) -> &str {
        &self.content_fingerprint
    }
}

// ============================================================================
// PROJECT METADATA
// ============================================================================

/// Immutable metadata snapshot for one workspace project.
///
/// A project can have a successfully inspected empty source inventory.
/// That is different from a source inventory that has not been collected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceProjectMetadata {
    project_id: ProjectId,
    relative_root: String,
    manifest: MetadataFile,
    source_files: Vec<MetadataFile>,
    source_inventory_complete: bool,
    content_fingerprint: String,
    project_fingerprint: String,
}

impl WorkspaceProjectMetadata {
    fn new(
        project_id: ProjectId,
        relative_root: String,
        manifest: MetadataFile,
        source_files: Vec<MetadataFile>,
        source_inventory_complete: bool,
        content_fingerprint: String,
        project_fingerprint: String,
    ) -> Self {
        Self {
            project_id,
            relative_root,
            manifest,
            source_files,
            source_inventory_complete,
            content_fingerprint,
            project_fingerprint,
        }
    }

    /// Return the authoritative project identity.
    #[must_use]
    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Return the workspace-relative project root.
    #[must_use]
    pub fn relative_root(&self) -> &str {
        &self.relative_root
    }

    /// Return the project manifest metadata.
    #[must_use]
    pub fn manifest(&self) -> &MetadataFile {
        &self.manifest
    }

    /// Borrow the deterministically ordered source-file metadata.
    #[must_use]
    pub fn source_files(&self) -> &[MetadataFile] {
        &self.source_files
    }

    /// Return whether the source inventory was actually collected.
    #[must_use]
    pub const fn source_inventory_complete(&self) -> bool {
        self.source_inventory_complete
    }

    /// Return the project content fingerprint.
    ///
    /// This fingerprint excludes project-relative paths.
    #[must_use]
    pub fn content_fingerprint(&self) -> &str {
        &self.content_fingerprint
    }

    /// Return the project fingerprint.
    ///
    /// This fingerprint includes the project-relative path and file
    /// identities in addition to the content fingerprint.
    #[must_use]
    pub fn project_fingerprint(&self) -> &str {
        &self.project_fingerprint
    }
}

// ============================================================================
// WORKSPACE METADATA
// ============================================================================

/// Deterministic snapshot of an inspected Zamani workspace.
///
/// This structure is immutable after construction.
///
/// It does not monitor subsequent filesystem changes. Construct a new
/// snapshot whenever updated workspace metadata is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMetadata {
    workspace_id: WorkspaceId,
    workspace_root: PathBuf,
    workspace_manifest: MetadataFile,
    projects: Vec<WorkspaceProjectMetadata>,
    source_inventory_complete: bool,
    content_fingerprint: String,
    workspace_fingerprint: String,
}

impl WorkspaceMetadata {
    /// Collect metadata for a resolved workspace.
    ///
    /// `members` must be a previously validated `WorkspaceMembers`
    /// collection.
    ///
    /// `workspace_manifest` must identify the actual workspace manifest.
    /// It is supplied explicitly because discovery permits configurable
    /// manifest filenames.
    ///
    /// `layouts` contains one inspected `ProjectLayout` for each resolved
    /// workspace member.
    ///
    /// Every member must have exactly one corresponding layout.
    ///
    /// Project layouts must have been inspected from the same canonical
    /// project roots as their corresponding workspace members.
    ///
    /// The workspace manifest and project manifests are read as bytes.
    /// Manifest interpretation remains the responsibility of the
    /// canonical manifest subsystem.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The workspace manifest is missing or unreadable.
    /// - A project manifest is missing or unreadable.
    /// - A project layout is missing or duplicated.
    /// - A layout belongs to a different project root.
    /// - A source file cannot be read.
    /// - A source file changes during fingerprint collection.
    /// - A source path cannot be represented portably.
    /// - Two projects contain the same canonical source file.
    /// - A source file escapes its project root.
    /// - A filesystem operation fails.
    pub fn collect(
        members: &WorkspaceMembers,
        workspace_manifest: impl AsRef<Path>,
        layouts: &[(ProjectId, ProjectLayout)],
    ) -> Result<Self> {
        let workspace_root = fs::canonicalize(
            members.workspace_root(),
        )
        .map_err(|error| {
            filesystem_error(
                "workspace.metadata.canonicalize_workspace_root",
                members.workspace_root(),
                error,
            )
        })?;

        let workspace_manifest_path = canonical_regular_file(
            workspace_manifest.as_ref(),
            "workspace.metadata.workspace_manifest",
        )?;

        ensure_contained(
            &workspace_root,
            &workspace_manifest_path,
            "workspace.metadata.validate_workspace_manifest",
        )?;

        let workspace_manifest_metadata = collect_file(
            &workspace_root,
            &workspace_manifest_path,
            MetadataFileRole::WorkspaceManifest,
        )?;

        // Build an identity index for layouts. Duplicate project identities
        // are rejected rather than silently overwritten.
        let mut layout_index: BTreeMap<
            ProjectId,
            &ProjectLayout,
        > = BTreeMap::new();

        for (project_id, layout) in layouts {
            if layout_index.insert(
                project_id.clone(),
                layout,
            ).is_some() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        "multiple project layouts were supplied for one project identity",
                    )
                    .with_operation("workspace.metadata.collect")
                    .with_context(project_id.to_string()),
                );
            }
        }

        let mut projects = Vec::new();

        let mut all_sources: BTreeMap<
            PathBuf,
            ProjectId,
        > = BTreeMap::new();

        let mut workspace_content_hasher = Sha256::new();
        let mut workspace_hasher = Sha256::new();

        hash_header(
            &mut workspace_content_hasher,
            b"zamani-workspace-content",
        );

        hash_header(
            &mut workspace_hasher,
            b"zamani-workspace-metadata",
        );

        hash_field(
            &mut workspace_content_hasher,
            b"fingerprint-algorithm",
            FINGERPRINT_ALGORITHM.as_bytes(),
        );

        hash_field(
            &mut workspace_hasher,
            b"fingerprint-algorithm",
            FINGERPRINT_ALGORITHM.as_bytes(),
        );

        hash_u64(
            &mut workspace_hasher,
            b"fingerprint-format-version",
            u64::from(FINGERPRINT_FORMAT_VERSION),
        );

        hash_file(
            &mut workspace_content_hasher,
            &workspace_manifest_metadata,
        );

        hash_file(
            &mut workspace_hasher,
            &workspace_manifest_metadata,
        );

        for member in members.iter() {
            let project_id = member.project_id();

            let layout = layout_index
                .remove(project_id)
                .ok_or_else(|| {
                    ToolchainError::new(
                        ToolchainErrorKind::NotFound,
                        "a resolved workspace member has no corresponding inspected project layout",
                    )
                    .with_operation("workspace.metadata.collect")
                    .with_context(project_id.to_string())
                })?;

            let canonical_member_root = fs::canonicalize(
                member.canonical_root(),
            )
            .map_err(|error| {
                filesystem_error(
                    "workspace.metadata.canonicalize_project_root",
                    member.canonical_root(),
                    error,
                )
            })?;

            if canonical_member_root != layout.project_root() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "project layout root does not match its resolved workspace member",
                    )
                    .with_operation("workspace.metadata.collect")
                    .with_context(project_id.to_string()),
                );
            }

            ensure_contained(
                &workspace_root,
                &canonical_member_root,
                "workspace.metadata.validate_project_containment",
            )?;

            let relative_root = portable_relative_path(
                &workspace_root,
                &canonical_member_root,
            )?;

            let manifest_path = canonical_regular_file(
                member.manifest_path(),
                "workspace.metadata.project_manifest",
            )?;

            ensure_contained(
                &canonical_member_root,
                &manifest_path,
                "workspace.metadata.validate_project_manifest",
            )?;

            let manifest = collect_file(
                &workspace_root,
                &manifest_path,
                MetadataFileRole::ProjectManifest,
            )?;

            let mut visited_sources = BTreeSet::new();

            // ProjectLayout owns traversal. This module owns metadata
            // collection and de-duplicates repeated callbacks from
            // overlapping layout traversal roots.
            layout.visit_source_files(|source_path| {
                let canonical_source = fs::canonicalize(
                    source_path,
                )
                .map_err(|error| {
                    filesystem_error(
                        "workspace.metadata.canonicalize_source",
                        source_path,
                        error,
                    )
                })?;

                ensure_contained(
                    &canonical_member_root,
                    &canonical_source,
                    "workspace.metadata.validate_source_containment",
                )?;

                if !visited_sources.insert(
                    canonical_source.clone(),
                ) {
                    return Ok(());
                }

                if let Some(existing_project) = all_sources.get(
                    &canonical_source,
                ) {
                    return Err(
                        ToolchainError::new(
                            ToolchainErrorKind::ValidationFailed,
                            "one canonical source file is included in multiple project inventories",
                        )
                        .with_operation("workspace.metadata.collect")
                        .with_context(
                            format!(
                                "source: {}; first project: {}; second project: {}",
                                canonical_source.display(),
                                existing_project,
                                project_id,
                            ),
                        ),
                    );
                }

                all_sources.insert(
                    canonical_source.clone(),
                    project_id.clone(),
                );

                Ok(())
            })?;

            // Collect file metadata after traversal has completed.
            // The set guarantees deterministic ordering and prevents
            // duplicate hashing within one project.
            let mut source_files = Vec::with_capacity(
                visited_sources.len(),
            );

            for source_path in visited_sources {
                source_files.push(collect_file(
                    &workspace_root,
                    &source_path,
                    MetadataFileRole::Source,
                )?);
            }

            source_files.sort_by(|left, right| {
                left.relative_path.cmp(&right.relative_path)
            });

            let source_inventory_complete = true;

            let content_fingerprint = fingerprint_project_content(
                &manifest,
                &source_files,
                source_inventory_complete,
            );

            let project_fingerprint = fingerprint_project(
                project_id,
                &relative_root,
                &manifest,
                &source_files,
                source_inventory_complete,
                &content_fingerprint,
            );

            hash_field(
                &mut workspace_content_hasher,
                b"project-content-fingerprint",
                content_fingerprint.as_bytes(),
            );

            hash_field(
                &mut workspace_hasher,
                b"project-id",
                project_id.to_string().as_bytes(),
            );

            hash_field(
                &mut workspace_hasher,
                b"project-relative-root",
                relative_root.as_bytes(),
            );

            hash_field(
                &mut workspace_hasher,
                b"project-fingerprint",
                project_fingerprint.as_bytes(),
            );

            hash_file(
                &mut workspace_content_hasher,
                &manifest,
            );

            for source in &source_files {
                hash_file(
                    &mut workspace_content_hasher,
                    source,
                );

                hash_file(
                    &mut workspace_hasher,
                    source,
                );
            }

            hash_file(
                &mut workspace_hasher,
                &manifest,
            );

            projects.push(
                WorkspaceProjectMetadata::new(
                    project_id.clone(),
                    relative_root,
                    manifest,
                    source_files,
                    source_inventory_complete,
                    content_fingerprint,
                    project_fingerprint,
                ),
            );
        }

        if !layout_index.is_empty() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    "project layouts were supplied for projects that are not resolved workspace members",
                )
                .with_operation("workspace.metadata.collect")
                .with_context(format!(
                    "unmatched layout count: {}",
                    layout_index.len(),
                )),
            );
        }

        projects.sort_by(|left, right| {
            left.relative_root.cmp(&right.relative_root)
        });

        let source_inventory_complete = projects
            .iter()
            .all(
                WorkspaceProjectMetadata::source_inventory_complete,
            );

        hash_field(
            &mut workspace_hasher,
            b"source-inventory-complete",
            if source_inventory_complete {
                b"true"
            } else {
                b"false"
            },
        );

        hash_field(
            &mut workspace_content_hasher,
            b"source-inventory-complete",
            if source_inventory_complete {
                b"true"
            } else {
                b"false"
            },
        );

        let content_fingerprint = finalize_hash(
            workspace_content_hasher,
        );

        hash_field(
            &mut workspace_hasher,
            b"workspace-content-fingerprint",
            content_fingerprint.as_bytes(),
        );

        let workspace_fingerprint = finalize_hash(
            workspace_hasher,
        );

        Ok(Self {
            workspace_id: members.workspace_id().clone(),
            workspace_root,
            workspace_manifest: workspace_manifest_metadata,
            projects,
            source_inventory_complete,
            content_fingerprint,
            workspace_fingerprint,
        })
    }

    /// Return the workspace identity.
    #[must_use]
    pub fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    /// Return the canonical workspace root.
    #[must_use]
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Return the workspace manifest metadata.
    #[must_use]
    pub fn workspace_manifest(&self) -> &MetadataFile {
        &self.workspace_manifest
    }

    /// Borrow the deterministically ordered project metadata.
    #[must_use]
    pub fn projects(&self) -> &[WorkspaceProjectMetadata] {
        &self.projects
    }

    /// Return the number of represented projects.
    #[must_use]
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    /// Return whether all project source inventories were collected.
    #[must_use]
    pub const fn source_inventory_complete(&self) -> bool {
        self.source_inventory_complete
    }

    /// Return the workspace content fingerprint.
    ///
    /// This fingerprint identifies manifest and source contents without
    /// using project-relative filesystem paths as content.
    #[must_use]
    pub fn content_fingerprint(&self) -> &str {
        &self.content_fingerprint
    }

    /// Return the complete workspace metadata fingerprint.
    #[must_use]
    pub fn workspace_fingerprint(&self) -> &str {
        &self.workspace_fingerprint
    }

    /// Find metadata for a project using its canonical identity.
    #[must_use]
    pub fn project(
        &self,
        project_id: &ProjectId,
    ) -> Option<&WorkspaceProjectMetadata> {
        self.projects
            .iter()
            .find(|project| project.project_id() == project_id)
    }
}

// ============================================================================
// FILE COLLECTION
// ============================================================================

fn canonical_regular_file(
    path: &Path,
    operation: &'static str,
) -> Result<PathBuf> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| filesystem_error(
            operation,
            path,
            error,
        ))?;

    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "metadata input must be a regular non-symbolic-link file",
            )
            .with_operation(operation)
            .with_context(path.display().to_string()),
        );
    }

    let canonical = fs::canonicalize(path)
        .map_err(|error| filesystem_error(
            operation,
            path,
            error,
        ))?;

    Ok(canonical)
}

fn collect_file(
    workspace_root: &Path,
    file_path: &Path,
    role: MetadataFileRole,
) -> Result<MetadataFile> {
    let canonical_path = fs::canonicalize(file_path)
        .map_err(|error| filesystem_error(
            "workspace.metadata.canonicalize_file",
            file_path,
            error,
        ))?;

    ensure_contained(
        workspace_root,
        &canonical_path,
        "workspace.metadata.validate_file_containment",
    )?;

    let relative_path = portable_relative_path(
        workspace_root,
        &canonical_path,
    )?;

    let mut file = File::open(&canonical_path)
        .map_err(|error| filesystem_error(
            "workspace.metadata.open_file",
            &canonical_path,
            error,
        ))?;

    let before = file.metadata()
        .map_err(|error| filesystem_error(
            "workspace.metadata.inspect_file_before_read",
            &canonical_path,
            error,
        ))?;

    if !before.is_file() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "metadata input is not a regular file",
            )
            .with_operation("workspace.metadata.collect_file")
            .with_context(canonical_path.display().to_string()),
        );
    }

    let (content_fingerprint, size_bytes) = hash_reader(
        &mut file,
        role,
    )
    .map_err(|error| filesystem_error(
        "workspace.metadata.hash_file",
        &canonical_path,
        error,
    ))?;

    let after = file.metadata()
        .map_err(|error| filesystem_error(
            "workspace.metadata.inspect_file_after_read",
            &canonical_path,
            error,
        ))?;

    if file_metadata_changed(
        &before,
        &after,
        size_bytes,
    ) {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "file changed while its metadata fingerprint was being collected",
            )
            .with_operation("workspace.metadata.collect_file")
            .with_context(canonical_path.display().to_string()),
        );
    }

    Ok(MetadataFile::new(
        relative_path,
        role,
        size_bytes,
        content_fingerprint,
    ))
}

fn file_metadata_changed(
    before: &Metadata,
    after: &Metadata,
    bytes_read: u64,
) -> bool {
    if before.len() != after.len()
        || after.len() != bytes_read
    {
        return true;
    }

    match (
        before.modified(),
        after.modified(),
    ) {
        (Ok(before_time), Ok(after_time)) => {
            before_time != after_time
        }

        _ => false,
    }
}

fn hash_reader<R: Read>(
    reader: &mut R,
    role: MetadataFileRole,
) -> io::Result<(String, u64)> {
    let mut hasher = Sha256::new();

    hash_header(
        &mut hasher,
        b"zamani-file-content",
    );

    hash_field(
        &mut hasher,
        b"role",
        role.as_str().as_bytes(),
    );

    let mut total_bytes = 0_u64;

    // Streaming buffer size is an implementation detail, not a maximum
    // permitted source-file size.
    let mut buffer = [0_u8; 65_536];

    loop {
        let read = reader.read(&mut buffer)?;

        if read == 0 {
            break;
        }

        total_bytes = total_bytes
            .checked_add(
                u64::try_from(read).map_err(|_| {
                    io::Error::other(
                        "read size cannot be represented as u64",
                    )
                })?,
            )
            .ok_or_else(|| {
                io::Error::other(
                    "file byte count exceeded u64 representation",
                )
            })?;

        hasher.update(&buffer[..read]);
    }

    hash_u64(
        &mut hasher,
        b"file-size",
        total_bytes,
    );

    Ok((
        finalize_hash(hasher),
        total_bytes,
    ))
}

// ============================================================================
// CONTENT FINGERPRINTS
// ============================================================================

fn fingerprint_project_content(
    manifest: &MetadataFile,
    source_files: &[MetadataFile],
    source_inventory_complete: bool,
) -> String {
    let mut hasher = Sha256::new();

    hash_header(
        &mut hasher,
        b"zamani-project-content",
    );

    hash_file(
        &mut hasher,
        manifest,
    );

    for source in source_files {
        hash_file(
            &mut hasher,
            source,
        );
    }

    hash_field(
        &mut hasher,
        b"source-inventory-complete",
        if source_inventory_complete {
            b"true"
        } else {
            b"false"
        },
    );

    finalize_hash(hasher)
}

fn fingerprint_project(
    project_id: &ProjectId,
    relative_root: &str,
    manifest: &MetadataFile,
    source_files: &[MetadataFile],
    source_inventory_complete: bool,
    content_fingerprint: &str,
) -> String {
    let mut hasher = Sha256::new();

    hash_header(
        &mut hasher,
        b"zamani-project-metadata",
    );

    hash_u64(
        &mut hasher,
        b"fingerprint-format-version",
        u64::from(FINGERPRINT_FORMAT_VERSION),
    );

    hash_field(
        &mut hasher,
        b"project-id",
        project_id.to_string().as_bytes(),
    );

    hash_field(
        &mut hasher,
        b"project-relative-root",
        relative_root.as_bytes(),
    );

    hash_field(
        &mut hasher,
        b"project-content-fingerprint",
        content_fingerprint.as_bytes(),
    );

    hash_field(
        &mut hasher,
        b"manifest-relative-path",
        manifest.relative_path().as_bytes(),
    );

    for source in source_files {
        hash_field(
            &mut hasher,
            b"source-relative-path",
            source.relative_path().as_bytes(),
        );

        hash_field(
            &mut hasher,
            b"source-content-fingerprint",
            source.content_fingerprint().as_bytes(),
        );
    }

    hash_field(
        &mut hasher,
        b"source-inventory-complete",
        if source_inventory_complete {
            b"true"
        } else {
            b"false"
        },
    );

    finalize_hash(hasher)
}

fn hash_file(
    hasher: &mut Sha256,
    file: &MetadataFile,
) {
    hash_field(
        hasher,
        b"file-role",
        file.role().as_str().as_bytes(),
    );

    hash_field(
        hasher,
        b"file-content-fingerprint",
        file.content_fingerprint().as_bytes(),
    );

    hash_u64(
        hasher,
        b"file-size",
        file.size_bytes(),
    );
}

// ============================================================================
// CANONICAL HASH ENCODING
// ============================================================================

fn hash_header(
    hasher: &mut Sha256,
    domain: &[u8],
) {
    hasher.update(b"ZAMANI-METADATA");
    hash_field(
        hasher,
        b"domain",
        domain,
    );
}

fn hash_field(
    hasher: &mut Sha256,
    name: &[u8],
    value: &[u8],
) {
    hasher.update(
        u64::try_from(name.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );

    hasher.update(name);

    hasher.update(
        u64::try_from(value.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );

    hasher.update(value);
}

fn hash_u64(
    hasher: &mut Sha256,
    name: &[u8],
    value: u64,
) {
    hash_field(
        hasher,
        name,
        &value.to_be_bytes(),
    );
}

fn finalize_hash(
    hasher: Sha256,
) -> String {
    let digest = hasher.finalize();

    let mut output = String::with_capacity(
        digest.len() * 2,
    );

    for byte in digest {
        use std::fmt::Write;

        // Writing to String is infallible.
        let _ = write!(
            output,
            "{byte:02x}",
        );
    }

    output
}

// ============================================================================
// PATH CONTRACTS
// ============================================================================

fn portable_relative_path(
    root: &Path,
    path: &Path,
) -> Result<String> {
    let relative = path.strip_prefix(root)
        .map_err(|_| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "path is not contained within its owning root",
            )
            .with_operation("workspace.metadata.relative_path")
            .with_context(path.display().to_string())
        })?;

    let mut components = Vec::new();

    for component in relative.components() {
        match component {
            std::path::Component::Normal(value) => {
                let text = value.to_str()
                    .ok_or_else(|| {
                        ToolchainError::new(
                            ToolchainErrorKind::ValidationFailed,
                            "path contains a non-UTF-8 component and cannot be represented by the portable metadata format",
                        )
                        .with_operation(
                            "workspace.metadata.relative_path",
                        )
                    })?;

                components.push(text);
            }

            std::path::Component::CurDir => {}

            _ => {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "relative metadata path contains an invalid path component",
                    )
                    .with_operation(
                        "workspace.metadata.relative_path",
                    )
                    .with_context(path.display().to_string()),
                );
            }
        }
    }

    Ok(components.join("/"))
}

fn ensure_contained(
    root: &Path,
    path: &Path,
    operation: &'static str,
) -> Result<()> {
    if !path.starts_with(root) {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "metadata path escapes its owning directory",
            )
            .with_operation(operation)
            .with_context(format!(
                "root: {}; path: {}",
                root.display(),
                path.display(),
            )),
        );
    }

    Ok(())
}

// ============================================================================
// FILESYSTEM ERRORS
// ============================================================================

fn filesystem_error(
    operation: &'static str,
    path: &Path,
    source: io::Error,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Io,
        "workspace metadata filesystem operation failed",
    )
    .with_operation(operation)
    .with_context(path.display().to_string())
    .with_context(source.to_string())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use sha2::{Digest, Sha256};

    use std::io::Cursor;

    #[test]
    fn fingerprint_algorithm_is_stable() {
        assert_eq!(
            FINGERPRINT_ALGORITHM,
            "sha256",
        );

        assert_eq!(
            FINGERPRINT_FORMAT_VERSION,
            1,
        );
    }

    #[test]
    fn file_roles_have_stable_identifiers() {
        assert_eq!(
            MetadataFileRole::WorkspaceManifest.as_str(),
            "workspace-manifest",
        );

        assert_eq!(
            MetadataFileRole::ProjectManifest.as_str(),
            "project-manifest",
        );

        assert_eq!(
            MetadataFileRole::Source.as_str(),
            "source",
        );
    }

    #[test]
    fn identical_content_and_role_produce_identical_fingerprints() {
        let content = b"fn main() {}";

        let mut first = Cursor::new(content);
        let mut second = Cursor::new(content);

        let first = hash_reader(
            &mut first,
            MetadataFileRole::Source,
        )
        .expect("first hash");

        let second = hash_reader(
            &mut second,
            MetadataFileRole::Source,
        )
        .expect("second hash");

        assert_eq!(first, second);
    }

    #[test]
    fn different_file_roles_produce_different_fingerprints() {
        let content = b"same bytes";

        let mut source = Cursor::new(content);
        let mut manifest = Cursor::new(content);

        let source = hash_reader(
            &mut source,
            MetadataFileRole::Source,
        )
        .expect("source hash");

        let manifest = hash_reader(
            &mut manifest,
            MetadataFileRole::ProjectManifest,
        )
        .expect("manifest hash");

        assert_ne!(source.0, manifest.0);
    }

    #[test]
    fn empty_files_are_valid_and_have_zero_size() {
        let mut reader = Cursor::new([]);

        let (fingerprint, size) = hash_reader(
            &mut reader,
            MetadataFileRole::Source,
        )
        .expect("empty file hash");

        assert_eq!(size, 0);
        assert_eq!(fingerprint.len(), 64);
    }

    #[test]
    fn streaming_hash_matches_expected_sha256_encoding() {
        let content = b"Zamani";

        let mut reader = Cursor::new(content);

        let (actual, size) = hash_reader(
            &mut reader,
            MetadataFileRole::Source,
        )
        .expect("hash");

        let mut expected = Sha256::new();

        hash_header(
            &mut expected,
            b"zamani-file-content",
        );

        hash_field(
            &mut expected,
            b"role",
            b"source",
        );

        expected.update(content);

        hash_u64(
            &mut expected,
            b"file-size",
            content.len() as u64,
        );

        assert_eq!(
            actual,
            finalize_hash(expected),
        );

        assert_eq!(
            size,
            content.len() as u64,
        );
    }

    #[test]
    fn portable_relative_paths_use_forward_slashes() {
        let root = PathBuf::from("/workspace");
        let path = root.join("applications").join("example");

        let relative = portable_relative_path(
            &root,
            &path,
        )
        .expect("portable path");

        assert_eq!(
            relative,
            "applications/example",
        );
    }

    #[test]
    fn portable_relative_paths_reject_outside_paths() {
        let root = PathBuf::from("/workspace");
        let path = PathBuf::from("/outside/project");

        assert!(
            portable_relative_path(
                &root,
                &path,
            )
            .is_err()
        );
    }

    #[test]
    fn hash_fields_are_domain_separated() {
        let mut first = Sha256::new();
        let mut second = Sha256::new();

        hash_header(
            &mut first,
            b"first-domain",
        );

        hash_header(
            &mut second,
            b"second-domain",
        );

        assert_ne!(
            finalize_hash(first),
            finalize_hash(second),
        );
    }

    #[test]
    fn path_is_not_part_of_content_fingerprint() {
        let first = MetadataFile::new(
            "src/main.zm".to_owned(),
            MetadataFileRole::Source,
            12,
            "same-content".to_owned(),
        );

        let second = MetadataFile::new(
            "application/main.zm".to_owned(),
            MetadataFileRole::Source,
            12,
            "same-content".to_owned(),
        );

        assert_eq!(
            first.content_fingerprint(),
            second.content_fingerprint(),
        );

        assert_ne!(
            first.relative_path(),
            second.relative_path(),
        );
    }

    #[test]
    fn metadata_change_detection_checks_size() {
        // A metadata change is detected whenever the bytes read do not
        // match the file's final reported size.
        let changed = file_metadata_changed(
            &fake_metadata(10),
            &fake_metadata(12),
            10,
        );

        assert!(changed);
    }

    fn fake_metadata(_size: u64) -> Metadata {
        // This helper is intentionally not used to construct synthetic
        // operating-system Metadata values. Metadata is obtained from
        // actual files in integration tests.
        //
        // Kept as a compile-time reminder that filesystem Metadata is
        // not publicly constructible.
        unreachable!(
            "use real filesystem metadata in integration tests"
        )
    }
}
