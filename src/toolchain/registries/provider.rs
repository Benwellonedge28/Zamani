
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Provider Contract
//!
//! Path: `src/toolchain/registries/provider.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the provider-neutral integration contract for package registries.
//!
//! Registry providers connect the Zamani toolchain to independently
//! implemented package distribution services.
//!
//! The contract supports:
//!
//! - Public registries.
//! - Private registries.
//! - Enterprise registries.
//! - Self-hosted registries.
//! - Federated registries.
//! - Local and remote registry implementations.
//! - Third-party registry implementations.
//! - Future registry technologies and protocols.
//!
//! No registry vendor, URL, network protocol, authentication mechanism,
//! package format, or computational domain is hard-coded.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Registry provider API contracts.
//! - Registry provider descriptors.
//! - Open-world operation identifiers.
//! - Provider-supported protocol declarations.
//! - Provider capability declarations.
//! - Registry operation request and response envelopes.
//! - Asynchronous provider execution boundaries.
//! - Provider request compatibility validation.
//! - Provider response identity validation.
//!
//! This module does not own:
//!
//! - Registry identity allocation.
//! - Registry configuration or endpoint selection.
//! - Registry discovery.
//! - Registry client orchestration.
//! - Network transport.
//! - HTTP clients or other transport implementations.
//! - Authentication or credential storage.
//! - Authorization policy.
//! - Package identity or dependency resolution.
//! - Package archive encoding or extraction.
//! - Package integrity verification.
//! - Cryptographic signatures.
//! - Publisher trust decisions.
//! - Package installation.
//! - Compiler execution.
//! - Backend selection.
//! - Hardware discovery.
//! - Runtime execution.
//!
//! Those responsibilities belong to their canonical subsystems.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Provider identity:
//!     crate::toolchain::ids::ProviderId
//!
//! Registry identity:
//!     crate::toolchain::ids::RegistryId
//!
//! Provider API version:
//!     crate::toolchain::version::ProviderApiVersion
//!
//! Registry protocol version:
//!     crate::toolchain::version::RegistryProtocolVersion
//!
//! Version compatibility:
//!     crate::toolchain::version::VersionRange
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Package identity:
//!     crate::toolchain::ids::PackageId
//!
//! Dependency model:
//!     crate::toolchain::dependencies::model
//!
//! Package verification:
//!     crate::toolchain::packages::verification
//!
//! Authentication and trust:
//!     Canonical toolchain security and authentication authorities.
//!
//! ## IMPORTANT ARCHITECTURAL RULE
//!
//! A registry provider is an implementation of a registry integration
//! contract. It is not the registry itself, a package, or a network client.
//!
//! A provider descriptor describes an implementation.
//!
//! A RegistryProvider trait object represents the executable implementation.
//!
//! A RegistryId identifies the configured registry instance.
//!
//! These identities must remain distinct.
//!
//! ## ASYNCHRONOUS EXECUTION
//!
//! Provider operations may involve remote services, local storage,
//! federated services, or other potentially long-running operations.
//!
//! This module uses standard-library futures rather than requiring a
//! particular asynchronous runtime.
//!
//! The caller owns runtime integration, concurrency policy, cancellation,
//! retry policy, deadlines, and operational budgets.
//!
//! ## POCO-REAF
//!
//! Registry providers must remain independent of the computational
//! domain of the package being distributed.
//!
//! A registry may distribute classical, quantum, HDL, AI, tensor,
//! hybrid, distributed, networking, cryptographic, embedded,
//! accelerator, or future-domain packages.
//!
//! No closed domain enumeration is maintained.
//!
//! No arbitrary maximum is imposed on:
//!
//! - Registry providers.
//! - Registry instances.
//! - Operations.
//! - Protocol versions.
//! - Supported domains.
//! - Provider capabilities.
//! - Package counts.
//!
//! Actual resource consumption remains subject to available resources
//! and caller-configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.

use std::collections::BTreeSet;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::{
    ProviderId,
    RegistryId,
};

use crate::toolchain::version::{
    ProviderApiVersion,
    RegistryProtocolVersion,
    VersionRange,
};

// ============================================================================
// CONSTANTS
// ============================================================================

const OPERATION_PROVIDER: &str = "registry.provider";
const OPERATION_PROVIDER_VALIDATE: &str = "registry.provider.validate";
const OPERATION_PROVIDER_EXECUTE: &str = "registry.provider.execute";

// ============================================================================
// ASYNCHRONOUS PROVIDER FUTURE
// ============================================================================

/// Standard asynchronous result returned by a registry provider.
///
/// The future is `Send` so provider operations can be coordinated by
/// multithreaded executors without requiring a specific runtime.
///
/// The lifetime is tied to the provider borrow, preventing the operation
/// from outliving the provider implementation.
pub type ProviderFuture<'a> =
    Pin<Box<dyn Future<Output = Result<RegistryResponse>> + Send + 'a>>;

// ============================================================================
// REGISTRY OPERATION IDENTIFIERS
// ============================================================================

/// A validated, open-world registry operation identifier.
///
/// Operation identifiers are namespaced strings rather than closed
/// enumerations. New registry operations can therefore be introduced
/// without modifying this module.
///
/// Examples of identifier shapes include:
///
/// - `packages.query`
/// - `packages.metadata`
/// - `packages.download`
/// - `packages.publish`
/// - `registries.discover`
/// - `extensions.custom_operation`
///
/// These are examples of identifier syntax, not a list of operations
/// that every provider must implement.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegistryOperationId(String);

impl RegistryOperationId {
    /// Construct a validated operation identifier.
    ///
    /// Identifiers must:
    ///
    /// - Be non-empty.
    /// - Begin and end with an ASCII alphanumeric character.
    /// - Contain only ASCII alphanumeric characters, `.`, `_`, `-`,
    ///   and `/`.
    /// - Not contain consecutive separators.
    ///
    /// No arbitrary maximum identifier length is imposed.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(invalid_operation(
                "registry operation identifier must not be empty",
            ));
        }

        if !value.is_ascii() {
            return Err(invalid_operation(
                "registry operation identifier must use ASCII characters",
            ));
        }

        let bytes = value.as_bytes();

        let is_alphanumeric = |byte: u8| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_uppercase()
                || byte.is_ascii_digit()
        };

        if !is_alphanumeric(bytes[0])
            || !is_alphanumeric(bytes[bytes.len() - 1])
        {
            return Err(invalid_operation(
                "registry operation identifier must begin and end \
                 with an alphanumeric character",
            ));
        }

        let mut previous_separator = false;

        for byte in bytes {
            let separator = matches!(*byte, b'.' | b'_' | b'-' | b'/');

            if !is_alphanumeric(*byte) && !separator {
                return Err(invalid_operation(
                    "registry operation identifier contains an invalid character",
                ));
            }

            if separator && previous_separator {
                return Err(invalid_operation(
                    "registry operation identifier contains \
                     consecutive separators",
                ));
            }

            previous_separator = separator;
        }

        Ok(Self(value))
    }

    /// Borrow the canonical operation identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RegistryOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PROVIDER DESCRIPTOR
// ============================================================================

/// Immutable description of one registry provider implementation.
///
/// A descriptor contains registration-time metadata.
///
/// It does not establish that the provider is:
///
/// - Initialized.
/// - Authenticated.
/// - Authorized.
/// - Trusted.
/// - Reachable.
/// - Available.
/// - Compatible with every registry.
/// - Capable of completing every operation.
///
/// Those conditions must be established by their respective authorities.
#[derive(Debug, Clone)]
pub struct RegistryProviderDescriptor {
    provider_id: ProviderId,
    display_name: String,
    api_version: ProviderApiVersion,
    supported_protocols: Vec<VersionRange<RegistryProtocolVersion>>,
    operations: BTreeSet<RegistryOperationId>,
    domains: BTreeSet<String>,
}

impl RegistryProviderDescriptor {
    /// Construct and validate a provider descriptor.
    ///
    /// `supported_protocols` must contain at least one valid range.
    ///
    /// `operations` must contain at least one supported operation.
    ///
    /// `domains` contains open-world domain identifiers. An empty set
    /// means that the provider does not declare domain-specific filtering.
    ///
    /// No domain enumeration is imposed.
    pub fn new(
        provider_id: ProviderId,
        display_name: impl Into<String>,
        api_version: ProviderApiVersion,
        supported_protocols: Vec<VersionRange<RegistryProtocolVersion>>,
        operations: BTreeSet<RegistryOperationId>,
        domains: BTreeSet<String>,
    ) -> Result<Self> {
        let descriptor = Self {
            provider_id,
            display_name: display_name.into(),
            api_version,
            supported_protocols,
            operations,
            domains,
        };

        descriptor.validate()?;

        Ok(descriptor)
    }

    /// Validate descriptor invariants.
    ///
    /// This method is also intended for validation at external
    /// registration and deserialization boundaries.
    pub fn validate(&self) -> Result<()> {
        if self.display_name.trim().is_empty() {
            return Err(ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "registry provider display name must not be empty",
            )
            .with_operation(OPERATION_PROVIDER_VALIDATE));
        }

        if self.supported_protocols.is_empty() {
            return Err(ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "registry provider must declare at least one supported protocol range",
            )
            .with_operation(OPERATION_PROVIDER_VALIDATE));
        }

        if self.operations.is_empty() {
            return Err(ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                "registry provider must declare at least one operation",
            )
            .with_operation(OPERATION_PROVIDER_VALIDATE));
        }

        for range in &self.supported_protocols {
            range.validate().map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::InvalidInput,
                    "registry provider contains an invalid protocol version range",
                )
                .with_operation(OPERATION_PROVIDER_VALIDATE)
                .with_source(error)
            })?;
        }

        for domain in &self.domains {
            validate_open_identifier(domain, "computational domain")?;
        }

        Ok(())
    }

    /// Return the provider's canonical identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Return the descriptive provider name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Return the provider API version.
    #[must_use]
    pub const fn api_version(&self) -> ProviderApiVersion {
        self.api_version
    }

    /// Return all supported registry protocol ranges.
    #[must_use]
    pub fn supported_protocols(
        &self,
    ) -> &[VersionRange<RegistryProtocolVersion>] {
        &self.supported_protocols
    }

    /// Return the declared operation identifiers.
    #[must_use]
    pub fn operations(&self) -> &BTreeSet<RegistryOperationId> {
        &self.operations
    }

    /// Return the declared computational domains.
    #[must_use]
    pub fn domains(&self) -> &BTreeSet<String> {
        &self.domains
    }

    /// Determine whether the provider declares support for a protocol.
    ///
    /// This establishes declared compatibility only. It does not
    /// establish successful communication or operational readiness.
    #[must_use]
    pub fn supports_protocol(
        &self,
        version: RegistryProtocolVersion,
    ) -> bool {
        self.supported_protocols
            .iter()
            .any(|range| range.contains(version))
    }

    /// Determine whether an operation is declared as supported.
    #[must_use]
    pub fn supports_operation(
        &self,
        operation: &RegistryOperationId,
    ) -> bool {
        self.operations.contains(operation)
    }

    /// Determine whether the provider declares a computational domain.
    ///
    /// An empty domain declaration means that the provider has not
    /// declared a domain-specific restriction. It does not prove
    /// universal semantic compatibility.
    #[must_use]
    pub fn declares_domain(&self, domain: &str) -> bool {
        self.domains.contains(domain)
    }

    /// Validate a request against this provider's declared contract.
    pub fn validate_request(
        &self,
        request: &RegistryRequest,
    ) -> Result<()> {
        self.validate()?;

        if !self.supports_protocol(request.protocol_version) {
            return Err(ToolchainError::new(
                ToolchainErrorKind::IncompatibleVersion,
                "registry provider does not declare support for the requested protocol version",
            )
            .with_operation(OPERATION_PROVIDER_VALIDATE)
            .with_subject(self.provider_id.to_string()));
        }

        if !self.supports_operation(&request.operation) {
            return Err(ToolchainError::new(
                ToolchainErrorKind::UnsupportedOperation,
                format!(
                    "registry provider does not declare operation '{}'",
                    request.operation
                ),
            )
            .with_operation(OPERATION_PROVIDER_VALIDATE)
            .with_subject(self.provider_id.to_string()));
        }

        Ok(())
    }
}

// ============================================================================
// REGISTRY REQUEST
// ============================================================================

/// Immutable provider operation request.
///
/// The payload is opaque to this module.
///
/// `protocol.rs` owns encoding, decoding, wire schemas and protocol-specific
/// validation.
///
/// This separation prevents the provider contract from depending on
/// a particular serialization format.
///
/// Credentials and authentication tokens must never be placed in this
/// request payload unless an independently authorized protocol explicitly
/// requires an appropriately protected credential reference.
///
/// Authentication belongs to the security and authentication boundary.
#[derive(Debug, Clone)]
pub struct RegistryRequest {
    registry_id: RegistryId,
    protocol_version: RegistryProtocolVersion,
    operation: RegistryOperationId,
    payload: Arc<[u8]>,
}

impl RegistryRequest {
    /// Construct a registry operation request.
    pub fn new(
        registry_id: RegistryId,
        protocol_version: RegistryProtocolVersion,
        operation: RegistryOperationId,
        payload: impl Into<Arc<[u8]>>,
    ) -> Self {
        Self {
            registry_id,
            protocol_version,
            operation,
            payload: payload.into(),
        }
    }

    /// Return the configured registry identity.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the requested protocol version.
    #[must_use]
    pub const fn protocol_version(&self) -> RegistryProtocolVersion {
        self.protocol_version
    }

    /// Return the requested operation.
    #[must_use]
    pub fn operation(&self) -> &RegistryOperationId {
        &self.operation
    }

    /// Borrow the opaque request payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

// ============================================================================
// REGISTRY RESPONSE
// ============================================================================

/// Immutable response returned by a registry provider.
///
/// The response preserves the registry identity and protocol version
/// used for the request.
///
/// The payload remains opaque until decoded and validated by the
/// authoritative protocol implementation.
#[derive(Debug, Clone)]
pub struct RegistryResponse {
    registry_id: RegistryId,
    protocol_version: RegistryProtocolVersion,
    payload: Arc<[u8]>,
}

impl RegistryResponse {
    /// Construct a provider response.
    pub fn new(
        registry_id: RegistryId,
        protocol_version: RegistryProtocolVersion,
        payload: impl Into<Arc<[u8]>>,
    ) -> Self {
        Self {
            registry_id,
            protocol_version,
            payload: payload.into(),
        }
    }

    /// Return the registry identity associated with the response.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the protocol version used by the response.
    #[must_use]
    pub const fn protocol_version(&self) -> RegistryProtocolVersion {
        self.protocol_version
    }

    /// Borrow the opaque response payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

// ============================================================================
// REGISTRY PROVIDER CONTRACT
// ============================================================================

/// Provider-neutral registry implementation contract.
///
/// Implementations may communicate with local storage, remote registries,
/// enterprise package services, federated registries or future systems.
///
/// The implementation must:
///
/// - Be safe to share between threads.
/// - Return its immutable validated descriptor.
/// - Implement only its declared operation contracts.
/// - Preserve registry identity.
/// - Return structured failures.
/// - Never report success without validating the actual operation result.
/// - Respect cancellation and operational policies supplied through its
///   integration layer.
/// - Avoid leaking credentials through errors or diagnostics.
///
/// This trait does not require a particular networking library,
/// asynchronous runtime, vendor SDK or authentication implementation.
///
/// Provider methods must not hold registry-management locks while
/// performing potentially blocking or asynchronous work.
pub trait RegistryProvider: Send + Sync {
    /// Return the immutable descriptor for this implementation.
    fn descriptor(&self) -> &RegistryProviderDescriptor;

    /// Execute one registry operation.
    ///
    /// The provider must validate the operation's protocol-specific
    /// payload and preserve the registry identity in its response.
    ///
    /// The caller is responsible for invoking
    /// [`execute_checked`] when it requires common descriptor and
    /// response-envelope validation.
    fn execute<'a>(
        &'a self,
        request: RegistryRequest,
    ) -> ProviderFuture<'a>;
}

// ============================================================================
// COMMON PROVIDER EXECUTION
// ============================================================================

/// Execute a provider operation with common contract validation.
///
/// This function:
///
/// 1. Validates the provider descriptor.
/// 2. Checks declared protocol compatibility.
/// 3. Checks declared operation support.
/// 4. Invokes the provider without holding a registry lock.
/// 5. Checks that the response belongs to the requested registry.
/// 6. Checks that the response uses the requested protocol version.
///
/// It does not authenticate the caller, authorize the operation,
/// verify package signatures, or validate protocol-specific payloads.
///
/// Those checks remain with their authoritative security and protocol
/// subsystems.
pub fn execute_checked<'a>(
    provider: &'a dyn RegistryProvider,
    request: RegistryRequest,
) -> ProviderFuture<'a> {
    Box::pin(async move {
        provider
            .descriptor()
            .validate_request(&request)?;

        let expected_registry = request.registry_id().clone();
        let expected_protocol = request.protocol_version();

        let response = provider.execute(request).await?;

        if response.registry_id() != &expected_registry {
            return Err(ToolchainError::new(
                ToolchainErrorKind::ProviderFailed,
                "registry provider returned a response for a different registry",
            )
            .with_operation(OPERATION_PROVIDER_EXECUTE)
            .with_subject(
                provider.descriptor().provider_id().to_string(),
            ));
        }

        if response.protocol_version() != expected_protocol {
            return Err(ToolchainError::new(
                ToolchainErrorKind::IncompatibleVersion,
                "registry provider returned a response using an unexpected protocol version",
            )
            .with_operation(OPERATION_PROVIDER_EXECUTE)
            .with_subject(
                provider.descriptor().provider_id().to_string(),
            ));
        }

        Ok(response)
    })
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn invalid_operation(message: impl Into<String>) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(OPERATION_PROVIDER)
}

/// Validate an open-world identifier without imposing a domain catalogue.
///
/// This is structural validation only. It does not establish that the
/// named domain exists or that a provider implements its semantics.
fn validate_open_identifier(
    value: &str,
    description: &str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(invalid_operation(format!(
            "{description} identifier must not be empty"
        )));
    }

    if !value.is_ascii() {
        return Err(invalid_operation(format!(
            "{description} identifier must use ASCII characters"
        )));
    }

    if value.bytes().any(|byte| {
        byte.is_ascii_control() || byte.is_ascii_whitespace()
    }) {
        return Err(invalid_operation(format!(
            "{description} identifier must not contain whitespace or control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_identifier_accepts_open_world_names() {
        let identifier =
            RegistryOperationId::new("extensions.future.operation");

        assert!(identifier.is_ok());

        assert_eq!(
            identifier.expect("validated identifier").as_str(),
            "extensions.future.operation"
        );
    }

    #[test]
    fn operation_identifier_accepts_slash_namespaces() {
        assert!(
            RegistryOperationId::new("custom/vendor.operation").is_ok()
        );
    }

    #[test]
    fn operation_identifier_rejects_empty_values() {
        assert!(RegistryOperationId::new("").is_err());
    }

    #[test]
    fn operation_identifier_rejects_whitespace() {
        assert!(
            RegistryOperationId::new("packages query").is_err()
        );
    }

    #[test]
    fn operation_identifier_rejects_non_ascii_values() {
        assert!(
            RegistryOperationId::new("packages.查询").is_err()
        );
    }

    #[test]
    fn operation_identifier_rejects_consecutive_separators() {
        assert!(
            RegistryOperationId::new("packages..query").is_err()
        );
    }

    #[test]
    fn operation_identifier_rejects_invalid_boundaries() {
        assert!(RegistryOperationId::new(".packages").is_err());
        assert!(RegistryOperationId::new("packages.").is_err());
        assert!(RegistryOperationId::new("_packages").is_err());
    }

    #[test]
    fn operation_identifiers_are_orderable() {
        let first =
            RegistryOperationId::new("packages.metadata")
                .expect("valid identifier");

        let second =
            RegistryOperationId::new("packages.publish")
                .expect("valid identifier");

        assert!(first < second);
    }

    #[test]
    fn open_identifier_validation_does_not_enumerate_domains() {
        assert!(
            validate_open_identifier(
                "future.quantum_accelerator",
                "computational domain"
            )
            .is_ok()
        );

        assert!(
            validate_open_identifier(
                "future.neuromorphic",
                "computational domain"
            )
            .is_ok()
        );
    }

    #[test]
    fn open_identifier_validation_rejects_control_characters() {
        assert!(
            validate_open_identifier(
                "domain\ninvalid",
                "computational domain"
            )
            .is_err()
        );
    }
}
