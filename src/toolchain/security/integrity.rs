
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Content Integrity
//!
//! Path: `src/toolchain/security/integrity.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provide the toolchain's common, streaming content-integrity verification
//! boundary for package entries, dependencies, build artifacts, downloaded
//! content and other byte-oriented resources.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Integrity verification orchestration.
//! - Expected-versus-actual digest comparison.
//! - Streaming byte accounting.
//! - Optional per-operation byte budgets.
//! - Structured integrity reports.
//! - The adapter contract for canonical hashing providers.
//!
//! This module does not own:
//!
//! - Hash algorithms or their implementations.
//! - Hash-provider discovery or registration.
//! - Signature creation or verification.
//! - Publisher or provider trust assessment.
//! - Authorization policy.
//! - Provenance generation or verification.
//! - Package archive parsing or extraction.
//! - Artifact serialization or storage.
//! - Dependency resolution.
//! - Compiler or backend execution.
//! - Hardware discovery or resource allocation.
//!
//! ## Canonical authorities
//!
//! Hash algorithms, digest representation and streaming hashing:
//! `crate::stdlib::security::hashing`
//!
//! Cryptographic provider contracts:
//! `crate::stdlib::security::cryptography`
//!
//! Package digest descriptors:
//! `crate::toolchain::packages::model::ContentDigest`
//!
//! Package verification orchestration:
//! `crate::toolchain::packages::verification`
//!
//! Artifact identity and reproducibility:
//! `crate::backend` and `crate::compiler::artifacts`
//!
//! Signatures:
//! `crate::toolchain::security::signatures`
//!
//! Trust:
//! `crate::toolchain::security::trust`
//!
//! Provenance:
//! `crate::toolchain::security::provenance`
//!
//! Authorization:
//! `crate::toolchain::security::policy`
//!
//! ## Security invariants
//!
//! - A digest match establishes byte equality under the selected algorithm;
//!   it does not establish authenticity, trust or authorization.
//! - An absent expected digest cannot be reported as verified.
//! - An algorithm mismatch is an invalid verification request.
//! - An unsupported algorithm is never silently replaced.
//! - A provider failure never produces a successful report.
//! - A byte-budget exhaustion never produces a successful report.
//! - Input and provider errors are propagated as failures.
//! - The complete input is never accumulated in memory by this module.
//! - Diagnostic output never includes the content being verified.
//!
//! ## Scalability
//!
//! Input is consumed incrementally. There is no universal content-size,
//! artifact-count, package-count, dependency-count, provider-count, target,
//! device, processor, accelerator, qubit or computing-domain ceiling.
//!
//! `max_bytes` is an optional operational budget for an individual
//! verification operation. `None` means this module imposes no such budget.
//! Host address-space, I/O, provider and physical-resource constraints still
//! apply. Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Given identical input bytes, algorithm, hashing implementation and
//! expected digest, the verification result is deterministic.
//!
//! The caller is responsible for selecting an algorithm and provider whose
//! security properties satisfy the applicable requirements and policy.

use std::io::{self, Read};

use crate::stdlib::security::cryptography::CryptoResult;
use crate::stdlib::security::hashing::{
    HashAlgorithm,
    HashDigest,
    Hasher,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

/// Stable component identifier.
pub const COMPONENT: &str = "toolchain.security.integrity";

/// Stable operation identifier for content hashing.
pub const HASH_OPERATION: &str = "toolchain.integrity.hash";

/// Stable operation identifier for content verification.
pub const VERIFY_OPERATION: &str = "toolchain.integrity.verify";

// ============================================================================
// HASH PROVIDER ADAPTER
// ============================================================================

/// Adapter contract for a canonical hashing implementation.
///
/// Implementations must delegate to `crate::stdlib::security::hashing` or
/// another explicitly integrated implementation of the canonical Zamani
/// hashing contracts.
///
/// This trait is an integration seam, not a second hashing API or provider
/// registry. It allows a toolchain consumer to supply an implementation
/// capable of algorithms beyond the built-in algorithms currently provided
/// by `stdlib::security::hashing::Hasher`.
///
/// Implementations must:
///
/// - Consume input incrementally.
/// - Return the canonical `HashDigest` representation.
/// - Preserve the requested algorithm identity.
/// - Return an error for unsupported algorithms.
/// - Never silently substitute another algorithm.
/// - Propagate read and provider failures.
/// - Avoid logging content or sensitive input.
///
/// A provider's declared capabilities are not proof of cryptographic
/// security. Algorithm and provider suitability remain policy decisions.
pub trait HashingProvider: Send + Sync {
    /// Hash an input stream using the requested canonical algorithm.
    ///
    /// # Errors
    ///
    /// Returns the canonical cryptographic error if the algorithm is
    /// unsupported, the input cannot be read, or the provider fails.
    fn hash_reader(
        &self,
        algorithm: &HashAlgorithm,
        input: &mut dyn Read,
    ) -> CryptoResult<HashDigest>;
}

/// Adapter for the built-in canonical Zamani hashing service.
impl HashingProvider for Hasher {
    fn hash_reader(
        &self,
        algorithm: &HashAlgorithm,
        input: &mut dyn Read,
    ) -> CryptoResult<HashDigest> {
        Hasher::hash_reader(self, algorithm, input)
    }
}

// ============================================================================
// OPERATIONAL BUDGET
// ============================================================================

/// Optional byte-processing budget for one integrity operation.
///
/// This is an operational safeguard, not a package-format, language,
/// hardware or algorithmic capacity limit.
///
/// `None` means no application-level byte limit is imposed by this module.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IntegrityBudget {
    max_bytes: Option<u128>,
}

impl IntegrityBudget {
    /// Construct a budget with an optional maximum number of input bytes.
    #[must_use]
    pub const fn new(max_bytes: Option<u128>) -> Self {
        Self { max_bytes }
    }

    /// Construct a budget without an application-level byte limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_bytes: None }
    }

    /// Return the configured maximum number of input bytes.
    #[must_use]
    pub const fn max_bytes(self) -> Option<u128> {
        self.max_bytes
    }
}

// ============================================================================
// RESULT TYPES
// ============================================================================

/// Outcome of comparing actual content with an expected digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntegrityStatus {
    /// The actual digest matches the expected digest.
    Verified,

    /// The actual digest does not match the expected digest.
    Mismatch,
}

/// Immutable result of a completed integrity operation.
///
/// A report with [`IntegrityStatus::Mismatch`] is not a successful
/// verification. Consumers must check [`Self::is_verified`] before
/// accepting content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityReport {
    algorithm: HashAlgorithm,
    expected: HashDigest,
    actual: HashDigest,
    bytes_processed: u128,
    status: IntegrityStatus,
}

impl IntegrityReport {
    /// Return the algorithm used for the operation.
    #[must_use]
    pub fn algorithm(&self) -> &HashAlgorithm {
        &self.algorithm
    }

    /// Return the expected digest.
    #[must_use]
    pub fn expected(&self) -> &HashDigest {
        &self.expected
    }

    /// Return the digest computed from the actual input.
    #[must_use]
    pub fn actual(&self) -> &HashDigest {
        &self.actual
    }

    /// Return the number of input bytes consumed.
    #[must_use]
    pub const fn bytes_processed(&self) -> u128 {
        self.bytes_processed
    }

    /// Return the integrity outcome.
    #[must_use]
    pub const fn status(&self) -> IntegrityStatus {
        self.status
    }

    /// Return whether the expected and actual digests match.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        matches!(self.status, IntegrityStatus::Verified)
    }

    /// Require successful integrity verification.
    ///
    /// This is useful at security-sensitive acceptance boundaries.
    ///
    /// # Errors
    ///
    /// Returns an integrity-verification error when the digests differ.
    pub fn require_verified(&self) -> ToolchainResult<()> {
        if self.is_verified() {
            Ok(())
        } else {
            Err(integrity_error(
                "content digest does not match the expected digest",
            ))
        }
    }
}

/// Immutable result of calculating a digest without an expected value.
///
/// A calculated digest identifies the bytes processed. It does not prove
/// that the bytes are authentic, trusted, authorized or semantically valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputedIntegrity {
    digest: HashDigest,
    bytes_processed: u128,
}

impl ComputedIntegrity {
    /// Return the computed digest.
    #[must_use]
    pub fn digest(&self) -> &HashDigest {
        &self.digest
    }

    /// Return the number of input bytes consumed.
    #[must_use]
    pub const fn bytes_processed(&self) -> u128 {
        self.bytes_processed
    }
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn integrity_error(message: impl Into<String>) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::IntegrityVerificationFailed,
        message,
    )
    .with_operation(VERIFY_OPERATION)
    .with_context(COMPONENT)
}

fn invalid_input(message: impl Into<String>) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(VERIFY_OPERATION)
    .with_context(COMPONENT)
}

fn resource_exhausted(message: impl Into<String>) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ResourceExhausted,
        message,
    )
    .with_operation(HASH_OPERATION)
    .with_context(COMPONENT)
}

fn hashing_error(
    error: impl std::error::Error + Send + Sync + 'static,
    operation: &'static str,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        "canonical hashing operation failed",
    )
    .with_operation(operation)
    .with_context(COMPONENT)
    .with_source(error)
}

// ============================================================================
// BUDGET-ENFORCING READER
// ============================================================================

/// Counts bytes read and optionally prevents a read beyond the configured
/// operation budget.
///
/// The reader probes one byte after the budget is consumed. This distinguishes
/// an input whose length is exactly the budget from an input that exceeds it.
struct BudgetReader<R> {
    inner: R,
    max_bytes: Option<u128>,
    bytes_read: u128,
    exceeded: bool,
}

impl<R> BudgetReader<R> {
    fn new(inner: R, budget: IntegrityBudget) -> Self {
        Self {
            inner,
            max_bytes: budget.max_bytes(),
            bytes_read: 0,
            exceeded: false,
        }
    }

    fn bytes_read(&self) -> u128 {
        self.bytes_read
    }

    fn exceeded(&self) -> bool {
        self.exceeded
    }

    fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for BudgetReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let Some(max_bytes) = self.max_bytes else {
            let count = self.inner.read(buffer)?;
            self.bytes_read = self
                .bytes_read
                .checked_add(count as u128)
                .ok_or_else(|| {
                    io::Error::other("integrity byte counter overflow")
                })?;
            return Ok(count);
        };

        let remaining = max_bytes.saturating_sub(self.bytes_read);

        if remaining == 0 {
            let mut probe = [0_u8; 1];

            return match self.inner.read(&mut probe)? {
                0 => Ok(0),
                _ => {
                    self.exceeded = true;
                    Err(io::Error::other(
                        "integrity operation byte budget exceeded",
                    ))
                }
            };
        }

        let allowed = remaining.min(buffer.len() as u128) as usize;
        let count = self.inner.read(&mut buffer[..allowed])?;

        self.bytes_read = self
            .bytes_read
            .checked_add(count as u128)
            .ok_or_else(|| {
                io::Error::other("integrity byte counter overflow")
            })?;

        Ok(count)
    }
}

// ============================================================================
// INTEGRITY VERIFIER
// ============================================================================

/// Stateless content-integrity verification service.
///
/// The default methods use the canonical built-in `Hasher`.
/// The `_with` methods accept an implementation of [`HashingProvider`]
/// without introducing a second provider registry.
#[derive(Debug, Clone, Copy, Default)]
pub struct IntegrityVerifier;

impl IntegrityVerifier {
    /// Construct the integrity verifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Calculate a digest using the built-in canonical hashing service.
    ///
    /// Input is consumed incrementally.
    ///
    /// # Errors
    ///
    /// Returns an error if hashing fails, input cannot be read, the
    /// configured byte budget is exceeded, or byte accounting overflows.
    pub fn compute<R: Read>(
        &self,
        algorithm: &HashAlgorithm,
        input: R,
        budget: IntegrityBudget,
    ) -> ToolchainResult<ComputedIntegrity> {
        self.compute_with(
            &Hasher::new(),
            algorithm,
            input,
            budget,
        )
    }

    /// Calculate a digest using an explicitly supplied hashing provider.
    ///
    /// This is the extension point for registered or externally integrated
    /// algorithms. Provider discovery and selection remain outside this
    /// module.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider fails, input cannot be read, the
    /// configured byte budget is exceeded, or byte accounting overflows.
    pub fn compute_with<R, H>(
        &self,
        provider: &H,
        algorithm: &HashAlgorithm,
        input: R,
        budget: IntegrityBudget,
    ) -> ToolchainResult<ComputedIntegrity>
    where
        R: Read,
        H: HashingProvider + ?Sized,
    {
        let mut reader = BudgetReader::new(input, budget);

        let digest = match provider.hash_reader(algorithm, &mut reader) {
            Ok(digest) => digest,
            Err(error) if reader.exceeded() => {
                return Err(resource_exhausted(format!(
                    "integrity operation exceeded its configured byte budget \
                     of {} bytes",
                    budget.max_bytes().unwrap_or_default()
                )));
            }
            Err(error) => {
                return Err(hashing_error(error, HASH_OPERATION));
            }
        };

        if digest.algorithm() != algorithm {
            return Err(ToolchainError::new(
                ToolchainErrorKind::ProviderFailed,
                "hashing provider returned a digest for a different algorithm",
            )
            .with_operation(HASH_OPERATION)
            .with_context(COMPONENT));
        }

        Ok(ComputedIntegrity {
            digest,
            bytes_processed: reader.bytes_read(),
        })
    }

    /// Verify an input stream against an expected digest using the built-in
    /// canonical hashing service.
    ///
    /// A digest mismatch is returned as a report with
    /// [`IntegrityStatus::Mismatch`]. It is never reported as verified.
    ///
    /// # Errors
    ///
    /// Returns an error for an algorithm mismatch, provider failure,
    /// unreadable input, byte-budget exhaustion or accounting overflow.
    pub fn verify<R: Read>(
        &self,
        algorithm: &HashAlgorithm,
        input: R,
        expected: &HashDigest,
        budget: IntegrityBudget,
    ) -> ToolchainResult<IntegrityReport> {
        self.verify_with(
            &Hasher::new(),
            algorithm,
            input,
            expected,
            budget,
        )
    }

    /// Verify an input stream using an explicitly supplied hashing provider.
    ///
    /// # Errors
    ///
    /// Returns an error for an algorithm mismatch, provider failure,
    /// unreadable input, byte-budget exhaustion or accounting overflow.
    pub fn verify_with<R, H>(
        &self,
        provider: &H,
        algorithm: &HashAlgorithm,
        input: R,
        expected: &HashDigest,
        budget: IntegrityBudget,
    ) -> ToolchainResult<IntegrityReport>
    where
        R: Read,
        H: HashingProvider + ?Sized,
    {
        if expected.algorithm() != algorithm {
            return Err(invalid_input(
                "expected digest algorithm does not match the requested algorithm",
            ));
        }

        let computed = self.compute_with(
            provider,
            algorithm,
            input,
            budget,
        )?;

        let status = if computed.digest().verify(expected) {
            IntegrityStatus::Verified
        } else {
            IntegrityStatus::Mismatch
        };

        Ok(IntegrityReport {
            algorithm: algorithm.clone(),
            expected: expected.clone(),
            actual: computed.digest,
            bytes_processed: computed.bytes_processed,
            status,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    fn sha256(bytes: &[u8]) -> HashDigest {
        Hasher::new()
            .hash(&HashAlgorithm::sha256(), bytes)
            .expect("test digest should be calculated")
    }

    #[test]
    fn verifies_matching_content() {
        let expected = sha256(b"Zamani integrity");
        let report = IntegrityVerifier::new()
            .verify(
                &HashAlgorithm::sha256(),
                Cursor::new(b"Zamani integrity"),
                &expected,
                IntegrityBudget::unlimited(),
            )
            .expect("verification should complete");

        assert_eq!(report.status(), IntegrityStatus::Verified);
        assert!(report.is_verified());
        assert_eq!(report.bytes_processed(), 16);
        report.require_verified().expect("digest should match");
    }

    #[test]
    fn reports_digest_mismatch_without_claiming_success() {
        let expected = sha256(b"original");
        let report = IntegrityVerifier::new()
            .verify(
                &HashAlgorithm::sha256(),
                Cursor::new(b"modified"),
                &expected,
                IntegrityBudget::unlimited(),
            )
            .expect("comparison should complete");

        assert_eq!(report.status(), IntegrityStatus::Mismatch);
        assert!(!report.is_verified());
        assert!(report.require_verified().is_err());
    }

    #[test]
    fn rejects_expected_digest_for_another_algorithm() {
        let expected = Hasher::new()
            .hash(&HashAlgorithm::sha512(), b"content")
            .expect("test digest should be calculated");

        let result = IntegrityVerifier::new().verify(
            &HashAlgorithm::sha256(),
            Cursor::new(b"content"),
            &expected,
            IntegrityBudget::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_input_exactly_at_budget() {
        let expected = sha256(b"12345");
        let report = IntegrityVerifier::new()
            .verify(
                &HashAlgorithm::sha256(),
                Cursor::new(b"12345"),
                &expected,
                IntegrityBudget::new(Some(5)),
            )
            .expect("input at the budget should be accepted");

        assert!(report.is_verified());
        assert_eq!(report.bytes_processed(), 5);
    }

    #[test]
    fn rejects_input_larger_than_budget() {
        let expected = sha256(b"123456");
        let result = IntegrityVerifier::new().verify(
            &HashAlgorithm::sha256(),
            Cursor::new(b"123456"),
            &expected,
            IntegrityBudget::new(Some(5)),
        );

        let error = result.expect_err("budget must be enforced");
        assert_eq!(error.kind(), ToolchainErrorKind::ResourceExhausted);
    }

    #[test]
    fn zero_byte_budget_accepts_only_empty_input() {
        let expected = sha256(b"");
        let report = IntegrityVerifier::new()
            .verify(
                &HashAlgorithm::sha256(),
                Cursor::new(b""),
                &expected,
                IntegrityBudget::new(Some(0)),
            )
            .expect("empty input should fit a zero-byte budget");

        assert!(report.is_verified());

        let nonempty = IntegrityVerifier::new().verify(
            &HashAlgorithm::sha256(),
            Cursor::new(b"x"),
            &sha256(b"x"),
            IntegrityBudget::new(Some(0)),
        );

        assert!(nonempty.is_err());
    }

    struct FailingReader;

    impl Read for FailingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("simulated input failure"))
        }
    }

    #[test]
    fn propagates_input_failure() {
        let result = IntegrityVerifier::new().compute(
            &HashAlgorithm::sha256(),
            FailingReader,
            IntegrityBudget::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn computes_empty_input_digest() {
        let computed = IntegrityVerifier::new()
            .compute(
                &HashAlgorithm::sha256(),
                Cursor::new(b""),
                IntegrityBudget::unlimited(),
            )
            .expect("empty input can be hashed");

        assert_eq!(computed.bytes_processed(), 0);
        assert_eq!(
            computed.digest().to_hex(),
            "e3b0c44298fc1c149afbf4c8996fb924\
             27ae41e4649b934ca495991b7852b855"
        );
    }
}
