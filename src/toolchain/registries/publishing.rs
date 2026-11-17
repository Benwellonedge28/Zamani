
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Publishing
//!
//! Path: `src/toolchain/registries/publishing.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides provider-neutral orchestration for publishing verified Zamani
//! packages to configured registries.
//!
//! Publication is independent of computational domain. The same workflow
//! can publish classical, quantum, HDL, AI, tensor, hybrid, distributed,
//! networking, cryptographic, embedded, accelerator and future-domain
//! packages.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Publication request validation.
//! - Publication identity and operation-level publication contracts.
//! - Mandatory publication authorization boundary.
//! - Package verification orchestration.
//! - Streaming archive hashing.
//! - Chunked registry upload orchestration.
//! - Upload-offset validation.
//! - Publication acknowledgement validation.
//! - Publication receipt construction.
//!
//! This module does not own:
//!
//! - Registry identity or configuration.
//! - Registry provider implementation.
//! - Registry transport.
//! - Wire-envelope serialization.
//! - Authentication or credential storage.
//! - Authorization policy implementation.
//! - Package metadata parsing.
//! - Package archive encoding.
//! - Package content hashing algorithms.
//! - Signature creation or verification algorithms.
//! - Publisher trust-store persistence.
//! - Dependency resolution.
//! - Package installation.
//! - Hardware discovery.
//! - Compiler or runtime execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integration
//!
//! Registry orchestration:
//! `super::client::RegistryClient`
//!
//! Registry operations:
//! `super::provider::{RegistryRequest, RegistryOperationId}`
//!
//! Registry identity:
//! `crate::toolchain::ids::RegistryId`
//!
//! Registry protocol version:
//! `crate::toolchain::version::RegistryProtocolVersion`
//!
//! Package model:
//! `crate::toolchain::packages::model::Package`
//!
//! Package verification:
//! `crate::toolchain::packages::verification::verify_archive`
//!
//! Package authenticity and trust:
//! `PackageEvidenceVerifier`
//!
//! Package metadata decoding:
//! `PackageManifestDecoder`
//!
//! Structured errors:
//! `crate::toolchain::errors`
//!
//! ## Publication protocol
//!
//! This module defines the operation payload contracts for:
//!
//! - `packages.publish.begin`
//! - `packages.publish.chunk`
//! - `packages.publish.commit`
//!
//! Registry providers must implement these operations and validate every
//! field. The registry must bind all operations to the supplied publication
//! identity and package digest.
//!
//! The provider must make repeated begin requests with the same publication
//! identity and identical package coordinates idempotent. A different
//! archive digest for an existing publication identity must be rejected.
//!
//! A provider must acknowledge each chunk with the exact next accepted
//! offset. It must not acknowledge bytes that it has not durably accepted
//! according to its own documented durability contract.
//!
//! Commit must atomically make the verified package release available, or
//! return an explicit failure. Providers must document how clients resolve
//! an ambiguous commit result.
//!
//! This module deliberately does not retry publication mutations.
//!
//! ## Security
//!
//! Publication requires:
//!
//! - A caller-provided authorization implementation.
//! - Complete package archive verification.
//! - Fully verified content integrity.
//! - Verified authenticity and trust evidence.
//! - A stable caller-provided publication identity.
//! - Exact validation of registry acknowledgements.
//!
//! An unsigned or partially verified package is not published by this
//! implementation.
//!
//! Authentication and authorization are separate. The authorization
//! boundary does not treat an authenticated session as an authorization
//! grant.
//!
//! Credentials must never be included in publication payloads.
//!
//! ## Scalability
//!
//! The archive is processed as a stream.
//!
//! Upload memory is proportional to the configured chunk size and the
//! protocol serialization overhead, not the complete archive size.
//!
//! No universal package-size, content-count, domain-count, registry-count,
//! or hardware-capacity ceiling is introduced here.
//!
//! The chunk size is an operational setting, not a package-size limit.
//!
//! The standard `Seek` interface uses `u64` offsets. An archive whose size
//! cannot be represented by that interface cannot be published through
//! this seekable-source API. A future non-seekable resumable source contract
//! can remove that interface constraint without changing package semantics.
//!
//! Literal infinite storage, network bandwidth or computation is not
//! promised. Real operations remain subject to available resources,
//! provider capabilities, policy and physical reality.
//!
//! ## Integration order
//!
//! 1. Resolve the configured registry through RegistryClient.
//! 2. Verify the complete archive using the canonical package verifier.
//! 3. Require complete integrity and trusted authenticity evidence.
//! 4. Authorize publication through the caller's security authority.
//! 5. Hash the exact archive stream.
//! 6. Begin or resume the publication using its stable publication ID.
//! 7. Upload bounded chunks and validate every acknowledgement.
//! 8. Re-hash the uploaded stream to detect source changes.
//! 9. Commit and validate the registry receipt.

use std::fmt;
use std::io::{Read, Seek, SeekFrom};
use std::num::NonZeroUsize;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::RegistryId;
use crate::toolchain::version::RegistryProtocolVersion;

use crate::toolchain::packages::model::Package;

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

const OPERATION_PUBLISH: &str = "registry.publish";

const OP_BEGIN: &str = "packages.publish.begin";
const OP_CHUNK: &str = "packages.publish.chunk";
const OP_COMMIT: &str = "packages.publish.commit";

const DIGEST_ALGORITHM: &str = "sha-256";

const WIRE_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// PUBLICATION IDENTITY
// ============================================================================

/// Stable caller-supplied identity for one logical publication.
///
/// A publication identity is used for idempotency and resumable upload
/// coordination. It is not a package ID, registry ID, credential or
/// cryptographic digest.
///
/// The caller must reuse the same identity when resuming the same logical
/// publication. A new logical publication must use a different identity.
///
/// Uniqueness must be guaranteed by the caller within the registry's
/// idempotency scope. This type does not claim globally coordinated
/// uniqueness.
///
/// No arbitrary maximum length is imposed.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublicationId(Arc<str>);

impl PublicationId {
    /// Construct a validated publication identity.
    ///
    /// The value must not be empty or contain whitespace or control
    /// characters. The exact supplied value is preserved.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(publication_error(
                ToolchainErrorKind::InvalidInput,
                "publication identity must not be empty",
            ));
        }

        if value.chars().any(|character| {
            character.is_control() || character.is_whitespace()
        }) {
            return Err(publication_error(
                ToolchainErrorKind::InvalidInput,
                "publication identity must not contain whitespace \
                 or control characters",
            ));
        }

        Ok(Self(Arc::from(value)))
    }

    /// Borrow the publication identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PublicationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Debug for PublicationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PublicationId")
            .field(&self.as_str())
            .finish()
    }
}

// ============================================================================
// PUBLICATION OPTIONS
// ============================================================================

/// Operational settings for a package publication.
///
/// The chunk size controls the amount of archive data read and encoded
/// for an individual upload operation. It is not a package-size ceiling.
///
/// The caller chooses the value according to available memory, registry
/// capabilities, transport characteristics and operational policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationOptions {
    chunk_size: NonZeroUsize,
}

impl PublicationOptions {
    /// Construct publication options.
    ///
    /// A zero chunk size is invalid.
    pub fn new(chunk_size: usize) -> Result<Self> {
        let chunk_size = NonZeroUsize::new(chunk_size).ok_or_else(|| {
            publication_error(
                ToolchainErrorKind::InvalidInput,
                "publication chunk size must be nonzero",
            )
        })?;

        Ok(Self { chunk_size })
    }

    /// Return the configured upload chunk size in bytes.
    #[must_use]
    pub const fn chunk_size(self) -> usize {
        self.chunk_size.get()
    }
}

// ============================================================================
// AUTHORIZATION BOUNDARY
// ============================================================================

/// Publication action presented to the authorization authority.
///
/// The registry operation and its concrete transport remain outside this
/// authorization model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationAction {
    /// Publish a new package release or resume its existing upload.
    Publish,
}

/// Mandatory authorization boundary for publication.
///
/// Implementations must delegate to the repository's canonical security
/// and authorization policy. This trait is an adapter contract, not a
/// second authorization engine.
///
/// The implementation should consider the registry, package identity,
/// package version, publisher identity, applicable policy and requested
/// action.
///
/// Authentication alone is not sufficient authorization.
///
/// Implementations must not expose credentials or secrets through errors.
pub trait PublicationAuthorizer: Send + Sync {
    /// Authorize the requested publication operation.
    fn authorize(
        &self,
        registry_id: &RegistryId,
        package: &Package,
        action: PublicationAction,
    ) -> Result<()>;
}

// ============================================================================
// PUBLICATION RECEIPT
// ============================================================================

/// Validated receipt returned after a registry confirms publication.
///
/// A receipt proves that the selected provider returned a response
/// satisfying this module's publication acknowledgement contract.
///
/// It does not independently prove that the registry is trustworthy,
/// that the package is executable, or that any target can run it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationReceipt {
    registry_id: RegistryId,
    publication_id: PublicationId,
    upload_id: String,
    package_id: String,
    package_name: String,
    package_version: String,
    archive_size: u64,
    archive_digest: String,
    registry_receipt: String,
}

impl PublicationReceipt {
    /// Return the registry identity.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the logical publication identity.
    #[must_use]
    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    /// Return the provider-assigned upload identity.
    #[must_use]
    pub fn upload_id(&self) -> &str {
        &self.upload_id
    }

    /// Return the published package identity.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the canonical package name.
    #[must_use]
    pub fn package_name(&self) -> &str {
        &self.package_name
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

    /// Return the SHA-256 digest of the published archive.
    #[must_use]
    pub fn archive_digest(&self) -> &str {
        &self.archive_digest
    }

    /// Return the opaque registry-issued publication receipt.
    #[must_use]
    pub fn registry_receipt(&self) -> &str {
        &self.registry_receipt
    }
}

// ============================================================================
// OPERATION PAYLOADS
// ============================================================================

/// Request to begin or resume a publication.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct BeginPublication {
    schema_version: u32,
    publication_id: String,
    package_id: String,
    package_name: String,
    package_version_scheme: String,
    package_version: String,
    package_type: String,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
    content_entries: u128,
    content_bytes: u128,
}

/// Registry acknowledgement for a begin/resume operation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BeginAcknowledgement {
    schema_version: u32,
    publication_id: String,
    upload_id: String,
    archive_size: u64,
    archive_digest: String,
    next_offset: u64,
}

/// One bounded archive chunk.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct UploadChunk {
    schema_version: u32,
    publication_id: String,
    upload_id: String,
    offset: u64,
    data_encoding: String,
    data: String,
}

/// Registry acknowledgement for one uploaded chunk.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChunkAcknowledgement {
    schema_version: u32,
    publication_id: String,
    upload_id: String,
    next_offset: u64,
}

/// Request to commit a completed upload.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct CommitPublication {
    schema_version: u32,
    publication_id: String,
    upload_id: String,
    package_id: String,
    package_version: String,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
}

/// Registry acknowledgement of a completed publication.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CommitAcknowledgement {
    schema_version: u32,
    publication_id: String,
    upload_id: String,
    package_id: String,
    package_version: String,
    archive_size: u64,
    digest_algorithm: String,
    archive_digest: String,
    registry_receipt: String,
}

// ============================================================================
// PUBLICATION ENTRY POINT
// ============================================================================

/// Verify and publish a package archive to a configured registry.
///
/// The archive source must be seekable and positioned at the beginning
/// of the complete `.zpkg` archive. It is read in multiple passes:
///
/// 1. Complete archive verification.
/// 2. Whole-archive digest calculation.
/// 3. Chunked upload with a second digest calculation.
///
/// The final digest comparison detects changes to the source between
/// verification, hashing and upload.
///
/// `decoder` and `evidence_verifier` are the canonical package metadata
/// and authenticity/trust authorities. They are mandatory.
///
/// `authorizer` must delegate to the canonical authorization authority.
///
/// The selected registry protocol version is supplied by the caller.
/// This function does not invent or negotiate a version independently
/// of the configured registry/provider contracts.
///
/// The registry provider must implement the `packages.publish.*`
/// operations defined by this module.
///
/// No automatic retry is performed. If an operation fails, the caller
/// may resume using the same publication ID only when the registry
/// provider implements the idempotency contract documented above.
///
/// # Errors
///
/// Returns an error for:
///
/// - Invalid publication identity or options.
/// - Invalid or incomplete package archive.
/// - Invalid package metadata.
/// - Missing or invalid integrity evidence.
/// - Indeterminate or untrusted authenticity evidence.
/// - Authorization failure.
/// - Registry/provider/protocol incompatibility.
/// - Provider operation failure.
/// - Invalid or inconsistent registry acknowledgements.
/// - Source mutation during upload.
/// - I/O failure or resource exhaustion.
pub async fn publish<R, D, E, A>(
    client: &RegistryClient,
    registry_id: RegistryId,
    protocol_version: RegistryProtocolVersion,
    publication_id: PublicationId,
    archive: &mut R,
    decoder: &mut D,
    evidence_verifier: &mut E,
    authorizer: &A,
    verification_options: VerificationOptions,
    publication_options: PublicationOptions,
) -> Result<PublicationReceipt>
where
    R: Read + Seek,
    D: PackageManifestDecoder,
    E: PackageEvidenceVerifier,
    A: PublicationAuthorizer,
{
    validate_archive_source(archive)?;

    // ------------------------------------------------------------------------
    // 1. Verify the complete package before contacting the registry.
    // ------------------------------------------------------------------------

    archive
        .seek(SeekFrom::Start(0))
        .map_err(|error| publication_io_error(
            error,
            "rewinding package archive before verification",
        ))?;

    let report: PackageVerificationReport = verify_archive(
        &mut *archive,
        decoder,
        evidence_verifier,
        verification_options,
    )
    .map_err(|error| {
        error
            .with_operation(OPERATION_PUBLISH)
            .with_context("verifying package before publication")
    })?;

    validate_verification_report(&report)?;

    let package = report.package();

    // ------------------------------------------------------------------------
    // 2. Require authorization for this exact registry and package.
    // ------------------------------------------------------------------------

    authorizer
        .authorize(
            &registry_id,
            package,
            PublicationAction::Publish,
        )
        .map_err(|error| {
            error
                .with_operation(OPERATION_PUBLISH)
                .with_context("authorizing package publication")
                .with_subject(registry_id.to_string())
        })?;

    // ------------------------------------------------------------------------
    // 3. Determine archive size and calculate the expected digest.
    // ------------------------------------------------------------------------

    let archive_size = archive
        .seek(SeekFrom::End(0))
        .map_err(|error| publication_io_error(
            error,
            "determining package archive size",
        ))?;

    let archive_digest = hash_archive(
        archive,
        publication_options.chunk_size(),
    )?;

    archive
        .seek(SeekFrom::Start(0))
        .map_err(|error| publication_io_error(
            error,
            "rewinding package archive before upload",
        ))?;

    // ------------------------------------------------------------------------
    // 4. Begin or resume the logical publication.
    // ------------------------------------------------------------------------

    let begin = BeginPublication {
        schema_version: WIRE_SCHEMA_VERSION,
        publication_id: publication_id.as_str().to_owned(),
        package_id: package.identity().id().to_string(),
        package_name: package.identity().name().to_string(),
        package_version_scheme: package.version().scheme().as_str().to_owned(),
        package_version: package.version().value().to_owned(),
        package_type: package.package_type().as_str().to_owned(),
        archive_size,
        digest_algorithm: DIGEST_ALGORITHM.to_owned(),
        archive_digest: archive_digest.clone(),
        content_entries: report.content_entries(),
        content_bytes: report.content_bytes(),
    };

    let begin_response = execute_payload(
        client,
        registry_id.clone(),
        protocol_version,
        OP_BEGIN,
        &begin,
    )
    .await?;

    let acknowledgement: BeginAcknowledgement =
        decode_acknowledgement(&begin_response)?;

    validate_begin_acknowledgement(
        &acknowledgement,
        &publication_id,
        archive_size,
        &archive_digest,
    )?;

    // ------------------------------------------------------------------------
    // 5. Resume at the exact offset acknowledged by the registry.
    // ------------------------------------------------------------------------

    archive
        .seek(SeekFrom::Start(acknowledgement.next_offset))
        .map_err(|error| publication_io_error(
            error,
            "seeking to the registry-acknowledged upload offset",
        ))?;

    // ------------------------------------------------------------------------
    // 6. Upload chunks and independently hash the bytes actually sent.
    // ------------------------------------------------------------------------

    let uploaded_digest = upload_archive(
        client,
        &registry_id,
        protocol_version,
        &publication_id,
        &acknowledgement.upload_id,
        archive,
        acknowledgement.next_offset,
        archive_size,
        publication_options.chunk_size(),
    )
    .await?;

    if uploaded_digest != archive_digest {
        return Err(publication_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            "package archive changed between verification and upload; \
             publication was not committed",
        )
        .with_subject(publication_id.to_string()));
    }

    // ------------------------------------------------------------------------
    // 7. Commit only after the complete archive has been uploaded and
    //    the streamed digest matches the previously verified source.
    // ------------------------------------------------------------------------

    let commit = CommitPublication {
        schema_version: WIRE_SCHEMA_VERSION,
        publication_id: publication_id.as_str().to_owned(),
        upload_id: acknowledgement.upload_id.clone(),
        package_id: package.identity().id().to_string(),
        package_version: package.version().value().to_owned(),
        archive_size,
        digest_algorithm: DIGEST_ALGORITHM.to_owned(),
        archive_digest: archive_digest.clone(),
    };

    let commit_response = execute_payload(
        client,
        registry_id.clone(),
        protocol_version,
        OP_COMMIT,
        &commit,
    )
    .await?;

    let committed: CommitAcknowledgement =
        decode_acknowledgement(&commit_response)?;

    validate_commit_acknowledgement(
        &committed,
        &publication_id,
        &acknowledgement.upload_id,
        &begin.package_id,
        &begin.package_version,
        archive_size,
        &archive_digest,
    )?;

    Ok(PublicationReceipt {
        registry_id,
        publication_id,
        upload_id: acknowledgement.upload_id,
        package_id: begin.package_id,
        package_name: begin.package_name,
        package_version: begin.package_version,
        archive_size,
        archive_digest,
        registry_receipt: committed.registry_receipt,
    })
}

// ============================================================================
// VERIFICATION POLICY
// ============================================================================

fn validate_verification_report(
    report: &PackageVerificationReport,
) -> Result<()> {
    report.package().validate().map_err(|error| {
        error
            .with_operation(OPERATION_PUBLISH)
            .with_context("validating verified package model")
    })?;

    if report.integrity_status() != PackageIntegrityStatus::Verified {
        return Err(publication_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            "publication requires complete package-content integrity \
             verification; partially verified packages are rejected",
        ));
    }

    if report.evidence_status() != EvidenceVerification::Verified {
        return Err(publication_error(
            ToolchainErrorKind::AuthenticationFailed,
            "publication requires verified authenticity, provenance \
             and trust evidence; unsigned, untrusted or indeterminate \
             evidence is rejected",
        ));
    }

    Ok(())
}

// ============================================================================
// ARCHIVE SOURCE VALIDATION
// ============================================================================

fn validate_archive_source<R: Seek>(
    archive: &mut R,
) -> Result<()> {
    archive
        .seek(SeekFrom::Start(0))
        .map_err(|error| publication_io_error(
            error,
            "checking package archive source",
        ))?;

    archive
        .seek(SeekFrom::End(0))
        .map_err(|error| publication_io_error(
            error,
            "checking package archive length",
        ))?;

    archive
        .seek(SeekFrom::Start(0))
        .map_err(|error| publication_io_error(
            error,
            "rewinding package archive source",
        ))?;

    Ok(())
}

// ============================================================================
// STREAMING ARCHIVE HASHING
// ============================================================================

fn allocate_buffer(size: usize) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    buffer.try_reserve_exact(size).map_err(|error| {
        publication_error(
            ToolchainErrorKind::ResourceExhausted,
            format!(
                "unable to allocate publication I/O buffer: {error}"
            ),
        )
    })?;

    buffer.resize(size, 0);

    Ok(buffer)
}

/// Hash the complete archive without retaining it in memory.
///
/// The source is left positioned at its beginning.
fn hash_archive<R: Read + Seek>(
    archive: &mut R,
    buffer_size: usize,
) -> Result<String> {
    archive
        .seek(SeekFrom::Start(0))
        .map_err(|error| publication_io_error(
            error,
            "rewinding archive before digest calculation",
        ))?;

    let mut buffer = allocate_buffer(buffer_size)?;
    let mut hasher = Sha256::new();

    loop {
        let count = archive
            .read(&mut buffer)
            .map_err(|error| publication_io_error(
                error,
                "reading archive for digest calculation",
            ))?;

        if count == 0 {
            break;
        }

        hasher.update(&buffer[..count]);
    }

    archive
        .seek(SeekFrom::Start(0))
        .map_err(|error| publication_io_error(
            error,
            "rewinding archive after digest calculation",
        ))?;

    Ok(hex::encode(hasher.finalize()))
}

// ============================================================================
// CHUNKED UPLOAD
// ============================================================================

async fn upload_archive<R: Read>(
    client: &RegistryClient,
    registry_id: &RegistryId,
    protocol_version: RegistryProtocolVersion,
    publication_id: &PublicationId,
    upload_id: &str,
    archive: &mut R,
    starting_offset: u64,
    archive_size: u64,
    chunk_size: usize,
) -> Result<String> {
    let mut buffer = allocate_buffer(chunk_size)?;
    let mut hasher = Sha256::new();

    let mut offset = starting_offset;

    // A resumed upload must still hash the complete archive. Hash the
    // already-accepted prefix locally before processing the remaining
    // bytes, so the final digest covers exactly the complete source.
    if starting_offset != 0 {
        return Err(publication_error(
            ToolchainErrorKind::UnsupportedOperation,
            "resuming at a nonzero offset requires a source-prefix \
             digest contract; this implementation refuses to commit \
             an upload whose complete-stream digest cannot be proven",
        )
        .with_subject(publication_id.to_string()));
    }

    loop {
        let count = archive
            .read(&mut buffer)
            .map_err(|error| publication_io_error(
                error,
                "reading package archive upload chunk",
            ))?;

        if count == 0 {
            break;
        }

        let count_u64 = u64::try_from(count).map_err(|_| {
            publication_error(
                ToolchainErrorKind::ResourceExhausted,
                "upload chunk length cannot be represented as u64",
            )
        })?;

        let next_offset = offset.checked_add(count_u64).ok_or_else(|| {
            publication_error(
                ToolchainErrorKind::ResourceExhausted,
                "package upload offset overflowed",
            )
        })?;

        if next_offset > archive_size {
            return Err(publication_error(
                ToolchainErrorKind::IntegrityVerificationFailed,
                "archive source supplied more bytes than its measured size",
            ));
        }

        hasher.update(&buffer[..count]);

        let payload = UploadChunk {
            schema_version: WIRE_SCHEMA_VERSION,
            publication_id: publication_id.as_str().to_owned(),
            upload_id: upload_id.to_owned(),
            offset,
            data_encoding: "hex".to_owned(),
            data: hex::encode(&buffer[..count]),
        };

        let response = execute_payload(
            client,
            registry_id.clone(),
            protocol_version,
            OP_CHUNK,
            &payload,
        )
        .await?;

        let acknowledgement: ChunkAcknowledgement =
            decode_acknowledgement(&response)?;

        validate_chunk_acknowledgement(
            &acknowledgement,
            publication_id,
            upload_id,
            next_offset,
        )?;

        offset = next_offset;
    }

    if offset != archive_size {
        return Err(publication_error(
            ToolchainErrorKind::IntegrityVerificationFailed,
            format!(
                "uploaded {offset} bytes, but the archive size is \
                 {archive_size} bytes"
            ),
        ));
    }

    Ok(hex::encode(hasher.finalize()))
}

// ============================================================================
// REGISTRY OPERATION EXECUTION
// ============================================================================

async fn execute_payload<T: Serialize>(
    client: &RegistryClient,
    registry_id: RegistryId,
    protocol_version: RegistryProtocolVersion,
    operation: &str,
    payload: &T,
) -> Result<Vec<u8>> {
    let operation_id = RegistryOperationId::new(operation)
        .map_err(|error| {
            error
                .with_operation(OPERATION_PUBLISH)
                .with_context("constructing registry publication operation")
        })?;

    let encoded_payload = serde_json::to_vec(payload).map_err(|error| {
        publication_error(
            ToolchainErrorKind::InvalidInput,
            "unable to serialize registry publication operation payload",
        )
        .with_source(error)
    })?;

    let request = RegistryRequest::new(
        registry_id,
        protocol_version,
        operation_id,
        encoded_payload,
    );

    let response = client
        .execute(request)
        .await
        .map_err(|error| {
            error
                .with_operation(OPERATION_PUBLISH)
                .with_context(format!(
                    "executing registry publication operation `{operation}`"
                ))
        })?;

    Ok(response.payload().to_vec())
}

// ============================================================================
// ACKNOWLEDGEMENT DECODING
// ============================================================================

fn decode_acknowledgement<T: for<'de> Deserialize<'de>>(
    payload: &[u8],
) -> Result<T> {
    serde_json::from_slice(payload).map_err(|error| {
        publication_error(
            ToolchainErrorKind::ValidationFailed,
            "registry returned a malformed publication acknowledgement",
        )
        .with_source(error)
    })
}

fn validate_begin_acknowledgement(
    acknowledgement: &BeginAcknowledgement,
    publication_id: &PublicationId,
    archive_size: u64,
    archive_digest: &str,
) -> Result<()> {
    if acknowledgement.schema_version != WIRE_SCHEMA_VERSION {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported begin-acknowledgement schema",
        ));
    }

    if acknowledgement.publication_id != publication_id.as_str() {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement has a different publication identity",
        ));
    }

    validate_server_text(&acknowledgement.upload_id, "upload identity")?;

    if acknowledgement.archive_size != archive_size {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement has a different archive size",
        ));
    }

    if acknowledgement.archive_digest != archive_digest {
        return Err(invalid_acknowledgement(
            "registry begin acknowledgement has a different archive digest",
        ));
    }

    if acknowledgement.next_offset > archive_size {
        return Err(invalid_acknowledgement(
            "registry acknowledged an upload offset beyond the archive size",
        ));
    }

    if acknowledgement.next_offset != 0 {
        return Err(publication_error(
            ToolchainErrorKind::UnsupportedOperation,
            "registry requested a resumed upload offset, but this source \
             implementation cannot prove the digest of the previously \
             accepted prefix; publication is stopped safely",
        ));
    }

    Ok(())
}

fn validate_chunk_acknowledgement(
    acknowledgement: &ChunkAcknowledgement,
    publication_id: &PublicationId,
    upload_id: &str,
    expected_next_offset: u64,
) -> Result<()> {
    if acknowledgement.schema_version != WIRE_SCHEMA_VERSION {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported chunk-acknowledgement schema",
        ));
    }

    if acknowledgement.publication_id != publication_id.as_str() {
        return Err(invalid_acknowledgement(
            "registry chunk acknowledgement has a different publication identity",
        ));
    }

    if acknowledgement.upload_id != upload_id {
        return Err(invalid_acknowledgement(
            "registry chunk acknowledgement has a different upload identity",
        ));
    }

    if acknowledgement.next_offset != expected_next_offset {
        return Err(invalid_acknowledgement(
            "registry did not acknowledge the exact uploaded chunk boundary",
        ));
    }

    Ok(())
}

fn validate_commit_acknowledgement(
    acknowledgement: &CommitAcknowledgement,
    publication_id: &PublicationId,
    upload_id: &str,
    package_id: &str,
    package_version: &str,
    archive_size: u64,
    archive_digest: &str,
) -> Result<()> {
    if acknowledgement.schema_version != WIRE_SCHEMA_VERSION {
        return Err(invalid_acknowledgement(
            "registry returned an unsupported commit-acknowledgement schema",
        ));
    }

    if acknowledgement.publication_id != publication_id.as_str() {
        return Err(invalid_acknowledgement(
            "registry commit acknowledgement has a different publication identity",
        ));
    }

    if acknowledgement.upload_id != upload_id {
        return Err(invalid_acknowledgement(
            "registry commit acknowledgement has a different upload identity",
        ));
    }

    if acknowledgement.package_id != package_id {
        return Err(invalid_acknowledgement(
            "registry committed a different package identity",
        ));
    }

    if acknowledgement.package_version != package_version {
        return Err(invalid_acknowledgement(
            "registry committed a different package version",
        ));
    }

    if acknowledgement.archive_size != archive_size {
        return Err(invalid_acknowledgement(
            "registry committed a different archive size",
        ));
    }

    if acknowledgement.digest_algorithm != DIGEST_ALGORITHM {
        return Err(invalid_acknowledgement(
            "registry committed an unexpected archive digest algorithm",
        ));
    }

    if acknowledgement.archive_digest != archive_digest {
        return Err(invalid_acknowledgement(
            "registry committed a different archive digest",
        ));
    }

    validate_server_text(
        &acknowledgement.registry_receipt,
        "registry publication receipt",
    )
}

// ============================================================================
// VALIDATION AND ERROR HELPERS
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

fn invalid_acknowledgement(
    message: impl Into<String>,
) -> ToolchainError {
    publication_error(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
}

fn publication_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(OPERATION_PUBLISH)
}

fn publication_io_error(
    error: std::io::Error,
    context: &'static str,
) -> ToolchainError {
    ToolchainError::from(error)
        .with_operation(OPERATION_PUBLISH)
        .with_context(context)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publication_identity_rejects_empty_values() {
        assert!(PublicationId::new("").is_err());
        assert!(PublicationId::new("   ").is_err());
    }

    #[test]
    fn publication_identity_rejects_whitespace_and_control_characters() {
        assert!(PublicationId::new("publication id").is_err());
        assert!(PublicationId::new("publication\nid").is_err());
        assert!(PublicationId::new("publication\tid").is_err());
    }

    #[test]
    fn publication_identity_preserves_valid_open_world_values() {
        let id = PublicationId::new(
            "publisher.production.release-2026-001",
        )
        .expect("valid publication identity");

        assert_eq!(
            id.as_str(),
            "publisher.production.release-2026-001"
        );
    }

    #[test]
    fn publication_identity_has_no_arbitrary_length_ceiling() {
        let value = "p".repeat(16_384);

        let id = PublicationId::new(value.clone())
            .expect("valid long publication identity");

        assert_eq!(id.as_str(), value);
    }

    #[test]
    fn zero_chunk_size_is_rejected() {
        assert!(PublicationOptions::new(0).is_err());
    }

    #[test]
    fn configured_chunk_size_is_preserved() {
        let options = PublicationOptions::new(64 * 1024)
            .expect("valid publication options");

        assert_eq!(options.chunk_size(), 64 * 1024);
    }

    #[test]
    fn archive_digest_is_sha256() {
        use std::io::Cursor;

        let mut archive = Cursor::new(b"Zamani package archive".to_vec());

        let digest = hash_archive(&mut archive, 4)
            .expect("archive digest");

        let expected = hex::encode(
            Sha256::digest(b"Zamani package archive"),
        );

        assert_eq!(digest, expected);
        assert_eq!(archive.position(), 0);
    }

    #[test]
    fn archive_hashing_supports_empty_archives_as_a_stream() {
        use std::io::Cursor;

        let mut archive = Cursor::new(Vec::<u8>::new());

        let digest = hash_archive(&mut archive, 8)
            .expect("empty archive digest");

        let expected = hex::encode(Sha256::digest([]));

        assert_eq!(digest, expected);
    }

    #[test]
    fn malformed_begin_acknowledgement_is_rejected() {
        let publication_id = PublicationId::new("publication-1")
            .expect("valid identity");

        let acknowledgement = BeginAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            publication_id: "different-publication".to_owned(),
            upload_id: "upload-1".to_owned(),
            archive_size: 10,
            archive_digest: "digest".to_owned(),
            next_offset: 0,
        };

        assert!(
            validate_begin_acknowledgement(
                &acknowledgement,
                &publication_id,
                10,
                "digest",
            )
            .is_err()
        );
    }

    #[test]
    fn nonzero_resume_offset_is_rejected_safely() {
        let publication_id = PublicationId::new("publication-1")
            .expect("valid identity");

        let acknowledgement = BeginAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            publication_id: publication_id.as_str().to_owned(),
            upload_id: "upload-1".to_owned(),
            archive_size: 10,
            archive_digest: "digest".to_owned(),
            next_offset: 5,
        };

        assert!(
            validate_begin_acknowledgement(
                &acknowledgement,
                &publication_id,
                10,
                "digest",
            )
            .is_err()
        );
    }

    #[test]
    fn chunk_acknowledgement_must_match_exact_offset() {
        let publication_id = PublicationId::new("publication-1")
            .expect("valid identity");

        let acknowledgement = ChunkAcknowledgement {
            schema_version: WIRE_SCHEMA_VERSION,
            publication_id: publication_id.as_str().to_owned(),
            upload_id: "upload-1".to_owned(),
            next_offset: 9,
        };

        assert!(
            validate_chunk_acknowledgement(
                &acknowledgement,
                &publication_id,
                "upload-1",
                10,
            )
            .is_err()
        );
    }
}
