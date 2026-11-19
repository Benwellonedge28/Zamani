
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Test Fixtures
//!
//! Path: `src/toolchain/testing/fixtures.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Provides isolated, reusable filesystem fixtures for toolchain tests.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Temporary fixture-directory lifecycle.
//! - Safe relative fixture-file creation.
//! - Canonical manifest fixture parsing.
//! - Canonical manifest adaptation.
//! - Zamani project fixtures.
//! - Local dependency fixtures.
//! - Package source-tree fixtures.
//! - Deterministic manifest test-data generation.
//!
//! It does not own:
//!
//! - The canonical Zamani manifest schema or TOML parser.
//! - Dependency resolution.
//! - Package archive serialization.
//! - Package installation.
//! - Registry communication.
//! - Compilation.
//! - Backend execution.
//! - Hardware discovery or allocation.
//! - Runtime execution.
//!
//! ## Canonical integration
//!
//! Manifest:
//!     crate::zamani_project_config
//!
//! Manifest adaptation:
//!     crate::toolchain::manifest::adapter
//!
//! Errors:
//!     crate::toolchain::errors
//!
//! Results:
//!     crate::toolchain::result
//!
//! Compiler:
//!     crate::compiler
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::zur
//!
//! Grammar:
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/antlr/ZamaniLexer.g4
//!     grammar/core/program.g4
//!
//! ## Domain independence
//!
//! Fixtures may contain any source text or file layout supported by
//! their intended test.
//!
//! Computing-domain identifiers are not enumerated or restricted here.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Fixture directories.
//! - Project files.
//! - Dependencies.
//! - Package files.
//! - Source length.
//! - Computing domains.
//! - Hardware descriptions.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Distributed participants.
//!
//! Actual operations are limited by available operating-system,
//! filesystem and process resources.
//!
//! ## Security
//!
//! - Unsafe Rust is prohibited.
//! - Absolute fixture paths are rejected.
//! - Parent-directory traversal is rejected.
//! - Existing fixture files are not silently overwritten.
//! - File creation uses exclusive creation.
//! - Manifest content is parsed by the canonical parser.
//! - No shell, network or external process is invoked.
//! - Temporary fixture directories are cleaned up on drop.
//!
//! ## Determinism
//!
//! Manifest generation sorts dependency declarations.
//!
//! Fixture source and package contents are supplied explicitly.
//!
//! Temporary directory names are operational identifiers and are
//! not part of fixture content or reproducibility fingerprints.

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::manifest::adapter::{
    AdaptedManifest,
    ManifestAdapter,
};

use crate::toolchain::result::Result;

use crate::zamani_project_config::ZamaniToml;

use std::collections::BTreeSet;
use std::fs::{
    self,
    File,
    OpenOptions,
};
use std::io::{
    self,
    Read,
    Write,
};
use std::path::{
    Component,
    Path,
    PathBuf,
};
use std::sync::atomic::{
    AtomicU64,
    Ordering,
};
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const CREATE_DIRECTORY_OPERATION: &str =
    "testing.fixtures.create_directory";

const CREATE_FILE_OPERATION: &str =
    "testing.fixtures.create_file";

const PARSE_MANIFEST_OPERATION: &str =
    "testing.fixtures.parse_manifest";

const CREATE_PROJECT_OPERATION: &str =
    "testing.fixtures.create_project";

const CREATE_PACKAGE_OPERATION: &str =
    "testing.fixtures.create_package";

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn io_error(
    operation: &'static str,
    path: &Path,
    error: io::Error,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Io,
        format!(
            "fixture filesystem operation failed for {}",
            path.display()
        ),
    )
    .with_operation(operation)
    .with_source(error)
}

fn invalid_fixture_input(
    operation: &'static str,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(operation)
}

fn validate_text(
    field: &str,
    value: &str,
    operation: &'static str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(invalid_fixture_input(
            operation,
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_fixture_input(
            operation,
            format!("{field} must not contain control characters"),
        ));
    }

    if value != value.trim() {
        return Err(invalid_fixture_input(
            operation,
            format!("{field} must not contain surrounding whitespace"),
        ));
    }

    Ok(())
}

// ============================================================================
// RELATIVE PATH VALIDATION
// ============================================================================

/// Validate a path intended to remain inside a fixture directory.
///
/// Only ordinary relative path components are accepted.
///
/// Absolute paths, parent traversal, root components and platform
/// prefixes are rejected.
///
/// The path is not canonicalized because the destination may not
/// exist yet.
fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(invalid_fixture_input(
            CREATE_FILE_OPERATION,
            "fixture file path must not be empty",
        ));
    }

    if path.is_absolute() {
        return Err(invalid_fixture_input(
            CREATE_FILE_OPERATION,
            "absolute fixture paths are not permitted",
        ));
    }

    let mut has_normal_component = false;

    for component in path.components() {
        match component {
            Component::Normal(value) => {
                if value.is_empty() {
                    return Err(invalid_fixture_input(
                        CREATE_FILE_OPERATION,
                        "empty fixture path components are not permitted",
                    ));
                }

                has_normal_component = true;
            }

            Component::CurDir => {
                return Err(invalid_fixture_input(
                    CREATE_FILE_OPERATION,
                    "explicit current-directory components are not permitted",
                ));
            }

            Component::ParentDir => {
                return Err(invalid_fixture_input(
                    CREATE_FILE_OPERATION,
                    "parent-directory traversal is not permitted",
                ));
            }

            Component::RootDir | Component::Prefix(_) => {
                return Err(invalid_fixture_input(
                    CREATE_FILE_OPERATION,
                    "root and platform-prefix components are not permitted",
                ));
            }
        }
    }

    if !has_normal_component {
        return Err(invalid_fixture_input(
            CREATE_FILE_OPERATION,
            "fixture path must contain a normal path component",
        ));
    }

    Ok(())
}

// ============================================================================
// TEMPORARY FIXTURE DIRECTORY
// ============================================================================

/// An isolated temporary directory owned by a fixture.
///
/// The directory and its contents are removed when this value is dropped.
///
/// Call [`FixtureDirectory::persist`] when a test needs to retain
/// its fixture for inspection after the owner is dropped.
///
/// Directory names are generated independently of fixture contents.
#[derive(Debug)]
pub struct FixtureDirectory {
    root: PathBuf,
    cleanup: bool,
}

impl FixtureDirectory {
    /// Create a new isolated temporary fixture directory.
    ///
    /// `label` is descriptive and is not interpreted as a path.
    ///
    /// No fixed retry count or fixture count is imposed.
    ///
    /// # Errors
    ///
    /// Returns an error if the operating system cannot create
    /// the fixture directory.
    pub fn create(label: &str) -> Result<Self> {
        validate_text(
            "fixture directory label",
            label,
            CREATE_DIRECTORY_OPERATION,
        )?;

        let parent = std::env::temp_dir();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::Internal,
                    "system clock is earlier than the Unix epoch",
                )
                .with_operation(CREATE_DIRECTORY_OPERATION)
                .with_source(error)
            })?
            .as_nanos();

        let process_id = std::process::id();

        let mut attempt = 0_u64;

        loop {
            let name = format!(
                "zamani-fixture-{process_id}-{timestamp}-{attempt}-{label}"
            );

            let root = parent.join(name);

            match fs::create_dir(&root) {
                Ok(()) => {
                    return Ok(Self {
                        root,
                        cleanup: true,
                    });
                }

                Err(error)
                    if error.kind() == io::ErrorKind::AlreadyExists =>
                {
                    attempt = attempt.checked_add(1).ok_or_else(|| {
                        ToolchainError::new(
                            ToolchainErrorKind::ResourceExhausted,
                            "fixture directory identity space exhausted",
                        )
                        .with_operation(CREATE_DIRECTORY_OPERATION)
                    })?;
                }

                Err(error) => {
                    return Err(io_error(
                        CREATE_DIRECTORY_OPERATION,
                        &root,
                        error,
                    ));
                }
            }
        }
    }

    /// Return the fixture root directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Return whether automatic cleanup is enabled.
    #[must_use]
    pub const fn cleanup_enabled(&self) -> bool {
        self.cleanup
    }

    /// Disable automatic cleanup.
    ///
    /// This transfers responsibility for removing the directory
    /// to the caller.
    pub fn persist(mut self) -> PathBuf {
        self.cleanup = false;
        self.root.clone()
    }

    /// Create a new file inside the fixture directory.
    ///
    /// Existing files are never overwritten.
    ///
    /// Parent directories are created when necessary.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid paths, existing files,
    /// filesystem failures or unsuccessful writes.
    pub fn create_file(
        &self,
        relative_path: impl AsRef<Path>,
        contents: &[u8],
    ) -> Result<PathBuf> {
        let relative_path = relative_path.as_ref();

        validate_relative_path(relative_path)?;

        let destination = self.root.join(relative_path);

        let parent = destination.parent().ok_or_else(|| {
            invalid_fixture_input(
                CREATE_FILE_OPERATION,
                "fixture file must have a parent directory",
            )
        })?;

        fs::create_dir_all(parent).map_err(|error| {
            io_error(
                CREATE_FILE_OPERATION,
                parent,
                error,
            )
        })?;

        // Reject symlinks in existing parent components.
        self.validate_existing_parent_components(relative_path)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&destination)
            .map_err(|error| {
                io_error(
                    CREATE_FILE_OPERATION,
                    &destination,
                    error,
                )
            })?;

        if let Err(error) = file.write_all(contents) {
            drop(file);
            let _ = fs::remove_file(&destination);

            return Err(io_error(
                CREATE_FILE_OPERATION,
                &destination,
                error,
            ));
        }

        file.flush().map_err(|error| {
            io_error(
                CREATE_FILE_OPERATION,
                &destination,
                error,
            )
        })?;

        Ok(destination)
    }

    /// Create a UTF-8 text file inside the fixture directory.
    pub fn create_text_file(
        &self,
        relative_path: impl AsRef<Path>,
        contents: &str,
    ) -> Result<PathBuf> {
        self.create_file(
            relative_path,
            contents.as_bytes(),
        )
    }

    /// Read an existing fixture file.
    ///
    /// The path must satisfy the same relative-path restrictions
    /// as file creation.
    pub fn read_file(
        &self,
        relative_path: impl AsRef<Path>,
    ) -> Result<Vec<u8>> {
        let relative_path = relative_path.as_ref();

        validate_relative_path(relative_path)?;

        let path = self.root.join(relative_path);

        self.validate_existing_parent_components(relative_path)?;

        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| {
                io_error(
                    CREATE_FILE_OPERATION,
                    &path,
                    error,
                )
            })?;

        if metadata.file_type().is_symlink()
            || !metadata.is_file()
        {
            return Err(invalid_fixture_input(
                CREATE_FILE_OPERATION,
                "fixture input must be a regular file, not a symlink or directory",
            ));
        }

        let mut file = File::open(&path)
            .map_err(|error| {
                io_error(
                    CREATE_FILE_OPERATION,
                    &path,
                    error,
                )
            })?;

        let mut contents = Vec::new();

        file.read_to_end(&mut contents)
            .map_err(|error| {
                io_error(
                    CREATE_FILE_OPERATION,
                    &path,
                    error,
                )
            })?;

        Ok(contents)
    }

    /// Read an existing fixture file as UTF-8.
    pub fn read_text_file(
        &self,
        relative_path: impl AsRef<Path>,
    ) -> Result<String> {
        let bytes = self.read_file(relative_path)?;

        String::from_utf8(bytes).map_err(|error| {
            ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "fixture file does not contain valid UTF-8",
            )
            .with_operation(CREATE_FILE_OPERATION)
            .with_source(error)
        })
    }

    fn validate_existing_parent_components(
        &self,
        relative_path: &Path,
    ) -> Result<()> {
        let mut current = self.root.clone();

        let components: Vec<_> = relative_path
            .components()
            .collect();

        for component in components.iter().take(
            components.len().saturating_sub(1),
        ) {
            let Component::Normal(name) = component else {
                return Err(invalid_fixture_input(
                    CREATE_FILE_OPERATION,
                    "invalid fixture path component",
                ));
            };

            current.push(name);

            let metadata = fs::symlink_metadata(&current)
                .map_err(|error| {
                    io_error(
                        CREATE_FILE_OPERATION,
                        &current,
                        error,
                    )
                })?;

            if metadata.file_type().is_symlink()
                || !metadata.is_dir()
            {
                return Err(invalid_fixture_input(
                    CREATE_FILE_OPERATION,
                    "fixture parent must be a real directory, not a symlink",
                ));
            }
        }

        Ok(())
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        if self.cleanup {
            // Cleanup is best-effort. Drop must not panic.
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

// ============================================================================
// DEPENDENCY SPECIFICATION
// ============================================================================

/// A dependency declaration used to generate a canonical test manifest.
///
/// Version constraints are preserved as text and interpreted by
/// the dependency resolver, not by this fixture module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencySpec {
    name: String,
    version: Option<String>,
    path: Option<PathBuf>,
    features: BTreeSet<String>,
}

impl DependencySpec {
    /// Create a registry-style dependency declaration.
    pub fn versioned(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self> {
        let name = name.into();
        let version = version.into();

        validate_text(
            "dependency name",
            &name,
            CREATE_PROJECT_OPERATION,
        )?;

        validate_text(
            "dependency version requirement",
            &version,
            CREATE_PROJECT_OPERATION,
        )?;

        Ok(Self {
            name,
            version: Some(version),
            path: None,
            features: BTreeSet::new(),
        })
    }

    /// Create a local path dependency declaration.
    ///
    /// The path is preserved as manifest data. Its actual existence,
    /// containment and compatibility are checked by the dependency
    /// source resolver.
    pub fn local(
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Result<Self> {
        let name = name.into();
        let path = path.into();

        validate_text(
            "dependency name",
            &name,
            CREATE_PROJECT_OPERATION,
        )?;

        if path.as_os_str().is_empty() {
            return Err(invalid_fixture_input(
                CREATE_PROJECT_OPERATION,
                "local dependency path must not be empty",
            ));
        }

        if path.is_absolute() {
            return Err(invalid_fixture_input(
                CREATE_PROJECT_OPERATION,
                "fixture local dependency path must be relative",
            ));
        }

        Ok(Self {
            name,
            version: None,
            path: Some(path),
            features: BTreeSet::new(),
        })
    }

    /// Attach a version requirement to a local dependency.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self> {
        let version = version.into();

        validate_text(
            "dependency version requirement",
            &version,
            CREATE_PROJECT_OPERATION,
        )?;

        self.version = Some(version);

        Ok(self)
    }

    /// Add a requested dependency feature.
    ///
    /// Feature ordering in generated manifests is deterministic.
    pub fn with_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Result<Self> {
        let feature = feature.into();

        validate_text(
            "dependency feature",
            &feature,
            CREATE_PROJECT_OPERATION,
        )?;

        if !self.features.insert(feature.clone()) {
            return Err(invalid_fixture_input(
                CREATE_PROJECT_OPERATION,
                format!("duplicate dependency feature: {feature}"),
            ));
        }

        Ok(self)
    }

    /// Return the dependency name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the optional version requirement.
    #[must_use]
    pub fn version_requirement(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Return the optional local path.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Return the requested features in deterministic order.
    #[must_use]
    pub fn features(&self) -> impl Iterator<Item = &str> {
        self.features.iter().map(String::as_str)
    }
}

// ============================================================================
// MANIFEST GENERATION
// ============================================================================

/// Canonically quote a string for use in a TOML basic string.
///
/// This function only serializes fixture metadata. It is not a
/// replacement for the canonical TOML parser.
fn toml_quote(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);

    output.push('"');

    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),

            value if value.is_control() => {
                use std::fmt::Write as _;

                let _ = write!(
                    output,
                    "\\u{:04X}",
                    value as u32
                );
            }

            value => output.push(value),
        }
    }

    output.push('"');

    output
}

/// Generate a deterministic Zamani.toml fixture.
///
/// Dependencies are emitted in lexicographic order.
///
/// The resulting TOML is always passed through the canonical parser
/// before a fixture is considered successfully constructed.
fn generate_manifest(
    package_name: &str,
    version: &str,
    package_type: &str,
    dependencies: &[DependencySpec],
) -> Result<String> {
    validate_text(
        "package name",
        package_name,
        CREATE_PROJECT_OPERATION,
    )?;

    validate_text(
        "package version",
        version,
        CREATE_PROJECT_OPERATION,
    )?;

    validate_text(
        "package type",
        package_type,
        CREATE_PROJECT_OPERATION,
    )?;

    let mut ordered: Vec<&DependencySpec> =
        dependencies.iter().collect();

    ordered.sort_unstable_by(|left, right| {
        left.name.cmp(&right.name)
    });

    for pair in ordered.windows(2) {
        if pair[0].name == pair[1].name {
            return Err(invalid_fixture_input(
                CREATE_PROJECT_OPERATION,
                format!(
                    "duplicate dependency declaration: {}",
                    pair[0].name
                ),
            ));
        }
    }

    let mut output = String::new();

    output.push_str("[package]\n");
    output.push_str(&format!(
        "name = {}\n",
        toml_quote(package_name)
    ));
    output.push_str(&format!(
        "version = {}\n",
        toml_quote(version)
    ));
    output.push_str("edition = \"2021\"\n");
    output.push_str(&format!(
        "description = {}\n",
        toml_quote("Zamani toolchain test fixture")
    ));
    output.push_str(&format!(
        "type = {}\n",
        toml_quote(package_type)
    ));

    if !ordered.is_empty() {
        output.push_str("\n[dependencies]\n");

        for dependency in ordered {
            output.push_str(&toml_quote(&dependency.name));
            output.push_str(" = { ");

            let mut fields = Vec::new();

            if let Some(version) = &dependency.version {
                fields.push(format!(
                    "version = {}",
                    toml_quote(version)
                ));
            }

            if let Some(path) = &dependency.path {
                fields.push(format!(
                    "path = {}",
                    toml_quote(&path.to_string_lossy())
                ));
            }

            if !dependency.features.is_empty() {
                let features = dependency
                    .features
                    .iter()
                    .map(|feature| toml_quote(feature))
                    .collect::<Vec<_>>()
                    .join(", ");

                fields.push(format!(
                    "features = [{features}]"
                ));
            }

            output.push_str(&fields.join(", "));
            output.push_str(" }\n");
        }
    }

    Ok(output)
}

// ============================================================================
// MANIFEST FIXTURE
// ============================================================================

/// A fixture containing canonical Zamani manifest data.
///
/// The manifest is parsed using `ZamaniToml::parse_from_str`.
///
/// Adaptation is delegated to `ManifestAdapter`.
#[derive(Debug, Clone)]
pub struct ManifestFixture {
    source: String,
    parsed: ZamaniToml,
    adapted: AdaptedManifest,
}

impl ManifestFixture {
    /// Parse and validate a manifest fixture.
    ///
    /// This operation uses the canonical repository parser and
    /// canonical toolchain adapter.
    pub fn parse(source: impl Into<String>) -> Result<Self> {
        let source = source.into();

        let parsed = ZamaniToml::parse_from_str(&source)
            .map_err(|message| {
                ToolchainError::new(
                    ToolchainErrorKind::ValidationFailed,
                    message,
                )
                .with_operation(PARSE_MANIFEST_OPERATION)
            })?;

        let adapted = ManifestAdapter::adapt_manifest(&parsed)
            .map_err(|error| {
                error.with_context(
                    "while adapting a fixture through the canonical manifest adapter",
                )
            })?;

        Ok(Self {
            source,
            parsed,
            adapted,
        })
    }

    /// Return the original TOML fixture text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Return the canonical parsed manifest.
    #[must_use]
    pub fn parsed(&self) -> &ZamaniToml {
        &self.parsed
    }

    /// Return the canonical adapted manifest.
    #[must_use]
    pub fn adapted(&self) -> &AdaptedManifest {
        &self.adapted
    }

    /// Return the canonical package name.
    #[must_use]
    pub fn package_name(&self) -> &str {
        &self.parsed.package.name
    }

    /// Return the canonical package version.
    #[must_use]
    pub fn package_version(&self) -> &str {
        &self.parsed.package.version
    }

    /// Return the number of direct manifest dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.adapted.dependency_count()
    }
}

// ============================================================================
// PACKAGE FIXTURE
// ============================================================================

/// A package source-tree fixture.
///
/// This represents source files and canonical package metadata
/// before package archive creation.
///
/// It does not fabricate a `.zpkg` archive or claim that the
/// package subsystem has successfully serialized or verified one.
#[derive(Debug)]
pub struct PackageFixture {
    directory: FixtureDirectory,
    manifest: ManifestFixture,
}

impl PackageFixture {
    /// Create a package source-tree fixture.
    ///
    /// `files` contains relative file paths and their UTF-8 contents.
    ///
    /// A canonical `Zamani.toml` is generated automatically.
    ///
    /// File creation is exclusive: duplicate paths are errors.
    pub fn create<I, P, S>(
        package_name: &str,
        version: &str,
        files: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = (P, S)>,
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let directory = FixtureDirectory::create("package")?;

        let manifest_source = generate_manifest(
            package_name,
            version,
            "library",
            &[],
        )?;

        let manifest = ManifestFixture::parse(
            manifest_source.clone(),
        )?;

        directory.create_text_file(
            "Zamani.toml",
            &manifest_source,
        )?;

        for (path, contents) in files {
            let path = path.as_ref();

            if path == Path::new("Zamani.toml") {
                return Err(invalid_fixture_input(
                    CREATE_PACKAGE_OPERATION,
                    "package file list must not replace the canonical Zamani.toml",
                ));
            }

            directory.create_text_file(
                path,
                contents.as_ref(),
            )?;
        }

        Ok(Self {
            directory,
            manifest,
        })
    }

    /// Return the package source-tree root.
    #[must_use]
    pub fn root(&self) -> &Path {
        self.directory.root()
    }

    /// Return the canonical manifest fixture.
    #[must_use]
    pub fn manifest(&self) -> &ManifestFixture {
        &self.manifest
    }

    /// Read a package source file.
    pub fn read_file(
        &self,
        relative_path: impl AsRef<Path>,
    ) -> Result<Vec<u8>> {
        self.directory.read_file(relative_path)
    }

    /// Return the underlying fixture directory.
    #[must_use]
    pub fn directory(&self) -> &FixtureDirectory {
        &self.directory
    }
}

// ============================================================================
// DEPENDENCY FIXTURE
// ============================================================================

/// A local dependency package fixture.
///
/// It is a package source tree whose root can be referenced from
/// another project's manifest using a relative path.
#[derive(Debug)]
pub struct DependencyFixture {
    package: PackageFixture,
}

impl DependencyFixture {
    /// Create a local dependency package.
    pub fn create<I, P, S>(
        package_name: &str,
        version: &str,
        files: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = (P, S)>,
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        Ok(Self {
            package: PackageFixture::create(
                package_name,
                version,
                files,
            )?,
        })
    }

    /// Return the dependency package root.
    #[must_use]
    pub fn root(&self) -> &Path {
        self.package.root()
    }

    /// Return the dependency package manifest.
    #[must_use]
    pub fn manifest(&self) -> &ManifestFixture {
        self.package.manifest()
    }

    /// Borrow the underlying package fixture.
    #[must_use]
    pub fn package(&self) -> &PackageFixture {
        &self.package
    }
}

// ============================================================================
// PROJECT FIXTURE
// ============================================================================

/// A complete isolated Zamani project fixture.
///
/// The project contains:
///
/// - `Zamani.toml`.
/// - A caller-supplied source file.
/// - Zero or more declared dependencies.
///
/// The fixture does not assert that its source compiles.
#[derive(Debug)]
pub struct ProjectFixture {
    directory: FixtureDirectory,
    manifest: ManifestFixture,
    source_path: PathBuf,
}

impl ProjectFixture {
    /// Create a project with no declared dependencies.
    ///
    /// The source text is supplied by the caller so that fixtures
    /// remain aligned with the actual canonical Zamani grammar.
    pub fn create(
        package_name: &str,
        version: &str,
        source: &str,
    ) -> Result<Self> {
        Self::create_with_dependencies(
            package_name,
            version,
            source,
            &[],
        )
    }

    /// Create a project with explicit dependency declarations.
    ///
    /// Dependencies are sorted when generating the manifest.
    ///
    /// Local dependency paths are preserved as declarations.
    /// Their existence and compatibility are not inferred.
    pub fn create_with_dependencies(
        package_name: &str,
        version: &str,
        source: &str,
        dependencies: &[DependencySpec],
    ) -> Result<Self> {
        let directory = FixtureDirectory::create("project")?;

        let manifest_source = generate_manifest(
            package_name,
            version,
            "application",
            dependencies,
        )?;

        let manifest = ManifestFixture::parse(
            manifest_source.clone(),
        )?;

        directory.create_text_file(
            "Zamani.toml",
            &manifest_source,
        )?;

        let source_path = PathBuf::from("src/main.zm");

        directory.create_text_file(
            &source_path,
            source,
        )?;

        Ok(Self {
            directory,
            manifest,
            source_path,
        })
    }

    /// Return the project root.
    #[must_use]
    pub fn root(&self) -> &Path {
        self.directory.root()
    }

    /// Return the manifest fixture.
    #[must_use]
    pub fn manifest(&self) -> &ManifestFixture {
        &self.manifest
    }

    /// Return the relative source-file path.
    #[must_use]
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    /// Return the absolute source-file path.
    #[must_use]
    pub fn absolute_source_path(&self) -> PathBuf {
        self.root().join(&self.source_path)
    }

    /// Read the project's main source file.
    pub fn source(&self) -> Result<String> {
        self.directory.read_text_file(&self.source_path)
    }

    /// Return the project fixture directory.
    #[must_use]
    pub fn directory(&self) -> &FixtureDirectory {
        &self.directory
    }

    /// Return the project manifest path.
    #[must_use]
    pub fn manifest_path(&self) -> PathBuf {
        self.root().join("Zamani.toml")
    }

    /// Preserve the project directory after this fixture is dropped.
    ///
    /// The caller becomes responsible for cleanup.
    pub fn persist(self) -> PathBuf {
        self.directory.persist()
    }
}

// ============================================================================
// FIXTURE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_reads_fixture_files() {
        let directory = FixtureDirectory::create("file-test")
            .expect("fixture directory should be created");

        directory
            .create_text_file(
                "src/example.zm",
                "fixture source",
            )
            .expect("fixture source should be created");

        let source = directory
            .read_text_file("src/example.zm")
            .expect("fixture source should be readable");

        assert_eq!(source, "fixture source");
    }

    #[test]
    fn rejects_parent_directory_traversal() {
        let directory = FixtureDirectory::create("path-test")
            .expect("fixture directory should be created");

        let result = directory.create_text_file(
            "../outside.zm",
            "not permitted",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_absolute_fixture_paths() {
        let directory = FixtureDirectory::create("absolute-test")
            .expect("fixture directory should be created");

        let result = directory.create_text_file(
            std::env::temp_dir().join("outside.zm"),
            "not permitted",
        );

        assert!(result.is_err());
    }

    #[test]
    fn refuses_to_overwrite_existing_files() {
        let directory = FixtureDirectory::create("overwrite-test")
            .expect("fixture directory should be created");

        directory
            .create_text_file("source.zm", "first")
            .expect("first creation should succeed");

        let result = directory.create_text_file(
            "source.zm",
            "second",
        );

        assert!(result.is_err());

        assert_eq!(
            directory
                .read_text_file("source.zm")
                .expect("original should remain"),
            "first",
        );
    }

    #[test]
    fn parses_a_canonical_manifest() {
        let source = generate_manifest(
            "fixture_project",
            "1.0.0",
            "application",
            &[],
        )
        .expect("manifest generation should succeed");

        let manifest = ManifestFixture::parse(source)
            .expect("canonical manifest should parse");

        assert_eq!(
            manifest.package_name(),
            "fixture_project",
        );

        assert_eq!(
            manifest.package_version(),
            "1.0.0",
        );

        assert_eq!(manifest.dependency_count(), 0);
    }

    #[test]
    fn manifest_dependencies_are_deterministic() {
        let dependencies = vec![
            DependencySpec::versioned(
                "zeta",
                "2",
            )
            .expect("valid dependency"),

            DependencySpec::versioned(
                "alpha",
                "1",
            )
            .expect("valid dependency"),
        ];

        let first = generate_manifest(
            "deterministic",
            "1.0.0",
            "application",
            &dependencies,
        )
        .expect("first manifest should generate");

        let second = generate_manifest(
            "deterministic",
            "1.0.0",
            "application",
            &dependencies,
        )
        .expect("second manifest should generate");

        assert_eq!(first, second);

        let manifest = ManifestFixture::parse(first)
            .expect("generated manifest should parse");

        assert_eq!(
            manifest.adapted().dependencies()[0].name(),
            "alpha",
        );

        assert_eq!(
            manifest.adapted().dependencies()[1].name(),
            "zeta",
        );
    }

    #[test]
    fn supports_local_path_dependencies() {
        let dependency = DependencySpec::local(
            "local_math",
            "../local_math",
        )
        .expect("local dependency should be valid");

        let manifest_source = generate_manifest(
            "application",
            "1.0.0",
            "application",
            &[dependency],
        )
        .expect("manifest should generate");

        let manifest = ManifestFixture::parse(manifest_source)
            .expect("canonical adapter should accept path dependency");

        let adapted = manifest
            .adapted()
            .dependency("local_math")
            .expect("dependency should exist");

        assert_eq!(
            adapted.path(),
            Some(Path::new("../local_math")),
        );
    }

    #[test]
    fn rejects_duplicate_dependency_declarations() {
        let dependencies = vec![
            DependencySpec::versioned(
                "math",
                "1",
            )
            .expect("first dependency"),

            DependencySpec::versioned(
                "math",
                "2",
            )
            .expect("second dependency"),
        ];

        assert!(
            generate_manifest(
                "application",
                "1.0.0",
                "application",
                &dependencies,
            )
            .is_err()
        );
    }

    #[test]
    fn creates_a_project_with_caller_supplied_source() {
        let project = ProjectFixture::create(
            "classical_fixture",
            "1.0.0",
            "",
        )
        .expect("project fixture should be created");

        assert!(project.manifest_path().is_file());

        assert!(project.absolute_source_path().is_file());

        assert_eq!(
            project.source().expect("source should be readable"),
            "",
        );
    }

    #[test]
    fn creates_a_package_source_tree() {
        let package = PackageFixture::create(
            "math_library",
            "1.0.0",
            [
                (
                    "src/lib.zm",
                    "caller supplied library source",
                ),
                (
                    "README.md",
                    "Package fixture documentation",
                ),
            ],
        )
        .expect("package fixture should be created");

        assert!(
            package.root().join("Zamani.toml").is_file()
        );

        assert_eq!(
            package.manifest().package_name(),
            "math_library",
        );

        assert_eq!(
            package
                .read_file("src/lib.zm")
                .expect("package source should be readable"),
            b"caller supplied library source",
        );
    }

    #[test]
    fn creates_a_local_dependency_package() {
        let dependency = DependencyFixture::create(
            "local_math",
            "1.0.0",
            [(
                "src/lib.zm",
                "dependency source",
            )],
        )
        .expect("dependency fixture should be created");

        assert!(
            dependency.root().join("Zamani.toml").is_file()
        );

        assert_eq!(
            dependency.manifest().package_name(),
            "local_math",
        );
    }

    #[test]
    fn package_files_cannot_replace_the_manifest() {
        let result = PackageFixture::create(
            "protected_package",
            "1.0.0",
            [(
                "Zamani.toml",
                "invalid replacement",
            )],
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_package_files_are_rejected() {
        let result = PackageFixture::create(
            "duplicate_package",
            "1.0.0",
            [
                ("src/lib.zm", "first"),
                ("src/lib.zm", "second"),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn fixture_contents_do_not_depend_on_temporary_path() {
        let first = ProjectFixture::create(
            "same_project",
            "1.0.0",
            "same source",
        )
        .expect("first project");

        let second = ProjectFixture::create(
            "same_project",
            "1.0.0",
            "same source",
        )
        .expect("second project");

        assert_ne!(first.root(), second.root());

        assert_eq!(
            first.manifest().source(),
            second.manifest().source(),
        );

        assert_eq!(
            first.source().expect("first source"),
            second.source().expect("second source"),
        );
    }
}
