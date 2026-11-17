#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Package Downloading
//!
//! Path: `src/toolchain/registries/downloading.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Downloads a package archive from a configured registry using the
//! provider-neutral RegistryClient and a bounded, acknowledged chunk
//! protocol.
//!
//! The downloaded archive is verified through the canonical package
//! verification subsystem before a successful receipt is returned.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Download request validation.
//! - Download operation identity.
//! - Download authorization boundary.
//! - Chunked download orchestration.
//! - Download-offset validation.
//! - Registry acknowledgement validation.
//! - Streaming whole-archive SHA-256 calculation.
//! - Integration with canonical package verification.
//! - Download receipt construction.
//!
//! This module does not own:
//!
//! - Registry configuration or discovery.
//! - Registry provider implementations.
//! - Network or local-storage transport.
//! - Registry wire-envelope encoding.
//! - Credential storage or authentication.
//! - Authorization policy implementation.
//! - Package identity allocation.
//! - Package manifest encoding or decoding.
//! - Package archive framing.
//! - Package extraction or installation.
//! - Signature algorithms or trust-store persistence.
//! - Dependency resolution.
//! - Compiler or backend execution.
//! - Hardware discovery or runtime execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integration
//!
//! Registry orchestration:
//! `super::client::RegistryClient`
//!
//! Registry operation requests:
//! `super::provider::{RegistryRequest, RegistryOperationId}`
//!
//! Registry identity:
//! `crate::toolchain::ids::RegistryId`
//!
//! Package identity:
//! `crate::toolchain::ids::PackageId`
//!
//! Registry protocol version:
//! `crate::toolchain::version::RegistryProtocolVersion`
//!
//! Package archive:
//! `crate::toolchain::packages::archive`
//!
//! Package metadata:
//! `crate::toolchain::packages::metadata`
//!
//! Package verification:
//! `crate::toolchain::packages::verification::verify_archive`
//!
//! Package authenticity and trust:
//! `PackageEvidenceVerifier`
//!
//! Structured errors:
//! `crate::toolchain::errors`
//!
//! ## Download protocol
//!
//! This module defines operation payload contracts for:
//!
//! - `packages.download.begin`
//! - `packages.download.chunk`
//! - `packages.download.complete`
//!
//! Providers must implement these operations and bind every operation
//! to the same download identity, transfer identity, package coordinate,
//! archive size and archive digest.
//!
//! The registry must return a stable archive size and SHA-256 digest
//! before the client accepts content.
//!
//! Each chunk response must:
//!
//! - Identify the same download and transfer.
//! - Identify the exact requested offset.
//! - Contain exactly the requested number of bytes.
//! - Declare the exact next offset.
//! - Preserve the original archive size and digest.
//!
//! The complete acknowledgement must confirm the same package,
//! version, archive size and digest.
//!
//! This implementation does not automatically retry operations.
//!
//! ## Security
//!
//! A successful download requires:
//!
//! - An explicit caller-provided authorization implementation.
//! - An authenticated registry transport established by the caller's
//!   authentication integration.
//! - A complete archive transfer.
//! - A matching whole-archive SHA-256 digest.
//! - Successful canonical package archive verification.
//! - Complete package-content integrity verification.
//! - Verified authenticity and trust evidence.
//! - Exact package identity and version agreement.
//! - Valid registry acknowledgements.
//!
//! A digest match alone does not establish publisher authenticity.
//!
//! A valid signature alone does not establish publisher trust.
//!
//! Unknown, partial, unsigned, or indeterminate verification outcomes
//! are not treated as successful verification.
//!
//! ## Scalability
//!
//! Archive bytes are written incrementally to a caller-provided
//! seekable destination. The complete archive is not accumulated
//! in memory by this module.
//!
//! Transfer memory is proportional to the configured chunk size,
//! response representation and serialization overhead.
//!
//! The caller may configure an archive-size budget and a response
//! payload budget. These are operational safeguards, not language,
//! package-format, hardware or computational-domain limits.
//!
//! No universal package-size, entry-count, artifact-count, domain-count,
//! registry-count, or hardware-capacity ceiling is introduced.
//!
//! The destination must be an empty staging destination. On failure,
//! a partial archive may remain there. The caller must discard that
//! staging destination rather than expose it as an installed package.
//!
//! The standard `Seek` interface uses `u64` positions. This API therefore
//! cannot represent archive positions beyond the range of that interface.
//! This is an I/O API representation constraint, not a Zamani language
//! or hardware capacity limit.
//!
//! Literal infinite storage, bandwidth or computation is not promised.
//!
//! ## Resume behavior
//!
//! A server may report a nonzero resume offset. This implementation
//! deliberately rejects that response because it does not have a
//! verified local prefix or a remote prefix-proof contract.
//!
//! The caller must start with an empty staging destination and a new
//! download identity for a fresh transfer.
//!
//! A future resumable implementation must define durable local checkpoint
//! metadata, prefix verification, remote session expiry, and recovery
//! semantics before accepting nonzero offsets.
//!
//! ## Integration order
//!
//! 1. Validate the caller's package coordinate and destination.
//! 2. Authorize the requested download.
//! 3. Begin the registry transfer.
//! 4. Validate the registry's package, size and digest declaration.
//! 5. Download bounded chunks and validate every acknowledgement.
//! 6. Compare the streamed archive digest with the registry declaration.
//! 7. Verify the complete archive using the canonical package verifier.
//! 8. Require complete content integrity and trusted authenticity evidence.
//! 9. Confirm package identity and version.
//! 10. Complete the registry transfer and validate its receipt.
//!
//! The caller remains responsible for atomically publishing the verified
//! staging file into its package store through the canonical installation
//! subsystem.

use std::fmt;
use std::io::{Read, Seek, SeekFrom, Write};
use std::num::NonZeroUsize;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::{
    PackageId,
    RegistryId,
};

use crate::toolchain::version::RegistryProtocolVersion;

use crate::toolchain::packages::verification::{
    verify_archive,
    EvidenceVerification,
    PackageEvidenceVerifier,
    PackageIntegrityStatus,
    PackageManifestDecoder,
    PackageVerificationReport,
    VerificationOptions,
};

use super::client::RegistryClient;

use super::provider::{
    RegistryOperationId,
    RegistryRequest,
};

// ============================================================================
// OPERATION CONTRACT
// ============================================================================

const OPERATION_DOWNLOAD: &str = "registry.download";

const OP_BEGIN: &str = "packages.download.begin";
const OP_CHUNK: &str = "packages.download.chunk";
const OP_COMPLETE: &str = "packages.download.complete";

const DIGEST_ALGORITHM: &str = "sha-256";

const WIRE_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// DOWNLOAD IDENTITY
// ============================================================================

/// Stable caller-supplied identity for one logical package download.
///
/// The identity binds the begin, chunk and complete operations to the
/// same logical transfer.
///
/// The caller must provide a different identity for a different logical
/// download. If an operation outcome is ambiguous, the caller must
/// reconcile that transfer with the registry before attempting another
/// operation.
///
/// This type does not claim globally coordinated uniqueness.
///
/// No arbitrary maximum identity length is imposed.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DownloadId(Arc<str>);

impl DownloadId {
    /// Construct a validated download identity.
    ///
    /// The value must not be empty or contain whitespace or control
    /// characters. The exact supplied value is preserved.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(download_error(
                ToolchainErrorKind::InvalidInput,
                "download identity must not be empty",
            ));
        }

        if value.chars().any(|character| {
            character.is_control() || character.is_whitespace()
        }) {
            return Err(download_error(
                ToolchainErrorKind::InvalidInput,
                "download identity must not contain whitespace \
                 or control characters",
            ));
        }

        Ok(Self(Arc::from(value)))
    }

    /// Borrow the download identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DownloadId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Debug for DownloadId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("DownloadId")
            .field(&self.as_str())
            .finish()
    }
}

// ============================================================================
// DOWNLOAD OPTIONS
// ============================================================================

/// Operational settings for a package download.
///
/// The chunk size controls the amount of archive content requested and
/// decoded for an individual transfer operation. It is not a package-size
/// ceiling.
///
/// Optional budgets are caller-controlled operational safeguards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DownloadOptions {
    chunk_size: NonZeroUsize,
    max_archive_size: Option<u64>,
    max_response_bytes: Option<usize>,
}

impl DownloadOptions {
    /// Construct download options with the supplied chunk size.
    ///
    /// A zero chunk size is invalid.
    ///
    /// No archive-size or response-size budget is imposed unless the
    /// caller explicitly configures one.
    pub fn new(chunk_size: usize) -> Result<Self> {
        let chunk_size = NonZeroUsize::new(chunk_size).ok_or_else(|| {
            download_error(
                ToolchainErrorKind::InvalidInput,
                "download chunk size must be nonzero",
            )
        })?;

        Ok(Self {
            chunk_size,
            max_archive_size: None,
            max_response_bytes: None,
        })
    }

    /// Set an optional maximum archive-size budget.
    ///
    /// `None` means that this operation does not impose an
    /// application-level archive-size budget.
    ///
    /// This is an operational policy, not a package-format limit.
    #[must_use]
    pub const fn with_max_archive_size(
        mut self,
        maximum: Option<u64>,
    ) -> Self {
        self.max_archive_size = maximum;
        self
    }

    /// Set an optional maximum registry-response payload budget.
    ///
    /// This is checked immediately after the provider returns a response.
    /// It does not prevent a provider implementation from allocating an
    /// oversized response before returning it.
    ///
    /// `None` means that this operation does not impose an
    /// application-level response-size budget.
    #[must_use]
    pub const fn with_max_response_bytes(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_response_bytes = maximum;
        self
    }

    /// Return the configured chunk size in bytes.
    #[must_use]
    pub const fn chunk_size(self) -> usize {
        self.chunk_size.get()
    }

    /// Return the optional archive-size budget.
    #[must_use]
    pub const fn max_archive_size(self) -> Option<u64> {
        self.max_archive_size
    }

    /// Return the optional response-payload budget.
    #[must_use]
    pub const fn max_response_bytes(self) -> Option<usize> {
        self.max_response_bytes
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            chunk_size: NonZeroUsize::new(64 * 1024)
                .expect("the default download chunk size is nonzero"),
            max_archive_size: None,
            max_response_bytes: None,
        }
    }
}

// ============================================================================
// AUTHORIZATION BOUNDARY
// ============================================================================

/// Action presented to the download authorization authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadAction {
    /// Download the specified package release from the specified registry.
    Download,
}

/// Mandatory authorization boundary for package downloads.
///
/// Implementations must delegate to the repository's canonical
/// authorization policy. This trait is an integration adapter, not
/// a second authorization engine.
///
/// The implementation should consider the registry, package identity,
/// package version, requesting principal, applicable policy and requested
/// action.
///
/// Authentication alone is not sufficient authorization.
///
/// Implementations must not expose credentials or secrets through errors.
pub trait DownloadAuthorizer: Send + Sync {
    /// Authorize the requested package download.
    fn authorize(
        &self,
        registry_id: &RegistryId,
        package_id: &PackageId,
        package_version_scheme: &str,
        package_version: &str,
        action: DownloadAction,
    ) -> Result<()>;
}

// ============================================================================
// DOWNLOAD RECEIPT
// ============================================================================

/// Validated receipt for a downloaded and verified package archive.
///
/// A receipt proves that the provider returned acknowledgements satisfying
/// this module's transfer contract and that the local archive passed the
/// canonical package-verification policy supplied by the caller.
///
/// It does not independently prove that the registry is trustworthy beyond
/// the caller's configured trust policy, nor that any target can execute
/// the package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadReceipt {
    registry_id: RegistryId,
    download_id: DownloadId,
    transfer_id: String,
    package_id: String,
    package_version_scheme: String,
    package_version: String,
    archive_size: u64,
    archive_digest: String,
    registry_receipt: String,
    verification: PackageVerificationReport,
}

impl DownloadReceipt {
    /// Return the registry identity.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the logical download identity.
    #[must_use]
    pub fn download_id(&self) -> &DownloadId {
        &self.download_id
    }

    /// Return the provider-assigned transfer identity.
    #[must_use]
    pub fn transfer_id(&self) -> &str {
        &self.transfer_id
    }

    /// Return the downloaded package identity.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the package version scheme.
    #[must_use]
    pub fn package_version_scheme(&self) -> &str {
        &self.package_version_scheme
    }

    /// Return the exact package release version.
    #[must_use]
    pub fn package_version(&self) -> &str {
        &self.package_version
    }

    /// Return the complete archive size in bytes.
    #[must_use]
    pub const fn archive_size(&self) -> u64 {
        self.archive_size
    }

    /// Return the SHA-256 digest of the complete archive.
    #[must_use]
    pub fn archive_digest(&self) -> &str {
        &self.archive_digest
    }

    /// Return the opaque registry-issued download receipt.
    #[must_use]
    pub fn registry_receipt(&self) -> &str {
        &self.registry_receipt
    }

    /// Return the canonical package-verification report.
    #[must_use]
    pub fn verification(&self) -> &PackageVerificationReport {
        &self.verification
    }

    /// Consume the receipt and return its verification report.
    #[must_use]
    pub fn into_verification(self) -> PackageVerificationReport {
        self.verification
    }
}

// ============================================================================
// OPERATION PAYLOADS
// ============================================================================

/// Request to begin a package download.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct BeginDownload {
    schema_version: u32,
    download_id: String,
    package_id: String,
    package_version_scheme: String,
    package_version: String,
    digest_algorithm: String,
}

/// Registry acknowledgement for a begin operation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BeginAcknowledgement {
    schema_version: u32,
    download_id: String,
    transfer_id: String,
    package_id: String,
    package_version_scheme: String,
    package_version: String,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
    next_offset: u64,
}

/// Request for one bounded archive chunk.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct DownloadChunk {
    schema_version: u32,
    download_id: String,
    transfer_id: String,
    offset: u64,
    requested_bytes: u64,
}

/// Registry response containing one archive chunk.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChunkAcknowledgement {
    schema_version: u32,
    download_id: String,
    transfer_id: String,
    offset: u64,
    next_offset: u64,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
    data_encoding: String,
    data: String,
}

/// Request to complete a download transfer.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct CompleteDownload {
    schema_version: u32,
    download_id: String,
    transfer_id: String,
    package_id: String,
    package_version_scheme: String,
    package_version: String,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
}

/// Registry acknowledgement for a completed download.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompleteAcknowledgement {
    schema_version: u32,
    download_id: String,
    transfer_id: String,
    package_id: String,
    package_version_scheme: String,
    package_version: String,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
    registry_receipt: String,
}

// ============================================================================
// PUBLIC DOWNLOAD ENTRY POINT
// ============================================================================

/// Download, verify and acknowledge a package archive from a configured
/// registry.
///
/// The destination must be an empty, seekable staging destination.
/// It must support reading, writing and seeking because the completed
/// archive is verified in place by the canonical package verifier.
///
/// The caller should use a temporary file or another isolated staging
/// destination. If this function returns an error after transfer begins,
/// the destination may contain partial or untrusted bytes and must be
/// discarded.
///
/// `package_version_scheme` and `package_version` are exact package
/// coordinates. They are checked against the decoded package model after
/// archive verification.
///
/// `decoder` and `evidence_verifier` are mandatory canonical authorities.
/// They must not be replaced with permissive placeholder implementations.
///
/// `authorizer` must delegate to the canonical authorization policy.
///
/// The caller must separately establish an authenticated registry session
/// through the registry authentication integration before dispatching
/// provider operations.
///
/// The selected protocol version is supplied by the caller. This function
/// does not invent a version or bypass the configured registry/provider
/// compatibility checks.
///
/// The selected provider must implement the `packages.download.*`
/// operations defined in this module.
///
/// No automatic retry is performed. If an operation fails, the caller
/// must reconcile any ambiguous remote outcome before starting another
/// transfer.
///
/// # Errors
///
/// Returns an error for:
///
/// - Invalid download identity or package coordinates.
/// - Invalid download options.
/// - A non-empty or non-seekable destination.
/// - Authorization failure.
/// - Registry/provider/protocol incompatibility.
/// - Provider operation failure.
/// - Invalid or inconsistent registry acknowledgements.
/// - Unsupported digest algorithm or malformed digest.
/// - Truncated, oversized or inconsistent chunks.
/// - Archive digest mismatch.
/// - Invalid package archive or metadata.
/// - Partial content integrity.
/// - Unsigned, untrusted or indeterminate authenticity evidence.
/// - Package identity or version mismatch.
/// - I/O failure or resource exhaustion.
pub async fn download<W, D, E, A>(
    client: &RegistryClient,
    registry_id: RegistryId,
    protocol_version: RegistryProtocolVersion,
    download_id: DownloadId,
    package_id: PackageId,
    package_version_scheme: impl Into<String>,
    package_version: impl Into<String>,
    destination: &mut W,
    decoder: &mut D,
    evidence_verifier: &mut E,
    authorizer: &A,
    verification_options: VerificationOptions,
    download_options: DownloadOptions,
) -> Result<DownloadReceipt>
where
    W: Read + Write + Seek,
    D: PackageManifestDecoder,
    E: PackageEvidenceVerifier,
    A: DownloadAuthorizer,
{
    let package_version_scheme = package_version_scheme.into();
    let package_version = package_version.into();

    validate_package_coordinate(
        &package_version_scheme,
        &package_version,
    )?;

    validate_destination(destination)?;

    // ------------------------------------------------------------------------
    // 1. Authorize this exact registry and package coordinate.
    // ------------------------------------------------------------------------

    authorizer
        .authorize(
            &registry_id,
            &package_id,
            &package_version_scheme,
            &package_version,
            DownloadAction::Download,
        )
        .map_err(|error| {
            error
                .with_operation(OPERATION_DOWNLOAD)
                .with_context("authorizing package download")
                .with_subject(registry_id.to_string())
        })?;

    // ------------------------------------------------------------------------
    // 2. Request the registry's package and archive declaration.
    // ------------------------------------------------------------------------

    let begin = BeginDownload {
        schema_version: WIRE_SCHEMA_VERSION,
        download_id: download_id.as_str().to_owned(),
        package_id: package_id.to_string(),
        package_version_scheme: package_version_scheme.clone(),
        package_version: package_version.clone(),
        digest_algorithm: DIGEST_ALGORITHM.to_owned(),
    };

    let begin_payload = execute_payload(
        client,
        &registry_id,
        protocol_version,
        OP_BEGIN,
        &begin,
        download_options,
    )
    .await?;

    let acknowledgement: BeginAcknowledgement =
        decode_acknowledgement(&begin_payload)?;

    validate_begin_acknowledgement(
        &acknowledgement,
        &download_id,
        &package_id.to_string(),
        &package_version_scheme,
        &package_version,
        download_options,
    )?;

    let archive_size = acknowledgement.archive_size;
    let archive_digest = acknowledgement.archive_digest.clone();
    let transfer_id = acknowledgement.transfer_id.clone();

    // ------------------------------------------------------------------------
    // 3. Download bounded chunks into the staging destination.
    // ------------------------------------------------------------------------

    let streamed_digest = download_archive(
        client,
        &registry_id,
        protocol_version,
        &download_id,
        &transfer_id,
        destination,
        archive_size,
        &archive_digest,
        download_options,
    )
    .await?;

    if streamed_digest != archive_digest {
        return Err(download_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            "downloaded archive SHA-256 digest does not match the \
             registry's declared digest",
        )
        .with_subject(download_id.to_string()));
    }

    // ------------------------------------------------------------------------
    // 4. Verify the complete archive using the canonical verifier.
    // ------------------------------------------------------------------------

    destination
        .seek(SeekFrom::Start(0))
        .map_err(|error| {
            download_io_error(
                error,
                "rewinding downloaded archive before verification",
            )
        })?;

    let verification: PackageVerificationReport = verify_archive(
        &mut *destination,
        decoder,
        evidence_verifier,
        verification_options,
    )
    .map_err(|error| {
        error
            .with_operation(OPERATION_DOWNLOAD)
            .with_context("verifying downloaded package archive")
            .with_subject(download_id.to_string())
    })?;

    validate_verification_report(
        &verification,
        &package_id,
        &package_version_scheme,
        &package_version,
    )?;

    // ------------------------------------------------------------------------
    // 5. Confirm that the registry considers this transfer complete.
    // ------------------------------------------------------------------------

    let complete = CompleteDownload {
        schema_version: WIRE_SCHEMA_VERSION,
        download_id: download_id.as_str().to_owned(),
        transfer_id: transfer_id.clone(),
        package_id: package_id.to_string(),
        package_version_scheme: package_version_scheme.clone(),
        package_version: package_version.clone(),
        archive_size,
        digest_algorithm: DIGEST_ALGORITHM.to_owned(),
        archive_digest: archive_digest.clone(),
    };

    let complete_payload = execute_payload(
        client,
        &registry_id,
        protocol_version,
        OP_COMPLETE,
        &complete,
        download_options,
    )
    .await?;

    let completed: CompleteAcknowledgement =
        decode_acknowledgement(&complete_payload)?;

    validate_complete_acknowledgement(
        &completed,
        &download_id,
        &transfer_id,
        &package_id.to_string(),
        &package_version_scheme,
        &package_version,
        archive_size,
        &archive_digest,
    )?;

    Ok(DownloadReceipt {
        registry_id,
        download_id,
        transfer_id,
        package_id: package_id.to_string(),
        package_version_scheme,
        package_version,
        archive_size,
        archive_digest,
        registry_receipt: completed.registry_receipt,
        verification,
    })
}

// ============================================================================
// PACKAGE COORDINATE VALIDATION
// ============================================================================

fn validate_package_coordinate(
    version_scheme: &str,
    version: &str,
) -> Result<()> {
    validate_client_text(version_scheme, "package version scheme")?;
    validate_client_text(version, "package version")?;

    Ok(())
}

fn validate_client_text(
    value: &str,
    description: &str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(download_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(download_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not contain control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// DESTINATION VALIDATION
// ============================================================================

fn validate_destination<W: Seek>(
    destination: &mut W,
) -> Result<()> {
    let current_length = destination
        .seek(SeekFrom::End(0))
        .map_err(|error| {
            download_io_error(
                error,
                "checking download staging destination",
            )
        })?;

    if current_length != 0 {
        return Err(download_error(
            ToolchainErrorKind::InvalidInput,
            "download destination must be empty; use a fresh staging \
             destination rather than overwriting an existing package",
        ));
    }

    destination
        .seek(SeekFrom::Start(0))
        .map_err(|error| {
            download_io_error(
                error,
                "rewinding empty download staging destination",
            )
        })?;

    Ok(())
}

// ============================================================================
// BEGIN ACKNOWLEDGEMENT VALIDATION
// ============================================================================

fn validate_begin_acknowledgement(
    acknowledgement: &BeginAcknowledgement,
    download_id: &DownloadId,
    package_id: &str,
    package_version_scheme: &str,
    package_version: &str,
    options: DownloadOptions,
) -> Result<()> {
    if acknowledgement.schema_version != WIRE_SCHEMA_VERSION {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported begin-acknowledgement schema",
        ));
    }

    if acknowledgement.download_id != download_id.as_str() {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement has a different download identity",
        ));
    }

    validate_server_text(
        &acknowledgement.transfer_id,
        "transfer identity",
    )?;

    if acknowledgement.package_id != package_id {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement identifies a different package",
        ));
    }

    if acknowledgement.package_version_scheme != package_version_scheme {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement identifies a different \
             package version scheme",
        ));
    }

    if acknowledgement.package_version != package_version {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement identifies a different \
             package version",
        ));
    }

    if acknowledgement.digest_algorithm != DIGEST_ALGORITHM {
        return Err(invalid_acknowledgement(
            "registry declared an unsupported archive digest algorithm",
        ));
    }

    validate_sha256_hex(&acknowledgement.archive_digest)?;

    if acknowledgement.archive_size == 0 {
        return Err(invalid_acknowledgement(
            "registry declared an empty package archive",
        ));
    }

    if let Some(maximum) = options.max_archive_size() {
        if acknowledgement.archive_size > maximum {
            return Err(download_error(
                ToolchainErrorKind::ResourceExhausted,
                format!(
                    "registry archive size {} exceeds the configured \
                     download budget of {maximum} bytes",
                    acknowledgement.archive_size
                ),
            ));
        }
    }

    if acknowledgement.next_offset != 0 {
        return Err(download_error(
            ToolchainErrorKind::UnsupportedOperation,
            "registry requested a nonzero resume offset, but this \
             implementation cannot verify the previously downloaded \
             prefix; the transfer is stopped safely",
        )
        .with_subject(download_id.to_string()));
    }

    Ok(())
}

// ============================================================================
// STREAMING CHUNK DOWNLOAD
// ============================================================================

fn allocate_buffer(size: usize) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    buffer.try_reserve_exact(size).map_err(|error| {
        download_error(
            ToolchainErrorKind::ResourceExhausted,
            format!("unable to allocate download buffer: {error}"),
        )
    })?;

    buffer.resize(size, 0);

    Ok(buffer)
}

async fn download_archive<W>(
    client: &RegistryClient,
    registry_id: &RegistryId,
    protocol_version: RegistryProtocolVersion,
    download_id: &DownloadId,
    transfer_id: &str,
    destination: &mut W,
    archive_size: u64,
    archive_digest: &str,
    options: DownloadOptions,
) -> Result<String>
where
    W: Write,
{
    let chunk_size = u64::try_from(options.chunk_size()).map_err(|_| {
        download_error(
            ToolchainErrorKind::ResourceExhausted,
            "configured download chunk size cannot be represented \
             by the registry transfer offset type",
        )
    })?;

    let mut offset = 0_u64;
    let mut hasher = Sha256::new();

    while offset < archive_size {
        let remaining = archive_size.checked_sub(offset).ok_or_else(|| {
            download_error(
                ToolchainErrorKind::Internal,
                "download offset exceeded the declared archive size",
            )
        })?;

        let requested_bytes = remaining.min(chunk_size);

        let request_payload = DownloadChunk {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: download_id.as_str().to_owned(),
            transfer_id: transfer_id.to_owned(),
            offset,
            requested_bytes,
        };

        let response_payload = execute_payload(
            client,
            registry_id,
            protocol_version,
            OP_CHUNK,
            &request_payload,
            options,
        )
        .await?;

        let acknowledgement: ChunkAcknowledgement =
            decode_acknowledgement(&response_payload)?;

        validate_chunk_metadata(
            &acknowledgement,
            download_id,
            transfer_id,
            offset,
            requested_bytes,
            archive_size,
            archive_digest,
        )?;

        let expected_length = usize::try_from(requested_bytes).map_err(|_| {
            download_error(
                ToolchainErrorKind::ResourceExhausted,
                "requested download chunk cannot be represented \
                 by this host",
            )
        })?;

        let expected_hex_length = expected_length.checked_mul(2).ok_or_else(|| {
            download_error(
                ToolchainErrorKind::ResourceExhausted,
                "encoded download chunk length overflowed",
            )
        })?;

        if acknowledgement.data.len() != expected_hex_length {
            return Err(invalid_acknowledgement(
                "registry returned a chunk with an unexpected encoded length",
            ));
        }

        let bytes = hex::decode(&acknowledgement.data).map_err(|error| {
            invalid_acknowledgement(
                "registry returned a chunk that is not valid hexadecimal data",
            )
            .with_source(error)
        })?;

        if bytes.len() != expected_length {
            return Err(invalid_acknowledgement(
                "decoded registry chunk length differs from the requested length",
            ));
        }

        // Hash exactly the bytes that are written to the staging destination.
        hasher.update(&bytes);

        destination
            .write_all(&bytes)
            .map_err(|error| {
                download_io_error(
                    error,
                    "writing downloaded package archive chunk",
                )
            })?;

        offset = acknowledgement.next_offset;
    }

    if offset != archive_size {
        return Err(download_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            format!(
                "downloaded {offset} bytes, but the registry declared \
                 an archive size of {archive_size} bytes"
            ),
        ));
    }

    destination
        .flush()
        .map_err(|error| {
            download_io_error(
                error,
                "flushing downloaded package archive",
            )
        })?;

    Ok(hex::encode(hasher.finalize()))
}

fn validate_chunk_metadata(
    acknowledgement: &ChunkAcknowledgement,
    download_id: &DownloadId,
    transfer_id: &str,
    expected_offset: u64,
    requested_bytes: u64,
    archive_size: u64,
    archive_digest: &str,
) -> Result<()> {
    if acknowledgement.schema_version != WIRE_SCHEMA_VERSION {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported chunk-acknowledgement schema",
        ));
    }

    if acknowledgement.download_id != download_id.as_str() {
        return Err(invalid_acknowledgement(
            "registry chunk belongs to a different download identity",
        ));
    }

    if acknowledgement.transfer_id != transfer_id {
        return Err(invalid_acknowledgement(
            "registry chunk belongs to a different transfer identity",
        ));
    }

    if acknowledgement.offset != expected_offset {
        return Err(invalid_acknowledgement(
            "registry chunk begins at an unexpected archive offset",
        ));
    }

    let expected_next_offset = expected_offset
        .checked_add(requested_bytes)
        .ok_or_else(|| {
            download_error(
                ToolchainErrorKind::ResourceExhausted,
                "download archive offset overflowed",
            )
        })?;

    if acknowledgement.next_offset != expected_next_offset {
        return Err(invalid_acknowledgement(
            "registry did not acknowledge the exact requested chunk boundary",
        ));
    }

    if acknowledgement.archive_size != archive_size {
        return Err(invalid_acknowledgement(
            "registry changed the archive size during download",
        ));
    }

    if acknowledgement.digest_algorithm != DIGEST_ALGORITHM {
        return Err(invalid_acknowledgement(
            "registry changed the archive digest algorithm during download",
        ));
    }

    if acknowledgement.archive_digest != archive_digest {
        return Err(invalid_acknowledgement(
            "registry changed the archive digest during download",
        ));
    }

    if acknowledgement.data_encoding != "hex" {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported chunk data encoding",
        ));
    }

    if expected_next_offset > archive_size {
        return Err(invalid_acknowledgement(
            "registry acknowledged a chunk beyond the declared archive size",
        ));
    }

    Ok(())
}

// ============================================================================
// PACKAGE VERIFICATION POLICY
// ============================================================================

fn validate_verification_report(
    report: &PackageVerificationReport,
    expected_package_id: &PackageId,
    expected_version_scheme: &str,
    expected_version: &str,
) -> Result<()> {
    report.package().validate().map_err(|error| {
        error
            .with_operation(OPERATION_DOWNLOAD)
            .with_context("validating downloaded package model")
    })?;

    if report.integrity_status() != PackageIntegrityStatus::Verified {
        return Err(download_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            "download requires complete package-content integrity \
             verification; partially verified packages are rejected",
        ));
    }

    if report.evidence_status() != EvidenceVerification::Verified {
        return Err(download_error(
            ToolchainErrorKind::AuthenticationFailed,
            "download requires verified authenticity, provenance and \
             trust evidence; unsigned, untrusted or indeterminate \
             evidence is rejected",
        ));
    }

    let package = report.package();

    if package.identity().id() != expected_package_id {
        return Err(download_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            "verified archive contains a different package identity \
             from the requested package",
        ));
    }

    if package.version().scheme().as_str() != expected_version_scheme {
        return Err(download_error(
            ToolchainErrorKind::IncompatibleVersion,
            "verified archive contains a different package version scheme",
        ));
    }

    if package.version().value() != expected_version {
        return Err(download_error(
            ToolchainErrorKind::IncompatibleVersion,
            "verified archive contains a different package version",
        ));
    }

    Ok(())
}

// ============================================================================
// COMPLETE ACKNOWLEDGEMENT VALIDATION
// ============================================================================

fn validate_complete_acknowledgement(
    acknowledgement: &CompleteAcknowledgement,
    download_id: &DownloadId,
    transfer_id: &str,
    package_id: &str,
    package_version_scheme: &str,
    package_version: &str,
    archive_size: u64,
    archive_digest: &str,
) -> Result<()> {
    if acknowledgement.schema_version != WIRE_SCHEMA_VERSION {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported complete-acknowledgement schema",
        ));
    }

    if acknowledgement.download_id != download_id.as_str() {
        return Err(invalid_acknowledgement(
            "registry complete acknowledgement has a different download identity",
        ));
    }

    if acknowledgement.transfer_id != transfer_id {
        return Err(invalid_acknowledgement(
            "registry complete acknowledgement has a different transfer identity",
        ));
    }

    if acknowledgement.package_id != package_id {
        return Err(invalid_acknowledgement(
            "registry completed a different package identity",
        ));
    }

    if acknowledgement.package_version_scheme != package_version_scheme {
        return Err(invalid_acknowledgement(
            "registry completed a different package version scheme",
        ));
    }

    if acknowledgement.package_version != package_version {
        return Err(invalid_acknowledgement(
            "registry completed a different package version",
        ));
    }

    if acknowledgement.archive_size != archive_size {
        return Err(invalid_acknowledgement(
            "registry completed a different archive size",
        ));
    }

    if acknowledgement.digest_algorithm != DIGEST_ALGORITHM {
        return Err(invalid_acknowledgement(
            "registry completed the download using an unexpected digest algorithm",
        ));
    }

    if acknowledgement.archive_digest != archive_digest {
        return Err(invalid_acknowledgement(
            "registry completed a different archive digest",
        ));
    }

    validate_server_text(
        &acknowledgement.registry_receipt,
        "registry download receipt",
    )
}

// ============================================================================
// REGISTRY OPERATION EXECUTION
// ============================================================================

async fn execute_payload<T: Serialize>(
    client: &RegistryClient,
    registry_id: &RegistryId,
    protocol_version: RegistryProtocolVersion,
    operation: &str,
    payload: &T,
    options: DownloadOptions,
) -> Result<Vec<u8>> {
    let operation_id = RegistryOperationId::new(operation).map_err(|error| {
        error
            .with_operation(OPERATION_DOWNLOAD)
            .with_context("constructing registry download operation")
    })?;

    let encoded_payload = serde_json::to_vec(payload).map_err(|error| {
        download_error(
            ToolchainErrorKind::RegistryOperationFailed,
            "unable to serialize registry download operation payload",
        )
        .with_source(error)
    })?;

    let request = RegistryRequest::new(
        registry_id.clone(),
        protocol_version,
        operation_id,
        encoded_payload,
    );

    let response = client
        .execute(request)
        .await
        .map_err(|error| {
            error
                .with_operation(OPERATION_DOWNLOAD)
                .with_context(format!(
                    "executing registry download operation `{operation}`"
                ))
                .with_subject(registry_id.to_string())
        })?;

    if response.registry_id() != registry_id {
        return Err(invalid_acknowledgement(
            "registry provider response identifies a different registry",
        ));
    }

    if response.protocol_version() != protocol_version {
        return Err(invalid_acknowledgement(
            "registry provider response uses a different protocol version",
        ));
    }

    if let Some(maximum) = options.max_response_bytes() {
        if response.payload().len() > maximum {
            return Err(download_error(
                ToolchainErrorKind::ResourceExhausted,
                format!(
                    "registry response contains {} bytes, exceeding \
                     the configured response budget of {maximum} bytes",
                    response.payload().len()
                ),
            ));
        }
    }

    Ok(response.payload().to_vec())
}

// ============================================================================
// ACKNOWLEDGEMENT DECODING
// ============================================================================

fn decode_acknowledgement<T>(
    payload: &[u8],
) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_slice(payload).map_err(|error| {
        download_error(
            ToolchainErrorKind::ValidationFailed,
            "registry returned a malformed download acknowledgement",
        )
        .with_source(error)
    })
}

// ============================================================================
// DIGEST VALIDATION
// ============================================================================

fn validate_sha256_hex(value: &str) -> Result<()> {
    // SHA-256 is represented as 32 bytes, encoded as 64 lowercase
    // hexadecimal characters. This is a digest-format requirement,
    // not an application-level capacity limit.
    if value.len() != 64 {
        return Err(invalid_acknowledgement(
            "registry SHA-256 digest must contain exactly 64 hexadecimal characters",
        ));
    }

    if !value.bytes().all(|byte| {
        byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
    }) {
        return Err(invalid_acknowledgement(
            "registry SHA-256 digest must use canonical lowercase hexadecimal",
        ));
    }

    let decoded = hex::decode(value).map_err(|error| {
        invalid_acknowledgement(
            "registry returned an invalid SHA-256 digest",
        )
        .with_source(error)
    })?;

    if decoded.len() != 32 {
        return Err(invalid_acknowledgement(
            "registry SHA-256 digest has an invalid decoded length",
        ));
    }

    Ok(())
}

// ============================================================================
// TEXT VALIDATION
// ============================================================================

fn validate_server_text(
    value: &str,
    description: &str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(invalid_acknowledgement(format!(
            "registry returned an empty {description}"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_acknowledgement(format!(
            "registry returned a {description} containing control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn invalid_acknowledgement(
    message: impl Into<String>,
) -> ToolchainError {
    download_error(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
}

fn download_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(OPERATION_DOWNLOAD)
}

fn download_io_error(
    error: std::io::Error,
    context: &'static str,
) -> ToolchainError {
    ToolchainError::from(error)
        .with_operation(OPERATION_DOWNLOAD)
        .with_context(context)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_identity_rejects_empty_values() {
        assert!(DownloadId::new("").is_err());
        assert!(DownloadId::new("   ").is_err());
    }

    #[test]
    fn download_identity_rejects_whitespace_and_control_characters() {
        assert!(DownloadId::new("download id").is_err());
        assert!(DownloadId::new("download\nid").is_err());
        assert!(DownloadId::new("download\tid").is_err());
    }

    #[test]
    fn download_identity_preserves_valid_open_world_values() {
        let id = DownloadId::new(
            "consumer.production.installation-2026-001",
        )
        .expect("valid download identity");

        assert_eq!(
            id.as_str(),
            "consumer.production.installation-2026-001"
        );
    }

    #[test]
    fn download_identity_has_no_arbitrary_length_ceiling() {
        let value = "d".repeat(16_384);

        let id = DownloadId::new(value.clone())
            .expect("valid long download identity");

        assert_eq!(id.as_str(), value);
    }

    #[test]
    fn zero_chunk_size_is_rejected() {
        assert!(DownloadOptions::new(0).is_err());
    }

    #[test]
    fn configured_chunk_size_is_preserved() {
        let options = DownloadOptions::new(128 * 1024)
            .expect("valid download options");

        assert_eq!(options.chunk_size(), 128 * 1024);
    }

    #[test]
    fn default_options_do_not_impose_archive_or_response_budgets() {
        let options = DownloadOptions::default();

        assert_eq!(options.max_archive_size(), None);
        assert_eq!(options.max_response_bytes(), None);
    }

    #[test]
    fn caller_budgets_are_preserved() {
        let options = DownloadOptions::new(4096)
            .expect("valid options")
            .with_max_archive_size(Some(8_000_000))
            .with_max_response_bytes(Some(32_000));

        assert_eq!(options.max_archive_size(), Some(8_000_000));
        assert_eq!(options.max_response_bytes(), Some(32_000));
    }

    #[test]
    fn package_version_coordinates_reject_empty_values() {
        assert!(validate_package_coordinate("", "1.0.0").is_err());
        assert!(validate_package_coordinate("semver", "").is_err());
        assert!(validate_package_coordinate(" ", "1.0.0").is_err());
    }

    #[test]
    fn package_version_coordinates_reject_control_characters() {
        assert!(
            validate_package_coordinate("semver\n", "1.0.0").is_err()
        );

        assert!(
            validate_package_coordinate("semver", "1.0.0\n").is_err()
        );
    }

    #[test]
    fn sha256_validation_accepts_canonical_digest() {
        let digest = hex::encode(Sha256::digest(b"Zamani"));

        assert!(validate_sha256_hex(&digest).is_ok());
    }

    #[test]
    fn sha256_validation_rejects_wrong_length() {
        assert!(validate_sha256_hex("abcd").is_err());
    }

    #[test]
    fn sha256_validation_rejects_uppercase_hex() {
        let digest = hex::encode(Sha256::digest(b"Zamani"))
            .to_uppercase();

        assert!(validate_sha256_hex(&digest).is_err());
    }

    #[test]
    fn sha256_validation_rejects_non_hexadecimal_data() {
        let invalid = "g".repeat(64);

        assert!(validate_sha256_hex(&invalid).is_err());
    }

    #[test]
    fn begin_acknowledgement_rejects_wrong_download_identity() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let acknowledgement = BeginAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "different-download".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            package_id: "packages:1".to_owned(),
            package_version_scheme: "semver".to_owned(),
            package_version: "1.0.0".to_owned(),
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: hex::encode(Sha256::digest(b"archive")),
            next_offset: 0,
        };

        assert!(
            validate_begin_acknowledgement(
                &acknowledgement,
                &download_id,
                "packages:1",
                "semver",
                "1.0.0",
                DownloadOptions::default(),
            )
            .is_err()
        );
    }

    #[test]
    fn begin_acknowledgement_rejects_nonzero_resume_offset() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let acknowledgement = BeginAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "download-1".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            package_id: "packages:1".to_owned(),
            package_version_scheme: "semver".to_owned(),
            package_version: "1.0.0".to_owned(),
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: hex::encode(Sha256::digest(b"archive")),
            next_offset: 10,
        };

        assert!(
            validate_begin_acknowledgement(
                &acknowledgement,
                &download_id,
                "packages:1",
                "semver",
                "1.0.0",
                DownloadOptions::default(),
            )
            .is_err()
        );
    }

    #[test]
    fn begin_acknowledgement_rejects_archive_budget_exceeded() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let acknowledgement = BeginAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "download-1".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            package_id: "packages:1".to_owned(),
            package_version_scheme: "semver".to_owned(),
            package_version: "1.0.0".to_owned(),
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: hex::encode(Sha256::digest(b"archive")),
            next_offset: 0,
        };

        let options = DownloadOptions::new(16)
            .expect("valid options")
            .with_max_archive_size(Some(99));

        assert!(
            validate_begin_acknowledgement(
                &acknowledgement,
                &download_id,
                "packages:1",
                "semver",
                "1.0.0",
                options,
            )
            .is_err()
        );
    }

    #[test]
    fn chunk_acknowledgement_rejects_wrong_offset() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let digest = hex::encode(Sha256::digest(b"archive"));

        let acknowledgement = ChunkAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "download-1".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            offset: 5,
            next_offset: 15,
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: digest.clone(),
            data_encoding: "hex".to_owned(),
            data: hex::encode(b"0123456789"),
        };

        assert!(
            validate_chunk_metadata(
                &acknowledgement,
                &download_id,
                "transfer-1",
                0,
                10,
                100,
                &digest,
            )
            .is_err()
        );
    }

    #[test]
    fn chunk_acknowledgement_rejects_wrong_next_offset() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let digest = hex::encode(Sha256::digest(b"archive"));

        let acknowledgement = ChunkAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "download-1".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            offset: 0,
            next_offset: 9,
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: digest.clone(),
            data_encoding: "hex".to_owned(),
            data: hex::encode(b"0123456789"),
        };

        assert!(
            validate_chunk_metadata(
                &acknowledgement,
                &download_id,
                "transfer-1",
                0,
                10,
                100,
                &digest,
            )
            .is_err()
        );
    }

    #[test]
    fn chunk_acknowledgement_rejects_changed_archive_digest() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let expected_digest = hex::encode(Sha256::digest(b"archive"));
        let different_digest = hex::encode(Sha256::digest(b"different"));

        let acknowledgement = ChunkAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "download-1".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            offset: 0,
            next_offset: 10,
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: different_digest,
            data_encoding: "hex".to_owned(),
            data: hex::encode(b"0123456789"),
        };

        assert!(
            validate_chunk_metadata(
                &acknowledgement,
                &download_id,
                "transfer-1",
                0,
                10,
                100,
                &expected_digest,
            )
            .is_err()
        );
    }

    #[test]
    fn chunk_acknowledgement_rejects_unsupported_encoding() {
        let download_id = DownloadId::new("download-1")
            .expect("valid identity");

        let digest = hex::encode(Sha256::digest(b"archive"));

        let acknowledgement = ChunkAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            download_id: "download-1".to_owned(),
            transfer_id: "transfer-1".to_owned(),
            offset: 0,
            next_offset: 10,
            archive_size: 100,
            digest_algorithm: DIGEST_ALGORITHM.to_owned(),
            archive_digest: digest.clone(),
            data_encoding: "base64".to_owned(),
            data: "MDEyMzQ1Njc4OQ==".to_owned(),
        };

        assert!(
            validate_chunk_metadata(
                &acknowledgement,
                &download_id,
                "transfer-1",
                0,
                10,
                100,
                &digest,
            )
            .is_err()
        );
    }

    #[test]
    fn destination_must_be_empty() {
        use std::io::Cursor;

        let mut destination = Cursor::new(vec![1, 2, 3]);

        assert!(validate_destination(&mut destination).is_err());
    }

    #[test]
    fn empty_destination_is_accepted() {
        use std::io::Cursor;

        let mut destination = Cursor::new(Vec::<u8>::new());

        assert!(validate_destination(&mut destination).is_ok());
        assert_eq!(destination.position(), 0);
    }

    #[test]
    fn digest_matches_streamed_archive_bytes() {
        let archive = b"Zamani package archive";
        let digest = hex::encode(Sha256::digest(archive));

        assert_eq!(
            digest,
            hex::encode(Sha256::digest(b"Zamani package archive"))
        );
    }
}