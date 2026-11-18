
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Command
//!
//! Path: src/toolchain/commands/package.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the public package-command orchestration boundary.
//!
//! The command coordinates canonical project preparation, dependency
//! resolution, explicit package-content selection, package creation,
//! canonical package-model materialization and .zpkg archive writing.
//!
//! It does not implement a second package manager.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Package-command request validation.
//! - Package-command orchestration.
//! - Preparation-provider integration.
//! - Canonical package-model adapter integration.
//! - Creation-draft and package-model consistency checks.
//! - Streaming archive serialization coordination.
//! - Cancellation-aware archive I/O.
//! - Structured package-command results.
//!
//! It does not own:
//!
//! - Source grammar, parsing or semantic analysis.
//! - Project-manifest parsing.
//! - Workspace discovery.
//! - Dependency resolution.
//! - Package identity allocation.
//! - Package model definitions.
//! - Package-content selection policy.
//! - Package file hashing.
//! - Archive framing or serialization.
//! - Package-manifest serialization.
//! - Signature creation or verification.
//! - Publisher trust decisions.
//! - Registry communication.
//! - Package installation or extraction.
//! - Compiler execution.
//! - Backend selection.
//! - Hardware discovery.
//! - Runtime execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integrations
//!
//! Project and manifest preparation:
//!     crate::zamani_project_config
//!     crate::toolchain::workspace
//!     crate::toolchain::manifest
//!
//! Dependency resolution:
//!     crate::toolchain::dependencies
//!
//! Package creation:
//!     crate::toolchain::packages::creation
//!
//! Package model:
//!     crate::toolchain::packages::model
//!
//! Package manifest:
//!     crate::toolchain::packages::metadata
//!
//! Package archive:
//!     crate::toolchain::packages::archive
//!
//! Package verification:
//!     crate::toolchain::packages::verification
//!
//! Errors:
//!     crate::toolchain::errors
//!
//! Results:
//!     crate::toolchain::result
//!
//! ## Package lifecycle
//!
//! Canonical project preparation
//!           |
//!           v
//! Manifest and workspace validation
//!           |
//!           v
//! Dependency resolution
//!           |
//!           v
//! Explicit package-content selection
//!           |
//!           v
//! PackageCreator
//!           |
//!           v
//! PackageDraft
//!           |
//!           v
//! Canonical package-model adapter
//!           |
//!           v
//! PackageManifest validation
//!           |
//!           v
//! Canonical manifest encoding
//!           |
//!           v
//! Versioned .zpkg archive writer
//!           |
//!           v
//! Completed package-command result
//!
//! ## POCO-REAF
//!
//! Package creation is independent of the physical machine on which
//! its contents will eventually execute.
//!
//! The command accepts domain-neutral package metadata and artifact
//! compatibility declarations. It does not hard-code classical,
//! quantum, HDL, AI, hybrid, distributed or future computing domains.
//!
//! Different target realizations can be distributed through the same
//! package infrastructure when their canonical artifact and compatibility
//! contracts have been supplied.
//!
//! Package creation does not establish execution feasibility.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum on:
//!
//! - Package files.
//! - Package bytes.
//! - Package metadata.
//! - Dependencies.
//! - Artifacts.
//! - Computing domains.
//! - Target descriptions.
//! - Hardware resources.
//! - Distributed participants.
//!
//! The command streams archive content and delegates package-file
//! hashing to the existing streaming package creator.
//!
//! Actual execution remains subject to available memory, storage,
//! processing time, operating-system constraints and explicitly
//! configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Security
//!
//! The command does not recursively package arbitrary project files.
//! The preparation provider must supply explicit, validated file
//! selections to PackageCreationRequest.
//!
//! The existing PackageCreator enforces project-root containment,
//! regular-file requirements, symbolic-link rejection, stable file
//! checks and configured creation budgets.
//!
//! Package authenticity and trust verification remain separate from
//! package creation. Publishing and installation must invoke the
//! canonical package-verification and security authorities.
//!
//! ## Concurrency
//!
//! This command does not create a worker pool or impose a fixed
//! concurrency level.
//!
//! The injected preparation provider and model adapter must document
//! their own concurrency guarantees.
//!
//! The command itself owns each request and its mutable archive stream.
//!
//! ## Failure semantics
//!
//! A successful result means:
//!
//! - Preparation completed successfully.
//! - Package creation completed successfully.
//! - The canonical package model was materialized.
//! - Its content inventory agrees with the creation draft.
//! - The canonical package manifest was validated and encoded.
//! - The archive writer completed successfully.
//! - The output writer was successfully flushed.
//!
//! It does not mean that the package has been signed, published,
//! installed, trusted or proven executable on a target.

use std::io::{self, Cursor, Read, Write};
use std::path::{Path, PathBuf};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;

use crate::toolchain::packages::archive::{
    write_archive_with_options,
    ArchiveIoOptions,
    ArchiveWriteSummary,
};

use crate::toolchain::packages::creation::{
    PackageCancellation,
    PackageCreationRequest,
    PackageCreator,
    PackageDraft,
    PackageDraftManifest,
    PackagePayload,
};

use crate::toolchain::packages::metadata::{
    encode_manifest,
    PackageManifest,
};

use crate::toolchain::packages::model::{
    PackagePath,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_PACKAGE: &str = "commands.package";

const OPERATION_PREPARE: &str = "commands.package.prepare";

const OPERATION_CREATE: &str = "commands.package.create";

const OPERATION_MATERIALIZE: &str = "commands.package.materialize";

const OPERATION_SERIALIZE: &str = "commands.package.serialize";

// ============================================================================
// PACKAGE COMMAND REQUEST
// ============================================================================

/// Immutable request to create a package archive.
///
/// The project root is a location reference. Its existence, manifest,
/// workspace membership, authorization and project validity are checked
/// by the injected preparation provider.
///
/// The archive I/O buffer controls I/O granularity only. It does not
/// restrict the size of a package or an individual content entry.
#[derive(Debug, Clone)]
pub struct PackageCommandRequest {
    project_root: PathBuf,
    archive_options: ArchiveIoOptions,
}

impl PackageCommandRequest {
    /// Construct a package-command request with default archive I/O options.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied project-root path is empty.
    pub fn new(
        project_root: impl Into<PathBuf>,
    ) -> ToolchainResult<Self> {
        Self::with_archive_options(
            project_root,
            ArchiveIoOptions::default(),
        )
    }

    /// Construct a package-command request with explicit archive options.
    ///
    /// # Errors
    ///
    /// Returns an error when the project-root path is empty.
    pub fn with_archive_options(
        project_root: impl Into<PathBuf>,
        archive_options: ArchiveIoOptions,
    ) -> ToolchainResult<Self> {
        let project_root = project_root.into();

        if project_root.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "package project root must not be empty",
                )
                .with_operation(OPERATION_PACKAGE),
            );
        }

        Ok(Self {
            project_root,
            archive_options,
        })
    }

    /// Return the requested project-root location.
    #[must_use]
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    /// Return the configured archive I/O options.
    #[must_use]
    pub const fn archive_options(&self) -> ArchiveIoOptions {
        self.archive_options
    }

    /// Validate request invariants that do not require filesystem access.
    ///
    /// Filesystem, manifest, authorization and project validation belong
    /// to the preparation provider.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.project_root.as_os_str().is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "package project root must not be empty",
                )
                .with_operation(OPERATION_PACKAGE),
            );
        }

        Ok(())
    }
}

// ============================================================================
// PROJECT AND DEPENDENCY PREPARATION
// ============================================================================

/// Authoritative preparation boundary for package creation.
///
/// The implementation must integrate the canonical project, workspace,
/// manifest and dependency services.
///
/// It must:
///
/// - Resolve the requested project and workspace.
/// - Load the canonical project manifest.
/// - Validate the manifest and its compatibility.
/// - Resolve package identity and release-version information.
/// - Resolve all declared dependencies.
/// - Preserve resolved dependency identities, sources, versions,
///   content digests and selected features.
/// - Select package files explicitly.
/// - Apply the project's package-inclusion policy.
/// - Preserve artifact format and compatibility declarations.
/// - Preserve source and build provenance.
/// - Construct a complete PackageCreationRequest.
///
/// It must not parse Zamani source files independently, implement
/// dependency resolution, invent package identities or infer that
/// an unverified dependency is valid.
///
/// The returned PackageCreationRequest must contain only resolved
/// dependencies and explicit package-file selections.
///
/// An incomplete or ambiguous preparation must return an error.
pub trait PackagePreparationProvider: Send + Sync {
    /// Prepare a complete package-creation request.
    ///
    /// # Errors
    ///
    /// Returns the original structured toolchain error when project
    /// discovery, manifest validation, authorization, dependency
    /// resolution, content selection or provenance preparation fails.
    fn prepare(
        &self,
        request: &PackageCommandRequest,
        cancellation: &PackageCancellation,
    ) -> ToolchainResult<PackageCreationRequest>;
}

// ============================================================================
// CANONICAL PACKAGE MODEL ADAPTER
// ============================================================================

/// Converts the package-creation snapshot into the canonical package model.
///
/// The existing PackageCreator produces a PackageDraftManifest.
/// The archive writer consumes a canonical Package model and a canonical
/// PackageManifest.
///
/// This adapter is the explicit integration boundary between those
/// contracts.
///
/// Implementations must preserve:
///
/// - The package's canonical identity and exact version scheme.
/// - Package type.
/// - All package metadata.
/// - Every selected content path.
/// - Every content kind.
/// - Every measured content size.
/// - Every content digest.
/// - Every artifact descriptor.
/// - Every artifact format.
/// - Every artifact compatibility declaration.
/// - Every domain capability declaration.
/// - All resolved dependency snapshots.
/// - Source provenance.
/// - Build provenance.
/// - Applicable compatibility and extension data.
///
/// The current Package model deliberately does not own dependency
/// resolution. Implementations must preserve dependency information
/// through the canonical package-manifest extension contract or another
/// explicitly versioned package metadata contract.
///
/// Silently dropping dependency or provenance information is forbidden.
///
/// This adapter must use canonical package-model and metadata APIs.
/// It must not create a competing package model or manifest serializer.
pub trait PackageModelAdapter: Send + Sync {
    /// Materialize a validated canonical package manifest from a draft.
    ///
    /// The returned manifest must retain all information required by
    /// the package-format and compatibility contracts.
    ///
    /// # Errors
    ///
    /// Returns an error if any draft information cannot be represented
    /// faithfully or if the resulting package model is invalid.
    fn materialize(
        &self,
        draft: &PackageDraftManifest,
    ) -> ToolchainResult<PackageManifest>;
}

// ============================================================================
// PACKAGE COMMAND RESULT
// ============================================================================

/// Successful package-command result.
///
/// The result retains the canonical package manifest and the archive
/// writer's measured output summary.
///
/// A result is only produced after successful archive serialization
/// and writer flushing.
///
/// It does not claim that the package has been signed, published,
/// installed, trusted or tested on a target.
#[derive(Debug, Clone, PartialEq)]
#[must_use = "inspect and report the completed package operation"]
pub struct PackageCommandResult {
    manifest: PackageManifest,
    archive: ArchiveWriteSummary,
}

impl PackageCommandResult {
    /// Return the canonical package manifest.
    #[must_use]
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }

    /// Return the canonical package model.
    #[must_use]
    pub fn package(&self) -> &crate::toolchain::packages::model::Package {
        self.manifest.package()
    }

    /// Return the completed archive-write summary.
    #[must_use]
    pub const fn archive_summary(&self) -> ArchiveWriteSummary {
        self.archive
    }

    /// Return the number of content entries written.
    #[must_use]
    pub const fn content_entries(&self) -> u128 {
        self.archive.entry_count()
    }

    /// Return the number of manifest bytes written.
    #[must_use]
    pub const fn manifest_bytes(&self) -> u128 {
        self.archive.manifest_bytes()
    }

    /// Return the number of package-content bytes written.
    #[must_use]
    pub const fn content_bytes(&self) -> u128 {
        self.archive.content_bytes()
    }
}

// ============================================================================
// PACKAGE COMMAND
// ============================================================================

/// Package-command orchestrator.
///
/// The preparation and model adapters are injected explicitly. This avoids
/// hidden global registries, implicit manifest parsers and fabricated
/// package metadata.
///
/// The package creator reuses the canonical package-creation implementation.
pub struct PackageCommand<P, M> {
    preparation: P,
    model_adapter: M,
    creator: PackageCreator,
}

impl<P, M> PackageCommand<P, M>
where
    P: PackagePreparationProvider,
    M: PackageModelAdapter,
{
    /// Construct a package command using the canonical default digest
    /// provider.
    #[must_use]
    pub fn new(
        preparation: P,
        model_adapter: M,
    ) -> Self {
        Self {
            preparation,
            model_adapter,
            creator: PackageCreator::new(),
        }
    }

    /// Construct a package command using an explicitly configured creator.
    ///
    /// This permits an approved digest provider to be injected without
    /// changing package-command orchestration.
    #[must_use]
    pub fn with_creator(
        preparation: P,
        model_adapter: M,
        creator: PackageCreator,
    ) -> Self {
        Self {
            preparation,
            model_adapter,
            creator,
        }
    }

    /// Borrow the preparation provider.
    #[must_use]
    pub fn preparation_provider(&self) -> &P {
        &self.preparation
    }

    /// Borrow the canonical package-model adapter.
    #[must_use]
    pub fn model_adapter(&self) -> &M {
        &self.model_adapter
    }

    /// Create a `.zpkg` archive and write it to the supplied output stream.
    ///
    /// The caller owns the output stream and its destination policy.
    /// This method supports files, network streams, in-memory writers
    /// and other `Write` implementations without assuming a filesystem
    /// destination.
    ///
    /// The archive writer is the sole owner of `.zpkg` framing.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The request is invalid.
    /// - Cancellation is requested.
    /// - Project or manifest preparation fails.
    /// - Dependency resolution fails.
    /// - Package creation fails.
    /// - Canonical package-model materialization fails.
    /// - The materialized package differs from the creation inventory.
    /// - Manifest encoding fails.
    /// - Archive serialization fails.
    /// - A selected payload cannot be read.
    /// - The output writer fails.
    /// - The output writer cannot be flushed.
    ///
    /// A failed operation never produces a successful result.
    pub fn execute<W>(
        &self,
        request: &PackageCommandRequest,
        writer: &mut W,
        cancellation: &PackageCancellation,
    ) -> ToolchainResult<PackageCommandResult>
    where
        W: Write,
    {
        request.validate()?;

        check_cancellation(cancellation, OPERATION_PACKAGE)?;

        // ------------------------------------------------------------
        // 1. Canonical project and dependency preparation.
        // ------------------------------------------------------------

        let creation_request = self
            .preparation
            .prepare(request, cancellation)
            .map_err(|error| {
                error
                    .with_operation(OPERATION_PREPARE)
                    .with_context(
                        "preparing canonical project and resolved dependencies",
                    )
            })?;

        check_cancellation(cancellation, OPERATION_PREPARE)?;

        // ------------------------------------------------------------
        // 2. Create an immutable, deterministic package draft.
        // ------------------------------------------------------------

        let draft = self
            .creator
            .create(creation_request, cancellation)
            .map_err(|error| {
                ToolchainError::package_operation_failed(
                    "canonical package creation failed",
                )
                .with_operation(OPERATION_CREATE)
                .with_source(error)
            })?;

        check_cancellation(cancellation, OPERATION_CREATE)?;

        // ------------------------------------------------------------
        // 3. Materialize the canonical package model.
        // ------------------------------------------------------------

        let (draft_manifest, mut payloads) = draft.into_parts();

        let manifest = self
            .model_adapter
            .materialize(&draft_manifest)
            .map_err(|error| {
                error
                    .with_operation(OPERATION_MATERIALIZE)
                    .with_context(
                        "materializing canonical package model",
                    )
            })?;

        manifest.validate().map_err(|error| {
            error
                .with_operation(OPERATION_MATERIALIZE)
                .with_context(
                    "validating materialized package manifest",
                )
        })?;

        validate_materialized_inventory(
            &draft_manifest,
            &manifest,
        )?;

        check_cancellation(cancellation, OPERATION_MATERIALIZE)?;

        // ------------------------------------------------------------
        // 4. Encode the canonical package manifest.
        // ------------------------------------------------------------

        let manifest_bytes = encode_manifest(&manifest).map_err(|error| {
            error
                .with_operation(OPERATION_SERIALIZE)
                .with_context(
                    "encoding canonical package manifest",
                )
        })?;

        let manifest_length =
            u128::try_from(manifest_bytes.len()).map_err(|_| {
                ToolchainError::resource_exhausted(
                    "encoded package manifest length cannot be represented",
                )
                .with_operation(OPERATION_SERIALIZE)
            })?;

        // ------------------------------------------------------------
        // 5. Stream the canonical archive.
        // ------------------------------------------------------------

        let mut manifest_reader =
            Cursor::new(manifest_bytes.as_slice());

        let archive_result = {
            let mut checked_writer =
                CancellationWriter::new(writer, cancellation);

            write_archive_with_options(
                &mut checked_writer,
                manifest.package(),
                &mut manifest_reader,
                manifest_length,
                |package_path| {
                    let payload = payloads
                        .iter_mut()
                        .find(|payload| {
                            payload.package_path()
                                == package_path.as_str()
                        })
                        .ok_or_else(|| {
                            ToolchainError::validation_failed(
                                format!(
                                    "package content `{package_path}` \
                                     has no corresponding open creation payload"
                                ),
                            )
                            .with_operation(OPERATION_SERIALIZE)
                        })?;

                    payload
                        .rewind()
                        .map_err(|error| {
                            ToolchainError::from(error)
                                .with_operation(OPERATION_SERIALIZE)
                                .with_context(
                                    format!(
                                        "rewinding package payload `{package_path}`"
                                    ),
                                )
                        })?;

                    // The creator retains the original verified file
                    // handle. try_clone avoids reopening an untrusted
                    // project-relative path during archive serialization.
                    let file = payload
                        .reader_mut()
                        .try_clone()
                        .map_err(|error| {
                            ToolchainError::from(error)
                                .with_operation(OPERATION_SERIALIZE)
                                .with_context(
                                    format!(
                                        "cloning verified payload handle `{package_path}`"
                                    ),
                                )
                        })?;

                    Ok(CancellationReader::new(
                        file,
                        cancellation,
                    ))
                },
                request.archive_options(),
            )
        };

        let archive = match archive_result {
            Ok(summary) => summary,

            Err(error) if cancellation.is_cancelled() => {
                return Err(cancelled(OPERATION_SERIALIZE));
            }

            Err(error) => {
                return Err(error
                    .with_operation(OPERATION_SERIALIZE)
                    .with_context(
                        "writing canonical .zpkg archive",
                    ));
            }
        };

        check_cancellation(cancellation, OPERATION_SERIALIZE)?;

        // ------------------------------------------------------------
        // 6. Flush the caller-owned output stream.
        // ------------------------------------------------------------

        writer.flush().map_err(|error| {
            if cancellation.is_cancelled() {
                cancelled(OPERATION_SERIALIZE)
            } else {
                ToolchainError::from(error)
                    .with_operation(OPERATION_SERIALIZE)
                    .with_context(
                        "flushing completed package archive",
                    )
            }
        })?;

        check_cancellation(cancellation, OPERATION_PACKAGE)?;

        // ------------------------------------------------------------
        // 7. Return structured completion evidence.
        // ------------------------------------------------------------

        Ok(PackageCommandResult {
            manifest,
            archive,
        })
    }
}

// ============================================================================
// MATERIALIZED INVENTORY VALIDATION
// ============================================================================

/// Verify that the canonical package model faithfully represents the
/// package-creation snapshot.
///
/// This is deliberately a structural consistency check. It does not
/// verify signatures, trust, dependency compatibility or target feasibility.
fn validate_materialized_inventory(
    draft: &PackageDraftManifest,
    manifest: &PackageManifest,
) -> ToolchainResult<()> {
    let package = manifest.package();

    package.validate().map_err(|error| {
        error
            .with_operation(OPERATION_MATERIALIZE)
            .with_context("validating canonical package model")
    })?;

    if package.contents().len() != draft.files().len() {
        return Err(
            ToolchainError::validation_failed(
                "materialized package content count differs from \
                 the package-creation inventory",
            )
            .with_operation(OPERATION_MATERIALIZE),
        );
    }

    for record in draft.files() {
        let path = PackagePath::new(record.package_path()).map_err(|error| {
            error
                .with_operation(OPERATION_MATERIALIZE)
                .with_context(
                    "validating package-creation inventory path",
                )
        })?;

        let content = package.contents().get(&path).ok_or_else(|| {
            ToolchainError::validation_failed(
                format!(
                    "materialized package is missing content `{path}`"
                ),
            )
            .with_operation(OPERATION_MATERIALIZE)
        })?;

        if content.kind().as_str() != record.content_kind() {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "content kind differs for package entry `{path}`"
                    ),
                )
                .with_operation(OPERATION_MATERIALIZE),
            );
        }

        if content.declared_size() != Some(record.byte_size()) {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "declared content size differs for package entry `{path}`"
                    ),
                )
                .with_operation(OPERATION_MATERIALIZE),
            );
        }

        let digest = content.digest().ok_or_else(|| {
            ToolchainError::integrity_verification_failed(
                format!(
                    "materialized package entry `{path}` has no content digest"
                ),
            )
            .with_operation(OPERATION_MATERIALIZE)
        })?;

        if digest.algorithm() != record.digest().algorithm()
            || digest.value() != record.digest().value()
        {
            return Err(
                ToolchainError::integrity_verification_failed(
                    format!(
                        "materialized package digest differs for entry `{path}`"
                    ),
                )
                .with_operation(OPERATION_MATERIALIZE),
            );
        }
    }

    let expected_artifacts = draft
        .files()
        .iter()
        .filter(|record| record.artifact().is_some())
        .map(|record| {
            PackagePath::new(record.package_path())
        })
        .collect::<ToolchainResult<
            std::collections::BTreeSet<PackagePath>,
        >>()
        .map_err(|error| {
            error.with_operation(OPERATION_MATERIALIZE)
        })?;

    let actual_artifacts = package
        .artifacts()
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    if actual_artifacts != expected_artifacts {
        return Err(
            ToolchainError::validation_failed(
                "materialized artifact inventory differs from \
                 the package-creation artifact inventory",
            )
            .with_operation(OPERATION_MATERIALIZE),
        );
    }

    Ok(())
}

// ============================================================================
// CANCELLATION
// ============================================================================

fn check_cancellation(
    cancellation: &PackageCancellation,
    operation: &'static str,
) -> ToolchainResult<()> {
    if cancellation.is_cancelled() {
        Err(cancelled(operation))
    } else {
        Ok(())
    }
}

fn cancelled(operation: &'static str) -> ToolchainError {
    ToolchainError::cancelled(
        "package creation was cancelled",
    )
    .with_operation(operation)
}

// ============================================================================
// CANCELLATION-AWARE STREAMS
// ============================================================================

/// Reader that observes package-operation cancellation.
///
/// Cancellation is cooperative. It is checked before each read.
/// It does not forcibly interrupt an operating-system call that is
/// already blocked.
struct CancellationReader<'a, R> {
    inner: R,
    cancellation: &'a PackageCancellation,
}

impl<'a, R> CancellationReader<'a, R> {
    fn new(
        inner: R,
        cancellation: &'a PackageCancellation,
    ) -> Self {
        Self {
            inner,
            cancellation,
        }
    }
}

impl<R: Read> Read for CancellationReader<'_, R> {
    fn read(
        &mut self,
        buffer: &mut [u8],
    ) -> io::Result<usize> {
        if self.cancellation.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "package operation cancelled",
            ));
        }

        self.inner.read(buffer)
    }
}

/// Writer that observes package-operation cancellation.
///
/// The wrapper preserves the caller's writer and does not own or close it.
struct CancellationWriter<'a, W> {
    inner: &'a mut W,
    cancellation: &'a PackageCancellation,
}

impl<'a, W> CancellationWriter<'a, W> {
    fn new(
        inner: &'a mut W,
        cancellation: &'a PackageCancellation,
    ) -> Self {
        Self {
            inner,
            cancellation,
        }
    }
}

impl<W: Write> Write for CancellationWriter<'_, W> {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        if self.cancellation.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "package operation cancelled",
            ));
        }

        self.inner.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.cancellation.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "package operation cancelled",
            ));
        }

        self.inner.flush()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    struct NeverCalledPreparation {
        called: AtomicBool,
    }

    impl PackagePreparationProvider for NeverCalledPreparation {
        fn prepare(
            &self,
            _request: &PackageCommandRequest,
            _cancellation: &PackageCancellation,
        ) -> ToolchainResult<PackageCreationRequest> {
            self.called.store(true, Ordering::Release);

            Err(ToolchainError::internal(
                "preparation must not be reached in this test",
            ))
        }
    }

    struct UnusedModelAdapter;

    impl PackageModelAdapter for UnusedModelAdapter {
        fn materialize(
            &self,
            _draft: &PackageDraftManifest,
        ) -> ToolchainResult<PackageManifest> {
            Err(ToolchainError::internal(
                "model adapter must not be reached in this test",
            ))
        }
    }

    #[test]
    fn rejects_empty_project_root() {
        let result = PackageCommandRequest::new(PathBuf::new());

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("empty root must be rejected")
                .kind(),
            ToolchainErrorKind::InvalidInput,
        );
    }

    #[test]
    fn accepts_nonempty_project_root_reference() {
        let request = PackageCommandRequest::new("project");

        assert!(request.is_ok());
    }

    #[test]
    fn cancellation_prevents_project_preparation() {
        let preparation = NeverCalledPreparation {
            called: AtomicBool::new(false),
        };

        let command = PackageCommand::new(
            preparation,
            UnusedModelAdapter,
        );

        let request = PackageCommandRequest::new("project")
            .expect("valid request");

        let cancellation = PackageCancellation::new();
        cancellation.cancel();

        let mut output = Vec::<u8>::new();

        let result = command.execute(
            &request,
            &mut output,
            &cancellation,
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("cancelled operation must fail")
                .kind(),
            ToolchainErrorKind::Cancelled,
        );

        assert!(
            !command
                .preparation_provider()
                .called
                .load(Ordering::Acquire),
            "preparation must not execute after cancellation",
        );

        assert!(output.is_empty());
    }

    #[test]
    fn cancellation_reader_rejects_reads_after_cancellation() {
        let cancellation = PackageCancellation::new();

        let mut reader = CancellationReader::new(
            Cursor::new(b"package content".to_vec()),
            &cancellation,
        );

        let mut buffer = [0_u8; 7];

        assert_eq!(
            reader.read(&mut buffer).expect("initial read"),
            7,
        );

        cancellation.cancel();

        let error = reader
            .read(&mut buffer)
            .expect_err("cancelled read must fail");

        assert_eq!(
            error.kind(),
            io::ErrorKind::Interrupted,
        );
    }

    #[test]
    fn cancellation_writer_rejects_writes_after_cancellation() {
        let cancellation = PackageCancellation::new();
        let mut output = Vec::<u8>::new();

        let mut writer = CancellationWriter::new(
            &mut output,
            &cancellation,
        );

        writer.write_all(b"first").expect("initial write");

        cancellation.cancel();

        let error = writer
            .write_all(b"second")
            .expect_err("cancelled write must fail");

        assert_eq!(
            error.kind(),
            io::ErrorKind::Interrupted,
        );

        assert_eq!(output, b"first");
    }
}
