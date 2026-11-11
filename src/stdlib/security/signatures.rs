
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Digital Signatures
//!
//! Path: `src/stdlib/security/signatures.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides portable, provider-independent signing and signature-verification
//! operations using the canonical contracts in [`super::cryptography`].
//!
//! This module does not implement cryptographic algorithms, store private
//! keys, resolve key references, select hardware, or schedule execution.
//!
//! ## Ownership
//!
//! - `cryptography.rs` owns provider and cryptographic operation contracts.
//! - `crypto/signature.rs` retains the existing Ed25519 implementation.
//! - Key-management providers own protected key resolution and lifecycle.
//! - This module owns the standard-library signing and verification facade.
//!
//! ## Security
//!
//! - Private key material is never accepted by this API.
//! - Signing and verification use separate operation identities.
//! - Provider capabilities are checked before execution.
//! - Returned metadata is checked against the request and provider.
//! - Invalid signatures are errors, not successful boolean results.
//! - Provider failures are propagated without exposing secret material.
//! - Signature bytes are streamed to caller-controlled output.
//!
//! Provider capability declarations are claims, not cryptographic
//! certification. Applications must use reviewed implementations.
//!
//! ## Scalability
//!
//! This API imposes no universal message-size, signature-count, key-count,
//! provider-count, algorithm-count, device-count or participant-count limit.
//! Message input and signature input are streamed.
//!
//! Actual limits are determined by the selected algorithm, provider,
//! available resources and any explicitly configured operational budgets.

use std::io::{Read, Write};

use super::cryptography::{
    validate_provider,
    CryptoAlgorithmId,
    CryptoError,
    CryptoErrorKind,
    CryptoOperationId,
    CryptoProvider,
    CryptoRequest,
    CryptoResult,
    SignatureMetadata,
};

/// Canonical operation identity for digital signing.
///
/// The identifier is open to future algorithm and provider extensions.
/// It identifies the operation, not a particular signature algorithm.
pub const SIGN_OPERATION_ID: &str = "cryptography::signature::sign";

/// Canonical operation identity for signature verification.
pub const VERIFY_OPERATION_ID: &str = "cryptography::signature::verify";

/// Returns the canonical signing operation identifier.
pub fn sign_operation() -> CryptoResult<CryptoOperationId> {
    CryptoOperationId::new(SIGN_OPERATION_ID)
}

/// Returns the canonical verification operation identifier.
pub fn verify_operation() -> CryptoResult<CryptoOperationId> {
    CryptoOperationId::new(VERIFY_OPERATION_ID)
}

/// Validates that a request represents the expected signature operation.
///
/// The request must contain a protected key reference. Raw key material
/// is deliberately not accepted by this API.
fn validate_request(
    request: &CryptoRequest,
    expected_operation: &CryptoOperationId,
) -> CryptoResult<()> {
    if request.operation() != expected_operation {
        return Err(
            CryptoError::new(CryptoErrorKind::InvalidRequest)
                .with_operation(request.operation().clone()),
        );
    }

    if request.key().is_none() {
        return Err(
            CryptoError::new(CryptoErrorKind::InvalidKeyReference)
                .with_operation(request.operation().clone()),
        );
    }

    Ok(())
}

/// Validates metadata returned by a successful signing operation.
///
/// The provider must report the same algorithm and key reference that
/// were requested, and its own provider identity.
///
/// Metadata is descriptive. It does not independently prove that the
/// provider performed a secure signing operation.
fn validate_signing_metadata(
    provider: &dyn CryptoProvider,
    request: &CryptoRequest,
    metadata: &SignatureMetadata,
) -> CryptoResult<()> {
    let requested_key = request.key().ok_or_else(|| {
        CryptoError::new(CryptoErrorKind::InvalidKeyReference)
            .with_operation(request.operation().clone())
            .with_provider(provider.id().clone())
    })?;

    if metadata.algorithm() != request.algorithm()
        || metadata.key() != requested_key
        || metadata.provider() != provider.id()
    {
        return Err(
            CryptoError::new(CryptoErrorKind::InvalidProviderResult)
                .with_operation(request.operation().clone())
                .with_provider(provider.id().clone()),
        );
    }

    Ok(())
}

/// Validates metadata before a verification operation.
///
/// This prevents metadata for a different algorithm, key or provider
/// from being silently used for the requested verification.
fn validate_verification_metadata(
    provider: &dyn CryptoProvider,
    request: &CryptoRequest,
    metadata: &SignatureMetadata,
) -> CryptoResult<()> {
    let requested_key = request.key().ok_or_else(|| {
        CryptoError::new(CryptoErrorKind::InvalidKeyReference)
            .with_operation(request.operation().clone())
            .with_provider(provider.id().clone())
    })?;

    if metadata.algorithm() != request.algorithm()
        || metadata.key() != requested_key
        || metadata.provider() != provider.id()
    {
        return Err(
            CryptoError::new(CryptoErrorKind::InvalidMetadata)
                .with_operation(request.operation().clone())
                .with_provider(provider.id().clone()),
        );
    }

    Ok(())
}

/// Adds operation and provider context to a provider error.
///
/// The original error's secret-bearing source is deliberately not copied
/// into a new diagnostic. The canonical error classification is retained.
fn contextualize_error(
    error: CryptoError,
    provider: &dyn CryptoProvider,
    operation: &CryptoOperationId,
) -> CryptoError {
    CryptoError::new(error.kind())
        .with_operation(operation.clone())
        .with_provider(provider.id().clone())
}

/// Signs a streaming message using a registered cryptographic provider.
///
/// # Arguments
///
/// - `provider`: The selected implementation, supplied by the caller.
/// - `request`: A validated signing request containing an algorithm and
///   protected signing-key reference.
/// - `message`: A streaming source of message bytes.
/// - `signature_output`: Destination for the resulting signature bytes.
///
/// # Returns
///
/// Metadata describing the completed signature.
///
/// # Errors
///
/// Returns an error when:
///
/// - The request is not a signing request.
/// - No protected key reference is supplied.
/// - The provider does not declare the requested capability.
/// - A mandatory requirement or constraint is not satisfied.
/// - The provider fails.
/// - The provider returns inconsistent metadata.
/// - Writing the signature output fails.
///
/// # Output behavior
///
/// A generic `Write` destination cannot guarantee rollback. If signing or
/// writing fails, the destination may contain partial output. Callers
/// requiring atomic publication should supply a transactional or
/// temporary destination and commit it only after success.
///
/// # Security
///
/// The provider is responsible for authenticating its key access,
/// validating algorithm parameters, protecting private key material,
/// and performing genuine cryptographic signing.
///
/// This function does not claim that a provider is secure merely because
/// it implements [`CryptoProvider`].
pub fn sign_to(
    provider: &dyn CryptoProvider,
    request: &CryptoRequest,
    message: &mut dyn Read,
    signature_output: &mut dyn Write,
) -> CryptoResult<SignatureMetadata> {
    let operation = sign_operation()?;

    validate_request(request, &operation)?;

    validate_provider(provider, request).map_err(|error| {
        contextualize_error(error, provider, request.operation())
    })?;

    let metadata = provider
        .sign(request, message, signature_output)
        .map_err(|error| {
            contextualize_error(error, provider, request.operation())
        })?;

    validate_signing_metadata(provider, request, &metadata)?;

    Ok(metadata)
}

/// Verifies a streaming message against a supplied signature.
///
/// # Arguments
///
/// - `provider`: The selected verification implementation.
/// - `request`: A verification request containing the algorithm and
///   protected verification-key reference.
/// - `metadata`: Metadata returned by the corresponding signing operation
///   or obtained from a trusted signature-metadata source.
/// - `message`: The original message as a streaming input.
/// - `signature`: The signature bytes as a streaming input.
///
/// # Returns
///
/// `Ok(())` only when the provider reports successful verification.
///
/// # Errors
///
/// Returns an error when:
///
/// - The request is not a verification request.
/// - No protected verification-key reference is supplied.
/// - Metadata does not match the request and provider.
/// - The provider does not declare the requested capability.
/// - A mandatory requirement or constraint is not satisfied.
/// - The signature is invalid.
/// - The provider or an input stream fails.
///
/// # Security
///
/// Invalid signatures must produce `CryptoErrorKind::SignatureVerificationFailed`.
/// They must never be converted into successful verification.
///
/// The provider must implement the actual verification algorithm.
pub fn verify(
    provider: &dyn CryptoProvider,
    request: &CryptoRequest,
    metadata: &SignatureMetadata,
    message: &mut dyn Read,
    signature: &mut dyn Read,
) -> CryptoResult<()> {
    let operation = verify_operation()?;

    validate_request(request, &operation)?;

    validate_verification_metadata(provider, request, metadata)?;

    validate_provider(provider, request).map_err(|error| {
        contextualize_error(error, provider, request.operation())
    })?;

    provider
        .verify(request, metadata, message, signature)
        .map_err(|error| {
            contextualize_error(error, provider, request.operation())
        })
}

/// Checks whether a request identifies the canonical signing operation.
///
/// This is a structural check only. It does not establish provider
/// availability, algorithm security or operational feasibility.
pub fn is_signing_request(request: &CryptoRequest) -> CryptoResult<bool> {
    Ok(request.operation() == &sign_operation()?)
}

/// Checks whether a request identifies the canonical verification operation.
///
/// This is a structural check only. It does not establish provider
/// availability, algorithm security or operational feasibility.
pub fn is_verification_request(request: &CryptoRequest) -> CryptoResult<bool> {
    Ok(request.operation() == &verify_operation()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{Cursor, Read, Write};

    use super::super::cryptography::{
        CryptoAlgorithmId,
        CryptoCapability,
        CryptoProviderId,
        CryptoRequirement,
        KeyReference,
    };

    const TEST_ALGORITHM: &str = "cryptography::signature::test_scheme";
    const TEST_PROVIDER: &str = "zamani::test::signature_provider";

    struct TestProvider {
        id: CryptoProviderId,
        capabilities: Vec<CryptoCapability>,
    }

    impl TestProvider {
        fn new() -> Self {
            let id = CryptoProviderId::new(TEST_PROVIDER).unwrap();

            let algorithm = CryptoAlgorithmId::new(TEST_ALGORITHM).unwrap();

            let sign = CryptoCapability::new(sign_operation().unwrap())
                .with_algorithm(algorithm.clone());

            let verify = CryptoCapability::new(verify_operation().unwrap())
                .with_algorithm(algorithm);

            Self {
                id,
                capabilities: vec![sign, verify],
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

        fn sign(
            &self,
            request: &CryptoRequest,
            message: &mut dyn Read,
            signature: &mut dyn Write,
        ) -> CryptoResult<SignatureMetadata> {
            let mut bytes = Vec::new();

            message.read_to_end(&mut bytes).map_err(|error| {
                CryptoError::new(CryptoErrorKind::InputFailure)
                    .with_operation(request.operation().clone())
                    .with_provider(self.id.clone())
                    .with_source(error)
            })?;

            // Test-only deterministic output. This is NOT a cryptographic
            // signature algorithm and is never exported as a real provider.
            signature.write_all(b"test-signature").map_err(|error| {
                CryptoError::new(CryptoErrorKind::OutputFailure)
                    .with_operation(request.operation().clone())
                    .with_provider(self.id.clone())
                    .with_source(error)
            })?;

            let key = request.key().cloned().ok_or_else(|| {
                CryptoError::new(CryptoErrorKind::InvalidKeyReference)
            })?;

            Ok(SignatureMetadata::new(
                request.algorithm().clone(),
                key,
                self.id.clone(),
            ))
        }

        fn verify(
            &self,
            request: &CryptoRequest,
            metadata: &SignatureMetadata,
            message: &mut dyn Read,
            signature: &mut dyn Read,
        ) -> CryptoResult<()> {
            let mut message_bytes = Vec::new();
            let mut signature_bytes = Vec::new();

            message.read_to_end(&mut message_bytes).map_err(|error| {
                CryptoError::new(CryptoErrorKind::InputFailure)
                    .with_operation(request.operation().clone())
                    .with_provider(self.id.clone())
                    .with_source(error)
            })?;

            signature.read_to_end(&mut signature_bytes).map_err(|error| {
                CryptoError::new(CryptoErrorKind::InputFailure)
                    .with_operation(request.operation().clone())
                    .with_provider(self.id.clone())
                    .with_source(error)
            })?;

            if signature_bytes != b"test-signature" {
                return Err(
                    CryptoError::new(
                        CryptoErrorKind::SignatureVerificationFailed,
                    )
                    .with_operation(request.operation().clone())
                    .with_provider(self.id.clone()),
                );
            }

            if metadata.algorithm() != request.algorithm() {
                return Err(
                    CryptoError::new(CryptoErrorKind::InvalidMetadata)
                );
            }

            let _ = message_bytes;

            Ok(())
        }
    }

    fn request(operation: CryptoOperationId) -> CryptoRequest {
        let algorithm = CryptoAlgorithmId::new(TEST_ALGORITHM).unwrap();

        let requirement = CryptoRequirement::new(operation.clone())
            .requiring_algorithm(algorithm.clone());

        CryptoRequest::new(operation, algorithm, requirement)
            .unwrap()
            .using_key(
                KeyReference::new("test::managed::key").unwrap(),
            )
    }

    #[test]
    fn operation_identifiers_are_open_world_and_distinct() {
        assert_ne!(sign_operation().unwrap(), verify_operation().unwrap());

        assert_eq!(
            sign_operation().unwrap().as_str(),
            SIGN_OPERATION_ID,
        );

        assert_eq!(
            verify_operation().unwrap().as_str(),
            VERIFY_OPERATION_ID,
        );
    }

    #[test]
    fn signing_requires_a_sign_request() {
        let provider = TestProvider::new();

        let request = request(verify_operation().unwrap());

        let mut message = Cursor::new(b"message".to_vec());
        let mut output = Vec::new();

        let result = sign_to(
            &provider,
            &request,
            &mut message,
            &mut output,
        );

        assert!(result.is_err());
        assert!(output.is_empty());
    }

    #[test]
    fn verification_requires_a_verify_request() {
        let provider = TestProvider::new();

        let request = request(sign_operation().unwrap());

        let metadata = SignatureMetadata::new(
            request.algorithm().clone(),
            request.key().unwrap().clone(),
            provider.id().clone(),
        );

        let mut message = Cursor::new(b"message".to_vec());
        let mut signature = Cursor::new(b"test-signature".to_vec());

        let result = verify(
            &provider,
            &request,
            &metadata,
            &mut message,
            &mut signature,
        );

        assert!(result.is_err());
    }

    #[test]
    fn signing_and_verification_use_the_provider_contract() {
        let provider = TestProvider::new();

        let signing_request = request(sign_operation().unwrap());

        let mut message = Cursor::new(b"message".to_vec());
        let mut signature_bytes = Vec::new();

        let metadata = sign_to(
            &provider,
            &signing_request,
            &mut message,
            &mut signature_bytes,
        )
        .unwrap();

        assert_eq!(signature_bytes, b"test-signature");

        let verification_request = request(verify_operation().unwrap());

        let mut message = Cursor::new(b"message".to_vec());
        let mut signature = Cursor::new(signature_bytes);

        verify(
            &provider,
            &verification_request,
            &metadata,
            &mut message,
            &mut signature,
        )
        .unwrap();
    }

    #[test]
    fn invalid_signature_is_not_successful() {
        let provider = TestProvider::new();

        let request = request(verify_operation().unwrap());

        let metadata = SignatureMetadata::new(
            request.algorithm().clone(),
            request.key().unwrap().clone(),
            provider.id().clone(),
        );

        let mut message = Cursor::new(b"message".to_vec());
        let mut signature = Cursor::new(b"invalid".to_vec());

        let error = verify(
            &provider,
            &request,
            &metadata,
            &mut message,
            &mut signature,
        )
        .unwrap_err();

        assert_eq!(
            error.kind(),
            CryptoErrorKind::SignatureVerificationFailed,
        );
    }

    #[test]
    fn signing_request_is_detected() {
        let request = request(sign_operation().unwrap());

        assert!(is_signing_request(&request).unwrap());
        assert!(!is_verification_request(&request).unwrap());
    }

    #[test]
    fn verification_request_is_detected() {
        let request = request(verify_operation().unwrap());

        assert!(is_verification_request(&request).unwrap());
        assert!(!is_signing_request(&request).unwrap());
    }

    #[test]
    fn metadata_from_another_provider_is_rejected() {
        let provider = TestProvider::new();

        let request = request(verify_operation().unwrap());

        let other_provider = CryptoProviderId::new(
            "another::signature_provider",
        )
        .unwrap();

        let metadata = SignatureMetadata::new(
            request.algorithm().clone(),
            request.key().unwrap().clone(),
            other_provider,
        );

        let mut message = Cursor::new(b"message".to_vec());
        let mut signature = Cursor::new(b"test-signature".to_vec());

        let error = verify(
            &provider,
            &request,
            &metadata,
            &mut message,
            &mut signature,
        )
        .unwrap_err();

        assert_eq!(error.kind(), CryptoErrorKind::InvalidMetadata);
    }

    #[test]
    fn metadata_for_another_key_is_rejected() {
        let provider = TestProvider::new();

        let request = request(verify_operation().unwrap());

        let other_key = KeyReference::new("test::different::key").unwrap();

        let metadata = SignatureMetadata::new(
            request.algorithm().clone(),
            other_key,
            provider.id().clone(),
        );

        let mut message = Cursor::new(b"message".to_vec());
        let mut signature = Cursor::new(b"test-signature".to_vec());

        let error = verify(
            &provider,
            &request,
            &metadata,
            &mut message,
            &mut signature,
        )
        .unwrap_err();

        assert_eq!(error.kind(), CryptoErrorKind::InvalidMetadata);
    }
}
