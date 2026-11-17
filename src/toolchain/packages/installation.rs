
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Installation
//!
//! Path: `src/toolchain/packages/installation.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Install one verified `.zpkg` package into an explicitly selected
//! installation destination.
//!
//! Installation is streaming and transactional at the package-directory
//! publication boundary:
//!
//! 1. Validate the requested package identity and destination.
//! 2. Reserve the destination against cooperating installers.
//! 3. Decode and verify the package manifest.
//! 4. Verify the expected package identity and exact release version.
//! 5. Extract and verify each declared package entry.
//! 6. Verify that the complete package inventory was installed.
//! 7. Publish the completed package through the extraction subsystem.
//! 8. Return an immutable installation report.
//!
//! The destination is not published when manifest, entry, inventory,
//! archive or extraction verification fails.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Package installation requests.
//! - Installation options.
//! - Installation verification orchestration.
//! - Destination reservation for cooperating installers.
//! - Package identity and release-version matching.
//! - Complete package-inventory verification.
//! - Installation reports.
//!
//! This module does not own:
//!
//! - Package identity allocation.
//! - Package model definitions.
//! - Package manifest syntax or serialization.
//! - Archive framing.
//! - Archive downloading.
//! - Package dependency resolution.
//! - Registry authentication.
//! - Cryptographic algorithms.
//! - Signature creation or verification algorithms.
//! - Publisher trust policy.
//! - Package extraction mechanics.
//! - Compiler execution.
//! - Backend selection.
//! - Hardware discovery.
//! - Runtime execution.
//! - Deployment or resource allocation.
//!
//! ## Authoritative integration
//!
//! Package model:
//! `crate::toolchain::packages::model`
//!
//! Archive:
//! `crate::toolchain::packages::archive`
//!
//! Extraction:
//! `crate::toolchain::packages::extraction`
//!
//! Package identities:
//! `crate::toolchain::ids`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Canonical manifest syntax:
//! `grammar/modules/packages.g4`
//!
//! Canonical dependency declarations:
//! `grammar/modules/dependencies.g4`
//!
//! Canonical source manifest:
//! `crate::zamani_project_config`
//!
//! ## Security
//!
//! Installation never trusts an archive merely because it can be decoded.
//!
//! The caller must provide a PackageInstallationVerifier backed by the
//! canonical package metadata, integrity, signature and trust authorities.
//!
//! A successful manifest verification must establish:
//!
//! - Valid package metadata.
//! - Valid package structure.
//! - Expected package identity.
//! - Expected release version.
//! - Applicable signature and trust requirements.
//! - Applicable package provenance requirements.
//!
//! Each extracted file must be checked against the corresponding package
//! content descriptor and its applicable integrity policy.
//!
//! The extraction subsystem owns archive-path validation, staging,
//! filesystem entry-type restrictions and publication.
//!
//! This module reserves destinations using exclusive file creation.
//! The reservation serializes cooperating Zamani installers targeting
//! the same destination. The installation parent directory must not be
//! writable by untrusted concurrent filesystem actors.
//!
//! A stale reservation left by abrupt process termination is not silently
//! deleted. It requires explicit recovery after the operator establishes
//! that no installation owns it.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Package size.
//! - Number of package files.
//! - Number of package artifacts.
//! - Package metadata.
//! - Package versions.
//! - Computing domains.
//! - Target realizations.
//! - Hardware resources described by package metadata.
//!
//! The archive is consumed as a stream.
//!
//! The verifier retains a set of paths already verified. Its memory use
//! therefore grows with package inventory size, not file payload size.
//!
//! Extraction budgets are supplied by the caller.
//!
//! Actual installation remains subject to filesystem, memory, storage,
//! operating-system and explicitly configured operational limits.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## POCO-REAF
//!
//! Package installation preserves logical package identity and exact
//! package release identity.
//!
//! It does not imply that a target-specific artifact is executable on
//! every machine.
//!
//! Artifact compatibility and target feasibility remain downstream
//! responsibilities of the compiler, backend, capability and runtime
//! authorities.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::Result;

use super::extraction::{
    extract_archive,
    extract_archive_with_options,
    ExtractionOptions,
    ExtractionReport,
    ExtractionVerifier,
};
use super::model::{
    Package,
    PackageIdentity,
    PackagePath,
    PackageVersion,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_INSTALL: &str = "package.install";
const OPERATION_INSTALL_VERIFY: &str = "package.install.verify";
const OPERATION_INSTALL_RESERVE: &str = "package.install.reserve";

// ============================================================================
// INSTALLATION REQUEST
// ============================================================================

/// Immutable request to install one exact package release.
///
/// The destination is explicitly selected by the caller. This module does
/// not impose a universal package-store layout or derive filesystem paths
/// from package names.
///
/// The expected package identity and version are mandatory. Installing
/// whichever package happens to be present in an archive is not permitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInstallRequest {
    expected_identity: PackageIdentity,
    expected_version: PackageVersion,
    destination: PathBuf,
}

impl PackageInstallRequest {
    /// Construct an installation request.
    ///
    /// The destination must identify a directory that does not already
    /// exist. Its parent directory must already exist.
    ///
    /// Filesystem validation is performed again immediately before
    /// installation.
    pub fn new(
        expected_identity: PackageIdentity,
        expected_version: PackageVersion,
        destination: impl Into<PathBuf>,
    ) -> Result<Self> {
        let destination = destination.into();

        if destination.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "package installation destination must not be empty",
                )
                .with_operation(OPERATION_INSTALL),
            );
        }

        if destination.file_name().is_none() {
            return Err(
                ToolchainError::invalid_input(
                    "package installation destination must name a directory",
                )
                .with_operation(OPERATION_INSTALL),
            );
        }

        Ok(Self {
            expected_identity,
            expected_version,
            destination,
        })
    }

    /// Return the expected package identity.
    #[must_use]
    pub fn expected_identity(&self) -> &PackageIdentity {
        &self.expected_identity
    }

    /// Return the expected exact release version.
    #[must_use]
    pub fn expected_version(&self) -> &PackageVersion {
        &self.expected_version
    }

    /// Return the requested installation destination.
    #[must_use]
    pub fn destination(&self) -> &Path {
        &self.destination
    }
}

// ============================================================================
// INSTALLATION OPTIONS
// ============================================================================

/// Options for one package installation operation.
///
/// Extraction resource budgets and I/O buffering remain owned by
/// `packages::extraction`.
///
/// This type deliberately reuses ExtractionOptions rather than introducing
/// a second, competing extraction-budget model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageInstallOptions {
    extraction: ExtractionOptions,
}

impl PackageInstallOptions {
    /// Construct installation options.
    #[must_use]
    pub const fn new(extraction: ExtractionOptions) -> Self {
        Self { extraction }
    }

    /// Return the extraction options.
    #[must_use]
    pub const fn extraction(self) -> ExtractionOptions {
        self.extraction
    }
}

impl Default for PackageInstallOptions {
    fn default() -> Self {
        Self {
            extraction: ExtractionOptions::default(),
        }
    }
}

// ============================================================================
// VERIFICATION AUTHORITY
// ============================================================================

/// Authoritative package verifier required by installation.
///
/// Implementations must integrate with the canonical package metadata,
/// integrity, signature, provenance and trust authorities.
///
/// This trait intentionally does not implement cryptography.
///
/// ## Manifest verification
///
/// `verify_manifest` must:
///
/// - Decode the canonical package manifest.
/// - Construct the canonical Package model.
/// - Validate its structure.
/// - Verify required signatures and trust evidence.
/// - Apply package provenance policy.
/// - Return the verified Package model.
///
/// Returning a Package is an assertion by the verifier that its required
/// manifest-level checks have succeeded.
///
/// ## Entry verification
///
/// `verify_entry` must:
///
/// - Confirm that the path belongs to the verified package inventory.
/// - Check applicable declared content digests.
/// - Apply applicable integrity and content policies.
/// - Reject content that differs from the verified package description.
///
/// The supplied file is inside the extraction subsystem's private staging
/// directory. Implementations must not modify it.
///
/// A successful callback does not independently establish that a
/// target-specific artifact is executable on a particular machine.
pub trait PackageInstallationVerifier {
    /// Decode and verify the canonical package manifest.
    fn verify_manifest(
        &mut self,
        manifest_path: &Path,
    ) -> Result<Package>;

    /// Verify one extracted package entry.
    fn verify_entry(
        &mut self,
        package: &Package,
        package_path: &PackagePath,
        staged_file: &Path,
    ) -> Result<()>;
}

// ============================================================================
// INSTALLATION REPORT
// ============================================================================

/// Immutable report describing a successfully installed package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInstallReport {
    package: Package,
    extraction: ExtractionReport,
}

impl PackageInstallReport {
    /// Return the verified installed package model.
    #[must_use]
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Return the installed package identity.
    #[must_use]
    pub fn identity(&self) -> &PackageIdentity {
        self.package.identity()
    }

    /// Return the exact installed package release version.
    #[must_use]
    pub fn version(&self) -> &PackageVersion {
        self.package.version()
    }

    /// Return the final published installation directory.
    #[must_use]
    pub fn destination(&self) -> &Path {
        self.extraction.destination()
    }

    /// Return the extraction report.
    #[must_use]
    pub fn extraction(&self) -> &ExtractionReport {
        &self.extraction
    }

    /// Return the number of installed package content entries.
    #[must_use]
    pub const fn installed_entries(&self) -> u128 {
        self.extraction.extracted_entries()
    }

    /// Return the total installed content bytes.
    #[must_use]
    pub const fn installed_content_bytes(&self) -> u128 {
        self.extraction.extracted_content_bytes()
    }
}

// ============================================================================
// DESTINATION RESERVATION
// ============================================================================

/// Exclusive reservation for one installation destination.
///
/// The lock is cooperative: all Zamani installation paths must use this
/// reservation protocol. The containing directory must be protected
/// against untrusted external writers.
///
/// The file is created with create_new, so an existing file, including a
/// symbolic link at the lock path, is never followed or overwritten.
struct DestinationReservation {
    path: PathBuf,
    _file: File,
}

impl DestinationReservation {
    /// Reserve a destination using an exclusive sibling lock file.
    fn acquire(
        parent: &Path,
        destination_name: &std::ffi::OsStr,
    ) -> Result<Self> {
        let mut lock_name = OsString::from(".");
        lock_name.push(destination_name);
        lock_name.push(".zamani-install.lock");

        let path = parent.join(lock_name);

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| {
                if error.kind() == io::ErrorKind::AlreadyExists {
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        "another installation owns this destination reservation",
                    )
                    .with_operation(OPERATION_INSTALL_RESERVE)
                    .with_context(
                        "an existing reservation must be investigated before recovery",
                    )
                } else {
                    ToolchainError::from(error)
                        .with_operation(OPERATION_INSTALL_RESERVE)
                        .with_context(
                            "creating exclusive installation reservation",
                        )
                }
            })?;

        Ok(Self {
            path,
            _file: file,
        })
    }
}

impl Drop for DestinationReservation {
    fn drop(&mut self) {
        // Cleanup must not replace the original installation result.
        //
        // If cleanup fails, the reservation remains visible and prevents
        // another cooperating installer from silently reusing the path.
        let _ = fs::remove_file(&self.path);
    }
}

// ============================================================================
// DESTINATION VALIDATION
// ============================================================================

/// Canonical destination and its already-existing parent directory.
struct PreparedDestination {
    parent: PathBuf,
    name: OsString,
    path: PathBuf,
}

impl PreparedDestination {
    /// Validate and canonicalize a requested installation destination.
    fn prepare(destination: &Path) -> Result<Self> {
        let name = destination.file_name().ok_or_else(|| {
            ToolchainError::invalid_input(
                "installation destination must have a final path component",
            )
            .with_operation(OPERATION_INSTALL)
        })?;

        let requested_parent = destination
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));

        let parent = fs::canonicalize(requested_parent).map_err(|error| {
            ToolchainError::from(error)
                .with_operation(OPERATION_INSTALL)
                .with_context(
                    "canonicalizing installation parent directory",
                )
        })?;

        let parent_metadata = fs::metadata(&parent).map_err(|error| {
            ToolchainError::from(error)
                .with_operation(OPERATION_INSTALL)
                .with_context("inspecting installation parent directory")
        })?;

        if !parent_metadata.is_dir() {
            return Err(
                ToolchainError::invalid_input(
                    "installation parent must be an existing directory",
                )
                .with_operation(OPERATION_INSTALL),
            );
        }

        let name = name.to_os_string();
        let path = parent.join(&name);

        match fs::symlink_metadata(&path) {
            Ok(_) => {
                return Err(
                    ToolchainError::new(
                        ToolchainErrorKind::AlreadyExists,
                        "installation destination already exists",
                    )
                    .with_operation(OPERATION_INSTALL)
                    .with_subject(path.display().to_string()),
                );
            }

            Err(error)
                if error.kind() == io::ErrorKind::NotFound => {}

            Err(error) => {
                return Err(
                    ToolchainError::from(error)
                        .with_operation(OPERATION_INSTALL)
                        .with_context(
                            "checking installation destination",
                        ),
                );
            }
        }

        Ok(Self {
            parent,
            name,
            path,
        })
    }
}

// ============================================================================
// EXTRACTION VERIFIER ADAPTER
// ============================================================================

/// Adapts the installation verification authority to the extraction API.
///
/// This adapter additionally enforces:
///
/// - Exact package identity.
/// - Exact release version.
/// - Package model validation.
/// - Archive paths belonging to the package inventory.
/// - Complete inventory coverage before publication.
struct InstallationExtractionVerifier<'a, V> {
    verifier: &'a mut V,
    expected_identity: &'a PackageIdentity,
    expected_version: &'a PackageVersion,
    package: Option<Package>,
    verified_paths: BTreeSet<PackagePath>,
}

impl<'a, V> InstallationExtractionVerifier<'a, V>
where
    V: PackageInstallationVerifier,
{
    fn new(
        verifier: &'a mut V,
        expected_identity: &'a PackageIdentity,
        expected_version: &'a PackageVersion,
    ) -> Self {
        Self {
            verifier,
            expected_identity,
            expected_version,
            package: None,
            verified_paths: BTreeSet::new(),
        }
    }

    fn package(&self) -> Result<&Package> {
        self.package.as_ref().ok_or_else(|| {
            ToolchainError::new(
                ToolchainErrorKind::Internal,
                "verified package model is unavailable",
            )
            .with_operation(OPERATION_INSTALL_VERIFY)
        })
    }

    fn into_package(self) -> Result<Package> {
        self.package.ok_or_else(|| {
            ToolchainError::new(
                ToolchainErrorKind::Internal,
                "installation completed without a verified package model",
            )
            .with_operation(OPERATION_INSTALL)
        })
    }
}

impl<V> ExtractionVerifier for InstallationExtractionVerifier<'_, V>
where
    V: PackageInstallationVerifier,
{
    fn verify_manifest(
        &mut self,
        manifest_path: &Path,
    ) -> Result<()> {
        let package = self
            .verifier
            .verify_manifest(manifest_path)
            .map_err(|error| {
                error
                    .with_operation(OPERATION_INSTALL_VERIFY)
                    .with_context("verifying package manifest")
            })?;

        package.validate().map_err(|error| {
            error
                .with_operation(OPERATION_INSTALL_VERIFY)
                .with_context("validating verified package model")
        })?;

        if package.identity() != self.expected_identity {
            return Err(
                ToolchainError::validation_failed(
                    "archive package identity does not match the requested identity",
                )
                .with_operation(OPERATION_INSTALL_VERIFY),
            );
        }

        if package.version() != self.expected_version {
            return Err(
                ToolchainError::validation_failed(
                    "archive package release version does not match the requested version",
                )
                .with_operation(OPERATION_INSTALL_VERIFY),
            );
        }

        self.package = Some(package);

        Ok(())
    }

    fn verify_entry(
        &mut self,
        package_path: &PackagePath,
        staged_file: &Path,
    ) -> Result<()> {
        let package = self.package()?;

        if !package.contents().contains_key(package_path) {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "archive contains undeclared package entry `{package_path}`"
                    ),
                )
                .with_operation(OPERATION_INSTALL_VERIFY),
            );
        }

        self.verifier
            .verify_entry(package, package_path, staged_file)
            .map_err(|error| {
                error
                    .with_operation(OPERATION_INSTALL_VERIFY)
                    .with_context(format!(
                        "verifying package content `{package_path}`"
                    ))
            })?;

        if !self.verified_paths.insert(package_path.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "package entry `{package_path}` was verified more than once"
                    ),
                )
                .with_operation(OPERATION_INSTALL_VERIFY),
            );
        }

        Ok(())
    }

    /// Verify complete package inventory before extraction publication.
    ///
    /// This method requires the extraction subsystem's finalization
    /// callback described in the integration section below.
    fn finish(&mut self) -> Result<()> {
        let package = self.package()?;

        let expected = package.contents().len();
        let actual = self.verified_paths.len();

        if actual != expected {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "package archive inventory is incomplete: \
                         expected {expected} entries, verified {actual}"
                    ),
                )
                .with_operation(OPERATION_INSTALL_VERIFY),
            );
        }

        Ok(())
    }
}

// ============================================================================
// INSTALLATION API
// ============================================================================

/// Install a package using default extraction options.
///
/// The archive is consumed as a stream. The caller remains responsible
/// for obtaining that stream through a trusted local source or registry
/// provider.
///
/// # Errors
///
/// Returns an error for:
///
/// - Invalid installation requests.
/// - Existing installation destinations.
/// - Existing destination reservations.
/// - Invalid archive framing.
/// - Invalid package manifests.
/// - Package identity or version mismatch.
/// - Invalid package content inventories.
/// - Signature, trust or integrity verification failures.
/// - Extraction failures.
/// - Filesystem failures.
/// - Configured resource-budget exhaustion.
/// - Cancellation or provider errors reported by the verifier.
pub fn install_package<R, V>(
    archive: R,
    request: &PackageInstallRequest,
    verifier: &mut V,
) -> Result<PackageInstallReport>
where
    R: Read,
    V: PackageInstallationVerifier,
{
    install_package_with_options(
        archive,
        request,
        verifier,
        PackageInstallOptions::default(),
    )
}

/// Install a package with explicit extraction options.
///
/// A successful return means that the requested package identity and
/// release version were verified, every archive entry was checked against
/// the package inventory, the archive was completely consumed, and the
/// extraction subsystem published the completed package directory.
///
/// It does not mean that a target-specific artifact can execute on the
/// current machine.
pub fn install_package_with_options<R, V>(
    archive: R,
    request: &PackageInstallRequest,
    verifier: &mut V,
    options: PackageInstallOptions,
) -> Result<PackageInstallReport>
where
    R: Read,
    V: PackageInstallationVerifier,
{
    let destination =
        PreparedDestination::prepare(request.destination())?;

    // Reserve before extraction. All cooperating installation calls
    // targeting this destination must use the same reservation protocol.
    let _reservation = DestinationReservation::acquire(
        &destination.parent,
        &destination.name,
    )?;

    // Recheck after reservation. This detects a destination created
    // between the initial validation and reservation acquisition.
    match fs::symlink_metadata(&destination.path) {
        Ok(_) => {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "installation destination appeared before installation began",
                )
                .with_operation(OPERATION_INSTALL),
            );
        }

        Err(error)
            if error.kind() == io::ErrorKind::NotFound => {}

        Err(error) => {
            return Err(
                ToolchainError::from(error)
                    .with_operation(OPERATION_INSTALL)
                    .with_context(
                        "rechecking installation destination",
                    ),
            );
        }
    }

    let mut extraction_verifier =
        InstallationExtractionVerifier::new(
            verifier,
            request.expected_identity(),
            request.expected_version(),
        );

    let extraction = extract_archive_with_options(
        archive,
        &destination.path,
        &mut extraction_verifier,
        options.extraction(),
    )?;

    // The extraction subsystem must invoke ExtractionVerifier::finish
    // before publication. This second check protects the report contract.
    let package = extraction_verifier.into_package()?;

    if extraction.extracted_entries()
        != package.contents().len() as u128
    {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::Internal,
                "published extraction inventory differs from verified package inventory",
            )
            .with_operation(OPERATION_INSTALL),
        );
    }

    Ok(PackageInstallReport {
        package,
        extraction,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn temporary_directory() -> PathBuf {
        let sequence =
            TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);

        let path = std::env::temp_dir().join(format!(
            "zamani-package-install-test-{}-{sequence}",
            std::process::id(),
        ));

        fs::create_dir(&path)
            .expect("create temporary test directory");

        path
    }

    #[test]
    fn default_install_options_reuse_extraction_defaults() {
        let options = PackageInstallOptions::default();

        assert_eq!(
            options.extraction(),
            ExtractionOptions::default(),
        );
    }

    #[test]
    fn destination_reservation_prevents_cooperating_collision() {
        let root = temporary_directory();

        let first = DestinationReservation::acquire(
            &root,
            std::ffi::OsStr::new("package"),
        )
        .expect("first reservation");

        let second = DestinationReservation::acquire(
            &root,
            std::ffi::OsStr::new("package"),
        );

        assert!(second.is_err());

        drop(first);

        let third = DestinationReservation::acquire(
            &root,
            std::ffi::OsStr::new("package"),
        );

        assert!(third.is_ok());

        drop(third);

        fs::remove_dir_all(root)
            .expect("remove temporary test directory");
    }

    #[test]
    fn existing_destination_is_rejected() {
        let root = temporary_directory();
        let destination = root.join("installed");

        fs::create_dir(&destination)
            .expect("create existing destination");

        let result = PreparedDestination::prepare(&destination);

        assert!(result.is_err());

        fs::remove_dir_all(root)
            .expect("remove temporary test directory");
    }

    #[test]
    fn destination_parent_must_exist() {
        let root = temporary_directory();
        let destination = root
            .join("missing-parent")
            .join("installed");

        let result = PreparedDestination::prepare(&destination);

        assert!(result.is_err());

        fs::remove_dir_all(root)
            .expect("remove temporary test directory");
    }

    #[test]
    fn destination_reservation_is_removed_on_drop() {
        let root = temporary_directory();

        let lock = DestinationReservation::acquire(
            &root,
            std::ffi::OsStr::new("package"),
        )
        .expect("reservation");

        let lock_path = lock.path.clone();

        assert!(lock_path.exists());

        drop(lock);

        assert!(!lock_path.exists());

        fs::remove_dir_all(root)
            .expect("remove temporary test directory");
    }
}
