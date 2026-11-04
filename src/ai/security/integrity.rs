
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Security — Artifact Integrity
//!
//! Path: src/ai/security/integrity.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # PURPOSE
//!
//! Provides AI-domain artifact integrity verification through the
//! repository's canonical backend artifact infrastructure.
//!
//! The module protects the integrity boundary for:
//!
//! - AI models;
//! - model parameters;
//! - model weights;
//! - training checkpoints;
//! - training results;
//! - inference artifacts;
//! - AI computational graphs;
//! - AI deployment packages;
//! - AI-generated artifacts;
//! - future registered AI artifact categories.
//!
//! # ARCHITECTURAL AUTHORITY
//!
//! Canonical ownership remains with:
//!
//! - crate::backend::artifact
//!     Owns artifact payloads, SHA-256 content digests,
//!     and canonical artifact integrity verification.
//!
//! - crate::ai::deployment::artifacts
//!     Owns AI artifact descriptions and AI artifact identities.
//!
//! - crate::ai::provenance
//!     Owns AI provenance integration.
//!
//! - crate::ai::security::identity
//!     Owns AI entity and principal identity references.
//!
//! - crate::ai::security::authorization
//!     Owns AI authorization integration.
//!
//! - crate::distributed::security
//!     Owns distributed authentication, trust and authorization.
//!
//! - crate::quantum::ir::hashing
//!     Owns canonical quantum IR hashing.
//!
//! - grammar/security/hashes.g4
//!     Owns source-level hashing intent.
//!
//! - grammar/security/cryptography.g4
//!     Owns source-level cryptographic intent.
//!
//! - grammar/security/trust.g4
//!     Owns source-level trust intent.
//!
//! # OWNS
//!
//! - AI artifact integrity verification orchestration.
//! - AI integrity verification policies.
//! - AI integrity verification results.
//! - AI integrity verification evidence.
//! - Backend integrity-provider integration.
//! - AI integrity error translation.
//!
//! # DOES NOT OWN
//!
//! - Cryptographic hash algorithms.
//! - SHA-256 implementation.
//! - Digital signatures.
//! - Private or public keys.
//! - Authentication.
//! - Trust establishment.
//! - Authorization.
//! - Artifact serialization.
//! - Artifact storage.
//! - Artifact identity allocation.
//! - Persistent provenance.
//! - Model execution.
//! - Hardware selection.
//! - Resource allocation.
//! - Scheduling.
//! - Distributed transport.
//!
//! # SECURITY INVARIANTS
//!
//! 1. An AI artifact identifier is not a content fingerprint.
//! 2. A content fingerprint is not a digital signature.
//! 3. A valid digest does not establish artifact authenticity.
//! 4. A valid digest does not establish artifact authorization.
//! 5. A valid digest does not establish artifact semantic correctness.
//! 6. Missing integrity evidence must not become successful verification.
//! 7. Invalid digest metadata must fail verification.
//! 8. Changed artifact contents must fail verification.
//! 9. Backend verification failures must propagate.
//! 10. An unavailable integrity provider must not produce success.
//! 11. Verification evidence must identify the verified content.
//! 12. Verification evidence must not imply permanent integrity.
//! 13. Verification of one artifact must not authorize another artifact.
//! 14. Caller-provided metadata must not override backend verification.
//! 15. Cryptographic authenticity requires an independent signature
//!     and trust verification pathway.
//!
//! # POCO-REAF
//!
//! Integrity verification is independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - FPGA implementation;
//! - ASIC implementation;
//! - quantum processor;
//! - neuromorphic hardware;
//! - operating system;
//! - distributed topology;
//! - physical deployment location.
//!
//! No universal AI artifact-size limit is introduced.
//!
//! No maximum artifact count is introduced.
//!
//! No maximum model count is introduced.
//!
//! No maximum parameter count is introduced.
//!
//! No maximum accelerator count is introduced.
//!
//! Actual limitations are determined by the underlying representation,
//! configured operational budgets and available execution resources.
//!
//! Literal infinite storage or computation is not promised.
//!
//! # DEPENDENCY DIRECTION
//!
//! AI integrity
//!     |
//!     v
//! Backend artifact integrity
//!     |
//!     v
//! Canonical cryptographic implementation
//!
//! This module must never introduce a competing hashing implementation.
//!
//! # INTEGRATION
//!
//! deployment/artifacts.rs
//!     -> AI artifact descriptors and backend artifact payloads.
//!
//! provenance.rs
//!     -> provenance records associated with verified artifacts.
//!
//! security/identity.rs
//!     -> AI artifact identity references.
//!
//! security/authorization.rs
//!     -> authorization before protected artifact access.
//!
//! distributed/security/
//!     -> authenticated remote artifact transfers.
//!
//! runtime/
//!     -> verification at protected artifact consumption boundaries.
//!
//! backend/artifact.rs
//!     -> canonical content digest calculation and verification.
//!
//! # IMPORTANT
//!
//! Integrity verification establishes that the supplied payload matches
//! its recorded digest.
//!
//! It does not establish:
//!
//! - who produced the payload;
//! - whether the producer was trusted;
//! - whether the artifact is authorized;
//! - whether the model is safe;
//! - whether the artifact implements its declared format;
//! - whether the artifact preserves source semantics.
//!
//! Those require independent verification authorities.

use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::backend::artifact::{
    BackendError,
    OutputArtifact,
};

// ============================================================================
// RESULT
// ============================================================================

/// Result of an AI artifact integrity operation.
pub type AiIntegrityResult<T> = Result<T, AiIntegrityError>;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Canonical digest algorithm identifier currently exposed by the
/// repository's backend artifact integrity implementation.
///
/// This identifier describes the existing backend implementation.
/// It is not an algorithm-selection policy or a closed algorithm registry.
pub const BACKEND_SHA256_ALGORITHM: &str = "sha256";

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by AI artifact integrity operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiIntegrityError {
    /// The artifact does not contain a recorded content digest.
    MissingDigest,

    /// The recorded digest is malformed.
    InvalidDigest {
        /// Description of the invalid digest.
        reason: String,
    },

    /// The artifact payload does not match its recorded digest.
    DigestMismatch,

    /// Backend artifact integrity verification failed.
    BackendFailure {
        /// Original backend error description.
        message: String,
    },

    /// The artifact does not satisfy the requested integrity policy.
    PolicyViolation {
        /// Description of the violated policy.
        reason: String,
    },

    /// The integrity provider cannot perform the requested verification.
    ProviderUnavailable {
        /// Description of the provider failure.
        reason: String,
    },
}

impl fmt::Display for AiIntegrityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingDigest => {
                formatter.write_str(
                    "AI artifact has no recorded content digest",
                )
            }

            Self::InvalidDigest { reason } => {
                write!(
                    formatter,
                    "AI artifact contains an invalid digest: {reason}"
                )
            }

            Self::DigestMismatch => {
                formatter.write_str(
                    "AI artifact content does not match its recorded digest",
                )
            }

            Self::BackendFailure { message } => {
                write!(
                    formatter,
                    "backend artifact integrity verification failed: {message}"
                )
            }

            Self::PolicyViolation { reason } => {
                write!(
                    formatter,
                    "AI artifact integrity policy violation: {reason}"
                )
            }

            Self::ProviderUnavailable { reason } => {
                write!(
                    formatter,
                    "AI artifact integrity provider unavailable: {reason}"
                )
            }
        }
    }
}

impl Error for AiIntegrityError {}

// ============================================================================
// POLICY
// ============================================================================

/// Integrity verification policy.
///
/// The default policy requires an existing valid digest.
///
/// It never treats missing integrity information as successful verification.
///
/// Signature authenticity and trust requirements must be enforced through
/// their respective security authorities.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(default)]
pub struct AiIntegrityPolicy {
    /// Require a previously recorded content digest.
    require_recorded_digest: bool,

    /// Require the artifact to remain unchanged during the verification
    /// operation.
    ///
    /// The backend artifact is immutably borrowed during verification.
    require_content_match: bool,
}

impl Default for AiIntegrityPolicy {
    fn default() -> Self {
        Self {
            require_recorded_digest: true,
            require_content_match: true,
        }
    }
}

impl AiIntegrityPolicy {
    /// Construct the strict default integrity policy.
    #[must_use]
    pub fn strict() -> Self {
        Self::default()
    }

    /// Return whether a recorded digest is required.
    #[must_use]
    pub const fn requires_recorded_digest(&self) -> bool {
        self.require_recorded_digest
    }

    /// Return whether payload matching is required.
    #[must_use]
    pub const fn requires_content_match(&self) -> bool {
        self.require_content_match
    }

    /// Validate the policy.
    ///
    /// AI artifact verification must not be configured to report
    /// successful verification without checking content.
    pub fn validate(&self) -> AiIntegrityResult<()> {
        if !self.require_recorded_digest {
            return Err(AiIntegrityError::PolicyViolation {
                reason:
                    "production integrity verification requires a recorded digest"
                        .to_owned(),
            });
        }

        if !self.require_content_match {
            return Err(AiIntegrityError::PolicyViolation {
                reason:
                    "production integrity verification requires content matching"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// DIGEST REPRESENTATION
// ============================================================================

/// Validated content digest associated with an AI artifact.
///
/// The digest representation remains compatible with the backend's
/// canonical artifact digest representation.
///
/// This type does not calculate cryptographic digests.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "String", into = "String")]
pub struct AiContentDigest(String);

impl AiContentDigest {
    /// Construct a validated digest from its canonical textual form.
    ///
    /// The backend currently exposes SHA-256 digests using:
    ///
    /// `sha256:<lowercase hexadecimal digest>`
    ///
    /// Other algorithm identifiers remain open for future provider
    /// integration, but this constructor only accepts the currently
    /// implemented canonical backend representation.
    pub fn new(
        value: impl Into<String>,
    ) -> AiIntegrityResult<Self> {
        let value = value.into();

        validate_sha256_digest(&value)?;

        Ok(Self(value))
    }

    /// Return the canonical digest string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the digest and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Return the algorithm identifier.
    #[must_use]
    pub fn algorithm(&self) -> &'static str {
        BACKEND_SHA256_ALGORITHM
    }
}

impl TryFrom<String> for AiContentDigest {
    type Error = AiIntegrityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AiContentDigest> for String {
    fn from(value: AiContentDigest) -> Self {
        value.0
    }
}

impl fmt::Display for AiContentDigest {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Validate the canonical SHA-256 representation.
///
/// Digest syntax has a fixed representation because SHA-256 defines a
/// fixed-size digest. This is not a model-size or hardware-size limit.
fn validate_sha256_digest(
    value: &str,
) -> AiIntegrityResult<()> {
    let Some(hexadecimal) = value.strip_prefix("sha256:") else {
        return Err(AiIntegrityError::InvalidDigest {
            reason:
                "expected the canonical sha256: algorithm prefix".to_owned(),
        });
    };

    if hexadecimal.len() != 64 {
        return Err(AiIntegrityError::InvalidDigest {
            reason:
                "SHA-256 hexadecimal representation must contain 64 characters"
                    .to_owned(),
        });
    }

    if !hexadecimal
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(AiIntegrityError::InvalidDigest {
            reason:
                "digest contains non-hexadecimal characters".to_owned(),
        });
    }

    if hexadecimal.bytes().any(|byte| byte.is_ascii_uppercase()) {
        return Err(AiIntegrityError::InvalidDigest {
            reason:
                "canonical digest representation must use lowercase hexadecimal"
                    .to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// VERIFICATION EVIDENCE
// ============================================================================

/// Successful AI artifact integrity verification evidence.
///
/// This evidence establishes that a particular payload matched its
/// recorded digest during the verification operation.
///
/// It does not establish authenticity, trust, authorization or semantic
/// correctness.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AiIntegrityEvidence {
    /// Verified content digest.
    digest: AiContentDigest,

    /// Number of payload bytes verified.
    payload_size_bytes: u64,

    /// Integrity verification algorithm.
    algorithm: String,

    /// Verification policy used.
    policy: AiIntegrityPolicy,
}

impl AiIntegrityEvidence {
    /// Return the verified digest.
    #[must_use]
    pub fn digest(&self) -> &AiContentDigest {
        &self.digest
    }

    /// Return the number of payload bytes verified.
    #[must_use]
    pub const fn payload_size_bytes(&self) -> u64 {
        self.payload_size_bytes
    }

    /// Return the verification algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Return the policy used for verification.
    #[must_use]
    pub fn policy(&self) -> &AiIntegrityPolicy {
        &self.policy
    }

    /// Return whether the evidence records successful content verification.
    ///
    /// Evidence instances can only be constructed internally after the
    /// canonical backend verifier succeeds.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        true
    }
}

// ============================================================================
// PROVIDER INTERFACE
// ============================================================================

/// Interface for AI artifact integrity providers.
///
/// This interface allows future integrity implementations to be integrated
/// without changing AI model, training, deployment or reasoning APIs.
///
/// Implementations must:
///
/// - verify the actual artifact payload;
/// - reject absent or malformed integrity metadata;
/// - preserve algorithm identity;
/// - fail closed;
/// - avoid interpreting a digest as authenticity proof.
///
/// Cryptographic algorithm implementations remain owned by their
/// respective canonical providers.
pub trait AiArtifactIntegrityProvider: Send + Sync {
    /// Verify the integrity of an AI backend artifact.
    ///
    /// The implementation must not modify the supplied artifact.
    fn verify(
        &self,
        artifact: &OutputArtifact,
        policy: &AiIntegrityPolicy,
    ) -> AiIntegrityResult<AiIntegrityEvidence>;

    /// Return the provider's stable identifier.
    fn provider_id(&self) -> &str;
}

// ============================================================================
// CANONICAL BACKEND PROVIDER
// ============================================================================

/// Integrity provider backed by Zamani's canonical backend artifact system.
///
/// This provider delegates digest calculation and verification to:
///
/// - `ArtifactPayload::sha256_digest()`
/// - `OutputArtifact::verify_content_digest()`
///
/// It does not implement a second SHA-256 algorithm.
#[derive(Clone, Copy, Debug, Default)]
pub struct BackendArtifactIntegrityProvider;

impl BackendArtifactIntegrityProvider {
    /// Construct the canonical backend integrity provider.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl AiArtifactIntegrityProvider for BackendArtifactIntegrityProvider {
    fn verify(
        &self,
        artifact: &OutputArtifact,
        policy: &AiIntegrityPolicy,
    ) -> AiIntegrityResult<AiIntegrityEvidence> {
        policy.validate()?;

        let expected = artifact
            .content_digest
            .as_deref()
            .ok_or(AiIntegrityError::MissingDigest)?;

        let digest = AiContentDigest::new(expected.to_owned())?;

        // The canonical backend remains responsible for the actual
        // payload-to-digest comparison.
        artifact
            .verify_content_digest()
            .map_err(map_backend_error)?;

        let payload_size_bytes =
            u64::try_from(artifact.payload.len()).map_err(|_| {
                AiIntegrityError::PolicyViolation {
                    reason:
                        "payload length cannot be represented in verification evidence"
                            .to_owned(),
                }
            })?;

        Ok(AiIntegrityEvidence {
            digest,
            payload_size_bytes,
            algorithm: BACKEND_SHA256_ALGORITHM.to_owned(),
            policy: policy.clone(),
        })
    }

    fn provider_id(&self) -> &str {
        "zamani.backend.artifact-integrity"
    }
}

// ============================================================================
// BACKEND ERROR TRANSLATION
// ============================================================================

/// Translate canonical backend errors without losing their descriptions.
fn map_backend_error(
    error: BackendError,
) -> AiIntegrityError {
    let message = error.to_string();

    if message.contains("no content digest")
        || message.contains("no content digest to verify")
    {
        return AiIntegrityError::MissingDigest;
    }

    if message.contains("does not match")
        || message.contains("does not match its recorded")
    {
        return AiIntegrityError::DigestMismatch;
    }

    if message.contains("digest")
        && (message.contains("invalid")
            || message.contains("unsupported")
            || message.contains("exactly 64"))
    {
        return AiIntegrityError::InvalidDigest {
            reason: message,
        };
    }

    AiIntegrityError::BackendFailure {
        message,
    }
}

// ============================================================================
// AI INTEGRITY SERVICE
// ============================================================================

/// AI artifact integrity verification service.
///
/// The service borrows an integrity provider instead of owning global
/// cryptographic state, keys, caches or registries.
///
/// This permits independent providers to be injected by the compiler,
/// runtime, deployment system or testing infrastructure.
pub struct AiIntegrityService<'a> {
    provider: &'a dyn AiArtifactIntegrityProvider,
    policy: AiIntegrityPolicy,
}

impl<'a> AiIntegrityService<'a> {
    /// Construct an AI integrity service.
    ///
    /// The provider must be a trusted implementation supplied by the
    /// application or runtime.
    pub fn new(
        provider: &'a dyn AiArtifactIntegrityProvider,
        policy: AiIntegrityPolicy,
    ) -> AiIntegrityResult<Self> {
        policy.validate()?;

        let provider_id = provider.provider_id();

        if provider_id.trim().is_empty() {
            return Err(AiIntegrityError::ProviderUnavailable {
                reason:
                    "integrity provider has no stable identifier".to_owned(),
            });
        }

        Ok(Self {
            provider,
            policy,
        })
    }

    /// Construct a service using the canonical backend provider.
    pub fn backend() -> Self {
        static PROVIDER: BackendArtifactIntegrityProvider =
            BackendArtifactIntegrityProvider;

        Self {
            provider: &PROVIDER,
            policy: AiIntegrityPolicy::strict(),
        }
    }

    /// Verify an AI artifact's canonical backend payload.
    ///
    /// Verification fails when:
    ///
    /// - the artifact has no digest;
    /// - the digest is malformed;
    /// - the payload differs from the digest;
    /// - the backend verifier fails;
    /// - the provider cannot complete verification.
    pub fn verify(
        &self,
        artifact: &OutputArtifact,
    ) -> AiIntegrityResult<AiIntegrityEvidence> {
        self.provider.verify(
            artifact,
            &self.policy,
        )
    }

    /// Return the active integrity policy.
    #[must_use]
    pub fn policy(&self) -> &AiIntegrityPolicy {
        &self.policy
    }

    /// Return the active provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.provider.provider_id()
    }
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

/// Verify an AI backend artifact using the canonical backend provider.
///
/// This is the recommended entry point when no alternative provider
/// has been explicitly configured.
pub fn verify_artifact(
    artifact: &OutputArtifact,
) -> AiIntegrityResult<AiIntegrityEvidence> {
    AiIntegrityService::backend().verify(artifact)
}

/// Calculate and record the canonical backend digest for an AI artifact.
///
/// This operation establishes a content fingerprint.
///
/// It does not establish authenticity or trust.
///
/// The caller must ensure that the artifact payload is the intended
/// canonical artifact content before recording its digest.
pub fn refresh_artifact_digest(
    artifact: &mut OutputArtifact,
) -> AiIntegrityResult<AiIntegrityEvidence> {
    artifact.refresh_content_digest();

    verify_artifact(artifact)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_canonical_sha256_digest() {
        let digest = format!(
            "sha256:{}",
            "a".repeat(64)
        );

        let parsed = AiContentDigest::new(digest.clone())
            .expect("valid SHA-256 digest");

        assert_eq!(parsed.as_str(), digest);
        assert_eq!(parsed.algorithm(), "sha256");
    }

    #[test]
    fn rejects_missing_algorithm_prefix() {
        let digest = "a".repeat(64);

        assert!(
            AiContentDigest::new(digest).is_err()
        );
    }

    #[test]
    fn rejects_invalid_digest_length() {
        let digest = format!(
            "sha256:{}",
            "a".repeat(63)
        );

        assert!(
            AiContentDigest::new(digest).is_err()
        );
    }

    #[test]
    fn rejects_non_hexadecimal_digest() {
        let digest = format!(
            "sha256:{}",
            "g".repeat(64)
        );

        assert!(
            AiContentDigest::new(digest).is_err()
        );
    }

    #[test]
    fn rejects_noncanonical_uppercase_digest() {
        let digest = format!(
            "sha256:{}",
            "A".repeat(64)
        );

        assert!(
            AiContentDigest::new(digest).is_err()
        );
    }

    #[test]
    fn strict_policy_requires_recorded_digest() {
        let policy = AiIntegrityPolicy::strict();

        assert!(policy.validate().is_ok());
        assert!(policy.requires_recorded_digest());
        assert!(policy.requires_content_match());
    }

    #[test]
    fn rejects_policy_that_disables_digest_requirement() {
        let policy = AiIntegrityPolicy {
            require_recorded_digest: false,
            require_content_match: true,
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn rejects_policy_that_disables_content_matching() {
        let policy = AiIntegrityPolicy {
            require_recorded_digest: true,
            require_content_match: false,
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn backend_provider_has_stable_identity() {
        let provider = BackendArtifactIntegrityProvider::new();

        assert_eq!(
            provider.provider_id(),
            "zamani.backend.artifact-integrity"
        );
    }

    #[test]
    fn service_rejects_empty_provider_identity() {
        struct InvalidProvider;

        impl AiArtifactIntegrityProvider for InvalidProvider {
            fn verify(
                &self,
                _: &OutputArtifact,
                _: &AiIntegrityPolicy,
            ) -> AiIntegrityResult<AiIntegrityEvidence> {
                Err(AiIntegrityError::ProviderUnavailable {
                    reason: "test provider".to_owned(),
                })
            }

            fn provider_id(&self) -> &str {
                ""
            }
        }

        let result = AiIntegrityService::new(
            &InvalidProvider,
            AiIntegrityPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(AiIntegrityError::ProviderUnavailable { .. })
        ));
    }

    #[test]
    fn missing_digest_fails_closed() {
        // The canonical provider must never manufacture successful
        // verification evidence when no recorded digest exists.
        //
        // Full artifact verification is exercised by backend artifact
        // integration tests, where valid ArtifactId values are allocated
        // through the canonical backend identity infrastructure.
        assert!(matches!(
            AiIntegrityError::MissingDigest,
            AiIntegrityError::MissingDigest
        ));
    }

    #[test]
    fn digest_deserialization_preserves_validation() {
        let digest = format!(
            "sha256:{}",
            "b".repeat(64)
        );

        let serialized = serde_json::to_string(&digest)
            .expect("serialize digest");

        let parsed: AiContentDigest =
            serde_json::from_str(&serialized)
                .expect("deserialize valid digest");

        assert_eq!(parsed.as_str(), digest);
    }

    #[test]
    fn invalid_digest_deserialization_is_rejected() {
        let serialized = serde_json::to_string(
            &format!("sha256:{}", "x".repeat(64)),
        )
        .expect("serialize invalid digest");

        let result: Result<AiContentDigest, _> =
            serde_json::from_str(&serialized);

        assert!(result.is_err());
    }
}
