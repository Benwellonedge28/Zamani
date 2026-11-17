
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Sources
//!
//! Path: `src/toolchain/dependencies/sources.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Implements source discovery for package dependencies. This module
//! provides a concrete local-path candidate provider and the integration
//! boundary for registry, Git, URL and future source providers.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Local path resolution relative to an explicit project directory.
//! - Canonical filesystem path handling for local package sources.
//! - Workspace path-containment policy for local dependencies.
//! - Local package manifest loading through the canonical manifest API.
//! - Conversion of local package manifests into resolver candidates.
//! - Source-provider integration guidance and source-specific errors.
//!
//! It does not own:
//!
//! - Dependency declarations or canonical dependency names.
//! - Candidate identity types or candidate inventory types.
//! - Candidate provider traits or provider registry.
//! - Version comparison or dependency solving.
//! - Feature unification or compatibility evaluation.
//! - Package archive formats or package installation.
//! - Registry protocols, authentication or credential storage.
//! - Git transport, URL transport or network access.
//! - Package signatures, trust decisions or artifact verification.
//! - Build graphs, compiler IR, target selection or hardware allocation.
//!
//! ## Existing authoritative contracts
//!
//! - `super::model` owns `DependencyName`, `DependencyDeclaration` and
//!   `DependencySource`.
//! - `super::resolver` owns `DependencyCandidateProvider`,
//!   `CandidateProviderRegistry`, `CandidateQuery`, `CandidateId`,
//!   `CandidateProviderId`, `DependencyCandidate` and `CandidateInventory`.
//! - `super::constraints` owns `CandidateVersion`, version schemes and
//!   completeness-aware candidate evidence.
//! - `crate::toolchain::manifest` adapts the canonical
//!   `crate::zamani_project_config` manifest.
//! - `crate::toolchain::errors` owns structured toolchain errors.
//!
//! This module deliberately does not create replacement versions of
//! those types.
//!
//! ## Completeness
//!
//! A local path identifies an explicit package directory. If its manifest
//! is read and validated, the provider can report a complete inventory for
//! that exact path scope.
//!
//! A registry provider must report `Partial` when pagination, authorization,
//! filtering, transport or another condition prevents it from establishing
//! a complete inventory. An unavailable source must report `Unavailable`.
//! Neither condition proves that no candidate exists.
//!
//! ## Security
//!
//! Canonicalization resolves the path used by this provider, but does not
//! by itself provide a sandbox or eliminate races caused by concurrent
//! hostile filesystem changes. The caller must apply its filesystem
//! authorization policy before invoking providers.
//!
//! This provider never reads credentials, executes processes or accesses
//! the network. A path outside the workspace is rejected by default;
//! callers can explicitly select the external-path policy.
//!
//! ## Version semantics
//!
//! The local provider records the package version exactly as declared by
//! the canonical manifest and attaches the caller-supplied version-scheme
//! identity. It does not assume SemVer or compare versions.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on dependencies, providers, paths,
//! manifest length, version text or computing domains. Memory and work
//! scale with the actual manifest and candidate data and remain subject
//! to available resources and caller-configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Integration
//!
//! Register `PathCandidateProvider` in the existing
//! `CandidateProviderRegistry` using a stable `CandidateProviderId`.
//! Register registry, Git, URL and custom providers through the same
//! `DependencyCandidateProvider` contract.
//!
//! The resolver remains the only component that chooses candidates.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::toolchain::errors::{ToolchainError, ToolchainErrorKind};
use crate::toolchain::result::Result;

use crate::toolchain::manifest::adapter::ManifestAdapter;
use crate::toolchain::manifest::ToolchainManifest;

use super::constraints::{
    CandidateVersion,
    Completeness,
    DependencyCandidateFacts,
    DependencyFeature,
    EvidenceMap,
    EvidenceSet,
    VersionSchemeId,
};

use super::model::{
    DependencyDeclaration,
    DependencyName,
    DependencySource,
};

use super::resolver::{
    CandidateInventory,
    CandidateQuery,
    DependencyCandidate,
    DependencyCandidateProvider,
};

const OPERATION_SOURCE: &str = "dependency.source";
const OPERATION_PATH: &str = "dependency.source.path";
const OPERATION_MANIFEST: &str = "dependency.source.manifest";

/// Canonical project manifest filename.
///
/// This is the existing Zamani project-manifest filename, not a
/// computational-domain or hardware limit.
pub const PROJECT_MANIFEST_FILE: &str = "Zamani.toml";

/// Policy controlling whether a local dependency may resolve outside
/// the configured workspace directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathBoundaryPolicy {
    /// Require the canonical package directory to remain inside the
    /// canonical workspace directory.
    WithinWorkspace,

    /// Permit an explicitly declared local dependency outside the
    /// workspace directory.
    ///
    /// The caller remains responsible for filesystem authorization.
    AllowOutsideWorkspace,
}

/// A provider that discovers package candidates from local paths.
///
/// The provider is deliberately independent of the dependency resolver's
/// selection policy. It only discovers candidates and reports their facts.
#[derive(Debug, Clone)]
pub struct PathCandidateProvider {
    workspace_root: PathBuf,
    boundary_policy: PathBoundaryPolicy,
    version_scheme: VersionSchemeId,
}

impl PathCandidateProvider {
    /// Create a local-path provider.
    ///
    /// `workspace_root` must identify an existing directory. Relative
    /// dependency paths are interpreted relative to this directory.
    ///
    /// `version_scheme` identifies the version comparison scheme that
    /// downstream constraint evaluators will use. The provider does not
    /// interpret that scheme.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the workspace cannot be canonicalized, or
    /// an invalid-input error if it is not a directory.
    pub fn new(
        workspace_root: impl AsRef<Path>,
        boundary_policy: PathBoundaryPolicy,
        version_scheme: VersionSchemeId,
    ) -> Result<Self> {
        let workspace_root = fs::canonicalize(workspace_root.as_ref())
            .map_err(|error| {
                ToolchainError::from_io(
                    OPERATION_PATH,
                    error,
                )
                .with_context("canonicalizing the dependency workspace root")
            })?;

        if !workspace_root.is_dir() {
            return Err(
                ToolchainError::invalid_input(
                    "dependency workspace root is not a directory",
                )
                .with_operation(OPERATION_PATH)
                .with_subject(workspace_root.display().to_string()),
            );
        }

        Ok(Self {
            workspace_root,
            boundary_policy,
            version_scheme,
        })
    }

    /// Return the canonical workspace root used for relative paths.
    #[must_use]
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Return the configured path-boundary policy.
    #[must_use]
    pub const fn boundary_policy(&self) -> PathBoundaryPolicy {
        self.boundary_policy
    }

    /// Discover candidates from a set of normalized declarations.
    ///
    /// This method is also useful to callers that need to inspect local
    /// source discovery independently of a complete resolver operation.
    ///
    /// Returns `Ok(None)` when none of the declarations names a local path.
    /// Returns a complete inventory when every explicitly declared local
    /// path has been inspected successfully.
    ///
    /// A path that cannot be inspected is an explicit error. It is never
    /// silently omitted from a supposedly complete inventory.
    pub fn discover_declarations(
        &self,
        package: &DependencyName,
        declarations: &[DependencyDeclaration],
    ) -> Result<Option<CandidateInventory>> {
        let mut declared_paths = BTreeSet::new();

        for declaration in declarations {
            declaration.validate()?;

            if declaration.name() != package {
                return Err(
                    ToolchainError::validation_failed(
                        format!(
                            "source query for '{package}' contains a declaration \
                             for '{}'",
                            declaration.name(),
                        ),
                    )
                    .with_operation(OPERATION_SOURCE),
                );
            }

            if let Some(DependencySource::Path(path)) = declaration.source() {
                declared_paths.insert(path.clone());
            }
        }

        if declared_paths.is_empty() {
            return Ok(None);
        }

        let mut canonical_roots = BTreeSet::new();

        for declared_path in declared_paths {
            let package_root = self.resolve_package_root(&declared_path)?;
            canonical_roots.insert(package_root);
        }

        let mut candidates = Vec::new();

        for package_root in canonical_roots {
            candidates.push(self.load_candidate(package, &package_root)?);
        }

        let inventory = CandidateInventory::new(
            candidates,
            Completeness::Complete,
        )?;

        Ok(Some(inventory))
    }

    fn resolve_package_root(
        &self,
        declared_path: &Path,
    ) -> Result<PathBuf> {
        if declared_path.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "local dependency path must not be empty",
                )
                .with_operation(OPERATION_PATH),
            );
        }

        let joined_path = if declared_path.is_absolute() {
            declared_path.to_path_buf()
        } else {
            self.workspace_root.join(declared_path)
        };

        let canonical_path = fs::canonicalize(&joined_path)
            .map_err(|error| {
                ToolchainError::from_io(OPERATION_PATH, error)
                    .with_subject(joined_path.display().to_string())
                    .with_context("resolving declared local dependency path")
            })?;

        let package_root = if canonical_path.is_dir() {
            canonical_path
        } else if canonical_path.is_file()
            && canonical_path.file_name().is_some_and(
                |name| name == PROJECT_MANIFEST_FILE,
            )
        {
            canonical_path
                .parent()
                .ok_or_else(|| {
                    ToolchainError::invalid_input(
                        "package manifest has no parent directory",
                    )
                    .with_operation(OPERATION_PATH)
                    .with_subject(
                        canonical_path.display().to_string(),
                    )
                })?
                .to_path_buf()
        } else {
            return Err(
                ToolchainError::invalid_input(
                    format!(
                        "local dependency path must identify a package \
                         directory or its {PROJECT_MANIFEST_FILE} file"
                    ),
                )
                .with_operation(OPERATION_PATH)
                .with_subject(canonical_path.display().to_string()),
            );
        };

        if self.boundary_policy == PathBoundaryPolicy::WithinWorkspace
            && !package_root.starts_with(&self.workspace_root)
        {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AuthenticationFailed,
                    "local dependency path is outside the permitted workspace",
                )
                .with_operation(OPERATION_PATH)
                .with_subject(package_root.display().to_string()),
            );
        }

        Ok(package_root)
    }

    fn load_candidate(
        &self,
        requested_name: &DependencyName,
        package_root: &Path,
    ) -> Result<DependencyCandidate> {
        let manifest_path = package_root.join(PROJECT_MANIFEST_FILE);

        let manifest_bytes = fs::read(&manifest_path)
            .map_err(|error| {
                ToolchainError::from_io(OPERATION_MANIFEST, error)
                    .with_subject(manifest_path.display().to_string())
                    .with_context("reading local package manifest")
            })?;

        let manifest_text = std::str::from_utf8(&manifest_bytes)
            .map_err(|error| {
                ToolchainError::invalid_input(
                    format!(
                        "package manifest is not valid UTF-8: {error}"
                    ),
                )
                .with_operation(OPERATION_MANIFEST)
                .with_subject(manifest_path.display().to_string())
            })?;

        // Use the existing canonical manifest parser and validator.
        let manifest = ToolchainManifest::parse(manifest_text)
            .map_err(|error| {
                error
                    .with_operation(OPERATION_MANIFEST)
                    .with_subject(manifest_path.display().to_string())
                    .with_context("validating local package manifest")
            })?;

        let actual_name = DependencyName::new(manifest.name().to_owned())?;

        if &actual_name != requested_name {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "local package name '{}' does not match requested \
                         dependency '{}'",
                        actual_name,
                        requested_name,
                    ),
                )
                .with_operation(OPERATION_MANIFEST)
                .with_subject(manifest_path.display().to_string()),
            );
        }

        let version = CandidateVersion::new(
            self.version_scheme.clone(),
            manifest.version().to_owned(),
        )?;

        // Adapt dependency declarations from the canonical manifest model.
        // This does not parse TOML a second time or resolve dependencies.
        let adapted = ManifestAdapter::new()
            .adapt(manifest.canonical())?;

        let dependencies = adapted
            .dependencies()
            .iter()
            .map(DependencyDeclaration::try_from)
            .collect::<Result<Vec<_>>>()?;

        let source = DependencySource::path(package_root.to_path_buf())?;

        let manifest_digest = Sha256::digest(&manifest_bytes);
        let manifest_digest_hex = hex::encode(manifest_digest);

        let mut metadata = std::collections::BTreeMap::new();
        metadata.insert(
            "zamani.source.manifest.sha256".to_owned(),
            manifest_digest_hex,
        );

        let facts = DependencyCandidateFacts::new()
            .with_version(version)
            .with_source(source)
            // The current canonical manifest adapter does not expose a
            // complete package feature inventory. Do not claim absence.
            .with_feature_inventory(EvidenceSet::unavailable())
            // The current manifest adapter does not expose a complete
            // capability inventory. Do not claim absence.
            .with_capability_inventory(EvidenceSet::unavailable())
            // The manifest digest is observed metadata, not a signature
            // or proof of package-content integrity.
            .with_metadata_inventory(EvidenceMap::partial(metadata));

        let candidate_id = CandidateIdForPath::from_parts(
            package_root,
            &manifest_bytes,
        );

        DependencyCandidate::new(
            candidate_id.into_candidate_id()?,
            actual_name,
            facts,
            dependencies,
            std::collections::BTreeMap::<
                DependencyFeature,
                Vec<DependencyDeclaration>,
            >::new(),
        )
    }
}

impl DependencyCandidateProvider for PathCandidateProvider {
    /// Discover candidates for local path declarations in the query.
    ///
    /// A query without local path declarations is not applicable to this
    /// provider and returns `Ok(None)`.
    fn discover(
        &self,
        query: &CandidateQuery,
    ) -> Result<Option<CandidateInventory>> {
        self.discover_declarations(
            query.package(),
            query.declarations(),
        )
    }
}

/// Internal stable identity derived from a canonical path and its manifest.
///
/// The candidate identity changes when either the canonical package path
/// or manifest bytes change. It is not a package-content digest and does
/// not replace archive integrity verification.
struct CandidateIdForPath(String);

impl CandidateIdForPath {
    fn from_parts(
        package_root: &Path,
        manifest_bytes: &[u8],
    ) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(b"zamani.dependency.path-candidate.v1");

        let path_bytes = platform_path_bytes(package_root.as_os_str());
        update_frame(&mut hasher, &path_bytes);
        update_frame(&mut hasher, manifest_bytes);

        let digest = hasher.finalize();

        Self(format!(
            "path:sha256:{}",
            hex::encode(digest),
        ))
    }

    fn into_candidate_id(
        self,
    ) -> Result<super::resolver::CandidateId> {
        super::resolver::CandidateId::new(self.0)
    }
}

fn update_frame(
    hasher: &mut Sha256,
    bytes: &[u8],
) {
    // u128 framing avoids truncating lengths to a fixed u32/u64 format.
    hasher.update((bytes.len() as u128).to_be_bytes());
    hasher.update(bytes);
}

/// Return a deterministic byte representation of a platform path.
///
/// This is used only to derive a local candidate identity. It does not
/// convert the path into a portable package locator.
#[cfg(unix)]
fn platform_path_bytes(value: &std::ffi::OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    value.as_bytes().to_vec()
}

/// Return a deterministic byte representation of a Windows path.
#[cfg(windows)]
fn platform_path_bytes(value: &std::ffi::OsStr) -> Vec<u8> {
    use std::os::windows::ffi::OsStrExt;

    value
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// Fallback path representation for other supported platforms.
#[cfg(not(any(unix, windows)))]
fn platform_path_bytes(value: &std::ffi::OsStr) -> Vec<u8> {
    value.to_string_lossy().as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let sequence = NEXT_TEST_DIRECTORY.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let path = std::env::temp_dir().join(format!(
                "zamani-dependency-source-{}-{sequence}",
                std::process::id(),
            ));

            fs::create_dir_all(&path)
                .expect("test directory should be created");

            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn version_scheme() -> VersionSchemeId {
        VersionSchemeId::new("zamani.opaque")
            .expect("valid version scheme")
    }

    fn package_manifest(name: &str, version: &str) -> String {
        format!(
            r#"
[package]
name = "{name}"
version = "{version}"
edition = "2021"
authors = []
description = ""
type = "library"
"#
        )
    }

    #[test]
    fn provider_ignores_declarations_without_path_sources() {
        let directory = TestDirectory::new();

        let provider = PathCandidateProvider::new(
            directory.path(),
            PathBoundaryPolicy::WithinWorkspace,
            version_scheme(),
        )
        .expect("provider should be constructed");

        let declaration = DependencyDeclaration::named(
            "org::zamani::math",
        )
        .expect("valid dependency");

        let result = provider
            .discover_declarations(
                declaration.name(),
                std::slice::from_ref(&declaration),
            )
            .expect("discovery should succeed");

        assert!(result.is_none());
    }

    #[test]
    fn provider_discovers_a_local_package() {
        let directory = TestDirectory::new();
        let package_directory = directory.path().join("math");

        fs::create_dir_all(&package_directory)
            .expect("package directory should be created");

        fs::write(
            package_directory.join(PROJECT_MANIFEST_FILE),
            package_manifest("org::zamani::math", "1.4.2"),
        )
        .expect("manifest should be written");

        let provider = PathCandidateProvider::new(
            directory.path(),
            PathBoundaryPolicy::WithinWorkspace,
            version_scheme(),
        )
        .expect("provider should be constructed");

        let declaration = DependencyDeclaration::named(
            "org::zamani::math",
        )
        .expect("valid dependency")
        .with_source(
            DependencySource::path("math")
                .expect("valid path source"),
        );

        let inventory = provider
            .discover_declarations(
                declaration.name(),
                std::slice::from_ref(&declaration),
            )
            .expect("discovery should succeed")
            .expect("path provider should apply");

        assert_eq!(inventory.candidates().len(), 1);
        assert_eq!(
            inventory.completeness(),
            Completeness::Complete,
        );

        let candidate = &inventory.candidates()[0];

        assert_eq!(
            candidate.name().as_str(),
            "org::zamani::math",
        );

        assert_eq!(
            candidate.version().map(CandidateVersion::value),
            Some("1.4.2"),
        );

        assert_eq!(
            candidate.facts().features().completeness(),
            Completeness::Unavailable,
        );

        assert_eq!(
            candidate.facts().capabilities().completeness(),
            Completeness::Unavailable,
        );

        assert!(
            candidate
                .facts()
                .metadata()
                .get("zamani.source.manifest.sha256")
                .is_some(),
        );
    }

    #[test]
    fn provider_rejects_a_manifest_with_a_different_package_name() {
        let directory = TestDirectory::new();
        let package_directory = directory.path().join("other");

        fs::create_dir_all(&package_directory)
            .expect("package directory should be created");

        fs::write(
            package_directory.join(PROJECT_MANIFEST_FILE),
            package_manifest("org::zamani::different", "1.0"),
        )
        .expect("manifest should be written");

        let provider = PathCandidateProvider::new(
            directory.path(),
            PathBoundaryPolicy::WithinWorkspace,
            version_scheme(),
        )
        .expect("provider should be constructed");

        let declaration = DependencyDeclaration::named(
            "org::zamani::expected",
        )
        .expect("valid dependency")
        .with_source(
            DependencySource::path("other")
                .expect("valid path source"),
        );

        assert!(
            provider
                .discover_declarations(
                    declaration.name(),
                    std::slice::from_ref(&declaration),
                )
                .is_err(),
        );
    }

    #[test]
    fn provider_rejects_paths_outside_the_workspace_by_default_policy() {
        let workspace = TestDirectory::new();
        let outside = TestDirectory::new();

        fs::write(
            outside.path().join(PROJECT_MANIFEST_FILE),
            package_manifest("org::zamani::outside", "1.0"),
        )
        .expect("manifest should be written");

        let provider = PathCandidateProvider::new(
            workspace.path(),
            PathBoundaryPolicy::WithinWorkspace,
            version_scheme(),
        )
        .expect("provider should be constructed");

        let declaration = DependencyDeclaration::named(
            "org::zamani::outside",
        )
        .expect("valid dependency")
        .with_source(
            DependencySource::path(outside.path())
                .expect("valid path source"),
        );

        assert!(
            provider
                .discover_declarations(
                    declaration.name(),
                    std::slice::from_ref(&declaration),
                )
                .is_err(),
        );
    }

    #[test]
    fn candidate_identity_changes_when_manifest_changes() {
        let directory = TestDirectory::new();

        let first = CandidateIdForPath::from_parts(
            directory.path(),
            b"manifest version one",
        );

        let second = CandidateIdForPath::from_parts(
            directory.path(),
            b"manifest version two",
        );

        assert_ne!(first.0, second.0);
    }

    #[test]
    fn candidate_identity_is_stable_for_identical_inputs() {
        let directory = TestDirectory::new();

        let first = CandidateIdForPath::from_parts(
            directory.path(),
            b"same manifest",
        );

        let second = CandidateIdForPath::from_parts(
            directory.path(),
            b"same manifest",
        );

        assert_eq!(first.0, second.0);
    }
}
