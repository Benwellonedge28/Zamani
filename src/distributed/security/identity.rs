
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Distributed Computing — Cryptographic Identity Foundation.
//!
//! Path: src/distributed/security/identity.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: No unsafe Rust.
//!
//! # Purpose
//!
//! Defines authenticated principal identity primitives for distributed
//! Zamani execution environments.
//!
//! This module provides:
//!
//! - PrincipalId;
//! - PublicKeyFingerprint;
//! - PeerIdentity;
//! - PeerIdentityDocument;
//! - IdentityKeyPair;
//! - IdentityProof;
//! - Ed25519 signing and verification;
//! - challenge-response identity verification;
//! - validated public identity serialization.
//!
//! # Architectural ownership
//!
//! This module owns cryptographic identity representation and proof
//! verification.
//!
//! It does not own:
//!
//! - source-language identity declarations;
//! - semantic identity resolution;
//! - distributed participant allocation;
//! - hardware identities;
//! - domain identities;
//! - physical endpoints;
//! - authentication protocols or network handshakes;
//! - trust establishment;
//! - authorization;
//! - credential storage;
//! - key distribution or revocation;
//! - certificate authorities;
//! - runtime execution.
//!
//! # Identity separation
//!
//! Principal identity:
//!     Who a distributed execution participant claims to be.
//!
//! Participant identity:
//!     Which logical participant is represented in the distributed model.
//!
//! Public-key identity:
//!     Which cryptographic public key is associated with a principal.
//!
//! Physical identity:
//!     Which actual machine, device, process, or endpoint is involved.
//!
//! These identities MUST NOT be interchangeable.
//!
//! # Security guarantees
//!
//! A valid IdentityProof establishes that the holder of the corresponding
//! private key signed a challenge associated with the supplied principal.
//!
//! It does not independently establish:
//!
//! - that the principal is trusted;
//! - that the key is currently authorized;
//! - that the key has not been revoked;
//! - that the identity belongs to a particular physical machine;
//! - that the challenge is fresh;
//! - that the peer is authorized to execute a workload.
//!
//! Freshness, replay protection, trust policy, authorization, and key
//! lifecycle are responsibilities of their respective security modules.
//!
//! # Scalability
//!
//! No fixed number of principals, participants, identities, machines,
//! services, or devices is imposed.
//!
//! Identifiers are dynamically sized strings.
//! Cryptographic key and signature sizes are determined by Ed25519.
//!
//! The implementation cannot guarantee literally infinite computation.
//! Actual resource availability and representational limits remain relevant.
//!
//! # Dependencies
//!
//! Existing Cargo.toml dependencies:
//!
//! - ed25519-dalek;
//! - sha2;
//! - rand;
//! - serde;
//! - hex.
//!
//! No additional dependency is required.

use core::fmt;
use core::str::FromStr;

use ed25519_dalek::{
    Signature,
    Signer,
    SigningKey,
    Verifier,
    VerifyingKey,
};

use rand::RngCore;

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

use sha2::{
    Digest,
    Sha256,
};

use crate::distributed::error::{
    DistributedError,
    DistributedResult,
};

// ============================================================================
// Cryptographic domain separation
// ============================================================================

/// Domain separation prevents identity proofs from being confused with
/// signatures generated for unrelated application protocols.
const IDENTITY_PROOF_DOMAIN: &[u8] =
    b"zamani.distributed.identity-proof.v1";

/// Fingerprint domain separation.
const IDENTITY_FINGERPRINT_DOMAIN: &[u8] =
    b"zamani.distributed.identity-fingerprint.v1";

// ============================================================================
// Principal identity
// ============================================================================

/// Stable logical security principal identifier.
///
/// A principal identifies a security subject independently of the machine,
/// process, device, network address, or execution provider.
///
/// The representation is intentionally open-ended.
///
/// Source-language qualified-name interpretation remains the responsibility
/// of the frontend and semantic security layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrincipalId(String);

impl PrincipalId {
    /// Construct a principal identity.
    ///
    /// The value must:
    ///
    /// - be non-empty;
    /// - have no surrounding whitespace;
    /// - contain no control characters;
    /// - contain no NUL characters.
    ///
    /// This constructor does not impose an arbitrary maximum identity length.
    pub fn new(
        value: impl Into<String>,
    ) -> DistributedResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(invalid_input(
                "principal identity cannot be empty",
            ));
        }

        if value.trim() != value {
            return Err(invalid_input(
                "principal identity cannot contain surrounding whitespace",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(invalid_input(
                "principal identity cannot contain control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical principal identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Return whether the identifier is empty.
    ///
    /// Successfully constructed identifiers are never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for PrincipalId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for PrincipalId {
    type Err = DistributedError;

    fn from_str(
        value: &str,
    ) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for PrincipalId {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for PrincipalId {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// ============================================================================
// Public-key fingerprint
// ============================================================================

/// Cryptographic identifier derived from a public key.
///
/// This is a public identifier, not a secret and not an authentication token.
///
/// Fingerprints are calculated using SHA-256 with explicit domain separation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct PublicKeyFingerprint([u8; 32]);

impl PublicKeyFingerprint {
    /// Derive a fingerprint from an Ed25519 public key.
    #[must_use]
    pub fn from_public_key(
        public_key: &[u8; 32],
    ) -> Self {
        let mut hasher = Sha256::new();

        hasher.update(IDENTITY_FINGERPRINT_DOMAIN);
        hasher.update(public_key);

        let digest = hasher.finalize();

        let mut fingerprint = [0_u8; 32];

        fingerprint.copy_from_slice(&digest);

        Self(fingerprint)
    }

    /// Return the raw fingerprint bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Return the canonical hexadecimal representation.
    #[must_use]
    pub fn to_hex(self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for PublicKeyFingerprint {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&hex::encode(self.0))
    }
}

// ============================================================================
// Authenticated peer identity
// ============================================================================

/// Public cryptographic identity associated with a principal.
///
/// This structure contains public information only.
///
/// Possession of this structure does not prove possession of the
/// corresponding private key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerIdentity {
    principal: PrincipalId,
    public_key: [u8; 32],
    fingerprint: PublicKeyFingerprint,
}

impl PeerIdentity {
    /// Construct an identity from a principal and an Ed25519 public key.
    pub fn new(
        principal: PrincipalId,
        public_key: [u8; 32],
    ) -> DistributedResult<Self> {
        VerifyingKey::from_bytes(&public_key)
            .map_err(|_| authentication_error(
                Some(principal.as_str()),
                "invalid Ed25519 public key",
            ))?;

        let fingerprint =
            PublicKeyFingerprint::from_public_key(&public_key);

        Ok(Self {
            principal,
            public_key,
            fingerprint,
        })
    }

    /// Return the principal.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Return the public key.
    #[must_use]
    pub const fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }

    /// Return the public-key fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> PublicKeyFingerprint {
        self.fingerprint
    }

    /// Verify that the public identity remains internally consistent.
    pub fn validate(&self) -> DistributedResult<()> {
        VerifyingKey::from_bytes(&self.public_key)
            .map_err(|_| authentication_error(
                Some(self.principal.as_str()),
                "invalid Ed25519 public key",
            ))?;

        let expected =
            PublicKeyFingerprint::from_public_key(
                &self.public_key,
            );

        if expected != self.fingerprint {
            return Err(authentication_error(
                Some(self.principal.as_str()),
                "public-key fingerprint mismatch",
            ));
        }

        Ok(())
    }

    /// Verify a challenge-response proof against this identity.
    pub fn verify(
        &self,
        challenge: &[u8],
        proof: &IdentityProof,
    ) -> DistributedResult<()> {
        self.validate()?;

        proof.verify(self, challenge)
    }

    /// Create a public serialization document.
    ///
    /// No private key material is included.
    #[must_use]
    pub fn to_document(&self) -> PeerIdentityDocument {
        PeerIdentityDocument {
            principal: self.principal.to_string(),
            public_key: self.public_key.to_vec(),
            fingerprint: self.fingerprint.0.to_vec(),
        }
    }

    /// Reconstruct and validate a public identity document.
    pub fn from_document(
        document: PeerIdentityDocument,
    ) -> DistributedResult<Self> {
        let principal =
            PrincipalId::new(document.principal)?;

        let public_key: [u8; 32] = document.public_key
            .try_into()
            .map_err(|_| invalid_input(
                "Ed25519 public key must contain exactly 32 bytes",
            ))?;

        let supplied_fingerprint: [u8; 32] =
            document.fingerprint
                .try_into()
                .map_err(|_| invalid_input(
                    "public-key fingerprint must contain exactly 32 bytes",
                ))?;

        let identity = Self::new(
            principal,
            public_key,
        )?;

        if identity.fingerprint.0 != supplied_fingerprint {
            return Err(authentication_error(
                Some(identity.principal.as_str()),
                "serialized public-key fingerprint does not match the public key",
            ));
        }

        Ok(identity)
    }
}

// ============================================================================
// Public identity serialization
// ============================================================================

/// Serializable public representation of a peer identity.
///
/// This is untrusted external input until successfully reconstructed through
/// PeerIdentity::from_document().
///
/// Public-key bytes and fingerprints are represented as dynamically sized
/// byte sequences so malformed lengths can be rejected explicitly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerIdentityDocument {
    /// Canonical principal identifier.
    pub principal: String,

    /// Raw Ed25519 public key.
    pub public_key: Vec<u8>,

    /// SHA-256 public-key fingerprint.
    pub fingerprint: Vec<u8>,
}

// ============================================================================
// Identity proof
// ============================================================================

/// Signed challenge-response proof of private-key possession.
///
/// The proof binds:
///
/// - the principal identity;
/// - the public-key fingerprint;
/// - the challenge digest;
/// - the identity-proof protocol domain.
///
/// The challenge must be generated freshly by the verifier.
///
/// The verifier is responsible for replay protection and challenge expiry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityProof {
    /// Principal associated with the proof.
    principal: PrincipalId,

    /// Public-key fingerprint.
    fingerprint: Vec<u8>,

    /// Digest of the verifier-supplied challenge.
    challenge_digest: Vec<u8>,

    /// Ed25519 signature.
    signature: Vec<u8>,
}

impl IdentityProof {
    /// Return the principal associated with this proof.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Return the public-key fingerprint.
    #[must_use]
    pub fn fingerprint(&self) -> &[u8] {
        &self.fingerprint
    }

    /// Verify this proof against a public identity and challenge.
    pub fn verify(
        &self,
        identity: &PeerIdentity,
        challenge: &[u8],
    ) -> DistributedResult<()> {
        identity.validate()?;

        if self.principal != identity.principal {
            return Err(authentication_error(
                Some(identity.principal.as_str()),
                "identity proof belongs to another principal",
            ));
        }

        if self.fingerprint.as_slice()
            != identity.fingerprint.as_bytes()
        {
            return Err(authentication_error(
                Some(identity.principal.as_str()),
                "identity proof uses a different public key",
            ));
        }

        let expected_digest = challenge_digest(challenge);

        if self.challenge_digest.as_slice()
            != expected_digest.as_slice()
        {
            return Err(authentication_error(
                Some(identity.principal.as_str()),
                "identity proof challenge mismatch",
            ));
        }

        let signature_bytes: [u8; 64] =
            self.signature.as_slice()
                .try_into()
                .map_err(|_| authentication_error(
                    Some(identity.principal.as_str()),
                    "invalid Ed25519 signature length",
                ))?;

        let signature =
            Signature::from_bytes(&signature_bytes);

        let verifying_key =
            VerifyingKey::from_bytes(&identity.public_key)
                .map_err(|_| authentication_error(
                    Some(identity.principal.as_str()),
                    "invalid Ed25519 verification key",
                ))?;

        let payload = signing_digest(
            &self.principal,
            &identity.fingerprint,
            &expected_digest,
        )?;

        verifying_key.verify(
            &payload,
            &signature,
        ).map_err(|_| authentication_error(
            Some(identity.principal.as_str()),
            "Ed25519 identity signature verification failed",
        ))?;

        Ok(())
    }
}

// ============================================================================
// Identity key pair
// ============================================================================

/// Ed25519 private/public identity key pair.
///
/// The private signing key is deliberately:
///
/// - private;
/// - non-serializable;
/// - non-cloneable;
/// - excluded from Debug output.
///
/// The cryptographic dependency provides its own secret-key memory handling.
/// This module never exposes private key bytes through its public API.
///
/// Key persistence and secure key storage belong to a dedicated credential
/// or key-management provider.
pub struct IdentityKeyPair {
    principal: PrincipalId,
    signing_key: SigningKey,
    peer_identity: PeerIdentity,
}

impl fmt::Debug for IdentityKeyPair {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("IdentityKeyPair")
            .field("principal", &self.principal)
            .field("fingerprint", &self.peer_identity.fingerprint)
            .field("private_key", &"[REDACTED]")
            .finish()
    }
}

impl IdentityKeyPair {
    /// Construct an identity key pair from externally supplied secret bytes.
    ///
    /// Secret material must come from an approved key-management facility.
    ///
    /// This method does not persist or distribute the private key.
    pub fn from_secret_bytes(
        principal: PrincipalId,
        secret: &[u8; 32],
    ) -> DistributedResult<Self> {
        let signing_key =
            SigningKey::from_bytes(secret);

        let public_key =
            signing_key.verifying_key().to_bytes();

        let peer_identity =
            PeerIdentity::new(
                principal.clone(),
                public_key,
            )?;

        Ok(Self {
            principal,
            signing_key,
            peer_identity,
        })
    }

    /// Generate a new Ed25519 identity key pair.
    ///
    /// Uses the existing rand dependency's operating-system-seeded
    /// thread-local random generator.
    ///
    /// Production deployments should also support externally managed keys
    /// through from_secret_bytes or a future key-provider abstraction.
    pub fn generate(
        principal: PrincipalId,
    ) -> DistributedResult<Self> {
        let mut secret = [0_u8; 32];

        rand::thread_rng()
            .try_fill_bytes(&mut secret)
            .map_err(|_| authentication_error(
                Some(principal.as_str()),
                "cryptographically secure random key generation failed",
            ))?;

        let result =
            Self::from_secret_bytes(
                principal,
                &secret,
            );

        // Clear the temporary stack copy.
        secret.fill(0);

        result
    }

    /// Return the public identity.
    #[must_use]
    pub fn peer_identity(&self) -> &PeerIdentity {
        &self.peer_identity
    }

    /// Return the principal associated with this key pair.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Sign a verifier-generated challenge.
    ///
    /// Challenge freshness, uniqueness and expiration are the verifier's
    /// responsibility.
    pub fn prove_challenge(
        &self,
        challenge: &[u8],
    ) -> DistributedResult<IdentityProof> {
        let digest = challenge_digest(challenge);

        let payload = signing_digest(
            &self.principal,
            &self.peer_identity.fingerprint,
            &digest,
        )?;

        let signature =
            self.signing_key.sign(&payload);

        Ok(IdentityProof {
            principal: self.principal.clone(),
            fingerprint: self.peer_identity.fingerprint.0.to_vec(),
            challenge_digest: digest.to_vec(),
            signature: signature.to_bytes().to_vec(),
        })
    }

    /// Sign and immediately verify a challenge against this identity.
    ///
    /// This is useful for local cryptographic self-tests. It is not a
    /// replacement for remote authentication.
    pub fn self_verify(
        &self,
        challenge: &[u8],
    ) -> DistributedResult<()> {
        let proof = self.prove_challenge(challenge)?;

        self.peer_identity.verify(
            challenge,
            &proof,
        )
    }
}

// ============================================================================
// Cryptographic helpers
// ============================================================================

/// Calculate a challenge digest.
///
/// Hashing is performed incrementally; challenge size is not restricted by
/// an arbitrary protocol constant in this identity layer.
fn challenge_digest(
    challenge: &[u8],
) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(IDENTITY_PROOF_DOMAIN);
    hasher.update(challenge);

    let digest = hasher.finalize();

    let mut result = [0_u8; 32];

    result.copy_from_slice(&digest);

    result
}

/// Produce a domain-separated canonical signing digest.
///
/// Length-prefixing the principal prevents ambiguous concatenations.
fn signing_digest(
    principal: &PrincipalId,
    fingerprint: &PublicKeyFingerprint,
    challenge: &[u8; 32],
) -> DistributedResult<[u8; 32]> {
    let principal_length =
        u64::try_from(principal.as_str().len())
            .map_err(|_| invalid_input(
                "principal identity length cannot be represented",
            ))?;

    let mut hasher = Sha256::new();

    hasher.update(IDENTITY_PROOF_DOMAIN);
    hasher.update(principal_length.to_be_bytes());
    hasher.update(principal.as_str().as_bytes());
    hasher.update(fingerprint.as_bytes());
    hasher.update(challenge);

    let digest = hasher.finalize();

    let mut result = [0_u8; 32];

    result.copy_from_slice(&digest);

    Ok(result)
}

// ============================================================================
// Error integration
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> DistributedError {
    DistributedError::InvalidInput {
        message: message.into(),
    }
}

fn authentication_error(
    principal: Option<&str>,
    message: impl Into<String>,
) -> DistributedError {
    DistributedError::AuthenticationFailed {
        peer: principal.map(str::to_owned),
        message: message.into(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn principal(value: &str) -> PrincipalId {
        PrincipalId::new(value)
            .expect("valid principal")
    }

    fn key_pair(value: &str, seed: u8) -> IdentityKeyPair {
        let mut secret = [0_u8; 32];

        secret.fill(seed);

        IdentityKeyPair::from_secret_bytes(
            principal(value),
            &secret,
        ).expect("valid identity key pair")
    }

    #[test]
    fn principal_identity_is_validated() {
        assert!(PrincipalId::new("application::worker").is_ok());

        assert!(PrincipalId::new("").is_err());

        assert!(PrincipalId::new(" worker").is_err());

        assert!(PrincipalId::new("worker ").is_err());

        assert!(PrincipalId::new("worker\0").is_err());
    }

    #[test]
    fn principal_identity_is_open_ended() {
        let value = format!(
            "future::{}",
            "worker".repeat(2_000)
        );

        assert!(PrincipalId::new(value).is_ok());
    }

    #[test]
    fn principal_identity_serialization_round_trip() {
        let original = principal("application::worker");

        let encoded =
            serde_json::to_string(&original).unwrap();

        let decoded: PrincipalId =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn public_key_fingerprints_are_deterministic() {
        let first = key_pair("worker", 1);
        let second = key_pair("worker", 1);

        assert_eq!(
            first.peer_identity().fingerprint(),
            second.peer_identity().fingerprint(),
        );
    }

    #[test]
    fn different_public_keys_have_different_fingerprints() {
        let first = key_pair("worker", 1);
        let second = key_pair("worker", 2);

        assert_ne!(
            first.peer_identity().fingerprint(),
            second.peer_identity().fingerprint(),
        );
    }

    #[test]
    fn generated_identity_can_prove_key_possession() {
        let identity =
            key_pair("application::worker", 7);

        let challenge =
            b"fresh-authentication-challenge";

        let proof =
            identity.prove_challenge(challenge).unwrap();

        identity.peer_identity()
            .verify(challenge, &proof)
            .unwrap();
    }

    #[test]
    fn self_verification_succeeds() {
        let identity = key_pair("worker", 3);

        identity.self_verify(
            b"self-verification",
        ).unwrap();
    }

    #[test]
    fn incorrect_challenge_is_rejected() {
        let identity = key_pair("worker", 4);

        let proof = identity.prove_challenge(
            b"challenge-one",
        ).unwrap();

        let result = identity.peer_identity().verify(
            b"challenge-two",
            &proof,
        );

        assert!(result.is_err());
    }

    #[test]
    fn another_principal_cannot_reuse_identity_proof() {
        let first = key_pair("worker-one", 5);
        let second = key_pair("worker-two", 6);

        let challenge = b"authentication";

        let proof = first.prove_challenge(
            challenge,
        ).unwrap();

        assert!(second.peer_identity().verify(
            challenge,
            &proof,
        ).is_err());
    }

    #[test]
    fn modified_signature_is_rejected() {
        let identity = key_pair("worker", 8);

        let mut proof = identity.prove_challenge(
            b"challenge",
        ).unwrap();

        proof.signature[0] ^= 0x01;

        assert!(identity.peer_identity().verify(
            b"challenge",
            &proof,
        ).is_err());
    }

    #[test]
    fn public_identity_document_round_trip() {
        let identity = key_pair("worker", 9);

        let document =
            identity.peer_identity().to_document();

        let restored =
            PeerIdentity::from_document(document)
                .unwrap();

        assert_eq!(
            restored,
            *identity.peer_identity(),
        );
    }

    #[test]
    fn modified_public_key_document_is_rejected() {
        let identity = key_pair("worker", 10);

        let mut document =
            identity.peer_identity().to_document();

        document.public_key[0] ^= 0x01;

        assert!(
            PeerIdentity::from_document(document).is_err()
        );
    }

    #[test]
    fn malformed_public_key_length_is_rejected() {
        let document = PeerIdentityDocument {
            principal: "worker".to_owned(),
            public_key: vec![1, 2, 3],
            fingerprint: vec![0; 32],
        };

        assert!(
            PeerIdentity::from_document(document).is_err()
        );
    }

    #[test]
    fn modified_fingerprint_is_rejected() {
        let identity = key_pair("worker", 11);

        let mut document =
            identity.peer_identity().to_document();

        document.fingerprint[0] ^= 0x01;

        assert!(
            PeerIdentity::from_document(document).is_err()
        );
    }

    #[test]
    fn identity_proof_survives_serialization() {
        let identity = key_pair("worker", 12);

        let challenge = b"serialization-challenge";

        let proof = identity.prove_challenge(
            challenge,
        ).unwrap();

        let encoded =
            serde_json::to_string(&proof).unwrap();

        let decoded: IdentityProof =
            serde_json::from_str(&encoded).unwrap();

        identity.peer_identity()
            .verify(challenge, &decoded)
            .unwrap();
    }

    #[test]
    fn identity_key_debug_does_not_expose_secret() {
        let identity = key_pair("worker", 13);

        let output = format!("{identity:?}");

        assert!(output.contains("REDACTED"));

        assert!(!output.contains(
            "13131313131313131313131313131313"
        ));
    }

    #[test]
    fn identity_is_independent_of_physical_location() {
        let first = key_pair("service::worker", 14);
        let second = key_pair("service::worker", 14);

        assert_eq!(
            first.peer_identity(),
            second.peer_identity(),
        );
    }

    #[test]
    fn invalid_serialized_principal_is_rejected() {
        let encoded = "\"worker\\u0000\"";

        let decoded =
            serde_json::from_str::<PrincipalId>(encoded);

        assert!(decoded.is_err());
    }
}
