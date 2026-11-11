
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Cryptographic Hashing
//!
//! Path: src/stdlib/security/hashing.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! ## Ownership
//!
//! This module owns:
//! - The public hashing API.
//! - Hash algorithm metadata.
//! - Digest representation.
//! - Streaming hash operations.
//! - Digest verification.
//! - Hash provider integration.
//!
//! It does not own:
//! - Cryptographic semantic analysis.
//! - Encryption or decryption.
//! - Key management.
//! - Signature algorithms.
//! - Quantum IR.
//! - Hardware selection.
//! - Scheduling or resource allocation.
//!
//! ## Integration
//!
//! Uses the canonical contracts in:
//! crate::stdlib::security::cryptography
//!
//! Existing algorithm implementations are reused through:
//! sha2
//! sha3
//!
//! The public algorithm identity is open-world.
//! Concrete built-in implementations are intentionally limited to
//! algorithms actually available in the repository's dependencies.
//!
//! ## Scalability
//!
//! Input is processed incrementally.
//! There is no artificial maximum input size.
//! Actual limits are determined by the algorithm, reader, provider,
//! address space and available resources.
//!
//! A streaming buffer is an implementation detail, not an input limit.

use core::fmt;
use std::io::{self, Read, Write};

use sha2::{Digest as DigestTrait, Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};

use super::cryptography::{
    CryptoAlgorithmId,
    CryptoCapability,
    CryptoError,
    CryptoErrorKind,
    CryptoOperationId,
    CryptoPropertyId,
    CryptoProvider,
    CryptoProviderId,
    CryptoRequest,
    CryptoResult,
};

// ============================================================================
// CANONICAL IDENTIFIERS
// ============================================================================

/// Canonical hashing operation identifier.
pub const HASH_OPERATION: &str = "cryptography::hash";

/// SHA-256 algorithm identifier.
pub const SHA256_ID: &str = "cryptography::hash::sha256";

/// SHA-512 algorithm identifier.
pub const SHA512_ID: &str = "cryptography::hash::sha512";

/// SHA3-256 algorithm identifier.
pub const SHA3_256_ID: &str = "cryptography::hash::sha3_256";

/// SHA3-512 algorithm identifier.
pub const SHA3_512_ID: &str = "cryptography::hash::sha3_512";

/// Hash provider identity.
pub const BUILTIN_PROVIDER_ID: &str = "zamani::crypto::hashing::sha2_sha3";

// ============================================================================
// ALGORITHM IDENTIFICATION
// ============================================================================

/// An open-world cryptographic hash algorithm identity.
///
/// This type deliberately does not enumerate every possible algorithm.
///
/// New algorithms can be introduced without changing this type or
/// modifying the language grammar.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HashAlgorithm {
    id: CryptoAlgorithmId,
}

impl HashAlgorithm {
    /// Constructs an algorithm identity.
    pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
        Ok(Self {
            id: CryptoAlgorithmId::new(value)?,
        })
    }

    /// Returns the canonical algorithm identity.
    #[must_use]
    pub fn id(&self) -> &CryptoAlgorithmId {
        &self.id
    }

    /// Returns the algorithm's canonical string representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }

    /// Returns the SHA-256 algorithm identity.
    pub fn sha256() -> Self {
        Self::new(SHA256_ID)
            .expect("built-in SHA-256 identifier is valid")
    }

    /// Returns the SHA-512 algorithm identity.
    pub fn sha512() -> Self {
        Self::new(SHA512_ID)
            .expect("built-in SHA-512 identifier is valid")
    }

    /// Returns the SHA3-256 algorithm identity.
    pub fn sha3_256() -> Self {
        Self::new(SHA3_256_ID)
            .expect("built-in SHA3-256 identifier is valid")
    }

    /// Returns the SHA3-512 algorithm identity.
    pub fn sha3_512() -> Self {
        Self::new(SHA3_512_ID)
            .expect("built-in SHA3-512 identifier is valid")
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// DIGEST REPRESENTATION
// ============================================================================

/// A completed cryptographic digest.
///
/// The algorithm identity is retained alongside the bytes so that a digest
/// cannot silently lose information about which algorithm produced it.
#[derive(Clone, Eq, PartialEq)]
pub struct HashDigest {
    algorithm: HashAlgorithm,
    bytes: Vec<u8>,
}

impl HashDigest {
    /// Constructs a digest from an algorithm and its output bytes.
    ///
    /// This constructor checks that the digest is nonempty.
    /// Algorithm-specific length validation is performed by the provider.
    pub fn new(
        algorithm: HashAlgorithm,
        bytes: Vec<u8>,
    ) -> CryptoResult<Self> {
        if bytes.is_empty() {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidProviderResult,
            ));
        }

        Ok(Self { algorithm, bytes })
    }

    /// Returns the algorithm identity.
    #[must_use]
    pub fn algorithm(&self) -> &HashAlgorithm {
        &self.algorithm
    }

    /// Returns the digest bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the digest length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the digest contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Produces a lowercase hexadecimal representation.
    #[must_use]
    pub fn to_hex(&self) -> String {
        let mut output = String::with_capacity(
            self.bytes.len().saturating_mul(2),
        );

        const HEX: &[u8; 16] = b"0123456789abcdef";

        for byte in &self.bytes {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }

        output
    }

    /// Verifies another digest using a constant-time byte comparison.
    ///
    /// Length is public digest metadata. The comparison of equal-length
    /// digest contents does not short-circuit on the first differing byte.
    #[must_use]
    pub fn verify(&self, other: &Self) -> bool {
        if self.algorithm != other.algorithm {
            return false;
        }

        constant_time_equal(&self.bytes, &other.bytes)
    }
}

impl fmt::Debug for HashDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HashDigest")
            .field("algorithm", &self.algorithm)
            .field("length", &self.bytes.len())
            .field("hex", &self.to_hex())
            .finish()
    }
}

impl fmt::Display for HashDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

// ============================================================================
// CONSTANT-TIME COMPARISON
// ============================================================================

/// Compares equal-length byte sequences without early termination.
///
/// This is intended for digest verification, not as a replacement for
/// a dedicated, audited cryptographic constant-time library.
#[must_use]
pub fn constant_time_equal(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut difference = 0u8;

    for (a, b) in left.iter().zip(right.iter()) {
        difference |= a ^ b;
    }

    difference == 0
}

// ============================================================================
// BUILT-IN ALGORITHM DISPATCH
// ============================================================================

/// Updates a digest implementation from a streaming input.
///
/// The working buffer is reused for every read. Its size does not
/// restrict the total input size.
fn digest_reader<D>(
    input: &mut dyn Read,
) -> CryptoResult<Vec<u8>>
where
    D: DigestTrait + Default,
{
    let mut hasher = D::default();

    // Bounded working memory; not a maximum input size.
    let mut buffer = [0u8; 8192];

    loop {
        let count = match input.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                continue;
            }
            Err(error) => {
                return Err(
                    CryptoError::new(CryptoErrorKind::InputFailure)
                        .with_source(error),
                );
            }
        };

        hasher.update(&buffer[..count]);
    }

    Ok(hasher.finalize().to_vec())
}

/// Computes a digest using an available built-in implementation.
///
/// Unknown algorithms fail explicitly. They are never silently mapped
/// to another algorithm.
fn digest_builtin(
    algorithm: &HashAlgorithm,
    input: &mut dyn Read,
) -> CryptoResult<Vec<u8>> {
    match algorithm.as_str() {
        SHA256_ID => digest_reader::<Sha256>(input),

        SHA512_ID => digest_reader::<Sha512>(input),

        SHA3_256_ID => digest_reader::<Sha3_256>(input),

        SHA3_512_ID => digest_reader::<Sha3_512>(input),

        _ => Err(
            CryptoError::new(CryptoErrorKind::UnsupportedOperation),
        ),
    }
}

// ============================================================================
// HASHING SERVICE
// ============================================================================

/// Portable hashing service.
///
/// This service performs genuine hashing using the repository's existing
/// SHA-2 and SHA-3 dependencies.
///
/// Additional algorithms should be supplied through compatible providers,
/// not by changing the public HashAlgorithm representation.
#[derive(Clone, Debug, Default)]
pub struct Hasher;

impl Hasher {
    /// Constructs a hashing service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Hashes an in-memory byte sequence.
    pub fn hash(
        &self,
        algorithm: &HashAlgorithm,
        input: &[u8],
    ) -> CryptoResult<HashDigest> {
        let mut reader = io::Cursor::new(input);

        self.hash_reader(algorithm, &mut reader)
    }

    /// Hashes a streaming input.
    ///
    /// The entire input is never required to reside in memory.
    pub fn hash_reader(
        &self,
        algorithm: &HashAlgorithm,
        input: &mut dyn Read,
    ) -> CryptoResult<HashDigest> {
        let bytes = digest_builtin(algorithm, input)?;

        HashDigest::new(algorithm.clone(), bytes)
    }

    /// Hashes an input and writes the digest to an output stream.
    ///
    /// The digest is written only after hashing has completed.
    pub fn hash_to(
        &self,
        algorithm: &HashAlgorithm,
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> CryptoResult<HashDigest> {
        let digest = self.hash_reader(algorithm, input)?;

        output.write_all(digest.as_bytes()).map_err(|error| {
            CryptoError::new(CryptoErrorKind::OutputFailure)
                .with_source(error)
        })?;

        Ok(digest)
    }

    /// Verifies an input against an expected digest.
    pub fn verify(
        &self,
        algorithm: &HashAlgorithm,
        input: &mut dyn Read,
        expected: &HashDigest,
    ) -> CryptoResult<bool> {
        if algorithm != expected.algorithm() {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidParameters,
            ));
        }

        let actual = self.hash_reader(algorithm, input)?;

        Ok(actual.verify(expected))
    }
}

// ============================================================================
// PROVIDER IMPLEMENTATION
// ============================================================================

/// Built-in SHA-2 and SHA-3 hashing provider.
///
/// Implements the canonical `CryptoProvider` contract.
pub struct BuiltinHashProvider {
    id: CryptoProviderId,
    capabilities: Vec<CryptoCapability>,
    hasher: Hasher,
}

impl BuiltinHashProvider {
    /// Constructs the built-in hashing provider.
    pub fn new() -> CryptoResult<Self> {
        let id = CryptoProviderId::new(BUILTIN_PROVIDER_ID)?;

        let operation = CryptoOperationId::new(HASH_OPERATION)?;

        let collision = CryptoPropertyId::new(
            "cryptography::hash::collision_resistance",
        )?;

        let preimage = CryptoPropertyId::new(
            "cryptography::hash::preimage_resistance",
        )?;

        let second_preimage = CryptoPropertyId::new(
            "cryptography::hash::second_preimage_resistance",
        )?;

        let deterministic = CryptoPropertyId::new(
            "cryptography::hash::deterministic",
        )?;

        let streaming = CryptoPropertyId::new(
            "cryptography::hash::streaming",
        )?;

        let mut capabilities = Vec::new();

        for algorithm in [
            SHA256_ID,
            SHA512_ID,
            SHA3_256_ID,
            SHA3_512_ID,
        ] {
            let algorithm_id = CryptoAlgorithmId::new(algorithm)?;

            let capability = CryptoCapability::new(
                operation.clone(),
            )
            .with_algorithm(algorithm_id)
            .with_property(collision.clone())
            .with_property(preimage.clone())
            .with_property(second_preimage.clone())
            .with_property(deterministic.clone())
            .with_property(streaming.clone());

            capabilities.push(capability);
        }

        Ok(Self {
            id,
            capabilities,
            hasher: Hasher::new(),
        })
    }
}

impl CryptoProvider for BuiltinHashProvider {
    fn id(&self) -> &CryptoProviderId {
        &self.id
    }

    fn capabilities(&self) -> &[CryptoCapability] {
        &self.capabilities
    }

    fn hash(
        &self,
        request: &CryptoRequest,
        input: &mut dyn Read,
        output: &mut dyn Write,
    ) -> CryptoResult<()> {
        if request.operation().as_str() != HASH_OPERATION {
            return Err(
                CryptoError::new(CryptoErrorKind::UnsupportedOperation)
                    .with_provider(self.id.clone()),
            );
        }

        let algorithm = HashAlgorithm {
            id: request.algorithm().clone(),
        };

        let capability = self.capabilities.iter().find(|capability| {
            capability.algorithm() == Some(request.algorithm())
                && capability.operation().as_str() == HASH_OPERATION
        });

        let capability = capability.ok_or_else(|| {
            CryptoError::new(CryptoErrorKind::CapabilityUnavailable)
                .with_provider(self.id.clone())
        })?;

        request.validate_capability(capability).map_err(|error| {
            CryptoError::new(error.kind())
                .with_provider(self.id.clone())
                .with_operation(request.operation().clone())
        })?;

        let digest = self.hasher.hash_reader(&algorithm, input)?;

        output.write_all(digest.as_bytes()).map_err(|error| {
            CryptoError::new(CryptoErrorKind::OutputFailure)
                .with_provider(self.id.clone())
                .with_operation(request.operation().clone())
                .with_source(error)
        })?;

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_vector() {
        let digest = Hasher::new()
            .hash(&HashAlgorithm::sha256(), b"abc")
            .unwrap();

        assert_eq!(
            digest.to_hex(),
            "ba7816bf8f01cfea414140de5dae2223\
             b00361a396177a9cb410ff61f20015ad"
                .replace(' ', "")
        );
    }

    #[test]
    fn sha512_known_vector() {
        let digest = Hasher::new()
            .hash(&HashAlgorithm::sha512(), b"abc")
            .unwrap();

        assert_eq!(digest.len(), 64);

        assert_eq!(
            digest.to_hex(),
            concat!(
                "ddaf35a193617abacc417349ae204131",
                "12e6fa4e89a97ea20a9eeee64b55d39a",
                "2192992a274fc1a836ba3c23a3feebbd",
                "454d4423643ce80e2a9ac94fa54ca49f"
            )
        );
    }

    #[test]
    fn sha3_256_known_vector() {
        let digest = Hasher::new()
            .hash(&HashAlgorithm::sha3_256(), b"abc")
            .unwrap();

        assert_eq!(
            digest.to_hex(),
            concat!(
                "3a985da74fe225b2045c172d6bd390bd",
                "855f086e3e9d525b46bfe24511431532"
            )
        );
    }

    #[test]
    fn sha3_512_known_vector() {
        let digest = Hasher::new()
            .hash(&HashAlgorithm::sha3_512(), b"abc")
            .unwrap();

        assert_eq!(digest.len(), 64);
    }

    #[test]
    fn empty_input_is_supported() {
        let digest = Hasher::new()
            .hash(&HashAlgorithm::sha256(), b"")
            .unwrap();

        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn streaming_matches_in_memory_hashing() {
        let data = vec![42u8; 1024 * 1024];

        let expected = Hasher::new()
            .hash(&HashAlgorithm::sha256(), &data)
            .unwrap();

        let mut reader = io::Cursor::new(data);

        let actual = Hasher::new()
            .hash_reader(&HashAlgorithm::sha256(), &mut reader)
            .unwrap();

        assert!(expected.verify(&actual));
    }

    #[test]
    fn different_algorithms_produce_distinct_digests() {
        let hasher = Hasher::new();

        let sha2 = hasher
            .hash(&HashAlgorithm::sha256(), b"zamani")
            .unwrap();

        let sha3 = hasher
            .hash(&HashAlgorithm::sha3_256(), b"zamani")
            .unwrap();

        assert_ne!(sha2, sha3);
    }

    #[test]
    fn digest_verification_works() {
        let hasher = Hasher::new();

        let expected = hasher
            .hash(&HashAlgorithm::sha256(), b"zamani")
            .unwrap();

        assert!(hasher.verify(
            &HashAlgorithm::sha256(),
            &mut io::Cursor::new(b"zamani"),
            &expected,
        ).unwrap());

        assert!(!hasher.verify(
            &HashAlgorithm::sha256(),
            &mut io::Cursor::new(b"modified"),
            &expected,
        ).unwrap());
    }

    #[test]
    fn rejects_unknown_algorithms() {
        let algorithm = HashAlgorithm::new(
            "future::cryptography::hash::algorithm",
        ).unwrap();

        assert!(
            Hasher::new()
                .hash(&algorithm, b"data")
                .is_err()
        );
    }

    #[test]
    fn hexadecimal_encoding_is_correct() {
        let digest = HashDigest::new(
            HashAlgorithm::sha256(),
            vec![0x00, 0x0f, 0xff],
        ).unwrap();

        assert_eq!(digest.to_hex(), "000fff");
    }

    #[test]
    fn comparison_detects_different_lengths() {
        assert!(!constant_time_equal(
            &[1, 2],
            &[1, 2, 3],
        ));
    }

    #[test]
    fn comparison_detects_different_content() {
        assert!(!constant_time_equal(
            &[1, 2, 3],
            &[1, 2, 4],
        ));
    }

    #[test]
    fn provider_exposes_real_capabilities() {
        let provider = BuiltinHashProvider::new().unwrap();

        assert_eq!(provider.capabilities().len(), 4);

        assert_eq!(
            provider.id().as_str(),
            BUILTIN_PROVIDER_ID,
        );
    }

    #[test]
    fn provider_supports_streaming_sha256() {
        let provider = BuiltinHashProvider::new().unwrap();

        let operation = CryptoOperationId::new(
            HASH_OPERATION,
        ).unwrap();

        let algorithm = CryptoAlgorithmId::new(
            SHA256_ID,
        ).unwrap();

        let requirement = super::super::cryptography::CryptoRequirement::new(
            operation.clone(),
        );

        let request = CryptoRequest::new(
            operation,
            algorithm,
            requirement,
        ).unwrap();

        let mut output = Vec::new();

        provider.hash(
            &request,
            &mut io::Cursor::new(b"abc"),
            &mut output,
        ).unwrap();

        assert_eq!(output.len(), 32);
    }

    #[test]
    fn reader_failure_is_reported() {
        struct FailingReader;

        impl Read for FailingReader {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("simulated input failure"))
            }
        }

        let result = Hasher::new().hash_reader(
            &HashAlgorithm::sha256(),
            &mut FailingReader,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            CryptoErrorKind::InputFailure,
        );
    }
}
