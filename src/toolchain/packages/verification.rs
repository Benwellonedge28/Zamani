
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Verification
//!
//! Path: `src/toolchain/packages/verification.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify a complete `.zpkg` package before it is accepted by package
//! installation, registry publication, dependency acquisition or another
//! security-sensitive package operation.
//!
//! Verification covers:
//!
//! - Archive structure and complete stream consumption.
//! - Archive-to-manifest inventory agreement.
//! - Canonical package-model validation.
//! - Content-entry identity and ordering.
//! - Declared content sizes.
//! - Declared content digests.
//! - Artifact content digests.
//! - Signature, provenance and trust-policy integration.
//!
//! ## Ownership
//!
//! This module owns package-verification orchestration and its report.
//!
//! It does not own:
//!
//! - `.zpkg` serialization or archive framing.
//! - Package manifest syntax or serialization.
//! - Package identity allocation.
//! - Dependency resolution.
//! - Package extraction or installation.
//! - Registry communication.
//! - Cryptographic signing.
//! - Publisher authentication.
//! - Signature algorithms.
//! - Trust-store persistence.
//! - Provenance ledger persistence.
//! - Artifact compatibility evaluation.
//! - Compiler or backend execution.
//! - Hardware discovery or runtime execution.
//!
//! ## Canonical authorities
//!
//! Package model:
//! `super::model`
//!
//! Archive format:
//! `super::archive`
//!
//! Package manifest decoding:
//! `super::metadata` (integration contract; implementation not yet present
//! in the inspected repository)
//!
//! Signature and trust decisions:
//! `crate::toolchain::security`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Toolchain results:
//! `crate::toolchain::result`
//!
//! SHA-256:
//! Existing `sha2` dependency.
//!
//! ## Verification semantics
//!
//! Structural validity, integrity, authenticity and trust are separate
//! properties. A successful content digest does not establish publisher
//! authenticity. A valid signature does not establish publisher trust.
//!
//! An absent digest is not treated as a verified digest.
//!
//! Unsupported digest algorithms are rejected rather than silently skipped.
//!
//! Authenticity evidence is evaluated by a mandatory external policy
//! provider. An indeterminate result is never converted into success.
//!
//! ## Scalability
//!
//! Package content is processed as a stream.
//!
//! The verifier does not load complete package files or archives into
//! memory. Manifest memory consumption is proportional to the manifest
//! representation because the current package metadata decoder contract
//! consumes a byte slice.
//!
//! A caller may configure a manifest budget. No universal package-size,
//! entry-count, artifact-count, domain-count, dependency-count, or
//! hardware-capacity ceiling is imposed here.
//!
//! Actual operation remains subject to available memory, processing time,
//! storage, operating-system limits and explicit operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Package content is compared against the deterministic ordered inventory
//! supplied by `Package::contents()`.
//!
//! Archive ordering is independently enforced by `ArchiveReader`.
//!
//! ## Security
//!
//! The verifier never executes package contents.
//!
//! It does not trust metadata merely because it parses successfully.
//!
//! It does not infer that an artifact is executable, compatible with a
//! particular target, or feasible on available hardware.
//!
//! ## Integration
//!
//! Consumed by:
//!
//! - `packages::installation`
//! - `packages::extraction`
//! - `packages::creation`
//! - `registries::downloading`
//! - `registries::publishing`
//! - `dependencies::sources`
//!
//! The metadata and security modules must implement the provider contracts
//! declared here or provide adapters with equivalent guarantees.

use std::io::{self, Read, Write};
use std::num::NonZeroUsize;

use sha2::{Digest, Sha256};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::Result;

use super::archive::{
    ArchiveFormatVersion,
    ArchiveHeader,
    ArchiveIoOptions,
    ArchiveReader,
    DEFAULT_IO_BUFFER_SIZE,
};

use super::model::{
    Package,
    PackageArtifact,
    PackageContent,
    PackagePath,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_VERIFY: &str = "package.verify";

const SHA256_ALGORITHM: &str = "sha-256";

// ============================================================================
// VERIFICATION OPTIONS
// ============================================================================

/// Operational and security-policy options for package verification.
///
/// These settings are per-operation policy. They do not define language
/// capabilities, package-format capabilities or hardware limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationOptions {
    io_buffer_size: NonZeroUsize,
    max_manifest_bytes: Option<u128>,
    require_content_digests: bool,
    allow_unsigned_packages: bool,
}

impl VerificationOptions {
    /// Construct package-verification options.
    ///
    /// A zero-sized I/O buffer is invalid.
    ///
    /// `max_manifest_bytes = None` means that this verifier does not impose
    /// a manifest-size budget. The host's actual memory capacity still
    /// applies.
    ///
    /// `require_content_digests` requires every package content entry to
    /// declare a supported digest.
    ///
    /// `allow_unsigned_packages` permits the evidence provider to explicitly
    /// report that authenticity evidence is not required. It does not
    /// convert an invalid signature or indeterminate trust decision into
    /// success.
    pub fn new(
        io_buffer_size: usize,
        max_manifest_bytes: Option<u128>,
        require_content_digests: bool,
        allow_unsigned_packages: bool,
    ) -> Result<Self> {
        let io_buffer_size = NonZeroUsize::new(io_buffer_size)
            .ok_or_else(|| {
                ToolchainError::invalid_input(
                    "package verification I/O buffer must be nonzero",
                )
                .with_operation(OPERATION_VERIFY)
            })?;

        Ok(Self {
            io_buffer_size,
            max_manifest_bytes,
            require_content_digests,
            allow_unsigned_packages,
        })
    }

    /// Return the I/O buffer size.
    #[must_use]
    pub const fn io_buffer_size(self) -> usize {
        self.io_buffer_size.get()
    }

    /// Return the optional manifest-size budget.
    #[must_use]
    pub const fn max_manifest_bytes(self) -> Option<u128> {
        self.max_manifest_bytes
    }

    /// Return whether every content entry must declare a digest.
    #[must_use]
    pub const fn require_content_digests(self) -> bool {
        self.require_content_digests
    }

    /// Return whether explicitly unsigned packages are permitted.
    #[must_use]
    pub const fn allow_unsigned_packages(self) -> bool {
        self.allow_unsigned_packages
    }
}

impl Default for VerificationOptions {
    fn default() -> Self {
        Self {
            io_buffer_size: NonZeroUsize::new(DEFAULT_IO_BUFFER_SIZE)
                .expect("the archive I/O default is nonzero"),
            max_manifest_bytes: None,
            require_content_digests: true,
            allow_unsigned_packages: false,
        }
    }
}

// ============================================================================
// MANIFEST DECODING CONTRACT
// ============================================================================

/// Canonical package-manifest decoding and validation authority.
///
/// The implementation belongs in `packages::metadata`.
///
/// The decoder must:
///
/// - Decode the canonical package manifest representation.
/// - Reject malformed and ambiguous representations.
/// - Reject duplicate metadata keys.
/// - Preserve package identity and exact release-version information.
/// - Preserve the complete declared package-content inventory.
/// - Preserve artifact compatibility declarations.
/// - Preserve signature and provenance references.
/// - Construct the canonical `Package` model.
/// - Call `Package::validate()` before returning.
///
/// The decoder must not silently discard unknown security-relevant fields.
///
/// Unknown extension fields may only be accepted according to the
/// applicable manifest-version and extension-compatibility contract.
///
/// This trait does not perform cryptographic verification.
pub trait PackageManifestDecoder {
    /// Decode and structurally validate a canonical package manifest.
    fn decode_and_validate(&mut self, bytes: &[u8]) -> Result<Package>;
}

// ============================================================================
// AUTHENTICITY AND TRUST CONTRACT
// ============================================================================

/// Result of evaluating package authenticity evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EvidenceVerification {
    /// All applicable signature, provenance and trust checks succeeded.
    Verified,

    /// The authoritative policy explicitly determined that authenticity
    /// evidence is not required for this package and operation.
    NotRequired,

    /// The evidence could not be evaluated conclusively.
    ///
    /// This result must never be interpreted as successful verification.
    Indeterminate,
}

/// Mandatory integration boundary for package authenticity and trust.
///
/// The implementation must delegate to the repository's canonical
/// security, signature, trust and provenance authorities.
///
/// The provider must inspect all applicable artifact signature references
/// and provenance references in the package model.
///
/// It must not assume that a declared signer is trusted merely because
/// the signature is mathematically valid.
///
/// It must not treat an absent signature as a valid signature.
///
/// It must not treat an unavailable trust service as a successful result.
///
/// The provider must not modify the package or manifest bytes.
pub trait PackageEvidenceVerifier {
    /// Verify package signatures, publisher trust and applicable provenance.
    fn verify_evidence(
        &mut self,
        package: &Package,
        canonical_manifest: &[u8],
    ) -> Result<EvidenceVerification>;
}

// ============================================================================
// VERIFICATION REPORT
// ============================================================================

/// Integrity status for the complete package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageIntegrityStatus {
    /// All required declared content digests matched the actual bytes.
    Verified,

    /// Integrity was checked only for entries that declared digests.
    ///
    /// At least one entry did not provide a digest and the caller did not
    /// require digests for every entry.
    PartiallyVerified,
}

/// Immutable report of a successfully verified package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageVerificationReport {
    package: Package,
    archive_version: ArchiveFormatVersion,
    manifest_bytes: u128,
    content_entries: u128,
    content_bytes: u128,
    verified_content_digests: u128,
    verified_artifact_digests: u128,
    integrity: PackageIntegrityStatus,
    evidence: EvidenceVerification,
}

impl PackageVerificationReport {
    /// Return the verified canonical package model.
    #[must_use]
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Return the archive-format version.
    #[must_use]
    pub const fn archive_version(&self) -> ArchiveFormatVersion {
        self.archive_version
    }

    /// Return the number of manifest bytes processed.
    #[must_use]
    pub const fn manifest_bytes(&self) -> u128 {
        self.manifest_bytes
    }

    /// Return the number of verified content entries.
    #[must_use]
    pub const fn content_entries(&self) -> u128 {
        self.content_entries
    }

    /// Return the total number of content bytes processed.
    #[must_use]
    pub const fn content_bytes(&self) -> u128 {
        self.content_bytes
    }

    /// Return the number of verified content-entry digests.
    #[must_use]
    pub const fn verified_content_digests(&self) -> u128 {
        self.verified_content_digests
    }

    /// Return the number of verified artifact digests.
    #[must_use]
    pub const fn verified_artifact_digests(&self) -> u128 {
        self.verified_artifact_digests
    }

    /// Return the overall package-content integrity status.
    #[must_use]
    pub const fn integrity_status(&self) -> PackageIntegrityStatus {
        self.integrity
    }

    /// Return the authenticity and trust evidence result.
    #[must_use]
    pub const fn evidence_status(&self) -> EvidenceVerification {
        self.evidence
    }

    /// Consume the report and return the verified package model.
    #[must_use]
    pub fn into_package(self) -> Package {
        self.package
    }
}

// ============================================================================
// INTERNAL HASHING WRITER
// ============================================================================

/// Streaming SHA-256 sink.
///
/// The complete content is never accumulated in memory.
struct Sha256Sink {
    hasher: Sha256,
    bytes_written: u128,
}

impl Sha256Sink {
    fn new() -> Self {
        Self {
            hasher: Sha256::new(),
            bytes_written: 0,
        }
    }

    fn finish(self) -> ([u8; 32], u128) {
        let digest = self.hasher.finalize();

        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(&digest);

        (bytes, self.bytes_written)
    }
}

impl Write for Sha256Sink {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let length = u128::try_from(buffer.len()).map_err(|_| {
            io::Error::other("content length cannot be represented")
        })?;

        let next = self.bytes_written.checked_add(length).ok_or_else(|| {
            io::Error::other("content byte counter overflow")
        })?;

        self.hasher.update(buffer);
        self.bytes_written = next;

        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn verification_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(OPERATION_VERIFY)
}

fn integrity_error(message: impl Into<String>) -> ToolchainError {
    ToolchainError::integrity_verification_failed(message)
        .with_operation(OPERATION_VERIFY)
}

fn resource_error(message: impl Into<String>) -> ToolchainError {
    ToolchainError::resource_exhausted(message)
        .with_operation(OPERATION_VERIFY)
}

fn io_error(
    error: io::Error,
    context: &'static str,
) -> ToolchainError {
    ToolchainError::from(error)
        .with_operation(OPERATION_VERIFY)
        .with_context(context)
}

// ============================================================================
// MANIFEST HANDLING
// ============================================================================

fn read_manifest<R: Read>(
    reader: &mut ArchiveReader<R>,
    header: ArchiveHeader,
    options: VerificationOptions,
) -> Result<Vec<u8>> {
    let length = header.manifest_length();

    if let Some(budget) = options.max_manifest_bytes() {
        if length > budget {
            return Err(resource_error(format!(
                "package manifest requires {length} bytes, \
                 exceeding the configured budget of {budget} bytes"
            )));
        }
    }

    let length = usize::try_from(length).map_err(|_| {
        resource_error(
            "package manifest cannot be represented by this host",
        )
    })?;

    let mut manifest = Vec::new();

    manifest.try_reserve_exact(length).map_err(|error| {
        resource_error(format!(
            "unable to allocate package manifest storage: {error}"
        ))
    })?;

    manifest.resize(length, 0);

    let copied = reader
        .copy_manifest_to(&mut manifest.as_mut_slice())
        .map_err(|error| {
            error
                .with_operation(OPERATION_VERIFY)
                .with_context("reading canonical package manifest")
        })?;

    if copied != length as u128 {
        return Err(integrity_error(
            "archive manifest length differs from its declared length",
        ));
    }

    Ok(manifest)
}

// ============================================================================
// PACKAGE MODEL VALIDATION
// ============================================================================

fn validate_package_model(package: &Package) -> Result<()> {
    package.validate().map_err(|error| {
        error
            .with_operation(OPERATION_VERIFY)
            .with_context("validating canonical package model")
    })?;

    for (path, content) in package.contents() {
        if path != content.path() {
            return Err(integrity_error(format!(
                "package content inventory key does not match \
                 its descriptor path `{path}`"
            )));
        }
    }

    for (path, artifact) in package.artifacts() {
        if path != artifact.content_path() {
            return Err(integrity_error(format!(
                "package artifact inventory key does not match \
                 its descriptor path `{path}`"
            )));
        }

        if !package.contents().contains_key(path) {
            return Err(integrity_error(format!(
                "package artifact `{path}` references missing content"
            )));
        }
    }

    Ok(())
}

// ============================================================================
// DIGEST VALIDATION
// ============================================================================

fn verify_declared_digest(
    path: &PackagePath,
    expected_algorithm: &str,
    expected_value: &str,
    actual: &[u8; 32],
) -> Result<()> {
    if expected_algorithm != SHA256_ALGORITHM {
        return Err(
            ToolchainError::unsupported_operation(format!(
                "unsupported package digest algorithm `{expected_algorithm}` \
                 for `{path}`"
            ))
            .with_operation(OPERATION_VERIFY),
        );
    }

    let expected = hex::decode(expected_value).map_err(|_| {
        integrity_error(format!(
            "package digest for `{path}` is not valid hexadecimal"
        ))
    })?;

    if expected.as_slice() != actual {
        return Err(integrity_error(format!(
            "SHA-256 digest mismatch for package content `{path}`"
        )));
    }

    Ok(())
}

fn verify_content_descriptor(
    path: &PackagePath,
    content: &PackageContent,
    artifact: Option<&PackageArtifact>,
    actual_digest: &[u8; 32],
    actual_size: u128,
    options: VerificationOptions,
) -> Result<bool> {
    if let Some(expected_size) = content.declared_size() {
        if actual_size != expected_size {
            return Err(integrity_error(format!(
                "package content `{path}` has {actual_size} bytes, \
                 but its manifest declares {expected_size} bytes"
            )));
        }
    }

    let mut verified = false;

    match content.digest() {
        Some(digest) => {
            verify_declared_digest(
                path,
                digest.algorithm(),
                digest.value(),
                actual_digest,
            )?;

            verified = true;
        }

        None if options.require_content_digests() => {
            return Err(integrity_error(format!(
                "package content `{path}` has no declared digest"
            )));
        }

        None => {}
    }

    if let Some(artifact) = artifact {
        if let Some(digest) = artifact.digest() {
            verify_declared_digest(
                path,
                digest.algorithm(),
                digest.value(),
                actual_digest,
            )?;

            verified = true;
        }
    }

    Ok(verified)
}

// ============================================================================
// EVIDENCE VALIDATION
// ============================================================================

fn verify_package_evidence<E: PackageEvidenceVerifier>(
    package: &Package,
    manifest: &[u8],
    verifier: &mut E,
    options: VerificationOptions,
) -> Result<EvidenceVerification> {
    let evidence = verifier
        .verify_evidence(package, manifest)
        .map_err(|error| {
            error
                .with_operation(OPERATION_VERIFY)
                .with_context(
                    "evaluating package signatures, provenance and trust",
                )
        })?;

    match evidence {
        EvidenceVerification::Verified => Ok(evidence),

        EvidenceVerification::NotRequired
            if options.allow_unsigned_packages() =>
        {
            Ok(evidence)
        }

        EvidenceVerification::NotRequired => {
            Err(ToolchainError::authentication_failed(
                "package authenticity evidence is required by policy",
            )
            .with_operation(OPERATION_VERIFY))
        }

        EvidenceVerification::Indeterminate => {
            Err(ToolchainError::authentication_failed(
                "package authenticity or trust could not be established",
            )
            .with_operation(OPERATION_VERIFY))
        }
    }
}

// ============================================================================
// PUBLIC VERIFICATION API
// ============================================================================

/// Verify a complete `.zpkg` archive.
///
/// This operation:
///
/// 1. Validates the archive header.
/// 2. Reads the manifest within the configured operational budget.
/// 3. Decodes the canonical package model.
/// 4. Validates the complete model and artifact references.
/// 5. Evaluates authenticity and trust evidence.
/// 6. Confirms archive and package inventory counts.
/// 7. Streams every archive entry through SHA-256.
/// 8. Checks declared sizes and content digests.
/// 9. Checks artifact-specific digests.
/// 10. Rejects undeclared or missing content.
/// 11. Consumes the complete archive and rejects trailing bytes.
///
/// The operation never executes package content.
///
/// A successful report establishes the checks described by its status
/// fields. It does not establish target compatibility or execution
/// feasibility.
///
/// # Type parameters
///
/// - `R`: archive byte-stream reader.
/// - `D`: canonical package manifest decoder.
/// - `E`: mandatory authenticity and trust evidence verifier.
///
/// # Errors
///
/// Returns an error for malformed archives, invalid manifests, missing
/// content, undeclared entries, digest mismatches, unsupported digest
/// algorithms, invalid authenticity evidence, indeterminate trust,
/// resource exhaustion or I/O failure.
pub fn verify_archive<R, D, E>(
    source: R,
    decoder: &mut D,
    evidence_verifier: &mut E,
    options: VerificationOptions,
) -> Result<PackageVerificationReport>
where
    R: Read,
    D: PackageManifestDecoder,
    E: PackageEvidenceVerifier,
{
    let archive_options =
        ArchiveIoOptions::new(options.io_buffer_size())?;

    let mut reader = ArchiveReader::new_with_options(
        source,
        archive_options,
    )
    .map_err(|error| {
        error
            .with_operation(OPERATION_VERIFY)
            .with_context("opening package archive")
    })?;

    let header = reader.header();

    let manifest = read_manifest(
        &mut reader,
        header,
        options,
    )?;

    let package = decoder
        .decode_and_validate(&manifest)
        .map_err(|error| {
            error
                .with_operation(OPERATION_VERIFY)
                .with_context("decoding canonical package metadata")
        })?;

    validate_package_model(&package)?;

    let declared_entries = package.contents().len() as u128;

    if header.entry_count() != declared_entries {
        return Err(integrity_error(format!(
            "archive declares {} entries, but the package manifest \
             declares {declared_entries}",
            header.entry_count()
        )));
    }

    let evidence = verify_package_evidence(
        &package,
        &manifest,
        evidence_verifier,
        options,
    )?;

    let mut verified_content_digests = 0_u128;
    let mut verified_artifact_digests = 0_u128;
    let mut content_bytes = 0_u128;
    let mut content_entries = 0_u128;
    let mut missing_digest_entries = 0_u128;

    loop {
        let entry = reader
            .next_entry()
            .map_err(|error| {
                error
                    .with_operation(OPERATION_VERIFY)
                    .with_context("opening package content entry")
            })?;

        let Some(mut entry) = entry else {
            break;
        };

        let path = entry.path().clone();

        let content = package.contents().get(&path).ok_or_else(|| {
            integrity_error(format!(
                "archive contains undeclared package content `{path}`"
            ))
        })?;

        let artifact = package.artifacts().get(&path);

        let mut sink = Sha256Sink::new();

        io::copy(&mut entry, &mut sink).map_err(|error| {
            io_error(error, "streaming package content through SHA-256")
                .with_context(format!("verifying package entry `{path}`"))
        })?;

        let (actual_digest, actual_size) = sink.finish();

        let archive_entry_size = entry.bytes_read();

        if actual_size != archive_entry_size {
            return Err(ToolchainError::internal(
                "package hashing byte count differs from archive reader accounting",
            )
            .with_operation(OPERATION_VERIFY));
        }

        let verified = verify_content_descriptor(
            &path,
            content,
            artifact,
            &actual_digest,
            actual_size,
            options,
        )?;

        if content.digest().is_some() {
            verified_content_digests = verified_content_digests
                .checked_add(1)
                .ok_or_else(|| resource_error(
                    "verified content digest counter overflowed"
                ))?;
        } else {
            missing_digest_entries = missing_digest_entries
                .checked_add(1)
                .ok_or_else(|| resource_error(
                    "missing digest counter overflowed"
                ))?;
        }

        if artifact.and_then(PackageArtifact::digest).is_some() {
            verified_artifact_digests = verified_artifact_digests
                .checked_add(1)
                .ok_or_else(|| resource_error(
                    "verified artifact digest counter overflowed"
                ))?;
        }

        if !verified && content.digest().is_none() {
            // An entry with no content or artifact digest was not
            // cryptographically verified, even if its size matched.
        }

        content_bytes = content_bytes
            .checked_add(actual_size)
            .ok_or_else(|| resource_error(
                "package content byte counter overflowed"
            ))?;

        content_entries = content_entries
            .checked_add(1)
            .ok_or_else(|| resource_error(
                "package content entry counter overflowed"
            ))?;

        // The entry is fully consumed. Dropping it now is safe.
        drop(entry);
    }

    if content_entries != declared_entries {
        return Err(integrity_error(format!(
            "verified {content_entries} entries, but the package \
             manifest declares {declared_entries}"
        )));
    }

    // This is essential: consuming all declared entries does not by itself
    // establish that the archive has no trailing or malformed data.
    reader.finish().map_err(|error| {
        error
            .with_operation(OPERATION_VERIFY)
            .with_context("validating complete archive termination")
    })?;

    let integrity = if missing_digest_entries == 0 {
        PackageIntegrityStatus::Verified
    } else {
        PackageIntegrityStatus::PartiallyVerified
    };

    Ok(PackageVerificationReport {
        package,
        archive_version: header.version(),
        manifest_bytes: manifest.len() as u128,
        content_entries,
        content_bytes,
        verified_content_digests,
        verified_artifact_digests,
        integrity,
        evidence,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_requires_content_digests() {
        let options = VerificationOptions::default();

        assert!(options.require_content_digests());
        assert!(!options.allow_unsigned_packages());
        assert_eq!(options.max_manifest_bytes(), None);
        assert!(options.io_buffer_size() > 0);
    }

    #[test]
    fn zero_io_buffer_is_rejected() {
        let result = VerificationOptions::new(
            0,
            None,
            true,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn explicit_manifest_budget_is_preserved() {
        let options = VerificationOptions::new(
            4096,
            Some(1024),
            true,
            false,
        )
        .expect("valid options");

        assert_eq!(options.max_manifest_bytes(), Some(1024));
    }

    #[test]
    fn sha256_sink_hashes_incrementally() {
        let mut sink = Sha256Sink::new();

        sink.write_all(b"Zamani")
            .expect("first write");

        sink.write_all(b" package")
            .expect("second write");

        let (actual, size) = sink.finish();

        let expected: [u8; 32] =
            Sha256::digest(b"Zamani package").into();

        assert_eq!(actual, expected);
        assert_eq!(size, 14);
    }

    #[test]
    fn sha256_sink_handles_empty_content() {
        let sink = Sha256Sink::new();

        let (actual, size) = sink.finish();

        let expected: [u8; 32] =
            Sha256::digest([]).into();

        assert_eq!(actual, expected);
        assert_eq!(size, 0);
    }

    #[test]
    fn digest_verification_accepts_correct_sha256() {
        let bytes = b"package-content";
        let digest: [u8; 32] = Sha256::digest(bytes).into();

        let result = verify_declared_digest(
            &PackagePath::new("src/main.zm")
                .expect("valid path"),
            SHA256_ALGORITHM,
            &hex::encode(digest),
            &digest,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn digest_verification_rejects_wrong_sha256() {
        let expected = Sha256::digest(b"expected");
        let actual: [u8; 32] =
            Sha256::digest(b"actual").into();

        let result = verify_declared_digest(
            &PackagePath::new("src/main.zm")
                .expect("valid path"),
            SHA256_ALGORITHM,
            &hex::encode(expected),
            &actual,
        );

        assert!(result.is_err());
    }

    #[test]
    fn unsupported_digest_algorithms_are_not_silently_accepted() {
        let actual: [u8; 32] =
            Sha256::digest(b"content").into();

        let result = verify_declared_digest(
            &PackagePath::new("artifact.bin")
                .expect("valid path"),
            "unknown-digest",
            "some-value",
            &actual,
        );

        assert!(result.is_err());
    }

    #[test]
    fn invalid_hexadecimal_digest_is_rejected() {
        let actual: [u8; 32] =
            Sha256::digest(b"content").into();

        let result = verify_declared_digest(
            &PackagePath::new("artifact.bin")
                .expect("valid path"),
            SHA256_ALGORITHM,
            "not-hexadecimal",
            &actual,
        );

        assert!(result.is_err());
    }

    #[test]
    fn authenticity_evidence_states_are_distinct() {
        assert_ne!(
            EvidenceVerification::Verified,
            EvidenceVerification::NotRequired,
        );

        assert_ne!(
            EvidenceVerification::Verified,
            EvidenceVerification::Indeterminate,
        );

        assert_ne!(
            EvidenceVerification::NotRequired,
            EvidenceVerification::Indeterminate,
        );
    }

    #[test]
    fn integrity_states_are_distinct() {
        assert_ne!(
            PackageIntegrityStatus::Verified,
            PackageIntegrityStatus::PartiallyVerified,
        );
    }
}
