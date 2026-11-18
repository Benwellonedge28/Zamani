
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Publish Command
//!
//! Path: src/toolchain/commands/publish.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides the developer-facing command service for publishing
//! verified Zamani packages to configured registries.
//!
//! This is an orchestration adapter over the canonical registry
//! publication service. It does not implement another publication
//! protocol or package-management subsystem.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The publish-command request contract.
//! - Command-level input coordination.
//! - Invocation of the canonical publication service.
//! - Command-level error context.
//!
//! This module does not own:
//!
//! - Package creation or archive serialization.
//! - Package metadata decoding.
//! - Package integrity verification.
//! - Signature verification or publisher trust.
//! - Authentication or credential storage.
//! - Authorization policy implementation.
//! - Registry configuration or provider discovery.
//! - Network communication.
//! - Dependency resolution.
//! - Compiler or backend execution.
//! - Hardware discovery or runtime execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ## Canonical integrations
//!
//! Publication:
//!     crate::toolchain::registries::publishing
//!
//! Registry client:
//!     crate::toolchain::registries::client
//!
//! Registry identity:
//!     crate::toolchain::ids::RegistryId
//!
//! Registry protocol:
//!     crate::toolchain::version::RegistryProtocolVersion
//!
//! Package verification:
//!     crate::toolchain::packages::verification
//!
//! Package metadata:
//!     PackageManifestDecoder
//!
//! Authenticity and trust:
//!     PackageEvidenceVerifier
//!
//! Publication authorization:
//!     PublicationAuthorizer
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! ## Security
//!
//! Publication requires:
//!
//! - A configured registry.
//! - An explicit publication identity.
//! - A complete package archive.
//! - A canonical package metadata decoder.
//! - A mandatory authenticity and trust verifier.
//! - A mandatory publication authorizer.
//! - Explicit verification and operational options.
//!
//! Credentials must never be passed through this command request.
//! Credentials remain within their authentication providers.
//!
//! This command does not treat successful authentication as
//! authorization.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary limits on:
//!
//! - Package size.
//! - Package content entries.
//! - Package metadata entries.
//! - Computational domains.
//! - Registry instances.
//! - Hardware resources.
//! - Target count.
//! - Package version length.
//!
//! Archive processing is streamed by the canonical publication
//! implementation. Its configured chunk size controls operational
//! memory consumption rather than imposing a package-size ceiling.
//!
//! Actual operation remains subject to available resources,
//! provider capabilities, policies and physical constraints.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Publication lifecycle
//!
//! Prepared package archive
//!         |
//!         v
//! PublishRequest
//!         |
//!         v
//! Command validation by canonical constructors
//!         |
//!         v
//! RegistryClient
//!         |
//!         v
//! Canonical package verification
//!         |
//!         v
//! Authenticity and trust verification
//!         |
//!         v
//! Publication authorization
//!         |
//!         v
//! Streaming archive hashing
//!         |
//!         v
//! Chunked registry publication
//!         |
//!         v
//! Registry acknowledgement verification
//!         |
//!         v
//! PublicationReceipt
//!
//! ## Failure semantics
//!
//! This command never converts an error into success.
//!
//! It preserves the original ToolchainError classification and
//! adds command-level operation context.
//!
//! Publication mutations are not automatically retried.
//! An ambiguous registry commit must be resolved using the
//! registry's documented idempotency and receipt-query contract.
//!
//! ## Freeze criteria
//!
//! - Uses the canonical publication service.
//! - Requires explicit authorization.
//! - Requires complete archive verification.
//! - Does not expose credentials.
//! - Does not duplicate package or registry functionality.
//! - Preserves structured failures.
//! - Compiles with Rust 1.97 or later.
//! - Uses no unsafe Rust.

use std::io::{Read, Seek};

use crate::toolchain::errors::Result;

use crate::toolchain::ids::RegistryId;

use crate::toolchain::version::RegistryProtocolVersion;

use crate::toolchain::packages::verification::{
    PackageEvidenceVerifier,
    PackageManifestDecoder,
    VerificationOptions,
};

use crate::toolchain::registries::client::RegistryClient;

use crate::toolchain::registries::publishing::{
    self,
    PublicationAuthorizer,
    PublicationId,
    PublicationOptions,
    PublicationReceipt,
};

// ============================================================================
// COMMAND IDENTITY
// ============================================================================

/// Stable operation identifier for the publish command.
pub const OPERATION_PUBLISH: &str = "toolchain.command.publish";

// ============================================================================
// PUBLISH REQUEST
// ============================================================================

/// Complete request to publish an existing Zamani package archive.
///
/// This request represents developer intent and publication settings.
/// It does not contain credentials, registry provider implementations,
/// cryptographic keys or executable code.
///
/// All identities and operational options are supplied explicitly.
///
/// The request is immutable after construction.
pub struct PublishRequest {
    registry_id: RegistryId,
    protocol_version: RegistryProtocolVersion,
    publication_id: PublicationId,
    verification_options: VerificationOptions,
    publication_options: PublicationOptions,
}

impl PublishRequest {
    /// Construct a publish request.
    ///
    /// The supplied publication identity and operational options
    /// must already have passed their canonical constructors.
    ///
    /// Registry existence, provider compatibility, authorization,
    /// archive validity and trust are checked by their respective
    /// authoritative services during execution.
    #[must_use]
    pub fn new(
        registry_id: RegistryId,
        protocol_version: RegistryProtocolVersion,
        publication_id: PublicationId,
        verification_options: VerificationOptions,
        publication_options: PublicationOptions,
    ) -> Self {
        Self {
            registry_id,
            protocol_version,
            publication_id,
            verification_options,
            publication_options,
        }
    }

    /// Return the requested registry identity.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the logical publication identity.
    #[must_use]
    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }
}

// ============================================================================
// PUBLISH COMMAND
// ============================================================================

/// Execute a checked package publication.
///
/// The archive must contain a complete `.zpkg` package and must
/// implement both `Read` and `Seek`.
///
/// The archive is borrowed mutably because the canonical publishing
/// service performs multiple streaming passes over the same source.
///
/// The caller supplies the canonical metadata decoder, authenticity
/// verifier and authorization implementation. These are mandatory:
/// no permissive default security implementation is constructed.
///
/// # Type parameters
///
/// - `R`: Seekable package archive source.
/// - `D`: Canonical package manifest decoder.
/// - `E`: Package authenticity and trust verifier.
/// - `A`: Publication authorization authority.
///
/// # Arguments
///
/// - `request`: Explicit publication identity and operational policy.
/// - `client`: Configured canonical registry client.
/// - `archive`: Complete package archive.
/// - `decoder`: Canonical package metadata decoder.
/// - `evidence_verifier`: Authenticity and trust verification authority.
/// - `authorizer`: Publication authorization authority.
///
/// # Returns
///
/// A validated `PublicationReceipt` after the registry confirms
/// publication and its acknowledgement satisfies the canonical
/// publication protocol.
///
/// # Errors
///
/// Returns the original structured toolchain error, augmented
/// with command-level context, for any failed operation.
///
/// Errors include invalid archives, incomplete integrity verification,
/// untrusted authenticity evidence, authorization failures,
/// provider incompatibility, registry failures, malformed
/// acknowledgements, source mutation and resource exhaustion.
///
/// # Security
///
/// No credentials are accepted as arguments.
///
/// No package is published before the canonical verification
/// and authorization requirements have been satisfied.
///
/// # Retry behavior
///
/// This function does not automatically retry publication mutations.
/// The caller must not blindly repeat an ambiguous commit operation.
///
/// The same publication identity may be used for recovery only
/// according to the selected registry's documented idempotency
/// and recovery contract.
pub async fn execute<R, D, E, A>(
    request: PublishRequest,
    client: &RegistryClient,
    archive: &mut R,
    decoder: &mut D,
    evidence_verifier: &mut E,
    authorizer: &A,
) -> Result<PublicationReceipt>
where
    R: Read + Seek,
    D: PackageManifestDecoder,
    E: PackageEvidenceVerifier,
    A: PublicationAuthorizer,
{
    let PublishRequest {
        registry_id,
        protocol_version,
        publication_id,
        verification_options,
        publication_options,
    } = request;

    publishing::publish(
        client,
        registry_id,
        protocol_version,
        publication_id,
        archive,
        decoder,
        evidence_verifier,
        authorizer,
        verification_options,
        publication_options,
    )
    .await
    .map_err(|error| {
        error
            .with_operation(OPERATION_PUBLISH)
            .with_context(
                "executing the Zamani toolchain package publication command",
            )
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The command must expose its stable operation identifier.
    #[test]
    fn publish_operation_identifier_is_stable() {
        assert_eq!(
            OPERATION_PUBLISH,
            "toolchain.command.publish"
        );
    }

    /// Publication configuration must use the canonical options type.
    #[test]
    fn publication_options_reject_zero_chunk_size() {
        assert!(PublicationOptions::new(0).is_err());
    }

    /// Publication identities must be validated by their canonical type.
    #[test]
    fn publication_identity_rejects_empty_values() {
        assert!(PublicationId::new("").is_err());
    }

    /// Publication identities must reject control characters.
    #[test]
    fn publication_identity_rejects_control_characters() {
        assert!(PublicationId::new("release\ninvalid").is_err());
    }

    /// Valid publication identities must be preserved exactly.
    #[test]
    fn publication_identity_preserves_valid_values() {
        let identity = PublicationId::new(
            "zamani.release.2026.001"
        )
        .expect("valid publication identity");

        assert_eq!(
            identity.as_str(),
            "zamani.release.2026.001"
        );
    }

    /// Different logical publications can have independent identities.
    #[test]
    fn publication_identities_are_not_globally_hard_coded() {
        let first = PublicationId::new("release.alpha")
            .expect("valid identity");

        let second = PublicationId::new("release.beta")
            .expect("valid identity");

        assert_ne!(first, second);
    }
}
