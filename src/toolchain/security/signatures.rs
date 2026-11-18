
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Digital Signatures
//!
//! Path: `src/toolchain/security/signatures.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the toolchain-facing integration boundary for digital signing
//! and signature verification.
//!
//! The actual cryptographic operations are delegated to the canonical
//! standard-library security implementation:
//!
//! - `crate::stdlib::security::cryptography`
//! - `crate::stdlib::security::signatures`
//!
//! This module also adapts the canonical cryptographic provider interface
//! to the toolchain trust verifier contract in
//! `crate::toolchain::security::trust`.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Toolchain-facing signature operation identifiers.
//! - Canonical trust-evidence signing-envelope encoding.
//! - Streaming access to canonical trust-evidence signing messages.
//! - The adapter from `CryptoProvider` to the toolchain
//!   `SignatureVerifier` contract.
//! - Conversion of cryptographic outcomes into trust-verification outcomes.
//!
//! This module does not own:
//!
//! - Cryptographic algorithms or their implementations.
//! - Provider discovery, registration or selection.
//! - Private-key or secret-key storage.
//! - Key generation, import, rotation or destruction.
//! - Authentication or identity resolution.
//! - Trust policy or trust-store persistence.
//! - Authorization.
//! - Artifact hashing or integrity verification.
//! - Provenance generation or verification.
//! - Package archive parsing or registry communication.
//! - Hardware discovery, allocation or runtime execution.
//!
//! ## Canonical authorities
//!
//! Cryptographic provider contracts:
//! `crate::stdlib::security::cryptography`
//!
//! Signing and verification operations:
//! `crate::stdlib::security::signatures`
//!
//! Trust evidence and trust decisions:
//! `crate::toolchain::security::trust`
//!
//! Authorization policy:
//! `crate::toolchain::security::policy`
//!
//! Package authenticity integration:
//! `crate::toolchain::packages::verification`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Source-level signature intent:
//! `grammar/security/signatures.g4`
//!
//! Source-level trust intent:
//! `grammar/security/trust.g4`
//!
//! ## Security invariants
//!
//! 1. No cryptographic algorithm is implemented here.
//! 2. Private key material is never accepted or stored here.
//! 3. Key references are opaque references, not key material.
//! 4. Signing and verification remain separate operations.
//! 5. Invalid signatures never produce successful verification.
//! 6. Unsupported or unavailable verification is indeterminate.
//! 7. Provider capability declarations are not security certification.
//! 8. A valid signature does not independently establish trust.
//! 9. Trust does not independently establish authorization.
//! 10. The trust-evidence signature binds its security-relevant metadata.
//! 11. Provider errors remain failures and are not converted into success.
//! 12. No credentials, keys, signatures or message contents are logged.
//!
//! ## Scalability
//!
//! Signature operations use the canonical streaming provider interfaces.
//! Trust-evidence messages can be read or written incrementally.
//!
//! This module introduces no universal limit on message size, signature
//! count, provider count, algorithm count, key count, target count,
//! participant count or computing domain.
//!
//! Actual limits are determined by the selected algorithm, provider,
//! available resources and explicitly configured operational budgets.
//! Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Trust-evidence envelope encoding is deterministic and versioned.
//! The encoding preserves exact identifier bytes and does not perform
//! case folding, locale conversion or implicit Unicode normalization.

use std::io::{self, Read, Write};

use crate::stdlib::security::cryptography::{
    CryptoAlgorithmId,
    CryptoCapability,
    CryptoError,
    CryptoErrorKind,
    CryptoOperationId,
    CryptoProvider,
    CryptoRequest,
    CryptoRequirement,
    KeyReference,
    SignatureMetadata,
};

use crate::stdlib::security::signatures::{
    sign_operation,
    verify_operation,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

use crate::toolchain::security::trust::{
    RevocationStatus,
    SignatureValidity,
    SignatureVerifier,
    TrustEvidence,
    TrustEvidenceId,
    TrustIssuerId,
    TrustKeyReference,
    TrustTimestamp,
    SignatureScheme,
};

/// Stable component identity.
pub const COMPONENT: &str = "toolchain.security.signatures";

/// Canonical signing-envelope format version.
///
/// This version is independent of the Zamani language, compiler,
/// package archive, registry protocol and cryptographic algorithm versions.
pub const TRUST_ENVELOPE_VERSION: u32 = 1;

/// Domain-separation prefix for toolchain trust-evidence signatures.
///
/// The trailing zero byte separates this message domain from other
/// application-defined signature messages.
pub const TRUST_ENVELOPE_DOMAIN: &[u8] =
    b"ZAMANI-TOOLCHAIN-TRUST-EVIDENCE\0";

/// Stable toolchain operation identity for signing.
pub const SIGN_OPERATION: &str = "toolchain.security.sign";

/// Stable toolchain operation identity for verification.
pub const VERIFY_OPERATION: &str = "toolchain.security.verify";

/// Stable toolchain operation identity for trust-evidence verification.
pub const VERIFY_TRUST_EVIDENCE_OPERATION: &str =
    "toolchain.security.verify_trust_evidence";

fn operation_error(
    error: CryptoError,
    operation: &'static str,
) -> ToolchainError {
    ToolchainError::provider_failed(
        "canonical cryptographic provider operation failed",
    )
    .with_operation(operation)
    .with_context(COMPONENT)
    .with_source(error)
}

fn io_error(
    error: io::Error,
    operation: &'static str,
) -> ToolchainError {
    ToolchainError::from_io(operation, error)
        .with_context(COMPONENT)
}

// ============================================================================
// CANONICAL STANDARD-LIBRARY API
// ============================================================================

/// Canonical cryptographic algorithm identity.
pub use crate::stdlib::security::cryptography::CryptoAlgorithmId as AlgorithmId;

/// Canonical cryptographic provider capability.
pub use crate::stdlib::security::cryptography::CryptoCapability as Capability;

/// Canonical cryptographic provider error.
pub use crate::stdlib::security::cryptography::CryptoError as SignatureError;

/// Canonical cryptographic provider error classification.
pub use crate::stdlib::security::cryptography::CryptoErrorKind as SignatureErrorKind;

/// Canonical cryptographic provider interface.
pub use crate::stdlib::security::cryptography::CryptoProvider as SignatureProvider;

/// Canonical cryptographic request.
pub use crate::stdlib::security::cryptography::CryptoRequest as SignatureRequest;

/// Canonical protected key reference.
pub use crate::stdlib::security::cryptography::KeyReference as SignatureKeyReference;

/// Canonical signature metadata.
pub use crate::stdlib::security::cryptography::SignatureMetadata as SignatureMetadata;

/// Canonical standard-library signing operation.
pub use crate::stdlib::security::signatures::sign_to;

/// Canonical standard-library verification operation.
pub use crate::stdlib::security::signatures::verify;

/// Canonical standard-library signing operation identifier.
pub use crate::stdlib::security::signatures::SIGN_OPERATION_ID;

/// Canonical standard-library verification operation identifier.
pub use crate::stdlib::security::signatures::VERIFY_OPERATION_ID;

// ============================================================================
// TRUST-EVIDENCE SIGNING ENVELOPE
// ============================================================================

/// Input fields bound by a toolchain trust-evidence signature.
///
/// The envelope binds the evidence identity, provider subject, issuer,
/// protected key reference, signature scheme, issuance time, optional
/// expiration time and application payload.
///
/// The payload is not interpreted by this module. Its meaning belongs to
/// the trust-evidence and semantic authorities.
///
/// Construct this value before signing. Use [`Self::write_to`] to stream
/// the exact bytes to the canonical signing provider. Construct the
/// corresponding [`TrustEvidence`] with the resulting signature bytes.
///
/// The same input fields must be used when verifying the evidence.
#[derive(Clone, Copy, Debug)]
pub struct TrustEvidenceSigningInput<'a> {
    evidence_id: &'a TrustEvidenceId,
    subject: &'a crate::stdlib::provider::ProviderId,
    issuer: &'a TrustIssuerId,
    key_reference: &'a TrustKeyReference,
    scheme: &'a SignatureScheme,
    issued_at: TrustTimestamp,
    expires_at: Option<TrustTimestamp>,
    payload: &'a [u8],
}

impl<'a> TrustEvidenceSigningInput<'a> {
    /// Creates trust-evidence signing input.
    ///
    /// This constructor only groups already validated identities and
    /// timestamps. It does not establish trust or validate a signature.
    #[must_use]
    pub const fn new(
        evidence_id: &'a TrustEvidenceId,
        subject: &'a crate::stdlib::provider::ProviderId,
        issuer: &'a TrustIssuerId,
        key_reference: &'a TrustKeyReference,
        scheme: &'a SignatureScheme,
        issued_at: TrustTimestamp,
        expires_at: Option<TrustTimestamp>,
        payload: &'a [u8],
    ) -> Self {
        Self {
            evidence_id,
            subject,
            issuer,
            key_reference,
            scheme,
            issued_at,
            expires_at,
            payload,
        }
    }

    /// Constructs signing input from an existing trust-evidence record.
    ///
    /// The returned value binds the same metadata and payload used by
    /// [`CanonicalTrustSignatureVerifier`].
    #[must_use]
    pub fn from_evidence(evidence: &'a TrustEvidence) -> Self {
        Self::new(
            evidence.id(),
            evidence.subject(),
            evidence.issuer(),
            evidence.key_reference(),
            evidence.scheme(),
            evidence.issued_at(),
            evidence.expires_at(),
            evidence.signed_payload(),
        )
    }

    /// Returns a reader over the canonical signing message.
    ///
    /// The reader does not copy the application payload into another
    /// allocation. It emits the small envelope header followed by the
    /// original payload bytes.
    ///
    /// # Errors
    ///
    /// Returns an error when the payload is empty.
    pub fn reader(&self) -> ToolchainResult<TrustEvidenceMessageReader<'a>> {
        if self.payload.is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "trust-evidence signing payload must not be empty",
                )
                .with_operation(VERIFY_TRUST_EVIDENCE_OPERATION)
                .with_context(COMPONENT),
            );
        }

        Ok(TrustEvidenceMessageReader {
            header: self.header(),
            header_offset: 0,
            payload: self.payload,
            payload_offset: 0,
        })
    }

    /// Writes the complete canonical signing message to a caller-owned
    /// output stream.
    ///
    /// This operation does not close, flush, seek or commit the output.
    /// Callers requiring atomic publication should write to a temporary
    /// or transactional destination and commit only after signing succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error when the payload is empty or the output stream fails.
    pub fn write_to(
        &self,
        output: &mut dyn Write,
    ) -> ToolchainResult<()> {
        if self.payload.is_empty() {
            return Err(
                ToolchainError::invalid_input(
                    "trust-evidence signing payload must not be empty",
                )
                .with_operation(SIGN_OPERATION)
                .with_context(COMPONENT),
            );
        }

        let header = self.header();

        output
            .write_all(&header)
            .map_err(|error| io_error(error, SIGN_OPERATION))?;

        output
            .write_all(self.payload)
            .map_err(|error| io_error(error, SIGN_OPERATION))?;

        Ok(())
    }

    fn header(&self) -> Vec<u8> {
        let mut output = Vec::new();

        output.extend_from_slice(TRUST_ENVELOPE_DOMAIN);
        output.extend_from_slice(&TRUST_ENVELOPE_VERSION.to_be_bytes());

        append_field(
            &mut output,
            1,
            self.evidence_id.as_str().as_bytes(),
        );

        append_field(
            &mut output,
            2,
            self.subject.as_str().as_bytes(),
        );

        append_field(
            &mut output,
            3,
            self.issuer.as_str().as_bytes(),
        );

        append_field(
            &mut output,
            4,
            self.key_reference.as_str().as_bytes(),
        );

        append_field(
            &mut output,
            5,
            self.scheme.as_str().as_bytes(),
        );

        append_field(
            &mut output,
            6,
            &self.issued_at.as_unix_nanos().to_be_bytes(),
        );

        let mut expiration = [0_u8; 17];

        match self.expires_at {
            Some(timestamp) => {
                expiration[0] = 1;
                expiration[1..].copy_from_slice(
                    &timestamp.as_unix_nanos().to_be_bytes(),
                );
            }
            None => {
                expiration[0] = 0;
            }
        }

        append_field(&mut output, 7, &expiration);

        append_field_header(
            &mut output,
            8,
            self.payload.len() as u128,
        );

        output
    }
}

/// Appends a tagged, length-prefixed field to the canonical envelope.
fn append_field(
    output: &mut Vec<u8>,
    tag: u16,
    value: &[u8],
) {
    append_field_header(output, tag, value.len() as u128);
    output.extend_from_slice(value);
}

/// Appends a field tag and its unsigned 128-bit length.
///
/// The fixed-width length prefix makes concatenated fields unambiguous.
/// It does not impose a package, message or application-level size limit.
fn append_field_header(
    output: &mut Vec<u8>,
    tag: u16,
    length: u128,
) {
    output.extend_from_slice(&tag.to_be_bytes());
    output.extend_from_slice(&length.to_be_bytes());
}

/// Streaming reader for a canonical trust-evidence signing message.
///
/// The envelope header is buffered because it contains only the fixed
/// set of evidence metadata fields. The application payload remains
/// borrowed and is read directly without being copied.
pub struct TrustEvidenceMessageReader<'a> {
    header: Vec<u8>,
    header_offset: usize,
    payload: &'a [u8],
    payload_offset: usize,
}

impl Read for TrustEvidenceMessageReader<'_> {
    fn read(
        &mut self,
        buffer: &mut [u8],
    ) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let mut written = 0;

        if self.header_offset < self.header.len() {
            let remaining = &self.header[self.header_offset..];
            let count = remaining.len().min(buffer.len());

            buffer[..count].copy_from_slice(&remaining[..count]);
            self.header_offset += count;
            written += count;
        }

        if written < buffer.len()
            && self.payload_offset < self.payload.len()
        {
            let remaining = &self.payload[self.payload_offset..];
            let available = buffer.len() - written;
            let count = remaining.len().min(available);

            buffer[written..written + count]
                .copy_from_slice(&remaining[..count]);

            self.payload_offset += count;
            written += count;
        }

        Ok(written)
    }
}

// ============================================================================
// CANONICAL TRUST SIGNATURE VERIFIER
// ============================================================================

/// Adapter from the canonical standard-library cryptographic provider
/// to the toolchain trust-verification contract.
///
/// The adapter receives an already selected provider. It does not discover,
/// register, rank or select providers.
///
/// It delegates all cryptographic work to
/// `crate::stdlib::security::signatures::verify`.
///
/// The adapter verifies the canonical trust-evidence envelope, not merely
/// the application payload. Consequently, the evidence identity, subject,
/// issuer, key reference, scheme and validity interval are bound to the
/// signature.
///
/// A mathematically valid signature is only one input to trust evaluation.
/// The caller must still evaluate issuer trust, expiry, revocation and
/// applicable policy through `toolchain::security::trust`.
pub struct CanonicalTrustSignatureVerifier<'a> {
    provider: &'a dyn CryptoProvider,
}

impl<'a> CanonicalTrustSignatureVerifier<'a> {
    /// Creates an adapter around an explicitly supplied cryptographic
    /// provider.
    ///
    /// Provider registration or capability declarations alone do not
    /// certify the provider's implementation.
    #[must_use]
    pub const fn new(provider: &'a dyn CryptoProvider) -> Self {
        Self { provider }
    }

    /// Returns the provider used by this adapter.
    #[must_use]
    pub fn provider(&self) -> &dyn CryptoProvider {
        self.provider
    }

    fn verify_evidence(
        &self,
        evidence: &TrustEvidence,
    ) -> ToolchainResult<SignatureValidity> {
        evidence.validate()?;

        let algorithm = CryptoAlgorithmId::new(
            evidence.scheme().as_str(),
        )
        .map_err(|error| {
            ToolchainError::validation_failed(
                "trust evidence contains an invalid signature scheme",
            )
            .with_operation(VERIFY_TRUST_EVIDENCE_OPERATION)
            .with_context(COMPONENT)
            .with_source(error)
        })?;

        let key = KeyReference::new(
            evidence.key_reference().as_str(),
        )
        .map_err(|error| {
            ToolchainError::validation_failed(
                "trust evidence contains an invalid protected key reference",
            )
            .with_operation(VERIFY_TRUST_EVIDENCE_OPERATION)
            .with_context(COMPONENT)
            .with_source(error)
        })?;

        let operation = verify_operation().map_err(|error| {
            ToolchainError::internal(
                "canonical signature verification operation is invalid",
            )
            .with_operation(VERIFY_TRUST_EVIDENCE_OPERATION)
            .with_context(COMPONENT)
            .with_source(error)
        })?;

        let requirement = CryptoRequirement::new(operation.clone())
            .requiring_algorithm(algorithm.clone());

        let request = CryptoRequest::new(
            operation,
            algorithm.clone(),
            requirement,
        )
        .map_err(|error| {
            ToolchainError::validation_failed(
                "could not construct the trust-evidence verification request",
            )
            .with_operation(VERIFY_TRUST_EVIDENCE_OPERATION)
            .with_context(COMPONENT)
            .with_source(error)
        })?
        .using_key(key.clone());

        let metadata = SignatureMetadata::new(
            algorithm,
            key,
            self.provider.id().clone(),
        );

        let signing_input = TrustEvidenceSigningInput::from_evidence(
            evidence,
        );

        let mut message = signing_input.reader()?;
        let mut signature = io::Cursor::new(evidence.signature());

        match crate::stdlib::security::signatures::verify(
            self.provider,
            &request,
            &metadata,
            &mut message,
            &mut signature,
        ) {
            Ok(()) => Ok(SignatureValidity::Valid),

            Err(error) => match error.kind() {
                CryptoErrorKind::SignatureVerificationFailed => {
                    Ok(SignatureValidity::Invalid)
                }

                CryptoErrorKind::UnsupportedOperation
                | CryptoErrorKind::CapabilityUnavailable
                | CryptoErrorKind::ProviderUnavailable => {
                    Ok(SignatureValidity::Indeterminate)
                }

                CryptoErrorKind::InvalidAlgorithm
                | CryptoErrorKind::InvalidKeyReference
                | CryptoErrorKind::InvalidRequest
                | CryptoErrorKind::RequirementUnsatisfied => {
                    Err(
                        ToolchainError::validation_failed(
                            "trust-evidence signature request is invalid or cannot satisfy its requirements",
                        )
                        .with_operation(VERIFY_TRUST_EVIDENCE_OPERATION)
                        .with_context(COMPONENT)
                        .with_source(error),
                    )
                }

                _ => Err(operation_error(
                    error,
                    VERIFY_TRUST_EVIDENCE_OPERATION,
                )),
            },
        }
    }
}

impl SignatureVerifier for CanonicalTrustSignatureVerifier<'_> {
    /// Verify the canonical signature over all trust-evidence metadata
    /// and the application payload.
    ///
    /// An invalid signature returns `SignatureValidity::Invalid`.
    /// An unavailable or unsupported verification operation returns
    /// `SignatureValidity::Indeterminate`.
    /// Operational provider failures remain errors.
    fn verify(
        &self,
        evidence: &TrustEvidence,
    ) -> ToolchainResult<SignatureValidity> {
        self.verify_evidence(evidence)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;

    use crate::stdlib::provider::ProviderId;

    use crate::stdlib::security::cryptography::{
        CryptoCapability,
        CryptoProviderId,
        CryptoResult,
    };

    use crate::toolchain::security::trust::{
        SignatureScheme,
        TrustEvidence,
        TrustEvidenceId,
        TrustIssuerId,
        TrustKeyReference,
        TrustTimestamp,
    };

    const TEST_PROVIDER_ID: &str = "org.example.signature-provider";
    const TEST_SCHEME: &str = "org.example.signature.test-scheme";

    struct TestProvider {
        id: CryptoProviderId,
        capabilities: Vec<CryptoCapability>,
        expected_message: Vec<u8>,
    }

    impl TestProvider {
        fn new(expected_message: Vec<u8>) -> Self {
            let id = CryptoProviderId::new(TEST_PROVIDER_ID)
                .expect("test provider identity must be valid");

            let algorithm = CryptoAlgorithmId::new(TEST_SCHEME)
                .expect("test scheme must be valid");

            let capability = CryptoCapability::new(
                verify_operation().expect("verify operation must be valid"),
            )
            .with_algorithm(algorithm);

            Self {
                id,
                capabilities: vec![capability],
                expected_message,
            }
        }
    }

    impl CryptoProvider for TestProvider {
        fn id(&self) -> &CryptoProviderId {
            &self.id
        }

        fn capabilities(&self) -> &[CryptoCapability] {
            &self.capabilities
        }

        fn verify(
            &self,
            _request: &CryptoRequest,
            _metadata: &SignatureMetadata,
            message: &mut dyn Read,
            signature: &mut dyn Read,
        ) -> CryptoResult<()> {
            let mut actual_message = Vec::new();
            let mut actual_signature = Vec::new();

            message.read_to_end(&mut actual_message).map_err(|error| {
                CryptoError::new(CryptoErrorKind::InputFailure)
                    .with_source(error)
            })?;

            signature
                .read_to_end(&mut actual_signature)
                .map_err(|error| {
                    CryptoError::new(CryptoErrorKind::InputFailure)
                        .with_source(error)
                })?;

            if actual_message != self.expected_message
                || actual_signature != b"accepted-signature"
            {
                return Err(CryptoError::new(
                    CryptoErrorKind::SignatureVerificationFailed,
                ));
            }

            Ok(())
        }
    }

    struct EvidenceFixture {
        id: TrustEvidenceId,
        subject: ProviderId,
        issuer: TrustIssuerId,
        key: TrustKeyReference,
        scheme: SignatureScheme,
        issued_at: TrustTimestamp,
        expires_at: Option<TrustTimestamp>,
        payload: Vec<u8>,
    }

    impl EvidenceFixture {
        fn new() -> Self {
            Self {
                id: TrustEvidenceId::new("evidence:one")
                    .expect("evidence identity must be valid"),
                subject: ProviderId::new("org.example.provider")
                    .expect("provider identity must be valid"),
                issuer: TrustIssuerId::new("org.example.issuer")
                    .expect("issuer identity must be valid"),
                key: TrustKeyReference::new("key-reference:one")
                    .expect("key reference must be valid"),
                scheme: SignatureScheme::new(TEST_SCHEME)
                    .expect("signature scheme must be valid"),
                issued_at: TrustTimestamp::from_unix_nanos(100),
                expires_at: Some(TrustTimestamp::from_unix_nanos(200)),
                payload: b"provider trust statement".to_vec(),
            }
        }

        fn signing_input(&self) -> TrustEvidenceSigningInput<'_> {
            TrustEvidenceSigningInput::new(
                &self.id,
                &self.subject,
                &self.issuer,
                &self.key,
                &self.scheme,
                self.issued_at,
                self.expires_at,
                &self.payload,
            )
        }

        fn evidence(&self, signature: Vec<u8>) -> TrustEvidence {
            TrustEvidence::new(
                self.id.clone(),
                self.subject.clone(),
                self.issuer.clone(),
                self.key.clone(),
                self.scheme.clone(),
                self.issued_at,
                self.expires_at,
                self.payload.clone(),
                signature,
            )
            .expect("test trust evidence must be structurally valid")
        }
    }

    fn read_all(
        mut reader: impl Read,
    ) -> Vec<u8> {
        let mut output = Vec::new();

        reader
            .read_to_end(&mut output)
            .expect("test reader must succeed");

        output
    }

    #[test]
    fn canonical_envelope_is_deterministic() {
        let fixture = EvidenceFixture::new();

        let first = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        let second = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        assert_eq!(first, second);
        assert!(first.starts_with(TRUST_ENVELOPE_DOMAIN));
        assert!(first.windows(4).any(|window| {
            window == TRUST_ENVELOPE_VERSION.to_be_bytes()
        }));
    }

    #[test]
    fn canonical_envelope_binds_evidence_metadata() {
        let fixture = EvidenceFixture::new();

        let original = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        let changed_issuer = TrustIssuerId::new("org.other.issuer")
            .expect("issuer identity must be valid");

        let changed = TrustEvidenceSigningInput::new(
            &fixture.id,
            &fixture.subject,
            &changed_issuer,
            &fixture.key,
            &fixture.scheme,
            fixture.issued_at,
            fixture.expires_at,
            &fixture.payload,
        );

        let changed = read_all(
            changed.reader().expect("reader must be valid"),
        );

        assert_ne!(original, changed);
    }

    #[test]
    fn streaming_reader_matches_write_to_output() {
        let fixture = EvidenceFixture::new();

        let mut written = Vec::new();

        fixture
            .signing_input()
            .write_to(&mut written)
            .expect("writing canonical envelope must succeed");

        let streamed = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        assert_eq!(written, streamed);
    }

    #[test]
    fn canonical_trust_verifier_accepts_valid_evidence_signature() {
        let fixture = EvidenceFixture::new();

        let expected_message = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        let provider = TestProvider::new(expected_message);
        let verifier = CanonicalTrustSignatureVerifier::new(&provider);

        let evidence = fixture.evidence(
            b"accepted-signature".to_vec(),
        );

        assert_eq!(
            SignatureVerifier::verify(&verifier, &evidence)
                .expect("verification operation must succeed"),
            SignatureValidity::Valid,
        );
    }

    #[test]
    fn invalid_trust_evidence_signature_is_rejected() {
        let fixture = EvidenceFixture::new();

        let expected_message = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        let provider = TestProvider::new(expected_message);
        let verifier = CanonicalTrustSignatureVerifier::new(&provider);

        let evidence = fixture.evidence(b"invalid-signature".to_vec());

        assert_eq!(
            SignatureVerifier::verify(&verifier, &evidence)
                .expect("invalid signature is a verification result"),
            SignatureValidity::Invalid,
        );
    }

    #[test]
    fn evidence_metadata_is_not_accepted_without_a_matching_signature() {
        let fixture = EvidenceFixture::new();

        let original_message = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        let provider = TestProvider::new(original_message);
        let verifier = CanonicalTrustSignatureVerifier::new(&provider);

        let changed_issuer = TrustIssuerId::new("org.changed.issuer")
            .expect("issuer identity must be valid");

        let changed_evidence = TrustEvidence::new(
            fixture.id.clone(),
            fixture.subject.clone(),
            changed_issuer,
            fixture.key.clone(),
            fixture.scheme.clone(),
            fixture.issued_at,
            fixture.expires_at,
            fixture.payload.clone(),
            b"accepted-signature".to_vec(),
        )
        .expect("changed evidence must be structurally valid");

        assert_eq!(
            SignatureVerifier::verify(&verifier, &changed_evidence)
                .expect("verification operation must complete"),
            SignatureValidity::Invalid,
        );
    }

    #[test]
    fn envelope_does_not_embed_or_expose_private_key_material() {
        let fixture = EvidenceFixture::new();

        let envelope = read_all(
            fixture
                .signing_input()
                .reader()
                .expect("reader must be valid"),
        );

        assert!(!envelope
            .windows(b"PRIVATE KEY".len())
            .any(|window| window == b"PRIVATE KEY"));
    }
}
