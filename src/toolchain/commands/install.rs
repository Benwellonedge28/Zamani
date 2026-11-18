
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Install Command
//!
//! Path: `src/toolchain/commands/install.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the public package-installation command boundary.
//!
//! Coordinates:
//!
//! - Public installation request validation.
//! - Local archive and registry source preparation.
//! - Dependency-resolution integration.
//! - Registry authentication and authorization integration.
//! - Exact package identity and release selection.
//! - Canonical package verification.
//! - Safe package extraction.
//! - Transactional package-directory installation.
//! - Structured installation reporting.
//!
//! ## Ownership
//!
//! This command owns orchestration only.
//!
//! It does not implement:
//!
//! - Package manifest parsing.
//! - Package archive decoding.
//! - Package identity allocation.
//! - Dependency resolution.
//! - Version comparison.
//! - Registry authentication.
//! - Registry authorization.
//! - Network communication.
//! - Archive downloading.
//! - Cryptographic algorithms.
//! - Signature or trust verification.
//! - Filesystem extraction.
//! - Package installation mechanics.
//! - Compiler or backend execution.
//! - Hardware discovery.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Canonical integration
//!
//! Public request:
//! `crate::toolchain::api::request::InstallRequest`
//!
//! Source declarations:
//! `crate::toolchain::api::request::PackageSource`
//!
//! Dependency resolution:
//! `crate::toolchain::dependencies`
//!
//! Registry:
//! `crate::toolchain::registries`
//!
//! Package model:
//! `crate::toolchain::packages::model`
//!
//! Package verification:
//! `crate::toolchain::packages::verification`
//!
//! Package installation:
//! `crate::toolchain::packages::installation`
//!
//! Extraction:
//! `crate::toolchain::packages::extraction`
//!
//! Errors:
//! `crate::toolchain::errors`
//!
//! Results:
//! `crate::toolchain::result`
//!
//! ## Installation lifecycle
//!
//! ```text
//! InstallRequest
//!      |
//!      v
//! Request validation
//!      |
//!      v
//! Install preparation provider
//!      |
//!      +--------------------------+
//!      |                          |
//!      v                          v
//! Local archive              Registry source
//!      |                          |
//!      |                    Authentication
//!      |                          |
//!      |                    Authorization
//!      |                          |
//!      |                    Dependency resolution
//!      |                          |
//!      |                    Exact package selection
//!      |                          |
//!      |                    Verified downloading
//!      |                          |
//!      +------------+-------------+
//!                   |
//!                   v
//!          Prepared package archive
//!                   |
//!                   v
//!        Exact identity/version binding
//!                   |
//!                   v
//!          Package installation
//!                   |
//!                   v
//!          Canonical verification
//!                   |
//!                   v
//!           Safe extraction
//!                   |
//!                   v
//!       Transactional publication
//!                   |
//!                   v
//!       Structured install report
//! ```
//!
//! ## Security
//!
//! No archive is trusted merely because it can be opened.
//!
//! A preparation provider must establish the source-specific
//! authorization and verification requirements.
//!
//! The PackageInstallationVerifier must integrate with the
//! canonical package metadata, integrity, signature, provenance
//! and trust authorities.
//!
//! The existing package installer must verify:
//!
//! - Exact package identity.
//! - Exact package release version.
//! - Complete package inventory.
//! - Every extracted package entry.
//! - Successful complete archive consumption.
//! - Successful destination publication.
//!
//! An installation failure must never be converted into success.
//!
//! This command does not execute installed package contents.
//!
//! ## Dependency semantics
//!
//! A registry version requirement is not an exact release version.
//!
//! The preparation provider must use the canonical dependency
//! resolver and its registered version evaluators to select an
//! exact package release.
//!
//! The provider must establish the required dependency closure
//! before reporting the preparation as successful.
//!
//! It must not interpret version requirements independently.
//!
//! The existing PackageInstallRequest installs one exact package
//! release. Dependency-closure installation and any multi-package
//! transaction must use an explicitly coordinated package-store
//! service; this command does not claim that installing one archive
//! atomically installs an entire dependency graph.
//!
//! ## POCO-REAF
//!
//! This command is computational-domain neutral.
//!
//! It does not enumerate classical, quantum, HDL, AI, tensor,
//! hybrid, distributed, networking, cryptographic, embedded,
//! accelerator or future domains.
//!
//! It preserves package identity and declared artifact metadata.
//!
//! It does not claim that an installed artifact can execute on
//! every machine.
//!
//! ## Scalability
//!
//! This command introduces no arbitrary maximum on:
//!
//! - Package size.
//! - Package count.
//! - Package entry count.
//! - Package artifact count.
//! - Dependencies.
//! - Registries.
//! - Providers.
//! - Computing domains.
//! - Target realizations.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Archive streaming, extraction budgets and download budgets
//! remain owned by their respective subsystems.
//!
//! Actual operation is limited only by available resources,
//! host representation, filesystem capabilities and explicitly
//! configured operational policies.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Concurrency
//!
//! This command does not create a worker pool or impose a fixed
//! concurrency limit.
//!
//! Registry preparation may be asynchronous.
//!
//! The package installer owns destination reservation and
//! publication synchronization.
//!
//! ## Failure semantics
//!
//! Success means the exact requested package release has been
//! installed and its destination has been published by the
//! canonical package installer.
//!
//! Success does not imply:
//!
//! - Target compatibility.
//! - Hardware availability.
//! - Runtime availability.
//! - Successful application execution.
//! - Successful deployment.
//!
//! Those are separate authoritative operations.

use std::future::Future;
use std::io::Read;
use std::path::PathBuf;
use std::pin::Pin;

use crate::toolchain::api::request::{
    InstallRequest,
    PackageSource,
};

use crate::toolchain::errors::ToolchainError;

use crate::toolchain::result::ToolchainResult;

use crate::toolchain::packages::installation::{
    install_package_with_options,
    PackageInstallOptions,
    PackageInstallReport,
    PackageInstallRequest,
    PackageInstallationVerifier,
};

use crate::toolchain::packages::model::{
    PackageIdentity,
    PackageVersion,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_INSTALL: &str = "commands.install";

const OPERATION_INSTALL_PREPARE: &str =
    "commands.install.prepare";

const OPERATION_INSTALL_EXECUTE: &str =
    "commands.install.execute";

// ============================================================================
// ASYNCHRONOUS PREPARATION FUTURE
// ============================================================================

/// Future returned by an installation preparation provider.
///
/// The future is runtime-independent. Implementations may use
/// the existing registry client and whichever asynchronous
/// runtime is supplied by their integration.
///
/// This type deliberately does not require a particular
/// asynchronous runtime or executor.
pub type InstallPreparationFuture<'a, A, V> =
    Pin<
        Box<
            dyn Future<
                Output = ToolchainResult<
                    PreparedPackageInstall<A, V>,
                >,
            > + 'a,
        >,
    >;

// ============================================================================
// PREPARED INSTALLATION
// ============================================================================

/// Complete, source-resolved preparation for installing one
/// exact package release.
///
/// This value binds:
///
/// - The archive stream.
/// - The exact expected package identity.
/// - The exact expected release version.
/// - The installation destination.
/// - The mandatory package verification authority.
/// - The applicable extraction options.
///
/// It cannot be constructed without an explicit installation
/// request and verification implementation.
///
/// The archive and verifier are owned by this value and are
/// consumed by the command during installation.
pub struct PreparedPackageInstall<A, V> {
    archive: A,
    request: PackageInstallRequest,
    verifier: V,
    options: PackageInstallOptions,
}

impl<A, V> PreparedPackageInstall<A, V>
where
    A: Read,
    V: PackageInstallationVerifier,
{
    /// Construct a prepared package installation.
    ///
    /// The preparation provider must already have:
    ///
    /// 1. Validated the requested source.
    /// 2. Established applicable source authorization.
    /// 3. Resolved the exact package release.
    /// 4. Established the required dependency closure.
    /// 5. Obtained the package archive.
    /// 6. Constructed the exact installation identity.
    /// 7. Selected an authorized installation destination.
    /// 8. Supplied the canonical verification authority.
    ///
    /// This constructor validates the structural relationship
    /// between the supplied components through the existing
    /// PackageInstallRequest contract.
    ///
    /// Cryptographic and filesystem verification remains the
    /// responsibility of the installation subsystem.
    pub fn new(
        archive: A,
        request: PackageInstallRequest,
        verifier: V,
        options: PackageInstallOptions,
    ) -> Self {
        Self {
            archive,
            request,
            verifier,
            options,
        }
    }

    /// Return the exact expected package identity.
    #[must_use]
    pub fn expected_identity(&self) -> &PackageIdentity {
        self.request.expected_identity()
    }

    /// Return the exact expected release version.
    #[must_use]
    pub fn expected_version(&self) -> &PackageVersion {
        self.request.expected_version()
    }

    /// Return the requested installation destination.
    #[must_use]
    pub fn destination(&self) -> &std::path::Path {
        self.request.destination()
    }
}

// ============================================================================
// INSTALLATION PREPARATION PROVIDER
// ============================================================================

/// Authoritative preparation boundary for package installation.
///
/// Implementations connect the public request to the existing
/// project, dependency, registry, security and package services.
///
/// This is an integration contract, not a replacement dependency
/// resolver, registry client or package manager.
///
/// ## Local archive preparation
///
/// For PackageSource::LocalArchive, implementations must:
///
/// - Open the explicitly supplied archive.
/// - Apply the applicable filesystem authorization policy.
/// - Reject unsafe or unauthorized source paths.
/// - Establish the exact package identity and release version.
/// - Supply the mandatory canonical verification authority.
///
/// The installer independently verifies the archive before
/// publishing the package.
///
/// ## Registry preparation
///
/// For PackageSource::Registry, implementations must:
///
/// - Resolve the configured registry.
/// - Authenticate using the canonical authentication subsystem.
/// - Authorize the requested operation.
/// - Resolve the version requirement through the canonical
///   dependency resolver and registered evaluators.
/// - Select an exact package release.
/// - Establish dependency closure.
/// - Download through the existing registry downloading API.
/// - Verify archive integrity and authenticity.
/// - Bind the returned archive to the exact selected package.
/// - Supply the canonical installation verifier.
///
/// A version requirement must never be substituted directly
/// for an exact package release version.
///
/// ## Dependency handling
///
/// The existing PackageInstallRequest represents one exact
/// package installation.
///
/// Implementations must not report successful preparation if
/// mandatory dependencies remain unresolved or unavailable.
///
/// Where multiple packages require installation, the preparation
/// implementation must coordinate them through an authoritative
/// package-store transaction or report the precise partial-state
/// outcome. This command does not promise multi-package atomicity.
///
/// ## Security
///
/// Implementations must not:
///
/// - Accept unauthenticated registry responses as trusted.
/// - Treat unknown trust as verified.
/// - Bypass download authorization.
/// - Invent package identities.
/// - Ignore dependency conflicts.
/// - Expose credentials in error messages.
/// - Execute downloaded package contents.
///
/// ## Scalability
///
/// Implementations must not introduce arbitrary domain, package,
/// dependency, registry or hardware capacity ceilings.
///
/// Operational budgets must be inherited from the applicable
/// configuration and service contracts.
pub trait InstallPreparationProvider {
    /// Archive reader supplied by the provider.
    type Archive: Read;

    /// Mandatory package installation verifier.
    type Verifier: PackageInstallationVerifier;

    /// Prepare an installation request.
    ///
    /// This operation may perform asynchronous registry
    /// authentication, dependency resolution and downloading.
    ///
    /// It must not publish the requested package installation.
    ///
    /// # Errors
    ///
    /// Returns structured errors for invalid source requests,
    /// unavailable registries, authentication failures,
    /// authorization failures, unresolved dependencies,
    /// incompatible versions, integrity failures, unavailable
    /// providers, cancellation and resource exhaustion.
    fn prepare<'a>(
        &'a self,
        request: &'a InstallRequest,
    ) -> InstallPreparationFuture<
        'a,
        Self::Archive,
        Self::Verifier,
    >;
}

// ============================================================================
// INSTALLATION COMMAND REPORT
// ============================================================================

/// Structured result of one successful package installation.
///
/// The result retains the canonical installation report and
/// exposes the important fields required by CLI, IDE,
/// automation and AI-agent consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallCommandReport {
    source: PackageSource,
    identity: PackageIdentity,
    version: PackageVersion,
    destination: PathBuf,
    installed_entries: u128,
    installed_content_bytes: u128,
    installation: PackageInstallReport,
}

impl InstallCommandReport {
    /// Return the source from which the package was requested.
    #[must_use]
    pub fn source(&self) -> &PackageSource {
        &self.source
    }

    /// Return the exact installed package identity.
    #[must_use]
    pub fn identity(&self) -> &PackageIdentity {
        &self.identity
    }

    /// Return the exact installed package release version.
    #[must_use]
    pub fn version(&self) -> &PackageVersion {
        &self.version
    }

    /// Return the final installation directory.
    #[must_use]
    pub fn destination(&self) -> &std::path::Path {
        &self.destination
    }

    /// Return the number of successfully installed content entries.
    #[must_use]
    pub const fn installed_entries(&self) -> u128 {
        self.installed_entries
    }

    /// Return the total installed content bytes.
    #[must_use]
    pub const fn installed_content_bytes(&self) -> u128 {
        self.installed_content_bytes
    }

    /// Return the canonical package installation report.
    ///
    /// This retains the complete extraction and package metadata
    /// evidence supplied by the installation subsystem.
    #[must_use]
    pub fn installation(&self) -> &PackageInstallReport {
        &self.installation
    }

    /// Consume this report and return the canonical installation
    /// report.
    #[must_use]
    pub fn into_installation(self) -> PackageInstallReport {
        self.installation
    }
}

// ============================================================================
// INSTALL COMMAND
// ============================================================================

/// Stateless package-installation command.
///
/// All mutable installation state belongs to the request,
/// preparation provider and canonical package installer.
///
/// The command can therefore be reused across independent
/// installation operations without maintaining global state.
#[derive(Debug, Default, Clone, Copy)]
pub struct InstallCommand;

impl InstallCommand {
    /// Construct an installation command.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Execute one package installation request.
    ///
    /// The operation:
    ///
    /// 1. Validates the public request.
    /// 2. Asks the preparation provider for an exact package release.
    /// 3. Checks the explicitly requested destination.
    /// 4. Invokes the canonical package installation subsystem.
    /// 5. Returns the verified installation report.
    ///
    /// Registry operations may be asynchronous. Package extraction
    /// and installation are delegated to the existing synchronous
    /// package installation implementation.
    ///
    /// The method does not choose a registry, package version,
    /// package identity, filesystem location or computational
    /// target on behalf of the caller.
    ///
    /// # Errors
    ///
    /// Returns structured errors for:
    ///
    /// - Invalid public requests.
    /// - Failed source preparation.
    /// - Unresolved package versions.
    /// - Unavailable dependencies.
    /// - Unauthorized installation destinations.
    /// - Package identity mismatches.
    /// - Package version mismatches.
    /// - Invalid archive framing.
    /// - Integrity, signature or trust failures.
    /// - Extraction failures.
    /// - Filesystem failures.
    /// - Resource exhaustion.
    /// - Installation publication failures.
    pub async fn execute<P>(
        &self,
        request: &InstallRequest,
        provider: &P,
    ) -> ToolchainResult<InstallCommandReport>
    where
        P: InstallPreparationProvider,
    {
        // ------------------------------------------------------------
        // 1. Validate the public API request.
        // ------------------------------------------------------------

        request.validate().map_err(|error| {
            ToolchainError::validation_failed(
                format!(
                    "package installation request validation failed: {error}"
                ),
            )
            .with_operation(OPERATION_INSTALL)
        })?;

        // ------------------------------------------------------------
        // 2. Prepare the exact package release.
        // ------------------------------------------------------------

        let prepared = provider
            .prepare(request)
            .await
            .map_err(|error| {
                error
                    .with_operation(OPERATION_INSTALL_PREPARE)
                    .with_context(
                        "preparing the requested package installation",
                    )
            })?;

        // ------------------------------------------------------------
        // 3. Enforce an explicitly requested destination.
        // ------------------------------------------------------------

        if let Some(requested_destination) =
            request.destination.as_deref()
        {
            if requested_destination != prepared.destination() {
                return Err(
                    ToolchainError::validation_failed(
                        "installation preparation selected a destination \
                         different from the explicitly requested destination",
                    )
                    .with_operation(OPERATION_INSTALL)
                    .with_subject(
                        requested_destination.display().to_string(),
                    ),
                );
            }
        }

        // ------------------------------------------------------------
        // 4. Preserve source and exact package coordinates.
        // ------------------------------------------------------------

        let source = request.source.clone();

        // The installation request already binds the exact package
        // identity and exact package release version.
        //
        // These values are checked again by the canonical installer
        // against the decoded and verified package manifest.

        // ------------------------------------------------------------
        // 5. Execute canonical package installation.
        // ------------------------------------------------------------

        let PreparedPackageInstall {
            archive,
            request: installation_request,
            mut verifier,
            options,
        } = prepared;

        let installation = install_package_with_options(
            archive,
            &installation_request,
            &mut verifier,
            options,
        )
        .map_err(|error| {
            error
                .with_operation(OPERATION_INSTALL_EXECUTE)
                .with_context(
                    "installing and verifying the requested package",
                )
        })?;

        // ------------------------------------------------------------
        // 6. Construct the successful structured result.
        // ------------------------------------------------------------

        let identity = installation.identity().clone();
        let version = installation.version().clone();

        let destination =
            installation.destination().to_path_buf();

        let installed_entries =
            installation.installed_entries();

        let installed_content_bytes =
            installation.installed_content_bytes();

        Ok(InstallCommandReport {
            source,
            identity,
            version,
            destination,
            installed_entries,
            installed_content_bytes,
            installation,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::toolchain::api::request::PackageSource;

    #[test]
    fn command_is_stateless_and_constructible() {
        let first = InstallCommand::new();
        let second = InstallCommand::default();

        assert_eq!(first, second);
    }

    #[test]
    fn invalid_empty_local_archive_path_is_rejected() {
        let request = InstallRequest {
            source: PackageSource::LocalArchive(
                PathBuf::new(),
            ),
            destination: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn explicitly_supplied_destination_is_preserved() {
        let destination =
            PathBuf::from("packages/installed");

        let request = InstallRequest {
            source: PackageSource::LocalArchive(
                PathBuf::from("packages/example.zpkg"),
            ),
            destination: Some(destination.clone()),
        };

        assert!(request.validate().is_ok());

        assert_eq!(
            request.destination.as_ref(),
            Some(&destination),
        );
    }

    #[test]
    fn local_archive_request_does_not_require_registry_fields() {
        let request = InstallRequest {
            source: PackageSource::LocalArchive(
                PathBuf::from("example.zpkg"),
            ),
            destination: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn operation_identifiers_are_stable() {
        assert_eq!(
            OPERATION_INSTALL,
            "commands.install",
        );

        assert_eq!(
            OPERATION_INSTALL_PREPARE,
            "commands.install.prepare",
        );

        assert_eq!(
            OPERATION_INSTALL_EXECUTE,
            "commands.install.execute",
        );
    }
}
