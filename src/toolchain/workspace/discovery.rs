
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Workspace Discovery
//!
//! Path: `src/toolchain/workspace/discovery.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Discovers Zamani projects and workspace roots from explicitly supplied
//! filesystem paths.
//!
//! Discovery is independent of the process working directory. It supports
//! starting from a project directory, a nested directory, a source file,
//! or an explicitly supplied manifest file.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Filesystem-based project and workspace root discovery.
//! - Ancestor traversal.
//! - Canonical path resolution.
//! - Discovery options.
//! - Recognition of package and workspace manifest sections.
//! - Structured discovery results.
//!
//! This module does not own:
//!
//! - The canonical Zamani manifest schema.
//! - Full manifest interpretation or validation.
//! - Workspace member expansion.
//! - Dependency resolution.
//! - Project or workspace identity allocation.
//! - Source parsing or semantic analysis.
//! - Compiler invocation.
//! - Target selection or hardware discovery.
//! - Physical resource allocation.
//!
//! ## Integration
//!
//! - `super::layout` consumes discovered root and manifest paths.
//! - `super::members` resolves workspace member declarations.
//! - `super::metadata` constructs workspace metadata.
//! - `super::validation` validates the discovered workspace.
//! - `crate::zamani_project_config` owns canonical manifest interpretation.
//! - `crate::toolchain::ids` owns ProjectId and WorkspaceId.
//! - `crate::toolchain::errors` owns the canonical error contract.
//! - `crate::toolchain::result` owns the canonical result aliases.
//!
//! ## Manifest parsing boundary
//!
//! The existing ZamaniToml parser requires a `[package]` section and does
//! not currently model workspace-only manifests.
//!
//! Therefore, this module uses `toml::Value` only to recognize the presence
//! and structural type of `[package]` and `[workspace]`.
//!
//! It does not interpret package fields, dependencies, workspace members,
//! features, targets, or other manifest semantics.
//!
//! Full manifest interpretation belongs to zamani_project_config.
//!
//! ## Discovery rules
//!
//! 1. The caller supplies the starting path.
//! 2. The path is canonicalized before traversal.
//! 3. If the path is a file, traversal starts at its parent directory.
//! 4. If the path is a directory, traversal starts at that directory.
//! 5. Ancestors are inspected from nearest to farthest.
//! 6. The nearest package manifest is selected.
//! 7. The nearest enclosing workspace manifest is selected.
//! 8. A workspace-only root is valid.
//! 9. A package project without an explicit workspace is valid.
//! 10. Malformed manifests are errors, not silently ignored.
//!
//! Discovery does not recursively scan project contents. Workspace member
//! expansion belongs to the members subsystem.
//!
//! ## Scalability
//!
//! No fixed project count, workspace count, member count, directory depth,
//! target count, hardware count, or computational-domain limit is imposed.
//!
//! Ancestor traversal uses memory proportional to the discovered result,
//! not to the number of files in the project.
//!
//! Filesystem operations remain subject to operating-system permissions,
//! path representation, available memory, storage, and physical resources.
//!
//! ## Security
//!
//! - No shell commands are executed.
//! - No external process is launched.
//! - No manifest content is executed.
//! - No directory tree is recursively traversed.
//! - Filesystem errors are preserved as causal errors.
//! - Paths returned by successful discovery are canonicalized.
//! - Malformed manifest files fail closed.
//!
//! Discovery does not establish that a project or its manifest is trusted.
//! Trust and manifest policy validation belong to downstream services.

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::Result;

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// ============================================================================
// DEFAULT MANIFEST CONVENTION
// ============================================================================

/// Conventional Zamani project manifest filename.
///
/// This is a project-format convention, not a hardware or computational
/// capacity limit. Integrations can supply another filename through
/// [`DiscoveryOptions`].
pub const DEFAULT_MANIFEST_FILE_NAME: &str = "Zamani.toml";

// ============================================================================
// DISCOVERY OPTIONS
// ============================================================================

/// Configuration for filesystem-based project and workspace discovery.
///
/// The default configuration recognizes `Zamani.toml`.
///
/// A custom manifest filename is useful for isolated integrations and
/// migration tooling. It does not change the canonical manifest schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryOptions {
    manifest_file_name: String,
}

impl Default for DiscoveryOptions {
    fn default() -> Self {
        Self {
            manifest_file_name: DEFAULT_MANIFEST_FILE_NAME.to_owned(),
        }
    }
}

impl DiscoveryOptions {
    /// Create default discovery options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the manifest filename recognized during discovery.
    ///
    /// The filename must be a single nonempty path component. Path
    /// separators and special directory components are rejected.
    ///
    /// This method does not allow arbitrary manifest schema substitution.
    pub fn with_manifest_file_name(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self> {
        let name = name.into();

        validate_manifest_file_name(&name)?;

        self.manifest_file_name = name;

        Ok(self)
    }

    /// Return the configured manifest filename.
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
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "manifest filename must be a nonempty filename component",
            )
            .with_operation("workspace.discovery.validate_options"),
        );
    }

    Ok(())
}

// ============================================================================
// MANIFEST CLASSIFICATION
// ============================================================================

/// Structural classification of a recognized Zamani manifest.
///
/// A manifest may contain both sections, only a package section, or only
/// a workspace section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestKind {
    /// Manifest contains `[package]` only.
    Package,

    /// Manifest contains `[workspace]` only.
    Workspace,

    /// Manifest contains both `[package]` and `[workspace]`.
    PackageAndWorkspace,
}

impl ManifestKind {
    /// Whether the manifest declares a package.
    #[must_use]
    pub const fn has_package(self) -> bool {
        matches!(
            self,
            Self::Package | Self::PackageAndWorkspace
        )
    }

    /// Whether the manifest declares a workspace.
    #[must_use]
    pub const fn has_workspace(self) -> bool {
        matches!(
            self,
            Self::Workspace | Self::PackageAndWorkspace
        )
    }
}

/// Recognized manifest location and structural classification.
///
/// This structure does not contain a parsed package or workspace model.
/// The canonical manifest subsystem remains responsible for that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredManifest {
    path: PathBuf,
    root: PathBuf,
    kind: ManifestKind,
}

impl DiscoveredManifest {
    fn new(
        path: PathBuf,
        root: PathBuf,
        kind: ManifestKind,
    ) -> Self {
        Self {
            path,
            root,
            kind,
        }
    }

    /// Return the canonical manifest path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Return the canonical directory containing the manifest.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Return the manifest's structural classification.
    #[must_use]
    pub const fn kind(&self) -> ManifestKind {
        self.kind
    }
}

// ============================================================================
// DISCOVERY RESULT
// ============================================================================

/// Successful project and workspace discovery result.
///
/// `project_manifest` identifies the nearest package-bearing manifest.
/// `workspace_manifest` identifies the nearest enclosing explicit workspace
/// manifest.
///
/// A workspace root can exist without containing a package.
///
/// A standalone package project can exist without an explicit workspace.
///
/// The effective workspace root is the explicit workspace root when
/// present, otherwise the discovered project root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryResult {
    start_directory: PathBuf,
    project_manifest: Option<DiscoveredManifest>,
    workspace_manifest: Option<DiscoveredManifest>,
}

impl DiscoveryResult {
    fn new(
        start_directory: PathBuf,
        project_manifest: Option<DiscoveredManifest>,
        workspace_manifest: Option<DiscoveredManifest>,
    ) -> Self {
        Self {
            start_directory,
            project_manifest,
            workspace_manifest,
        }
    }

    /// Return the canonical directory from which discovery began.
    #[must_use]
    pub fn start_directory(&self) -> &Path {
        &self.start_directory
    }

    /// Return the discovered package manifest, if one exists.
    #[must_use]
    pub fn project_manifest(&self) -> Option<&DiscoveredManifest> {
        self.project_manifest.as_ref()
    }

    /// Return the discovered project root, if a package manifest exists.
    #[must_use]
    pub fn project_root(&self) -> Option<&Path> {
        self.project_manifest
            .as_ref()
            .map(DiscoveredManifest::root)
    }

    /// Return the discovered explicit workspace manifest, if one exists.
    #[must_use]
    pub fn workspace_manifest(&self) -> Option<&DiscoveredManifest> {
        self.workspace_manifest.as_ref()
    }

    /// Return the explicit workspace root, if one exists.
    #[must_use]
    pub fn workspace_root(&self) -> Option<&Path> {
        self.workspace_manifest
            .as_ref()
            .map(DiscoveredManifest::root)
    }

    /// Return the effective workspace root.
    ///
    /// An explicit workspace root takes precedence. If there is no explicit
    /// workspace, the discovered project root is the effective root.
    #[must_use]
    pub fn effective_workspace_root(&self) -> Option<&Path> {
        self.workspace_root().or_else(|| self.project_root())
    }

    /// Whether discovery found a package project.
    #[must_use]
    pub fn has_project(&self) -> bool {
        self.project_manifest.is_some()
    }

    /// Whether discovery found an explicit workspace.
    #[must_use]
    pub fn has_explicit_workspace(&self) -> bool {
        self.workspace_manifest.is_some()
    }

    /// Whether discovery found either a package project or a workspace.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.has_project() || self.has_explicit_workspace()
    }
}

// ============================================================================
// PUBLIC DISCOVERY API
// ============================================================================

/// Discover a Zamani project or workspace from an explicitly supplied path.
///
/// This function never assumes that the process current working directory
/// identifies the project.
///
/// The starting path may identify:
///
/// - A project directory.
/// - A workspace directory.
/// - A nested project directory.
/// - A source file.
/// - A manifest file.
///
/// The function walks ancestors only. It does not recursively enumerate
/// workspace members or inspect arbitrary project source files.
///
/// # Errors
///
/// Returns a structured toolchain error when:
///
/// - The supplied path does not exist.
/// - The path cannot be canonicalized.
/// - Filesystem access fails.
/// - A recognized manifest is malformed.
/// - A recognized manifest has neither a `[package]` nor `[workspace]`
///   table.
/// - No project or workspace manifest is discovered.
pub fn discover(
    start: impl AsRef<Path>,
) -> Result<DiscoveryResult> {
    discover_with_options(start, &DiscoveryOptions::default())
}

/// Discover a Zamani project or workspace with explicit options.
///
/// See [`discover`] for the discovery rules and failure semantics.
pub fn discover_with_options(
    start: impl AsRef<Path>,
    options: &DiscoveryOptions,
) -> Result<DiscoveryResult> {
    validate_manifest_file_name(
        options.manifest_file_name(),
    )?;

    let supplied_path = start.as_ref();

    if supplied_path.as_os_str().is_empty() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "workspace discovery requires a nonempty starting path",
            )
            .with_operation("workspace.discovery"),
        );
    }

    let canonical_start = fs::canonicalize(supplied_path)
        .map_err(|error| {
            filesystem_error(
                "workspace.discovery.canonicalize_start",
                supplied_path,
                error,
            )
        })?;

    let metadata = fs::metadata(&canonical_start)
        .map_err(|error| {
            filesystem_error(
                "workspace.discovery.inspect_start",
                &canonical_start,
                error,
            )
        })?;

    let start_directory = if metadata.is_dir() {
        canonical_start.clone()
    } else if metadata.is_file() {
        canonical_start
            .parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| {
                ToolchainError::new(
                    ToolchainErrorKind::InvalidInput,
                    "starting file has no parent directory",
                )
                .with_operation("workspace.discovery")
                .with_context(canonical_start.display().to_string())
            })?
    } else {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "starting path must identify a regular file or directory",
            )
            .with_operation("workspace.discovery")
            .with_context(canonical_start.display().to_string()),
        );
    };

    discover_ancestors(
        start_directory,
        options.manifest_file_name(),
    )
}

/// Discover from a manifest path explicitly supplied by the caller.
///
/// Unlike ordinary ancestor discovery, this function requires the supplied
/// file itself to be the configured manifest filename. It validates the
/// manifest's structural classification and then performs normal ancestor
/// discovery from its containing directory.
pub fn discover_from_manifest(
    manifest_path: impl AsRef<Path>,
) -> Result<DiscoveryResult> {
    discover_from_manifest_with_options(
        manifest_path,
        &DiscoveryOptions::default(),
    )
}

/// Discover from an explicitly supplied manifest using custom options.
pub fn discover_from_manifest_with_options(
    manifest_path: impl AsRef<Path>,
    options: &DiscoveryOptions,
) -> Result<DiscoveryResult> {
    let path = manifest_path.as_ref();

    validate_manifest_file_name(
        options.manifest_file_name(),
    )?;

    let canonical_path = fs::canonicalize(path)
        .map_err(|error| {
            filesystem_error(
                "workspace.discovery.canonicalize_manifest",
                path,
                error,
            )
        })?;

    if canonical_path.file_name()
        != Some(OsStr::new(options.manifest_file_name()))
    {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "the supplied file does not have the configured manifest filename",
            )
            .with_operation("workspace.discovery.from_manifest")
            .with_context(canonical_path.display().to_string()),
        );
    }

    let metadata = fs::metadata(&canonical_path)
        .map_err(|error| {
            filesystem_error(
                "workspace.discovery.inspect_manifest",
                &canonical_path,
                error,
            )
        })?;

    if !metadata.is_file() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "the supplied manifest path is not a regular file",
            )
            .with_operation("workspace.discovery.from_manifest")
            .with_context(canonical_path.display().to_string()),
        );
    }

    // Validate the explicit manifest before beginning ancestor discovery.
    let _ = read_manifest(
        &canonical_path,
        canonical_path
            .parent()
            .ok_or_else(|| {
                ToolchainError::invalid_input(
                    "manifest has no parent directory",
                )
            })?,
    )?;

    let parent = canonical_path
        .parent()
        .ok_or_else(|| {
            ToolchainError::invalid_input(
                "manifest has no parent directory",
            )
        })?;

    discover_with_options(parent, options)
}

// ============================================================================
// ANCESTOR DISCOVERY
// ============================================================================

fn discover_ancestors(
    start_directory: PathBuf,
    manifest_file_name: &str,
) -> Result<DiscoveryResult> {
    let mut current = Some(start_directory.clone());

    let mut project_manifest: Option<DiscoveredManifest> = None;
    let mut workspace_manifest: Option<DiscoveredManifest> = None;

    while let Some(directory) = current {
        let candidate = directory.join(manifest_file_name);

        match inspect_candidate(&candidate, &directory)? {
            Some(manifest) => {
                if manifest.kind().has_package()
                    && project_manifest.is_none()
                {
                    project_manifest = Some(manifest.clone());
                }

                if manifest.kind().has_workspace()
                    && workspace_manifest.is_none()
                {
                    workspace_manifest = Some(manifest);
                }
            }

            None => {}
        }

        // Once the explicit workspace boundary has been found, do not
        // accidentally adopt a package from an unrelated ancestor.
        if workspace_manifest.is_some() {
            break;
        }

        current = directory.parent().map(Path::to_path_buf);
    }

    if project_manifest.is_none()
        && workspace_manifest.is_none()
    {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::NotFound,
                format!(
                    "no package or workspace manifest named \
                     `{manifest_file_name}` was found in the starting \
                     directory or its ancestors"
                ),
            )
            .with_operation("workspace.discovery")
            .with_context(
                start_directory.display().to_string(),
            ),
        );
    }

    Ok(DiscoveryResult::new(
        start_directory,
        project_manifest,
        workspace_manifest,
    ))
}

// ============================================================================
// MANIFEST RECOGNITION
// ============================================================================

fn inspect_candidate(
    candidate: &Path,
    root: &Path,
) -> Result<Option<DiscoveredManifest>> {
    let metadata = match fs::metadata(candidate) {
        Ok(metadata) => metadata,

        Err(error)
            if error.kind() == io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }

        Err(error) => {
            return Err(filesystem_error(
                "workspace.discovery.inspect_candidate",
                candidate,
                error,
            ));
        }
    };

    if !metadata.is_file() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "the configured manifest path exists but is not a regular file",
            )
            .with_operation("workspace.discovery.inspect_candidate")
            .with_context(candidate.display().to_string()),
        );
    }

    let manifest = read_manifest(candidate, root)?;

    Ok(Some(manifest))
}

/// Read TOML and recognize only the top-level structural sections owned by
/// project/workspace discovery.
///
/// This function intentionally does not deserialize `ZamaniToml`, because
/// the current canonical parser requires `[package]` and cannot yet
/// represent a workspace-only root.
fn read_manifest(
    path: &Path,
    root: &Path,
) -> Result<DiscoveredManifest> {
    let content = fs::read_to_string(path)
        .map_err(|error| {
            filesystem_error(
                "workspace.discovery.read_manifest",
                path,
                error,
            )
        })?;

    let document: toml::Value =
        toml::from_str(&content).map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "the discovered Zamani manifest is not valid TOML",
            )
            .with_operation("workspace.discovery.parse_manifest")
            .with_context(path.display().to_string())
            .with_source(error)
        })?;

    let package_section = match document.get("package") {
        Some(value) => {
            if value.as_table().is_none() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "`[package]` must be a TOML table",
                    )
                    .with_operation(
                        "workspace.discovery.classify_manifest",
                    )
                    .with_context(path.display().to_string()),
                );
            }

            true
        }

        None => false,
    };

    let workspace_section = match document.get("workspace") {
        Some(value) => {
            if value.as_table().is_none() {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::ValidationFailed,
                        "`[workspace]` must be a TOML table",
                    )
                    .with_operation(
                        "workspace.discovery.classify_manifest",
                    )
                    .with_context(path.display().to_string()),
                );
            }

            true
        }

        None => false,
    };

    let kind = match (package_section, workspace_section) {
        (true, false) => ManifestKind::Package,
        (false, true) => ManifestKind::Workspace,
        (true, true) => ManifestKind::PackageAndWorkspace,

        (false, false) => {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    "manifest must contain a `[package]` or `[workspace]` table",
                )
                .with_operation(
                    "workspace.discovery.classify_manifest",
                )
                .with_context(path.display().to_string()),
            );
        }
    };

    Ok(DiscoveredManifest::new(
        path.to_path_buf(),
        root.to_path_buf(),
        kind,
    ))
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn filesystem_error(
    operation: &'static str,
    path: &Path,
    error: io::Error,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Io,
        format!(
            "filesystem operation failed for `{}`",
            path.display()
        ),
    )
    .with_operation(operation)
    .with_context(error.to_string())
    .with_source(error)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::Path;
    use std::sync::atomic::{
        AtomicU64,
        Ordering,
    };

    static NEXT_TEST_DIRECTORY: AtomicU64 =
        AtomicU64::new(1);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let sequence = NEXT_TEST_DIRECTORY.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let path = std::env::temp_dir().join(format!(
                "zamani-workspace-discovery-{}-{sequence}",
                std::process::id(),
            ));

            fs::create_dir_all(&path)
                .expect("test directory should be created");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn create_directory(
            &self,
            relative: impl AsRef<Path>,
        ) -> PathBuf {
            let path = self.path.join(relative);

            fs::create_dir_all(&path)
                .expect("nested test directory should be created");

            path
        }

        fn write_manifest(
            &self,
            directory: impl AsRef<Path>,
            content: &str,
        ) -> PathBuf {
            let path = directory
                .as_ref()
                .join(DEFAULT_MANIFEST_FILE_NAME);

            fs::write(&path, content)
                .expect("test manifest should be written");

            path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    const PACKAGE_MANIFEST: &str = r#"
[package]
name = "example"
version = "1.0.0"
edition = "2021"
"#;

    const WORKSPACE_MANIFEST: &str = r#"
[workspace]
members = []
"#;

    // ------------------------------------------------------------------------
    // Basic discovery
    // ------------------------------------------------------------------------

    #[test]
    fn discovers_project_from_its_root_directory() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            PACKAGE_MANIFEST,
        );

        let result = discover(temporary.path())
            .expect("project should be discovered");

        assert!(result.has_project());
        assert!(!result.has_explicit_workspace());

        assert_eq!(
            result.project_root(),
            Some(temporary.path()),
        );

        assert_eq!(
            result.effective_workspace_root(),
            Some(temporary.path()),
        );
    }

    #[test]
    fn discovers_project_from_nested_directory() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            PACKAGE_MANIFEST,
        );

        let nested = temporary.create_directory(
            "src/modules/example",
        );

        let result = discover(&nested)
            .expect("ancestor project should be discovered");

        assert_eq!(
            result.project_root(),
            Some(temporary.path()),
        );
    }

    #[test]
    fn discovers_project_from_source_file() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            PACKAGE_MANIFEST,
        );

        let source_directory = temporary.create_directory(
            "src",
        );

        let source_file = source_directory.join("main.zm");

        fs::write(&source_file, "fn main() {}")
            .expect("source fixture should be written");

        let result = discover(&source_file)
            .expect("project should be discovered");

        assert_eq!(
            result.project_root(),
            Some(temporary.path()),
        );

        assert_eq!(
            result.start_directory(),
            source_directory,
        );
    }

    // ------------------------------------------------------------------------
    // Workspace discovery
    // ------------------------------------------------------------------------

    #[test]
    fn discovers_enclosing_workspace_and_nearest_project() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            WORKSPACE_MANIFEST,
        );

        let member = temporary.create_directory(
            "applications/example",
        );

        temporary.write_manifest(
            &member,
            PACKAGE_MANIFEST,
        );

        let nested = temporary.create_directory(
            "applications/example/src",
        );

        let result = discover(&nested)
            .expect("workspace and project should be discovered");

        assert_eq!(
            result.workspace_root(),
            Some(temporary.path()),
        );

        assert_eq!(
            result.project_root(),
            Some(member.as_path()),
        );

        assert!(result.has_explicit_workspace());
        assert!(result.has_project());
    }

    #[test]
    fn discovers_workspace_without_package_at_root() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            WORKSPACE_MANIFEST,
        );

        let nested = temporary.create_directory(
            "tools",
        );

        let result = discover(&nested)
            .expect("workspace-only root should be discovered");

        assert!(result.has_explicit_workspace());
        assert!(!result.has_project());

        assert_eq!(
            result.workspace_root(),
            Some(temporary.path()),
        );

        assert_eq!(
            result.effective_workspace_root(),
            Some(temporary.path()),
        );
    }

    #[test]
    fn project_and_workspace_can_share_one_manifest() {
        let temporary = TestDirectory::new();

        let content = r#"
[package]
name = "root"
version = "1.0.0"
edition = "2021"

[workspace]
members = []
"#;

        temporary.write_manifest(
            temporary.path(),
            content,
        );

        let result = discover(temporary.path())
            .expect("combined root should be discovered");

        assert!(result.has_project());
        assert!(result.has_explicit_workspace());

        assert_eq!(
            result.project_root(),
            Some(temporary.path()),
        );

        assert_eq!(
            result.workspace_root(),
            Some(temporary.path()),
        );
    }

    // ------------------------------------------------------------------------
    // Explicit manifest discovery
    // ------------------------------------------------------------------------

    #[test]
    fn discovers_from_explicit_manifest_path() {
        let temporary = TestDirectory::new();

        let manifest = temporary.write_manifest(
            temporary.path(),
            PACKAGE_MANIFEST,
        );

        let result = discover_from_manifest(&manifest)
            .expect("explicit manifest should be discovered");

        assert_eq!(
            result.project_root(),
            Some(temporary.path()),
        );
    }

    #[test]
    fn rejects_non_manifest_file_as_explicit_manifest() {
        let temporary = TestDirectory::new();

        let file = temporary.path().join("main.zm");

        fs::write(&file, "fn main() {}")
            .expect("source fixture should be written");

        let result = discover_from_manifest(&file);

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Custom manifest convention
    // ------------------------------------------------------------------------

    #[test]
    fn supports_configured_manifest_filename() {
        let temporary = TestDirectory::new();

        let custom_name = "Zamani.project.toml";

        let custom_manifest = temporary
            .path()
            .join(custom_name);

        fs::write(&custom_manifest, PACKAGE_MANIFEST)
            .expect("custom manifest should be written");

        let options = DiscoveryOptions::new()
            .with_manifest_file_name(custom_name)
            .expect("custom filename should be valid");

        let result = discover_with_options(
            temporary.path(),
            &options,
        )
        .expect("custom manifest should be discovered");

        assert_eq!(
            result.project_root(),
            Some(temporary.path()),
        );
    }

    #[test]
    fn rejects_manifest_filename_with_directory_components() {
        assert!(
            DiscoveryOptions::new()
                .with_manifest_file_name("../Zamani.toml")
                .is_err()
        );

        assert!(
            DiscoveryOptions::new()
                .with_manifest_file_name("")
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Validation and failure behavior
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_malformed_manifest_instead_of_ignoring_it() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            "[package\nname = invalid",
        );

        let result = discover(temporary.path());

        assert!(result.is_err());

        let error = result.expect_err(
            "malformed manifest must fail",
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ValidationFailed,
        );
    }

    #[test]
    fn rejects_manifest_without_recognized_sections() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            r#"
[unrelated]
value = "example"
"#,
        );

        let result = discover(temporary.path());

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_package_section_type() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            r#"
package = "not-a-table"
"#,
        );

        let result = discover(temporary.path());

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_workspace_section_type() {
        let temporary = TestDirectory::new();

        temporary.write_manifest(
            temporary.path(),
            r#"
workspace = "not-a-table"
"#,
        );

        let result = discover(temporary.path());

        assert!(result.is_err());
    }

    #[test]
    fn reports_missing_starting_path() {
        let temporary = TestDirectory::new();

        let missing = temporary.path().join("does-not-exist");

        let result = discover(missing);

        assert!(result.is_err());
    }

    #[test]
    fn reports_existing_directory_without_project_or_workspace() {
        let temporary = TestDirectory::new();

        let result = discover(temporary.path());

        assert!(result.is_err());

        let error = result.expect_err(
            "missing manifest should be reported",
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::NotFound,
        );
    }

    // ------------------------------------------------------------------------
    // Classification contracts
    // ------------------------------------------------------------------------

    #[test]
    fn manifest_classification_reports_package_and_workspace() {
        assert!(ManifestKind::Package.has_package());
        assert!(!ManifestKind::Package.has_workspace());

        assert!(!ManifestKind::Workspace.has_package());
        assert!(ManifestKind::Workspace.has_workspace());

        assert!(
            ManifestKind::PackageAndWorkspace.has_package()
        );

        assert!(
            ManifestKind::PackageAndWorkspace.has_workspace()
        );
    }

    #[test]
    fn default_manifest_filename_is_canonical() {
        assert_eq!(
            DiscoveryOptions::default().manifest_file_name(),
            "Zamani.toml",
        );
    }
}
