
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Manifest Loading
//!
//! Path: `src/toolchain/manifest/loading.rs`
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Loads a Zamani project manifest from an explicitly supplied filesystem
//! path and delegates TOML interpretation to the canonical
//! `crate::zamani_project_config::ZamaniToml` parser.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Explicit manifest-file loading.
//! - Explicit project-directory loading.
//! - Manifest path canonicalization.
//! - Regular-file verification.
//! - UTF-8 file reading.
//! - Invocation of the canonical manifest parser.
//! - The loaded-manifest result and its canonical file path.
//! - Translation of filesystem and parser failures into toolchain errors.
//!
//! This module does not own:
//!
//! - The Zamani project-manifest schema.
//! - TOML parsing or serialization.
//! - Manifest field validation.
//! - Manifest-version compatibility.
//! - Workspace discovery or workspace-member expansion.
//! - Dependency resolution or fetching.
//! - Package identity allocation.
//! - Build planning or compilation.
//! - Target discovery or feasibility.
//! - Hardware discovery or allocation.
//! - Runtime execution or deployment.
//!
//! ## Canonical integration
//!
//! The canonical parser is:
//!
//! `crate::zamani_project_config::ZamaniToml::parse_from_str`
//!
//! This loader must not introduce a second TOML parser or a competing
//! representation of the project manifest.
//!
//! The resulting `ZamaniToml` is passed to:
//!
//! - `manifest::adapter` for deterministic toolchain-facing adaptation.
//! - `manifest::validation` for complete manifest validation.
//! - `manifest::compatibility` for manifest-version compatibility.
//! - `workspace` for workspace operations when the canonical schema
//!   provides the required declarations.
//!
//! ## Path semantics
//!
//! The caller must supply either:
//!
//! - The explicit path to a manifest file; or
//! - A project directory and an explicit manifest filename.
//!
//! This module never assumes that the process working directory is the
//! project root.
//!
//! The loaded file path is canonicalized and retained in the result.
//! The file is opened after canonicalization and its opened-file metadata
//! must identify a regular file.
//!
//! The loader does not change the process working directory.
//!
//! ## Error semantics
//!
//! Filesystem failures retain their original `std::io::Error` as a
//! causal source through `ToolchainError::from_io`.
//!
//! Parser failures are reported as manifest validation failures and
//! include the parser's diagnostic text as contextual information.
//!
//! A failed load never returns a partially initialized manifest.
//!
//! ## Security boundary
//!
//! - No unsafe Rust.
//! - No shell execution.
//! - No process execution.
//! - No network access.
//! - No manifest expression evaluation.
//! - No implicit dependency fetching.
//! - No credential handling.
//! - No silent fallback to another manifest.
//! - No silent truncation.
//!
//! Canonicalization and regular-file verification do not eliminate every
//! time-of-check/time-of-use race on a filesystem concurrently modified by
//! an adversary. Callers loading manifests from untrusted shared locations
//! must also enforce appropriate filesystem permissions or use a stronger
//! platform-specific, handle-relative filesystem boundary.
//!
//! ## Important canonical-parser limitation
//!
//! The current `ZamaniToml::parse_from_str` implementation is permissive:
//! some malformed or wrongly typed fields are defaulted or discarded.
//! This loader deliberately does not duplicate TOML parsing to compensate.
//! Strict type checking, unknown-field policy, workspace schema support,
//! and lossless parser diagnostics must be implemented in the canonical
//! `src/zamani_project_config/` subsystem.
//!
//! Until that parser contract is corrected, successful loading means that
//! the canonical parser returned a `ZamaniToml`; it does not by itself
//! certify that every source field was strictly validated.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - Manifest file size.
//! - Number of dependencies.
//! - Number of authors.
//! - Number of feature declarations.
//! - Number of workspace members.
//! - Number of targets or computing domains.
//! - Hardware-resource counts.
//!
//! The loader reads the complete manifest because the existing canonical
//! parser accepts a string. Its memory consumption is therefore
//! proportional to the manifest's byte size plus the parsed representation.
//! This is subject to available memory, address-space limits, filesystem
//! capabilities, and any explicit operational budget enforced by a caller.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Freeze boundary
//!
//! This file depends on the stable public `ZamaniToml::parse_from_str`
//! contract and the shared toolchain error/result contracts.
//!
//! Changes to the manifest schema should be made in
//! `src/zamani_project_config/`. Such changes should not require this
//! loader to know the meaning of newly introduced manifest fields.

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;
use crate::zamani_project_config::ZamaniToml;

use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const LOAD_OPERATION: &str = "manifest.loading.load";
const LOAD_DIRECTORY_OPERATION: &str = "manifest.loading.load_from_directory";
const PARSE_OPERATION: &str = "manifest.loading.parse";

// ============================================================================
// LOADED MANIFEST
// ============================================================================

/// A successfully loaded canonical Zamani manifest and its canonical path.
///
/// The manifest remains the canonical `ZamaniToml` type owned by
/// `crate::zamani_project_config`. This wrapper adds filesystem provenance
/// without redefining the manifest schema.
///
/// Construct values through [`ManifestLoader`].
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedManifest {
    canonical_path: PathBuf,
    manifest: ZamaniToml,
}

impl LoadedManifest {
    /// Return the canonical absolute path from which the manifest was read.
    ///
    /// This path identifies the file location resolved during loading.
    /// It is not a permanent content identity or a cryptographic
    /// fingerprint.
    #[must_use]
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    /// Return the parsed canonical manifest.
    #[must_use]
    pub fn manifest(&self) -> &ZamaniToml {
        &self.manifest
    }

    /// Consume the loaded result and return its canonical manifest.
    ///
    /// The canonical path is discarded.
    #[must_use]
    pub fn into_manifest(self) -> ZamaniToml {
        self.manifest
    }

    /// Consume the loaded result and return both its canonical path
    /// and parsed manifest.
    #[must_use]
    pub fn into_parts(self) -> (PathBuf, ZamaniToml) {
        (self.canonical_path, self.manifest)
    }
}

// ============================================================================
// MANIFEST LOADER
// ============================================================================

/// Stateless loader for canonical Zamani project manifests.
///
/// The loader has no implicit project root, current-directory dependency,
/// global mutable state, network client, process runner, or filesystem cache.
///
/// It can be shared by CLI, IDE, automation, CI, and agent-facing toolchain
/// services. Each call operates on the path supplied by that caller.
#[derive(Debug, Default, Clone, Copy)]
pub struct ManifestLoader;

impl ManifestLoader {
    /// Construct a stateless manifest loader.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Load a manifest from an explicitly supplied file path.
    ///
    /// The path may be relative or absolute. Relative paths are interpreted
    /// by the operating system relative to the process working directory;
    /// callers that require deterministic project selection should supply
    /// an absolute path or a path derived from an already discovered
    /// project root.
    ///
    /// The returned path is canonicalized.
    ///
    /// This method:
    ///
    /// 1. Rejects an empty path.
    /// 2. Canonicalizes the supplied path.
    /// 3. Opens the canonical path.
    /// 4. Verifies that the opened object is a regular file.
    /// 5. Reads the complete file as UTF-8.
    /// 6. Calls the canonical `ZamaniToml::parse_from_str` parser.
    /// 7. Returns the parsed manifest and canonical path.
    ///
    /// It does not search parent directories or choose another manifest
    /// when the requested path is missing or invalid.
    ///
    /// # Errors
    ///
    /// Returns a structured toolchain error when:
    ///
    /// - The supplied path is empty.
    /// - The path cannot be canonicalized.
    /// - The path cannot be opened or read.
    /// - The opened object is not a regular file.
    /// - The file is not valid UTF-8.
    /// - The canonical manifest parser rejects its contents.
    pub fn load(
        &self,
        manifest_path: impl AsRef<Path>,
    ) -> Result<LoadedManifest> {
        let requested_path = manifest_path.as_ref();

        if requested_path.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "manifest path must not be empty",
                )
                .with_operation(LOAD_OPERATION),
            );
        }

        let canonical_path = fs::canonicalize(requested_path).map_err(
            |source| {
                ToolchainError::from_io(LOAD_OPERATION, source)
                    .with_context(format!(
                        "requested manifest path: {}",
                        requested_path.display()
                    ))
            },
        )?;

        let mut file = File::open(&canonical_path).map_err(|source| {
            ToolchainError::from_io(LOAD_OPERATION, source)
                .with_context(format!(
                    "canonical manifest path: {}",
                    canonical_path.display()
                ))
        })?;

        let metadata = file.metadata().map_err(|source| {
            ToolchainError::from_io(LOAD_OPERATION, source)
                .with_context(format!(
                    "unable to inspect opened manifest: {}",
                    canonical_path.display()
                ))
        })?;

        if !metadata.is_file() {
            return Err(
                ToolchainError::validation_failed(
                    "the supplied manifest path does not identify a regular file",
                )
                .with_operation(LOAD_OPERATION)
                .with_context(format!(
                    "canonical manifest path: {}",
                    canonical_path.display()
                )),
            );
        }

        let mut contents = String::new();

        file.read_to_string(&mut contents).map_err(|source| {
            ToolchainError::from_io(LOAD_OPERATION, source)
                .with_context(format!(
                    "unable to read manifest as UTF-8: {}",
                    canonical_path.display()
                ))
        })?;

        let manifest = ZamaniToml::parse_from_str(&contents).map_err(
            |diagnostic| {
                ToolchainError::validation_failed(
                    "the canonical Zamani manifest parser rejected the file",
                )
                .with_operation(PARSE_OPERATION)
                .with_context(format!(
                    "manifest path: {}",
                    canonical_path.display()
                ))
                .with_context(format!(
                    "canonical parser diagnostic: {diagnostic}"
                ))
            },
        )?;

        Ok(LoadedManifest {
            canonical_path,
            manifest,
        })
    }

    /// Load a manifest from a project directory and explicit filename.
    ///
    /// The filename must be exactly one ordinary filename component.
    /// It cannot be empty, `.`, `..`, contain a path separator, or contain
    /// a NUL byte.
    ///
    /// The filename is supplied explicitly rather than silently assuming
    /// a particular project layout. Callers using the conventional
    /// `Zamani.toml` name may pass that name directly or obtain the
    /// convention from the workspace discovery subsystem.
    ///
    /// The directory is not searched recursively. No parent directories
    /// are examined. The selected file is loaded through [`Self::load`],
    /// so all canonicalization, file-type, UTF-8, parser and error rules
    /// remain identical.
    ///
    /// # Errors
    ///
    /// Returns an error when the directory path is empty, the filename is
    /// invalid, or loading the selected manifest fails.
    pub fn load_from_directory(
        &self,
        project_directory: impl AsRef<Path>,
        manifest_file_name: impl AsRef<str>,
    ) -> Result<LoadedManifest> {
        let directory = project_directory.as_ref();
        let file_name = manifest_file_name.as_ref();

        if directory.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "project directory must not be empty",
                )
                .with_operation(LOAD_DIRECTORY_OPERATION),
            );
        }

        validate_manifest_file_name(file_name)?;

        let manifest_path = directory.join(file_name);

        self.load(&manifest_path).map_err(|error| {
            error
                .with_operation(LOAD_DIRECTORY_OPERATION)
                .with_context(format!(
                    "project directory: {}",
                    directory.display()
                ))
                .with_context(format!(
                    "manifest filename: {file_name}"
                ))
        })
    }
}

// ============================================================================
// INPUT VALIDATION
// ============================================================================

fn validate_manifest_file_name(file_name: &str) -> Result<()> {
    let path = Path::new(file_name);

    let mut components = path.components();

    let is_one_normal_component = matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(_)), None)
    );

    let contains_separator = file_name.contains('/')
        || file_name.contains('\\');

    let invalid = file_name.is_empty()
        || file_name == "."
        || file_name == ".."
        || file_name.contains('\0')
        || contains_separator
        || !is_one_normal_component;

    if invalid {
        return Err(
            ToolchainError::invalid_input(
                "manifest filename must be one nonempty ordinary filename component",
            )
            .with_operation(LOAD_DIRECTORY_OPERATION)
            .with_context(format!(
                "received filename: {file_name:?}"
            )),
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{ManifestLoader, validate_manifest_file_name};

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use crate::toolchain::errors::ToolchainErrorKind;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

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
                "zamani-manifest-loading-{}-{sequence}",
                std::process::id()
            ));

            fs::create_dir(&path)
                .expect("test directory should be created");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write_manifest(&self, contents: &str) -> PathBuf {
            let path = self.path.join("Zamani.toml");

            fs::write(&path, contents)
                .expect("test manifest should be written");

            path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn valid_manifest() -> &'static str {
        r#"
[package]
name = "loading-example"
version = "1.0.0"
edition = "2021"
description = "Manifest loader test"
type = "application"

[dependencies]
math = ">=1"
"#
    }

    #[test]
    fn constructs_a_stateless_loader() {
        let _loader = ManifestLoader::new();
    }

    #[test]
    fn loads_a_manifest_from_an_explicit_file_path() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(valid_manifest());

        let loaded = ManifestLoader::new()
            .load(&manifest_path)
            .expect("valid manifest should load");

        assert_eq!(
            loaded.manifest().package.name,
            "loading-example"
        );

        assert_eq!(
            loaded.canonical_path(),
            fs::canonicalize(&manifest_path)
                .expect("manifest path should canonicalize")
        );
    }

    #[test]
    fn loads_a_manifest_from_a_project_directory() {
        let directory = TestDirectory::new();
        directory.write_manifest(valid_manifest());

        let loaded = ManifestLoader::new()
            .load_from_directory(
                directory.path(),
                "Zamani.toml",
            )
            .expect("valid project manifest should load");

        assert_eq!(
            loaded.manifest().package.name,
            "loading-example"
        );
    }

    #[test]
    fn preserves_the_canonical_manifest_type() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(valid_manifest());

        let loaded = ManifestLoader::new()
            .load(&manifest_path)
            .expect("valid manifest should load");

        let manifest: &crate::zamani_project_config::ZamaniToml =
            loaded.manifest();

        assert_eq!(manifest.package.version, "1.0.0");
    }

    #[test]
    fn consumes_into_the_canonical_manifest() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(valid_manifest());

        let manifest = ManifestLoader::new()
            .load(&manifest_path)
            .expect("valid manifest should load")
            .into_manifest();

        assert_eq!(manifest.package.name, "loading-example");
    }

    #[test]
    fn consumes_into_path_and_manifest_parts() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(valid_manifest());

        let (canonical_path, manifest) = ManifestLoader::new()
            .load(&manifest_path)
            .expect("valid manifest should load")
            .into_parts();

        assert!(canonical_path.is_absolute());
        assert_eq!(manifest.package.name, "loading-example");
    }

    #[test]
    fn rejects_an_empty_manifest_path() {
        let result = ManifestLoader::new().load("");

        let error = result.expect_err("empty path must be rejected");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::InvalidInput
        );
    }

    #[test]
    fn rejects_a_missing_manifest_file() {
        let directory = TestDirectory::new();
        let missing_path = directory.path().join("missing.toml");

        let result = ManifestLoader::new().load(&missing_path);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_a_directory_as_a_manifest_file() {
        let directory = TestDirectory::new();

        let result = ManifestLoader::new().load(directory.path());

        let error = result.expect_err(
            "a directory must not be accepted as a manifest file"
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ValidationFailed
        );
    }

    #[test]
    fn rejects_invalid_toml() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(
            "[package\nname = 'invalid'",
        );

        let result = ManifestLoader::new().load(&manifest_path);

        let error = result.expect_err(
            "invalid TOML must be rejected by the canonical parser"
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ValidationFailed
        );

        assert_eq!(
            error.operation(),
            Some("manifest.loading.parse")
        );
    }

    #[test]
    fn rejects_a_manifest_without_the_required_package_table() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(
            "[dependencies]\nmath = '1'",
        );

        let result = ManifestLoader::new().load(&manifest_path);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_utf8() {
        let directory = TestDirectory::new();
        let manifest_path = directory.path().join("Zamani.toml");

        fs::write(
            &manifest_path,
            [0xFF_u8, 0xFE_u8, 0xFD_u8],
        )
        .expect("invalid UTF-8 fixture should be written");

        let result = ManifestLoader::new().load(&manifest_path);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_an_empty_project_directory() {
        let result = ManifestLoader::new()
            .load_from_directory("", "Zamani.toml");

        let error = result.expect_err(
            "empty project directory must be rejected"
        );

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::InvalidInput
        );
    }

    #[test]
    fn rejects_an_empty_manifest_filename() {
        assert!(validate_manifest_file_name("").is_err());
    }

    #[test]
    fn rejects_current_directory_as_a_filename() {
        assert!(validate_manifest_file_name(".").is_err());
    }

    #[test]
    fn rejects_parent_directory_as_a_filename() {
        assert!(validate_manifest_file_name("..").is_err());
    }

    #[test]
    fn rejects_path_separators_in_a_filename() {
        assert!(
            validate_manifest_file_name("nested/Zamani.toml")
                .is_err()
        );

        assert!(
            validate_manifest_file_name("nested\\Zamani.toml")
                .is_err()
        );
    }

    #[test]
    fn rejects_nul_in_a_filename() {
        assert!(
            validate_manifest_file_name("Zamani\0.toml")
                .is_err()
        );
    }

    #[test]
    fn accepts_a_single_ordinary_manifest_filename() {
        assert!(
            validate_manifest_file_name("Zamani.toml").is_ok()
        );

        assert!(
            validate_manifest_file_name("project.manifest").is_ok()
        );
    }

    #[test]
    fn does_not_change_the_process_working_directory() {
        let directory = TestDirectory::new();
        let manifest_path = directory.write_manifest(valid_manifest());

        let before = std::env::current_dir()
            .expect("current directory should be available");

        let _loaded = ManifestLoader::new()
            .load(&manifest_path)
            .expect("valid manifest should load");

        let after = std::env::current_dir()
            .expect("current directory should remain available");

        assert_eq!(before, after);
    }

    #[test]
    fn loading_does_not_require_dependency_resolution() {
        let directory = TestDirectory::new();

        let manifest_path = directory.write_manifest(
            r#"
[package]
name = "unresolved-dependencies"
version = "1.0.0"
edition = "2021"

[dependencies]
future.package = ">=1"
"#,
        );

        let loaded = ManifestLoader::new()
            .load(&manifest_path)
            .expect("loading must not fetch or resolve dependencies");

        assert_eq!(
            loaded.manifest().dependencies.len(),
            1
        );
    }
}
