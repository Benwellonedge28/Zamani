
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Project Layout
//!
//! Path: `src/toolchain/workspace/layout.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Interprets the physical directory and source-file organization of a
//! discovered Zamani project or workspace.
//!
//! This module supports:
//!
//! - Conventional source-directory layouts.
//! - Flat source layouts.
//! - Mixed source layouts.
//! - Custom source directories.
//! - Custom source-file extensions.
//! - Multiple entry points.
//! - Library source files.
//! - Arbitrarily nested source directories.
//! - Iterative source-file traversal.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Source-layout conventions.
//! - Source-directory recognition.
//! - Entry-point recognition.
//! - Project-layout descriptions.
//! - Deterministic directory inspection.
//! - Safe source-file traversal.
//!
//! It does not own:
//!
//! - Project or workspace discovery.
//! - Manifest parsing or interpretation.
//! - Workspace member expansion.
//! - Package identity.
//! - Source-language parsing.
//! - AST construction.
//! - Semantic validation.
//! - Compilation.
//! - Target selection.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! ## Integration
//!
//! - `super::discovery` supplies discovered project/workspace roots.
//! - `super::members` consumes workspace member declarations and
//!   may inspect the layout of each resolved member.
//! - `super::metadata` consumes the resulting layout description.
//! - `super::validation` validates required project structure.
//! - `crate::zamani_project_config` remains the manifest authority.
//! - `crate::frontend` remains the source-processing authority.
//! - `crate::toolchain::errors` owns structured toolchain errors.
//!
//! ## Scalability
//!
//! No fixed source-file count, directory count, nesting depth,
//! project size, computational-domain count, target count, or
//! hardware-resource limit is imposed.
//!
//! Ordinary inspection examines the project root and configured
//! source roots without recursively loading the entire source tree.
//!
//! Recursive traversal uses an explicit stack rather than recursive
//! Rust function calls. Memory consumption is proportional to the
//! pending traversal paths and directory-entry batches.
//!
//! Actual feasibility remains subject to operating-system and
//! implementation resources.
//!
//! ## Security
//!
//! - No unsafe Rust.
//! - No shell execution.
//! - No external process execution.
//! - No manifest execution.
//! - No automatic modification of project files.
//! - No following directory symlinks during source traversal.
//! - No accepting parent-directory traversal in configured paths.
//! - No silent conversion of filesystem failures into empty results.
//! - Canonical project roots are used for containment checks.
//!
//! Layout inspection does not establish project trust.

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::errors::Result;

use super::discovery::DiscoveryResult;

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

// ============================================================================
// DEFAULT CONVENTIONS
// ============================================================================

/// Conventional Zamani source-file extension.
///
/// This is a configurable project convention, not a language-level
/// restriction.
pub const DEFAULT_SOURCE_EXTENSION: &str = "zm";

/// Conventional source directory.
///
/// Projects are not required to use this directory.
pub const DEFAULT_SOURCE_DIRECTORY: &str = "src";

/// Conventional executable entry-point filename.
pub const DEFAULT_EXECUTABLE_ENTRY_POINT: &str = "main.zm";

/// Conventional library entry-point filename.
pub const DEFAULT_LIBRARY_ENTRY_POINT: &str = "lib.zm";

// ============================================================================
// SOURCE LAYOUT OPTIONS
// ============================================================================

/// Configurable conventions used to interpret a project layout.
///
/// Paths in this configuration are relative to the canonical project root.
///
/// The default configuration recognizes:
///
/// - `src/` as a conventional source directory.
/// - `main.zm` as a conventional executable entry point.
/// - `src/main.zm` as a conventional executable entry point.
/// - `lib.zm` as a conventional library entry point.
/// - `src/lib.zm` as a conventional library entry point.
///
/// None of these conventions is mandatory.
///
/// Custom projects may register different source roots, entry points,
/// and source extensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLayoutOptions {
    source_roots: Vec<PathBuf>,
    executable_entry_points: Vec<PathBuf>,
    library_entry_points: Vec<PathBuf>,
    source_extension: OsString,
}

impl Default for SourceLayoutOptions {
    fn default() -> Self {
        Self {
            source_roots: vec![
                PathBuf::from(DEFAULT_SOURCE_DIRECTORY),
            ],
            executable_entry_points: vec![
                PathBuf::from(DEFAULT_EXECUTABLE_ENTRY_POINT),
                PathBuf::from(DEFAULT_SOURCE_DIRECTORY)
                    .join(DEFAULT_EXECUTABLE_ENTRY_POINT),
            ],
            library_entry_points: vec![
                PathBuf::from(DEFAULT_LIBRARY_ENTRY_POINT),
                PathBuf::from(DEFAULT_SOURCE_DIRECTORY)
                    .join(DEFAULT_LIBRARY_ENTRY_POINT),
            ],
            source_extension: OsString::from(
                DEFAULT_SOURCE_EXTENSION,
            ),
        }
    }
}

impl SourceLayoutOptions {
    /// Construct default Zamani layout conventions.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an additional source directory.
    ///
    /// The path must be relative to the project root and must not
    /// contain parent-directory, root, or platform-prefix components.
    ///
    /// Duplicate registrations are ignored.
    pub fn with_source_root(
        mut self,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        let path = validate_relative_path(path.as_ref())?;

        if !self.source_roots.contains(&path) {
            self.source_roots.push(path);
        }

        Ok(self)
    }

    /// Replace the configured source directories.
    ///
    /// An empty collection is valid. It means that no named source
    /// directories are configured.
    ///
    /// Root-level source files can still be recognized.
    pub fn with_source_roots<I, P>(
        mut self,
        paths: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut roots = Vec::new();

        for path in paths {
            let path = validate_relative_path(path.as_ref())?;

            if !roots.contains(&path) {
                roots.push(path);
            }
        }

        self.source_roots = roots;

        Ok(self)
    }

    /// Register an executable entry-point candidate.
    ///
    /// The path is relative to the project root.
    ///
    /// The candidate is recognized only when it exists as a regular
    /// file with the configured source extension.
    pub fn with_executable_entry_point(
        mut self,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        let path = validate_relative_path(path.as_ref())?;

        if !self.executable_entry_points.contains(&path) {
            self.executable_entry_points.push(path);
        }

        Ok(self)
    }

    /// Register a library entry-point candidate.
    ///
    /// The candidate is recognized only when it exists as a regular
    /// file with the configured source extension.
    pub fn with_library_entry_point(
        mut self,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        let path = validate_relative_path(path.as_ref())?;

        if !self.library_entry_points.contains(&path) {
            self.library_entry_points.push(path);
        }

        Ok(self)
    }

    /// Replace executable entry-point candidates.
    pub fn with_executable_entry_points<I, P>(
        mut self,
        paths: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        self.executable_entry_points =
            validate_relative_paths(paths)?;

        Ok(self)
    }

    /// Replace library entry-point candidates.
    pub fn with_library_entry_points<I, P>(
        mut self,
        paths: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        self.library_entry_points =
            validate_relative_paths(paths)?;

        Ok(self)
    }

    /// Replace the source-file extension.
    ///
    /// The extension must be one nonempty filename component.
    ///
    /// The leading dot is optional.
    ///
    /// For example, both `zm` and `.zm` represent the same extension.
    ///
    /// This allows registered source-format integrations without
    /// hard-coding every possible source format.
    pub fn with_source_extension(
        mut self,
        extension: impl Into<OsString>,
    ) -> Result<Self> {
        let mut extension = extension.into();

        if extension.to_string_lossy().starts_with('.') {
            let text = extension.to_string_lossy();
            extension = OsString::from(
                text.trim_start_matches('.'),
            );
        }

        validate_extension(&extension)?;

        self.source_extension = extension;

        Ok(self)
    }

    /// Return the configured source directories.
    #[must_use]
    pub fn source_roots(&self) -> &[PathBuf] {
        &self.source_roots
    }

    /// Return executable entry-point candidates.
    #[must_use]
    pub fn executable_entry_points(&self) -> &[PathBuf] {
        &self.executable_entry_points
    }

    /// Return library entry-point candidates.
    #[must_use]
    pub fn library_entry_points(&self) -> &[PathBuf] {
        &self.library_entry_points
    }

    /// Return the configured source extension.
    #[must_use]
    pub fn source_extension(&self) -> &OsStr {
        &self.source_extension
    }
}

fn validate_relative_path(path: &Path) -> Result<PathBuf> {
    if path.as_os_str().is_empty() {
        return Err(invalid_layout_input(
            "configured project-relative path must not be empty",
        ));
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}

            Component::CurDir => {}

            Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return Err(invalid_layout_input(
                    "configured project paths must not escape the project root",
                ));
            }
        }
    }

    if path.components().next().is_none() {
        return Err(invalid_layout_input(
            "configured project-relative path is invalid",
        ));
    }

    Ok(path.to_path_buf())
}

fn validate_relative_paths<I, P>(
    paths: I,
) -> Result<Vec<PathBuf>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut validated = Vec::new();

    for path in paths {
        let path = validate_relative_path(path.as_ref())?;

        if !validated.contains(&path) {
            validated.push(path);
        }
    }

    Ok(validated)
}

fn validate_extension(extension: &OsStr) -> Result<()> {
    if extension.is_empty()
        || extension == OsStr::new(".")
        || extension == OsStr::new("..")
        || extension.to_string_lossy().contains('/')
        || extension.to_string_lossy().contains('\\')
        || extension.to_string_lossy().contains('\0')
    {
        return Err(invalid_layout_input(
            "source extension must be a nonempty filename extension",
        ));
    }

    Ok(())
}

// ============================================================================
// LAYOUT CLASSIFICATION
// ============================================================================

/// Classification of the detected project source layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutKind {
    /// Source files were found directly in the project root,
    /// without recognized configured source directories.
    Flat,

    /// Source files were found in configured source directories,
    /// without root-level source files.
    DirectoryBased,

    /// Source files were found both at the root and inside
    /// configured source directories.
    Mixed,

    /// No source files were found in the inspected locations.
    ///
    /// This does not establish that the project is invalid.
    ///
    /// Source files may be generated, stored in custom locations,
    /// or supplied by registered providers.
    Empty,
}

// ============================================================================
// SOURCE ROOT
// ============================================================================

/// Description of one recognized source directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRoot {
    relative_path: PathBuf,
    canonical_path: PathBuf,
    source_files: Vec<PathBuf>,
}

impl SourceRoot {
    fn new(
        relative_path: PathBuf,
        canonical_path: PathBuf,
        source_files: Vec<PathBuf>,
    ) -> Self {
        Self {
            relative_path,
            canonical_path,
            source_files,
        }
    }

    /// Return the source directory relative to the project root.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Return the canonical absolute source directory.
    #[must_use]
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    /// Return direct source files found in this directory.
    ///
    /// These paths are absolute canonical paths.
    ///
    /// Nested source directories are inspected separately by
    /// [`ProjectLayout::visit_source_files`].
    #[must_use]
    pub fn source_files(&self) -> &[PathBuf] {
        &self.source_files
    }
}

// ============================================================================
// PROJECT LAYOUT
// ============================================================================

/// Interpreted physical layout of a Zamani project.
///
/// This structure describes the project directory without imposing
/// source-language or computational-domain semantics.
///
/// All stored paths are canonical absolute paths, except paths
/// explicitly documented as relative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectLayout {
    project_root: PathBuf,
    kind: LayoutKind,
    source_roots: Vec<SourceRoot>,
    root_source_files: Vec<PathBuf>,
    executable_entry_points: Vec<PathBuf>,
    library_entry_points: Vec<PathBuf>,
    source_extension: OsString,
}

impl ProjectLayout {
    fn new(
        project_root: PathBuf,
        kind: LayoutKind,
        source_roots: Vec<SourceRoot>,
        root_source_files: Vec<PathBuf>,
        executable_entry_points: Vec<PathBuf>,
        library_entry_points: Vec<PathBuf>,
        source_extension: OsString,
    ) -> Self {
        Self {
            project_root,
            kind,
            source_roots,
            root_source_files,
            executable_entry_points,
            library_entry_points,
            source_extension,
        }
    }

    /// Return the canonical project root.
    #[must_use]
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    /// Return the detected layout classification.
    #[must_use]
    pub const fn kind(&self) -> LayoutKind {
        self.kind
    }

    /// Return recognized source directories.
    #[must_use]
    pub fn source_roots(&self) -> &[SourceRoot] {
        &self.source_roots
    }

    /// Return direct source files located in the project root.
    #[must_use]
    pub fn root_source_files(&self) -> &[PathBuf] {
        &self.root_source_files
    }

    /// Return recognized executable entry points.
    #[must_use]
    pub fn executable_entry_points(&self) -> &[PathBuf] {
        &self.executable_entry_points
    }

    /// Return recognized library entry points.
    #[must_use]
    pub fn library_entry_points(&self) -> &[PathBuf] {
        &self.library_entry_points
    }

    /// Return the source-file extension used during inspection.
    #[must_use]
    pub fn source_extension(&self) -> &OsStr {
        &self.source_extension
    }

    /// Whether any source files were found in inspected locations.
    #[must_use]
    pub fn has_source_files(&self) -> bool {
        !self.root_source_files.is_empty()
            || self
                .source_roots
                .iter()
                .any(|root| !root.source_files.is_empty())
    }

    /// Whether an executable entry point was recognized.
    #[must_use]
    pub fn has_executable_entry_point(&self) -> bool {
        !self.executable_entry_points.is_empty()
    }

    /// Whether a library entry point was recognized.
    #[must_use]
    pub fn has_library_entry_point(&self) -> bool {
        !self.library_entry_points.is_empty()
    }

    /// Visit source files under the project root and recognized
    /// source directories.
    ///
    /// Traversal is iterative and does not follow directory symlinks.
    ///
    /// Files are visited in deterministic lexical path order within
    /// each directory.
    ///
    /// The callback receives canonical absolute file paths.
    ///
    /// Returning an error from the callback stops traversal.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - A directory cannot be read.
    /// - A directory entry cannot be inspected.
    /// - A path cannot be canonicalized.
    /// - A discovered path escapes the canonical project root.
    /// - The callback returns an error.
    ///
    /// # Resource behavior
    ///
    /// The method does not impose an artificial traversal-depth or
    /// source-file-count limit.
    ///
    /// It retains pending directories in an explicit stack and sorts
    /// each directory's entries before processing them.
    ///
    /// The caller controls the lifetime of visited-file state.
    pub fn visit_source_files<F>(
        &self,
        mut visitor: F,
    ) -> Result<()>
    where
        F: FnMut(&Path) -> Result<()>,
    {
        let mut pending = Vec::new();

        pending.push(self.project_root.clone());

        for source_root in &self.source_roots {
            if source_root.canonical_path != self.project_root {
                pending.push(source_root.canonical_path.clone());
            }
        }

        while let Some(directory) = pending.pop() {
            let mut entries = read_sorted_entries(&directory)?;

            // Reverse insertion makes stack-based traversal process
            // entries in ascending lexical order.
            for entry in entries.drain(..).rev() {
                let entry_path = entry.path();

                let file_type = entry.file_type().map_err(|error| {
                    filesystem_error(
                        "workspace.layout.inspect_entry",
                        &entry_path,
                        error,
                    )
                })?;

                // Do not follow symbolic links. This prevents directory
                // cycles and avoids traversal outside the project root.
                if file_type.is_symlink() {
                    continue;
                }

                if file_type.is_dir() {
                    pending.push(entry_path);
                    continue;
                }

                if !file_type.is_file()
                    || !has_extension(
                        &entry_path,
                        &self.source_extension,
                    )
                {
                    continue;
                }

                let canonical_file =
                    fs::canonicalize(&entry_path).map_err(|error| {
                        filesystem_error(
                            "workspace.layout.canonicalize_source",
                            &entry_path,
                            error,
                        )
                    })?;

                ensure_contained(
                    &self.project_root,
                    &canonical_file,
                )?;

                visitor(&canonical_file)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// PUBLIC INSPECTION API
// ============================================================================

/// Inspect an explicitly supplied project root using default conventions.
///
/// The current working directory is never used as an implicit project root.
///
/// # Errors
///
/// Returns an error when the supplied path does not identify an
/// accessible directory or when a filesystem operation fails.
pub fn inspect(
    project_root: impl AsRef<Path>,
) -> Result<ProjectLayout> {
    inspect_with_options(
        project_root,
        &SourceLayoutOptions::default(),
    )
}

/// Inspect an explicitly supplied project root with custom conventions.
///
/// The supplied root is canonicalized before inspection.
///
/// Only the root directory and configured source directories are
/// enumerated during this initial inspection. Recursive enumeration
/// is available separately through [`ProjectLayout::visit_source_files`].
pub fn inspect_with_options(
    project_root: impl AsRef<Path>,
    options: &SourceLayoutOptions,
) -> Result<ProjectLayout> {
    validate_options(options)?;

    let supplied_root = project_root.as_ref();

    if supplied_root.as_os_str().is_empty() {
        return Err(invalid_layout_input(
            "project root must not be empty",
        ));
    }

    let canonical_root =
        fs::canonicalize(supplied_root).map_err(|error| {
            filesystem_error(
                "workspace.layout.canonicalize_root",
                supplied_root,
                error,
            )
        })?;

    let root_metadata =
        fs::metadata(&canonical_root).map_err(|error| {
            filesystem_error(
                "workspace.layout.inspect_root",
                &canonical_root,
                error,
            )
        })?;

    if !root_metadata.is_dir() {
        return Err(invalid_layout_input(
            "project root must identify a directory",
        )
        .with_context(canonical_root.display().to_string()));
    }

    let root_entries = read_sorted_entries(&canonical_root)?;

    let root_source_files = collect_direct_source_files(
        &canonical_root,
        &root_entries,
        options.source_extension(),
    )?;

    let mut source_roots = Vec::new();

    for relative_root in options.source_roots() {
        let candidate = canonical_root.join(relative_root);

        match fs::symlink_metadata(&candidate) {
            Ok(metadata) => {
                // Configured source roots must be real directories.
                // Symbolic links are deliberately not followed.
                if metadata.file_type().is_symlink() {
                    continue;
                }

                if !metadata.is_dir() {
                    continue;
                }

                let canonical_source_root =
                    fs::canonicalize(&candidate).map_err(|error| {
                        filesystem_error(
                            "workspace.layout.canonicalize_source_root",
                            &candidate,
                            error,
                        )
                    })?;

                ensure_contained(
                    &canonical_root,
                    &canonical_source_root,
                )?;

                let entries =
                    read_sorted_entries(&canonical_source_root)?;

                let files = collect_direct_source_files(
                    &canonical_source_root,
                    &entries,
                    options.source_extension(),
                )?;

                source_roots.push(SourceRoot::new(
                    relative_root.clone(),
                    canonical_source_root,
                    files,
                ));
            }

            Err(error)
                if error.kind() == io::ErrorKind::NotFound =>
            {
                // An absent optional source directory is not an error.
                continue;
            }

            Err(error) => {
                return Err(filesystem_error(
                    "workspace.layout.inspect_source_root",
                    &candidate,
                    error,
                ));
            }
        }
    }

    let executable_entry_points = recognize_entry_points(
        &canonical_root,
        options.executable_entry_points(),
        options.source_extension(),
    )?;

    let library_entry_points = recognize_entry_points(
        &canonical_root,
        options.library_entry_points(),
        options.source_extension(),
    )?;

    let has_root_files = !root_source_files.is_empty();

    let has_directory_files = source_roots
        .iter()
        .any(|root| !root.source_files.is_empty());

    let kind = match (has_root_files, has_directory_files) {
        (true, true) => LayoutKind::Mixed,
        (true, false) => LayoutKind::Flat,
        (false, true) => LayoutKind::DirectoryBased,
        (false, false) => LayoutKind::Empty,
    };

    Ok(ProjectLayout::new(
        canonical_root,
        kind,
        source_roots,
        root_source_files,
        executable_entry_points,
        library_entry_points,
        options.source_extension().to_os_string(),
    ))
}

/// Inspect the project or workspace identified by discovery.
///
/// The nearest discovered package root is preferred. If discovery
/// found only a workspace root, the workspace root is inspected.
///
/// This function does not perform discovery itself.
///
/// # Errors
///
/// Returns an error when discovery contains neither a project root
/// nor an explicit workspace root.
pub fn inspect_discovered(
    discovery: &DiscoveryResult,
) -> Result<ProjectLayout> {
    inspect_discovered_with_options(
        discovery,
        &SourceLayoutOptions::default(),
    )
}

/// Inspect a discovered project/workspace with custom conventions.
pub fn inspect_discovered_with_options(
    discovery: &DiscoveryResult,
    options: &SourceLayoutOptions,
) -> Result<ProjectLayout> {
    let root = discovery
        .project_root()
        .or_else(|| discovery.workspace_root())
        .ok_or_else(|| {
            ToolchainError::new(
                ToolchainErrorKind::NotFound,
                "discovery result contains neither a project root nor a workspace root",
            )
            .with_operation("workspace.layout.inspect_discovered")
        })?;

    inspect_with_options(root, options)
}

// ============================================================================
// FILESYSTEM HELPERS
// ============================================================================

fn validate_options(options: &SourceLayoutOptions) -> Result<()> {
    validate_extension(options.source_extension())?;

    for path in options.source_roots() {
        validate_relative_path(path)?;
    }

    for path in options.executable_entry_points() {
        validate_relative_path(path)?;
    }

    for path in options.library_entry_points() {
        validate_relative_path(path)?;
    }

    Ok(())
}

fn collect_direct_source_files(
    directory: &Path,
    entries: &[fs::DirEntry],
    extension: &OsStr,
) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in entries {
        let path = entry.path();

        let file_type = entry.file_type().map_err(|error| {
            filesystem_error(
                "workspace.layout.inspect_source_file",
                &path,
                error,
            )
        })?;

        if !file_type.is_file()
            || !has_extension(&path, extension)
        {
            continue;
        }

        let canonical_path =
            fs::canonicalize(&path).map_err(|error| {
                filesystem_error(
                    "workspace.layout.canonicalize_source_file",
                    &path,
                    error,
                )
            })?;

        ensure_contained(directory, &canonical_path)?;

        files.push(canonical_path);
    }

    files.sort();

    Ok(files)
}

fn recognize_entry_points(
    project_root: &Path,
    candidates: &[PathBuf],
    extension: &OsStr,
) -> Result<Vec<PathBuf>> {
    let mut recognized = Vec::new();

    for relative_path in candidates {
        if !has_extension(relative_path, extension) {
            continue;
        }

        let candidate = project_root.join(relative_path);

        let metadata = match fs::symlink_metadata(&candidate) {
            Ok(metadata) => metadata,

            Err(error)
                if error.kind() == io::ErrorKind::NotFound =>
            {
                continue;
            }

            Err(error) => {
                return Err(filesystem_error(
                    "workspace.layout.inspect_entry_point",
                    &candidate,
                    error,
                ));
            }
        };

        // Entry-point symlinks are rejected rather than followed.
        if metadata.file_type().is_symlink()
            || !metadata.is_file()
        {
            continue;
        }

        let canonical_path =
            fs::canonicalize(&candidate).map_err(|error| {
                filesystem_error(
                    "workspace.layout.canonicalize_entry_point",
                    &candidate,
                    error,
                )
            })?;

        ensure_contained(project_root, &canonical_path)?;

        if !recognized.contains(&canonical_path) {
            recognized.push(canonical_path);
        }
    }

    recognized.sort();

    Ok(recognized)
}

fn read_sorted_entries(
    directory: &Path,
) -> Result<Vec<fs::DirEntry>> {
    let read_directory = fs::read_dir(directory).map_err(|error| {
        filesystem_error(
            "workspace.layout.read_directory",
            directory,
            error,
        )
    })?;

    let mut entries = Vec::new();

    for entry in read_directory {
        let entry = entry.map_err(|error| {
            filesystem_error(
                "workspace.layout.read_directory_entry",
                directory,
                error,
            )
        })?;

        entries.push(entry);
    }

    entries.sort_by_key(|entry| entry.file_name());

    Ok(entries)
}

fn has_extension(
    path: &Path,
    extension: &OsStr,
) -> bool {
    path.extension() == Some(extension)
}

fn ensure_contained(
    root: &Path,
    candidate: &Path,
) -> Result<()> {
    if !candidate.starts_with(root) {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "project layout path escapes the canonical project root",
            )
            .with_operation("workspace.layout.check_containment")
            .with_context(candidate.display().to_string())
            .with_context(root.display().to_string()),
        );
    }

    Ok(())
}

fn invalid_layout_input(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation("workspace.layout")
}

fn filesystem_error(
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
                "zamani-layout-{}-{sequence}",
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
                .expect("nested directory should be created");

            path
        }

        fn write_file(
            &self,
            relative: impl AsRef<Path>,
            contents: &str,
        ) -> PathBuf {
            let path = self.path.join(relative);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .expect("parent directory should exist");
            }

            fs::write(&path, contents)
                .expect("test file should be written");

            path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    // ------------------------------------------------------------------------
    // Default layout
    // ------------------------------------------------------------------------

    #[test]
    fn recognizes_conventional_source_directory() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "src/main.zm",
            "fn main() {}",
        );

        temporary.write_file(
            "src/lib.zm",
            "module example;",
        );

        let layout = inspect(temporary.path())
            .expect("layout inspection should succeed");

        assert_eq!(
            layout.kind(),
            LayoutKind::DirectoryBased,
        );

        assert!(layout.has_source_files());

        assert!(layout.has_executable_entry_point());

        assert!(layout.has_library_entry_point());

        assert_eq!(
            layout.source_roots().len(),
            1,
        );

        assert_eq!(
            layout.source_roots()[0].source_files().len(),
            2,
        );
    }

    #[test]
    fn recognizes_flat_project_layout() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "main.zm",
            "fn main() {}",
        );

        temporary.write_file(
            "lib.zm",
            "module example;",
        );

        let layout = inspect(temporary.path())
            .expect("layout inspection should succeed");

        assert_eq!(layout.kind(), LayoutKind::Flat);

        assert!(layout.has_executable_entry_point());

        assert!(layout.has_library_entry_point());

        assert_eq!(
            layout.root_source_files().len(),
            2,
        );
    }

    #[test]
    fn recognizes_mixed_project_layout() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "main.zm",
            "fn main() {}",
        );

        temporary.write_file(
            "src/module.zm",
            "module example;",
        );

        let layout = inspect(temporary.path())
            .expect("layout inspection should succeed");

        assert_eq!(layout.kind(), LayoutKind::Mixed);
    }

    #[test]
    fn empty_layout_is_not_automatically_invalid() {
        let temporary = TestDirectory::new();

        let layout = inspect(temporary.path())
            .expect("empty project layout should be inspectable");

        assert_eq!(layout.kind(), LayoutKind::Empty);

        assert!(!layout.has_source_files());
    }

    // ------------------------------------------------------------------------
    // Custom conventions
    // ------------------------------------------------------------------------

    #[test]
    fn supports_custom_source_directories() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "application/source/main.zm",
            "fn main() {}",
        );

        let options = SourceLayoutOptions::new()
            .with_source_roots(["application/source"])
            .expect("custom source root should be valid")
            .with_executable_entry_point(
                "application/source/main.zm",
            )
            .expect("custom entry point should be valid");

        let layout = inspect_with_options(
            temporary.path(),
            &options,
        )
        .expect("custom layout inspection should succeed");

        assert!(layout.has_executable_entry_point());

        assert_eq!(
            layout.kind(),
            LayoutKind::DirectoryBased,
        );
    }

    #[test]
    fn supports_custom_source_extensions() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "source/main.zmx",
            "custom source",
        );

        let options = SourceLayoutOptions::new()
            .with_source_roots(["source"])
            .expect("source root should be valid")
            .with_source_extension("zmx")
            .expect("extension should be valid")
            .with_executable_entry_point("source/main.zmx")
            .expect("entry point should be valid");

        let layout = inspect_with_options(
            temporary.path(),
            &options,
        )
        .expect("inspection should succeed");

        assert!(layout.has_source_files());

        assert!(layout.has_executable_entry_point());
    }

    #[test]
    fn rejects_parent_directory_escape() {
        assert!(
            SourceLayoutOptions::new()
                .with_source_root("../outside")
                .is_err()
        );

        assert!(
            SourceLayoutOptions::new()
                .with_executable_entry_point(
                    "../../outside/main.zm",
                )
                .is_err()
        );
    }

    #[test]
    fn rejects_invalid_extensions() {
        assert!(
            SourceLayoutOptions::new()
                .with_source_extension("")
                .is_err()
        );

        assert!(
            SourceLayoutOptions::new()
                .with_source_extension("../zm")
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Filesystem correctness
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_a_file_as_project_root() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "main.zm",
            "fn main() {}",
        );

        assert!(
            inspect(
                temporary.path().join("main.zm"),
            )
            .is_err()
        );
    }

    #[test]
    fn reports_missing_project_root() {
        let temporary = TestDirectory::new();

        let missing = temporary.path().join("missing");

        assert!(inspect(missing).is_err());
    }

    #[test]
    fn canonicalizes_project_root() {
        let temporary = TestDirectory::new();

        let layout = inspect(temporary.path())
            .expect("inspection should succeed");

        let expected = fs::canonicalize(temporary.path())
            .expect("root should canonicalize");

        assert_eq!(layout.project_root(), expected);
    }

    #[test]
    fn ignores_unrecognized_extensions() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "src/main.rs",
            "fn main() {}",
        );

        temporary.write_file(
            "src/readme.md",
            "documentation",
        );

        let layout = inspect(temporary.path())
            .expect("inspection should succeed");

        assert_eq!(layout.kind(), LayoutKind::Empty);
    }

    // ------------------------------------------------------------------------
    // Recursive source traversal
    // ------------------------------------------------------------------------

    #[test]
    fn visits_nested_source_files() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "src/main.zm",
            "fn main() {}",
        );

        temporary.write_file(
            "src/modules/one.zm",
            "module one;",
        );

        temporary.write_file(
            "src/modules/deep/two.zm",
            "module two;",
        );

        let layout = inspect(temporary.path())
            .expect("inspection should succeed");

        let mut visited = Vec::new();

        layout
            .visit_source_files(|path| {
                visited.push(path.to_path_buf());
                Ok(())
            })
            .expect("source traversal should succeed");

        visited.sort();

        assert_eq!(visited.len(), 3);
    }

    #[test]
    fn traversal_order_is_deterministic() {
        let temporary = TestDirectory::new();

        temporary.write_file("src/z.zm", "");
        temporary.write_file("src/a.zm", "");
        temporary.write_file("src/m.zm", "");

        let layout = inspect(temporary.path())
            .expect("inspection should succeed");

        let mut first = Vec::new();

        layout
            .visit_source_files(|path| {
                first.push(path.to_path_buf());
                Ok(())
            })
            .expect("first traversal should succeed");

        let mut second = Vec::new();

        layout
            .visit_source_files(|path| {
                second.push(path.to_path_buf());
                Ok(())
            })
            .expect("second traversal should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn traversal_propagates_callback_errors() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "src/main.zm",
            "",
        );

        let layout = inspect(temporary.path())
            .expect("inspection should succeed");

        let result = layout.visit_source_files(|_| {
            Err(ToolchainError::invalid_input(
                "intentional callback failure",
            ))
        });

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Discovery integration
    // ------------------------------------------------------------------------

    #[test]
    fn inspect_discovered_uses_project_root() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "Zamani.toml",
            r#"
[package]
name = "example"
version = "1.0.0"
"#,
        );

        temporary.write_file(
            "src/main.zm",
            "fn main() {}",
        );

        let discovery =
            super::super::discovery::discover(
                temporary.path(),
            )
            .expect("project should be discovered");

        let layout = inspect_discovered(&discovery)
            .expect("discovered layout should be inspectable");

        assert_eq!(
            layout.project_root(),
            discovery.project_root().expect("project root"),
        );

        assert!(layout.has_executable_entry_point());
    }

    #[test]
    fn supports_workspace_only_discovery() {
        let temporary = TestDirectory::new();

        temporary.write_file(
            "Zamani.toml",
            r#"
[workspace]
members = []
"#,
        );

        let discovery =
            super::super::discovery::discover(
                temporary.path(),
            )
            .expect("workspace should be discovered");

        let layout = inspect_discovered(&discovery)
            .expect("workspace layout should be inspectable");

        assert_eq!(
            layout.project_root(),
            fs::canonicalize(temporary.path())
                .expect("canonical root"),
        );
    }

    // ------------------------------------------------------------------------
    // Deduplication and API stability
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_source_roots_are_ignored() {
        let options = SourceLayoutOptions::new()
            .with_source_root("src")
            .expect("valid root")
            .with_source_root("src")
            .expect("duplicate root");

        assert_eq!(options.source_roots().len(), 1);
    }

    #[test]
    fn leading_dot_in_extension_is_normalized() {
        let options = SourceLayoutOptions::new()
            .with_source_extension(".zm")
            .expect("valid extension");

        assert_eq!(
            options.source_extension(),
            OsStr::new("zm"),
        );
    }
}
