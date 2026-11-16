
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Workspace Members
//!
//! Path: `src/toolchain/workspace/members.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Resolves normalized workspace-member declarations into validated,
//! canonical project-member records.
//!
//! This module is deliberately independent of the concrete manifest schema.
//! The canonical manifest subsystem parses workspace declarations and the
//! toolchain manifest adapter converts them into
//! [`WorkspaceMemberDeclaration`] values.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Workspace-member declaration validation.
//! - Resolution of declared member directories.
//! - Member package-manifest verification.
//! - Canonical member-path containment checks.
//! - Duplicate declaration and identity detection.
//! - Deterministic workspace-member ordering.
//! - The resolved workspace-members collection.
//!
//! This module does not own:
//!
//! - Manifest parsing or workspace schema.
//! - Workspace-root discovery.
//! - Project or workspace identity allocation.
//! - Dependency resolution.
//! - Build or compiler dependency graphs.
//! - Source-layout inspection.
//! - Language parsing or semantic analysis.
//! - Target selection or hardware discovery.
//! - Physical resource allocation.
//! - Runtime or distributed scheduling.
//!
//! ## Integration
//!
//! - `super::discovery` supplies the canonical workspace root and manifest
//!   discovery contract.
//! - `super::layout` may inspect the layout of each resolved member.
//! - `super::metadata` consumes the resolved member collection.
//! - `super::validation` consumes the collection for workspace-level checks.
//! - `crate::zamani_project_config` owns the canonical workspace manifest
//!   schema and parsing.
//! - `crate::toolchain::manifest::adapter` converts parsed manifest member
//!   declarations into this module's normalized declaration type.
//! - `crate::toolchain::ids` owns `ProjectId` and `WorkspaceId`.
//! - `crate::toolchain::errors` and `crate::toolchain::result` own the
//!   canonical toolchain error and result contracts.
//!
//! ## Path and membership semantics
//!
//! Member paths are relative to the canonical workspace root.
//!
//! A declaration identifies one literal directory. Glob expansion,
//! environment-variable expansion, shell expansion, and implicit directory
//! discovery are intentionally not performed here.
//!
//! The path `.` explicitly identifies the workspace root itself. It is
//! accepted only as an explicit declaration; the root package is never
//! silently inserted.
//!
//! Each declared member must:
//!
//! - Resolve to an existing directory.
//! - Remain inside the canonical workspace root.
//! - Contain the configured manifest file as a regular, non-symlink file.
//! - Be recognized by workspace discovery as a package project.
//!
//! Nested workspaces are not recursively merged. A declared package remains
//! one member; its own workspace membership is a separate relationship.
//!
//! ## Determinism
//!
//! Declarations are sorted by their literal relative paths before resolution.
//! Resolved members are stored in that order.
//!
//! Duplicate literal paths, canonical directory aliases, and duplicate
//! `ProjectId` values are errors rather than silently deduplicated.
//!
//! The caller-supplied identity resolver must itself provide stable identity
//! semantics. This module does not pretend that an in-memory counter provides
//! persistent or globally coordinated identities.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No shell or process execution.
//! - No manifest content execution.
//! - No recursive traversal of arbitrary workspace contents.
//! - No absolute member paths.
//! - No parent-directory traversal.
//! - No symlinked member path components.
//! - No symlinked member manifest.
//! - Canonical containment is checked after path resolution.
//! - Filesystem errors are not converted into empty membership results.
//!
//! Standard portable filesystem APIs cannot eliminate every time-of-check /
//! time-of-use race against a hostile process concurrently replacing path
//! components. Callers operating on adversarial shared filesystems must also
//! enforce suitable filesystem permissions or use an OS-specific,
//! handle-relative filesystem boundary.
//!
//! ## Scalability
//!
//! No arbitrary maximum member count, path depth, project size, source-file
//! count, domain count, target count, or hardware-resource count is imposed.
//!
//! Resolution stores one record and identity/path index entry per member.
//! Its memory use is therefore proportional to the number and total path
//! representation of declared members, subject to available resources.
//!
//! This is not a promise of literal infinite storage or execution.
//!
//! ## Freeze boundary
//!
//! The normalized declaration type and resolution contract are intended to
//! remain stable when the canonical manifest schema is expanded. The adapter
//! may evolve independently as long as it supplies literal member paths and
//! does not change their meaning.

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::ids::{
    ProjectId,
    WorkspaceId,
};
use crate::toolchain::result::Result;

use super::discovery::{
    discover_from_manifest_with_options,
    DiscoveryOptions,
    DiscoveryResult,
};

use std::collections::BTreeSet;
use std::fs;
use std::path::{
    Component,
    Path,
    PathBuf,
};

// ============================================================================
// DEFAULT MANIFEST CONVENTION
// ============================================================================

/// Conventional Zamani project manifest filename.
///
/// This is a configurable project-format convention, not a language or
/// computational-domain restriction.
pub const DEFAULT_MEMBER_MANIFEST_FILE_NAME: &str = "Zamani.toml";

// ============================================================================
// MEMBER DECLARATION
// ============================================================================

/// A normalized declaration identifying one workspace member directory.
///
/// The declaration is intentionally independent of TOML, JSON, YAML, or
/// another manifest representation.
///
/// Construct declarations through [`WorkspaceMemberDeclaration::new`] so
/// malformed and escaping paths are rejected before resolution.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkspaceMemberDeclaration {
    relative_path: PathBuf,
}

impl WorkspaceMemberDeclaration {
    /// Construct a member declaration from a workspace-relative path.
    ///
    /// The special path `.` explicitly identifies the workspace root.
    ///
    /// All other paths must consist exclusively of normal relative path
    /// components. Absolute paths, parent traversal, root components,
    /// platform prefixes, and embedded current-directory components are
    /// rejected.
    ///
    /// # Errors
    ///
    /// Returns `InvalidInput` when the path is empty, absolute, or contains
    /// a forbidden component.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        validate_member_relative_path(path)?;

        Ok(Self {
            relative_path: path.to_path_buf(),
        })
    }

    /// Borrow the declared workspace-relative path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }
}

// ============================================================================
// RESOLUTION OPTIONS
// ============================================================================

/// Configuration for resolving workspace member declarations.
///
/// The manifest filename must match the filename used by workspace
/// discovery. The default is `Zamani.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberResolutionOptions {
    manifest_file_name: String,
}

impl Default for MemberResolutionOptions {
    fn default() -> Self {
        Self {
            manifest_file_name:
                DEFAULT_MEMBER_MANIFEST_FILE_NAME.to_owned(),
        }
    }
}

impl MemberResolutionOptions {
    /// Construct default member-resolution options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the manifest filename required in each member directory.
    ///
    /// The name must be one nonempty filename component. It cannot contain
    /// path separators, a NUL byte, or a special directory name.
    ///
    /// The selected name is passed to the existing discovery subsystem;
    /// this module does not implement another manifest parser.
    pub fn with_manifest_file_name(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self> {
        let name = name.into();

        validate_manifest_file_name(&name)?;

        self.manifest_file_name = name;

        Ok(self)
    }

    /// Borrow the configured manifest filename.
    #[must_use]
    pub fn manifest_file_name(&self) -> &str {
        &self.manifest_file_name
    }
}

fn validate_manifest_file_name(name: &str) -> Result<()> {
    let invalid = name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\\')
        || name.contains('\0');

    if invalid {
        return Err(
            ToolchainError::invalid_input(
                "member manifest filename must be one nonempty filename component",
            )
            .with_operation("workspace.members.validate_options"),
        );
    }

    Ok(())
}

// ============================================================================
// RESOLVED MEMBER
// ============================================================================

/// One validated package project belonging to a workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMember {
    project_id: ProjectId,
    relative_path: PathBuf,
    canonical_root: PathBuf,
    manifest_path: PathBuf,
}

impl WorkspaceMember {
    fn new(
        project_id: ProjectId,
        relative_path: PathBuf,
        canonical_root: PathBuf,
        manifest_path: PathBuf,
    ) -> Self {
        Self {
            project_id,
            relative_path,
            canonical_root,
            manifest_path,
        }
    }

    /// Return the persistent project identity supplied by the identity
    /// authority.
    #[must_use]
    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    /// Return the original workspace-relative declaration.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Return the canonical member project directory.
    #[must_use]
    pub fn canonical_root(&self) -> &Path {
        &self.canonical_root
    }

    /// Return the canonical package-manifest path.
    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }
}

// ============================================================================
// RESOLVED WORKSPACE MEMBERS
// ============================================================================

/// A validated, deterministically ordered set of workspace members.
///
/// This is a snapshot of the declarations resolved at construction time.
/// It does not watch the filesystem. Callers must resolve again when they
/// need to observe subsequent manifest or directory changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMembers {
    workspace_id: WorkspaceId,
    workspace_root: PathBuf,
    members: Vec<WorkspaceMember>,
}

impl WorkspaceMembers {
    /// Resolve normalized declarations against an explicitly discovered
    /// workspace.
    ///
    /// `identity_for_member` is the identity authority. It receives each
    /// canonical member directory and must return that project's stable
    /// `ProjectId`. A persistent project database or another coordinated
    /// identity service should back this callback when identities must
    /// survive process restarts or be shared across machines.
    ///
    /// The callback is invoked in deterministic relative-path order and only
    /// after the corresponding member directory and package manifest have
    /// passed filesystem validation.
    ///
    /// This function requires an explicit workspace manifest in
    /// `discovery`. A standalone project is not silently reinterpreted as a
    /// multi-member workspace.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - Discovery did not find an explicit workspace.
    /// - A declaration is invalid or duplicated.
    /// - A member directory is missing or is not a directory.
    /// - A path component or manifest is a symbolic link.
    /// - A canonical member path escapes the workspace root.
    /// - A member has no package manifest.
    /// - A manifest is malformed or is not a package manifest.
    /// - Two declarations resolve to the same canonical directory.
    /// - The identity authority returns duplicate project identities.
    /// - Filesystem access or identity resolution fails.
    pub fn resolve<I, F>(
        workspace_id: WorkspaceId,
        discovery: &DiscoveryResult,
        declarations: I,
        mut identity_for_member: F,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = WorkspaceMemberDeclaration>,
        F: FnMut(&Path) -> Result<ProjectId>,
    {
        Self::resolve_with_options(
            workspace_id,
            discovery,
            declarations,
            &MemberResolutionOptions::default(),
            identity_for_member,
        )
    }

    /// Resolve declarations with explicit member-resolution options.
    ///
    /// See [`WorkspaceMembers::resolve`] for the identity and error
    /// contracts.
    pub fn resolve_with_options<I, F>(
        workspace_id: WorkspaceId,
        discovery: &DiscoveryResult,
        declarations: I,
        options: &MemberResolutionOptions,
        mut identity_for_member: F,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = WorkspaceMemberDeclaration>,
        F: FnMut(&Path) -> Result<ProjectId>,
    {
        if !discovery.has_explicit_workspace() {
            return Err(
                ToolchainError::invalid_input(
                    "workspace member resolution requires an explicitly discovered workspace manifest",
                )
                .with_operation("workspace.members.resolve"),
            );
        }

        validate_manifest_file_name(options.manifest_file_name())?;

        let workspace_root = discovery
            .workspace_root()
            .ok_or_else(|| {
                ToolchainError::internal(
                    "discovery reports an explicit workspace but has no workspace root",
                )
                .with_operation("workspace.members.resolve"),
            })?
            .to_path_buf();

        let workspace_root = fs::canonicalize(&workspace_root)
            .map_err(|error| {
                filesystem_error(
                    "workspace.members.canonicalize_workspace_root",
                    &workspace_root,
                    error,
                )
            })?;

        let root_metadata = fs::symlink_metadata(&workspace_root)
            .map_err(|error| {
                filesystem_error(
                    "workspace.members.inspect_workspace_root",
                    &workspace_root,
                    error,
                )
            })?;

        if !root_metadata.is_dir() || root_metadata.file_type().is_symlink() {
            return Err(
                ToolchainError::invalid_input(
                    "the canonical workspace root is not a regular directory",
                )
                .with_operation("workspace.members.resolve")
                .with_context(workspace_root.display().to_string()),
            );
        }

        let mut declarations: Vec<WorkspaceMemberDeclaration> =
            declarations.into_iter().collect();

        // Stable ordering makes identity resolution and result construction
        // independent of the caller's input collection ordering.
        declarations.sort();

        let mut seen_declarations = BTreeSet::new();
        let mut seen_canonical_roots = BTreeSet::new();
        let mut seen_project_ids = BTreeSet::new();

        let mut members = Vec::new();

        let discovery_options = DiscoveryOptions::new()
            .with_manifest_file_name(
                options.manifest_file_name().to_owned(),
            )?;

        for declaration in declarations {
            let relative_path = declaration.relative_path;

            if !seen_declarations.insert(relative_path.clone()) {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        "workspace member path was declared more than once",
                    )
                    .with_operation("workspace.members.resolve")
                    .with_context(relative_path.display().to_string()),
                );
            }

            let candidate_root =
                resolve_member_directory(
                    &workspace_root,
                    &relative_path,
                )?;

            if !seen_canonical_roots.insert(candidate_root.clone()) {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        "multiple workspace member declarations resolve to the same canonical directory",
                    )
                    .with_operation("workspace.members.resolve")
                    .with_context(candidate_root.display().to_string()),
                );
            }

            let manifest_path = candidate_root.join(
                options.manifest_file_name(),
            );

            verify_regular_non_symlink_file(&manifest_path)?;

            let member_discovery =
                discover_from_manifest_with_options(
                    &manifest_path,
                    &discovery_options,
                )
                .map_err(|error| {
                    error
                        .with_operation(
                            "workspace.members.discover_member_manifest",
                        )
                        .with_context(
                            format!(
                                "member path: {}",
                                relative_path.display()
                            ),
                        )
                })?;

            let discovered_project_root =
                member_discovery.project_root().ok_or_else(|| {
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "workspace member manifest does not declare a package",
                    )
                    .with_operation("workspace.members.resolve")
                    .with_context(
                        manifest_path.display().to_string(),
                    )
                })?;

            if discovered_project_root != candidate_root {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "member manifest discovery resolved a different project root",
                    )
                    .with_operation("workspace.members.resolve")
                    .with_context(
                        format!(
                            "declared directory: {}; discovered directory: {}",
                            candidate_root.display(),
                            discovered_project_root.display(),
                        ),
                    ),
                );
            }

            let project_manifest = member_discovery
                .project_manifest()
                .ok_or_else(|| {
                    ToolchainError::internal(
                        "discovery returned a project root without a project manifest",
                    )
                    .with_operation("workspace.members.resolve")
                })?;

            if !project_manifest.kind().has_package() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "workspace member manifest is not package-bearing",
                    )
                    .with_operation("workspace.members.resolve")
                    .with_context(
                        project_manifest.path().display().to_string(),
                    ),
                );
            }

            // Recheck the manifest path after discovery. This detects many
            // ordinary replacement races, though portable std filesystem
            // APIs cannot provide a fully race-free hostile-filesystem
            // transaction.
            verify_regular_non_symlink_file(
                project_manifest.path(),
            )?;

            let project_id =
                identity_for_member(&candidate_root)
                    .map_err(|error| {
                        error
                            .with_operation(
                                "workspace.members.resolve_project_identity",
                            )
                            .with_context(
                                candidate_root.display().to_string(),
                            )
                    })?;

            if !seen_project_ids.insert(project_id.clone()) {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        "identity authority returned a project identity already assigned to another workspace member",
                    )
                    .with_operation("workspace.members.resolve")
                    .with_subject(project_id.to_string()),
                );
            }

            members.push(WorkspaceMember::new(
                project_id,
                relative_path,
                candidate_root,
                project_manifest.path().to_path_buf(),
            ));
        }

        Ok(Self {
            workspace_id,
            workspace_root,
            members,
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

    /// Borrow the resolved members in deterministic order.
    #[must_use]
    pub fn as_slice(&self) -> &[WorkspaceMember] {
        &self.members
    }

    /// Iterate over resolved members in deterministic order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &WorkspaceMember> {
        self.members.iter()
    }

    /// Return the number of resolved members.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Return whether the workspace has no declared members.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Find a member by its canonical project identity.
    #[must_use]
    pub fn get_by_project_id(
        &self,
        project_id: &ProjectId,
    ) -> Option<&WorkspaceMember> {
        self.members
            .iter()
            .find(|member| member.project_id() == project_id)
    }

    /// Find a member by its original workspace-relative declaration.
    #[must_use]
    pub fn get_by_relative_path(
        &self,
        relative_path: impl AsRef<Path>,
    ) -> Option<&WorkspaceMember> {
        self.members
            .iter()
            .find(|member| {
                member.relative_path() == relative_path.as_ref()
            })
    }

    /// Find a member by its canonical project directory.
    #[must_use]
    pub fn get_by_canonical_root(
        &self,
        canonical_root: impl AsRef<Path>,
    ) -> Option<&WorkspaceMember> {
        self.members
            .iter()
            .find(|member| {
                member.canonical_root() == canonical_root.as_ref()
            })
    }
}

// ============================================================================
// PATH VALIDATION
// ============================================================================

fn validate_member_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(
            ToolchainError::invalid_input(
                "workspace member path must be a nonempty relative path",
            )
            .with_operation("workspace.members.validate_path")
            .with_context(path.display().to_string()),
        );
    }

    // The workspace root is included only through an explicit "." entry.
    if path == Path::new(".") {
        return Ok(());
    }

    let mut saw_normal_component = false;

    for component in path.components() {
        match component {
            Component::Normal(_) => {
                saw_normal_component = true;
            }

            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return Err(
                    ToolchainError::invalid_input(
                        "workspace member path contains a forbidden path component",
                    )
                    .with_operation("workspace.members.validate_path")
                    .with_context(path.display().to_string()),
                );
            }
        }
    }

    if !saw_normal_component {
        return Err(
            ToolchainError::invalid_input(
                "workspace member path must identify a directory below the workspace root or explicitly use '.'",
            )
            .with_operation("workspace.members.validate_path")
            .with_context(path.display().to_string()),
        );
    }

    Ok(())
}

// ============================================================================
// MEMBER DIRECTORY RESOLUTION
// ============================================================================

fn resolve_member_directory(
    workspace_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf> {
    validate_member_relative_path(relative_path)?;

    let mut current = workspace_root.to_path_buf();

    if relative_path != Path::new(".") {
        for component in relative_path.components() {
            let Component::Normal(name) = component else {
                return Err(
                    ToolchainError::invalid_input(
                        "workspace member path contains a non-normal component",
                    )
                    .with_operation("workspace.members.resolve_path")
                    .with_context(relative_path.display().to_string()),
                );
            };

            current.push(name);

            let metadata = fs::symlink_metadata(&current)
                .map_err(|error| {
                    filesystem_error(
                        "workspace.members.inspect_member_component",
                        &current,
                        error,
                    )
                })?;

            if metadata.file_type().is_symlink() {
                return Err(
                    ToolchainError::validation_failed(
                        "symbolic links are not permitted in workspace member paths",
                    )
                    .with_operation("workspace.members.resolve_path")
                    .with_context(current.display().to_string()),
                );
            }

            if !metadata.is_dir() {
                return Err(
                    ToolchainError::validation_failed(
                        "workspace member path component is not a directory",
                    )
                    .with_operation("workspace.members.resolve_path")
                    .with_context(current.display().to_string()),
                );
            }
        }
    }

    let canonical_member_root =
        fs::canonicalize(&current).map_err(|error| {
            filesystem_error(
                "workspace.members.canonicalize_member",
                &current,
                error,
            )
        })?;

    if !canonical_member_root.starts_with(workspace_root) {
        return Err(
            ToolchainError::validation_failed(
                "canonical workspace member directory escapes the workspace root",
            )
            .with_operation("workspace.members.resolve_path")
            .with_context(
                canonical_member_root.display().to_string(),
            ),
        );
    }

    let metadata =
        fs::symlink_metadata(&canonical_member_root)
            .map_err(|error| {
                filesystem_error(
                    "workspace.members.inspect_canonical_member",
                    &canonical_member_root,
                    error,
                )
            })?;

    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(
            ToolchainError::validation_failed(
                "resolved workspace member is not a regular directory",
            )
            .with_operation("workspace.members.resolve_path")
            .with_context(
                canonical_member_root.display().to_string(),
            ),
        );
    }

    Ok(canonical_member_root)
}

// ============================================================================
// MANIFEST FILE VALIDATION
// ============================================================================

fn verify_regular_non_symlink_file(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| {
            filesystem_error(
                "workspace.members.inspect_member_manifest",
                path,
                error,
            )
        })?;

    if metadata.file_type().is_symlink() {
        return Err(
            ToolchainError::validation_failed(
                "workspace member manifest must not be a symbolic link",
            )
            .with_operation("workspace.members.validate_manifest")
            .with_context(path.display().to_string()),
        );
    }

    if !metadata.is_file() {
        return Err(
            ToolchainError::validation_failed(
                "workspace member manifest is not a regular file",
            )
            .with_operation("workspace.members.validate_manifest")
            .with_context(path.display().to_string()),
        );
    }

    Ok(())
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn filesystem_error(
    operation: &'static str,
    path: &Path,
    source: std::io::Error,
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

    use crate::toolchain::ids::{
        IdAllocator,
        IdNamespace,
    };

    use std::fs;
    use std::sync::atomic::{
        AtomicU64,
        Ordering,
    };
    use std::time::{
        SystemTime,
        UNIX_EPOCH,
    };

    static NEXT_TEST_DIRECTORY: AtomicU64 =
        AtomicU64::new(1);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock must be after the Unix epoch")
                .as_nanos();

            let sequence =
                NEXT_TEST_DIRECTORY.fetch_add(
                    1,
                    Ordering::Relaxed,
                );

            let path = std::env::temp_dir().join(
                format!(
                    "zamani-workspace-members-{}-{time}-{sequence}",
                    std::process::id(),
                ),
            );

            fs::create_dir_all(&path)
                .expect("test directory must be created");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write(
            &self,
            relative: impl AsRef<Path>,
            contents: &str,
        ) {
            let path = self.path.join(relative);

            fs::create_dir_all(
                path.parent().expect("file path has a parent"),
            )
            .expect("parent directory must be created");

            fs::write(path, contents)
                .expect("test file must be written");
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn namespace(value: &str) -> IdNamespace {
        IdNamespace::new(value).expect("valid test namespace")
    }

    fn workspace_discovery(root: &Path) -> DiscoveryResult {
        super::super::discovery::discover(root)
            .expect("workspace must be discovered")
    }

    fn workspace_id() -> WorkspaceId {
        WorkspaceId::new(
            namespace("zamani.test.workspace"),
            1,
        )
        .expect("valid workspace ID")
    }

    fn project_allocator() -> IdAllocator<ProjectId> {
        IdAllocator::new(namespace("zamani.test.project"))
    }

    fn package_manifest(name: &str) -> String {
        format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n"
        )
    }

    fn workspace_manifest(members: &[&str]) -> String {
        let values = members
            .iter()
            .map(|member| format!("\"{member}\""))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "[workspace]\nmembers = [{values}]\n"
        )
    }

    #[test]
    fn resolves_multiple_members_and_assigns_supplied_identities() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["apps/one", "libs/two"]),
        );

        directory.write(
            "apps/one/Zamani.toml",
            &package_manifest("one"),
        );

        directory.write(
            "libs/two/Zamani.toml",
            &package_manifest("two"),
        );

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let members = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("libs/two")
                    .expect("valid declaration"),
                WorkspaceMemberDeclaration::new("apps/one")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        )
        .expect("members must resolve");

        assert_eq!(members.len(), 2);

        assert_eq!(
            members.as_slice()[0].relative_path(),
            Path::new("apps/one"),
        );

        assert_eq!(
            members.as_slice()[1].relative_path(),
            Path::new("libs/two"),
        );

        assert_ne!(
            members.as_slice()[0].project_id(),
            members.as_slice()[1].project_id(),
        );
    }

    #[test]
    fn permits_explicit_workspace_root_package_member() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            "[package]\nname = \"root-package\"\nversion = \"0.1.0\"\n[workspace]\nmembers = [\".\"]\n",
        );

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let members = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new(".")
                    .expect("explicit root member is valid"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        )
        .expect("root package must resolve");

        assert_eq!(members.len(), 1);
        assert_eq!(
            members.as_slice()[0].canonical_root(),
            fs::canonicalize(directory.path())
                .expect("canonical test root"),
        );
    }

    #[test]
    fn empty_explicit_workspace_is_valid() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&[]),
        );

        let discovery = workspace_discovery(directory.path());

        let members = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            std::iter::empty(),
            |_| {
                Err(ToolchainError::internal(
                    "identity resolver must not be called",
                ))
            },
        )
        .expect("empty workspace is valid");

        assert!(members.is_empty());
        assert_eq!(members.len(), 0);
    }

    #[test]
    fn rejects_duplicate_literal_declarations() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["member"]),
        );

        directory.write(
            "member/Zamani.toml",
            &package_manifest("member"),
        );

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let result = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("member")
                    .expect("valid declaration"),
                WorkspaceMemberDeclaration::new("member")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_project_identities() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["one", "two"]),
        );

        directory.write(
            "one/Zamani.toml",
            &package_manifest("one"),
        );

        directory.write(
            "two/Zamani.toml",
            &package_manifest("two"),
        );

        let discovery = workspace_discovery(directory.path());

        let shared_id = ProjectId::new(
            namespace("zamani.test.project"),
            7,
        )
        .expect("valid project ID");

        let result = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("one")
                    .expect("valid declaration"),
                WorkspaceMemberDeclaration::new("two")
                    .expect("valid declaration"),
            ],
            |_| Ok(shared_id.clone()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_member_directory() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["missing"]),
        );

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let result = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("missing")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_directory_without_package_manifest() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["member"]),
        );

        fs::create_dir_all(directory.path().join("member"))
            .expect("member directory");

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let result = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("member")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_absolute_and_parent_traversal_paths() {
        assert!(
            WorkspaceMemberDeclaration::new("/outside").is_err()
        );

        assert!(
            WorkspaceMemberDeclaration::new("../outside").is_err()
        );

        assert!(
            WorkspaceMemberDeclaration::new("one/../outside").is_err()
        );

        assert!(
            WorkspaceMemberDeclaration::new("").is_err()
        );

        assert!(
            WorkspaceMemberDeclaration::new("./member").is_err()
        );

        assert!(
            WorkspaceMemberDeclaration::new(".").is_ok()
        );
    }

    #[test]
    fn rejects_a_member_that_is_a_file() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["member"]),
        );

        directory.write("member", "not a directory");

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let result = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("member")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        );

        assert!(result.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinked_member_directories() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new();
        let outside = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["linked"]),
        );

        outside.write(
            "Zamani.toml",
            &package_manifest("outside"),
        );

        symlink(
            outside.path(),
            directory.path().join("linked"),
        )
        .expect("symlink must be created");

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let result = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("linked")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn lookup_uses_project_identity_and_paths() {
        let directory = TestDirectory::new();

        directory.write(
            "Zamani.toml",
            &workspace_manifest(&["member"]),
        );

        directory.write(
            "member/Zamani.toml",
            &package_manifest("member"),
        );

        let discovery = workspace_discovery(directory.path());
        let allocator = project_allocator();

        let members = WorkspaceMembers::resolve(
            workspace_id(),
            &discovery,
            [
                WorkspaceMemberDeclaration::new("member")
                    .expect("valid declaration"),
            ],
            |_| {
                allocator
                    .try_next()
                    .map_err(|error| {
                        ToolchainError::resource_exhausted(
                            error.to_string(),
                        )
                    })
            },
        )
        .expect("members must resolve");

        let member = &members.as_slice()[0];

        assert!(
            members
                .get_by_project_id(member.project_id())
                .is_some()
        );

        assert!(
            members
                .get_by_relative_path("member")
                .is_some()
        );

        assert!(
            members
                .get_by_canonical_root(member.canonical_root())
                .is_some()
        );

        assert!(
            members
                .get_by_relative_path("missing")
                .is_none()
        );
    }
}
