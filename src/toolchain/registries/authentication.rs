#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Authentication
//!
//! Path: `src/toolchain/registries/authentication.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines an extensible, provider-neutral authentication contract for
//! configured Zamani package registries.
//!
//! The contract supports authentication implementations supplied by
//! Zamani, third parties, private deployments and future providers.
//! It does not prescribe a particular authentication protocol, credential
//! format, identity service, transport, vendor or hosting platform.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Authentication scheme identifiers.
//! - Credential references.
//! - Authentication challenges.
//! - Authentication request contracts.
//! - Authentication provider descriptors.
//! - Authentication provider execution contracts.
//! - Authenticated session metadata and handles.
//! - Common validation of authentication results.
//!
//! This module does not own:
//!
//! - Credential storage or secret persistence.
//! - Password, token or private-key storage.
//! - Cryptographic primitives.
//! - Cryptographic signature verification.
//! - Authorization policy or access-control decisions.
//! - Registry configuration or lifecycle.
//! - Registry provider execution.
//! - Network transport.
//! - HTTP, TLS, SSH or other protocol implementations.
//! - Package integrity or publisher trust.
//! - Dependency resolution or package installation.
//!
//! Those responsibilities remain with their canonical subsystems or
//! explicitly registered implementations.
//!
//! ## Authentication and authorization
//!
//! Authentication answers:
//!
//!     Who or what has authenticated?
//!
//! Authorization answers:
//!
//!     Is that identity permitted to perform this operation?
//!
//! Successful authentication MUST NOT be interpreted as authorization.
//! The caller MUST apply the relevant authorization policy before
//! dispatching a registry operation.
//!
//! ## Secret-handling contract
//!
//! Authentication requests carry credential references, not raw
//! passwords, tokens or private keys.
//!
//! Implementations MUST obtain secret material through an appropriately
//! protected credential authority.
//!
//! Implementations MUST NOT:
//!
//! - Include credentials in error messages.
//! - Include credentials in Debug output.
//! - Serialize credentials into registry request payloads.
//! - Store credentials in registry configuration.
//! - Log authentication headers or secret-bearing challenge responses.
//! - Treat a credential reference as proof of authentication.
//!
//! A credential reference is an opaque locator, not a secret and not
//! proof that the referenced credential exists.
//!
//! Concrete authentication providers own the handling of secret material.
//! They should avoid retaining secrets longer than required and should
//! use platform-appropriate secure storage and memory-handling facilities.
//!
//! ## Session contract
//!
//! An authenticated session is an opaque handle owned by the
//! authentication implementation.
//!
//! The session handle allows an implementation to retain transport- or
//! protocol-specific authentication state without forcing this module
//! to depend on a particular transport.
//!
//! A session is not automatically a reusable credential, an authorization
//! grant, or proof that a later operation is permitted.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Authentication providers.
//! - Authentication schemes.
//! - Registry instances.
//! - Credential references.
//! - Authentication sessions.
//! - Computational domains.
//!
//! Actual memory, storage, network and execution consumption remains
//! subject to available resources and caller-configured operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Integration
//!
//! - `ids.rs` supplies `ProviderId` and `RegistryId`.
//! - `provider.rs` supplies `RegistryOperationId`.
//! - `registry.rs` supplies configured registry identity and lifecycle.
//! - `client.rs` coordinates registry operations.
//! - `security/` owns authorization, trust and security policy.
//! - Concrete registry providers integrate their transport with the
//!   authenticated session supplied by their authentication implementation.
//!
//! This module does not modify the provider request or response wire format.
//!
//! ## Asynchronous execution
//!
//! The contract uses standard-library futures and does not require a
//! particular asynchronous runtime.
//!
//! The caller owns executor integration, cancellation, deadlines,
//! retry policy and operational budgets.
//!
//! Authentication implementations MUST NOT hold shared registry-catalog
//! or provider-catalog locks while performing authentication work.

use std::collections::BTreeSet;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::{
    ProviderId,
    RegistryId,
};

use super::provider::RegistryOperationId;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_AUTHENTICATION: &str = "registry.authentication";
const OPERATION_VALIDATE: &str = "registry.authentication.validate";
const OPERATION_EXECUTE: &str = "registry.authentication.execute";

// ============================================================================
// ASYNCHRONOUS PROVIDER FUTURE
// ============================================================================

/// Future returned by an authentication provider.
///
/// The future is `Send` so authentication can be coordinated by
/// multithreaded executors without requiring a particular runtime.
///
/// The returned session is an opaque handle. Its concrete implementation
/// remains owned by the authentication provider.
pub type AuthenticationFuture<'a> = Pin<
    Box<
        dyn Future<Output = Result<AuthenticatedSession>> + Send + 'a
    >
>;

// ============================================================================
// AUTHENTICATION SCHEME IDENTIFIER
// ============================================================================

/// Open-world authentication scheme identifier.
///
/// Scheme identifiers are namespaced strings rather than a closed
/// enumeration. New schemes can therefore be introduced without modifying
/// this module.
///
/// Examples of identifier shapes include:
///
/// - `password`
— `token.bearer`
/// - `certificate.client`
/// - `key.signature`
/// - `enterprise.identity`
/// - `custom.vendor.scheme`
///
/// These are identifier examples, not built-in implementations.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthenticationSchemeId(Arc<str>);

impl AuthenticationSchemeId {
    /// Create a validated authentication scheme identifier.
    ///
    /// The identifier must:
    ///
    /// - Be non-empty.
    /// - Begin and end with an ASCII alphanumeric character.
    /// - Contain only ASCII alphanumeric characters, `.`, `_`, `-` or `/`.
    /// - Not contain consecutive separators.
    ///
    /// No arbitrary maximum identifier length is imposed.
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "authentication scheme identifier must not be empty",
            ));
        }

        if !value.is_ascii() {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "authentication scheme identifier must use ASCII characters",
            ));
        }

        let bytes = value.as_bytes();

        let is_alphanumeric = |byte: u8| {
            byte.is_ascii_alphanumeric()
        };

        if !is_alphanumeric(bytes[0])
            || !is_alphanumeric(bytes[bytes.len() - 1])
        {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "authentication scheme identifier must begin and end \
                 with an ASCII alphanumeric character",
            ));
        }

        let mut previous_separator = false;

        for byte in bytes {
            let separator = matches!(*byte, b'.' | b'_' | b'-' | b'/');

            if !is_alphanumeric(*byte) && !separator {
                return Err(authentication_error(
                    ToolchainErrorKind::InvalidInput,
                    "authentication scheme identifier contains \
                     an invalid character",
                ));
            }

            if separator && previous_separator {
                return Err(authentication_error(
                    ToolchainErrorKind::InvalidInput,
                    "authentication scheme identifier contains \
                     consecutive separators",
                ));
            }

            previous_separator = separator;
        }

        Ok(Self(Arc::from(value)))
    }

    /// Borrow the canonical scheme identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuthenticationSchemeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Debug for AuthenticationSchemeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AuthenticationSchemeId")
            .field(&self.as_str())
            .finish()
    }
}

// ============================================================================
// CREDENTIAL REFERENCE
// ============================================================================

/// Opaque reference to a credential managed by a credential authority.
///
/// This value is a locator, not the credential itself. Its interpretation
/// belongs to the selected authentication provider or credential authority.
///
/// A reference must not contain raw passwords, access tokens, private keys
/// or other secret material.
///
/// Its Debug representation is redacted because reference names can
/// themselves reveal sensitive account or infrastructure information.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CredentialReference(Arc<str>);

impl CredentialReference {
    /// Create a credential reference.
    ///
    /// The value must be non-empty and contain no control characters.
    /// No arbitrary maximum length is imposed.
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref();

        if value.trim().is_empty() {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "credential reference must not be empty",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "credential reference must not contain control characters",
            ));
        }

        Ok(Self(Arc::from(value)))
    }

    /// Borrow the opaque credential reference.
    ///
    /// Callers should avoid writing this value to logs or diagnostics.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for CredentialReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialReference")
            .field(&"[REDACTED]")
            .finish()
    }
}

// ============================================================================
// AUTHENTICATION CHALLENGE
// ============================================================================

/// Opaque challenge supplied to an authentication provider.
///
/// Challenge contents are protocol-specific. This type does not interpret
/// challenge bytes or assume a particular authentication exchange.
///
/// Challenge data is not automatically trusted. Providers must validate
/// it according to their authentication protocol.
#[derive(Clone, PartialEq, Eq)]
pub struct AuthenticationChallenge {
    scheme: AuthenticationSchemeId,
    payload: Arc<[u8]>,
}

impl AuthenticationChallenge {
    /// Construct an authentication challenge.
    pub fn new(
        scheme: AuthenticationSchemeId,
        payload: impl Into<Arc<[u8]>>,
    ) -> Self {
        Self {
            scheme,
            payload: payload.into(),
        }
    }

    /// Return the authentication scheme associated with this challenge.
    #[must_use]
    pub fn scheme(&self) -> &AuthenticationSchemeId {
        &self.scheme
    }

    /// Borrow the opaque challenge payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl fmt::Debug for AuthenticationChallenge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticationChallenge")
            .field("scheme", &self.scheme)
            .field("payload", &"[REDACTED]")
            .finish()
    }
}

// ============================================================================
// AUTHENTICATION REQUEST
// ============================================================================

/// Request to authenticate with a configured registry.
///
/// The request identifies the registry, authentication scheme,
/// credential reference and intended registry operation.
///
/// It does not contain raw credentials.
///
/// Authentication providers MUST verify that the requested scheme is
/// supported and that the credential reference is appropriate for the
/// requested registry and audience.
#[derive(Clone)]
pub struct AuthenticationRequest {
    registry_id: RegistryId,
    scheme: AuthenticationSchemeId,
    credential: CredentialReference,
    operation: RegistryOperationId,
    audience: Option<Arc<str>>,
    challenge: Option<AuthenticationChallenge>,
}

impl AuthenticationRequest {
    /// Construct an authentication request.
    pub fn new(
        registry_id: RegistryId,
        scheme: AuthenticationSchemeId,
        credential: CredentialReference,
        operation: RegistryOperationId,
    ) -> Self {
        Self {
            registry_id,
            scheme,
            credential,
            operation,
            audience: None,
            challenge: None,
        }
    }

    /// Set the intended authentication audience.
    ///
    /// Audience interpretation belongs to the authentication provider.
    /// An audience is descriptive protocol context, not authorization.
    pub fn with_audience(
        mut self,
        audience: impl AsRef<str>,
    ) -> Result<Self> {
        let audience = audience.as_ref();

        validate_nonempty_text(audience, "authentication audience")?;

        self.audience = Some(Arc::from(audience));

        Ok(self)
    }

    /// Attach an authentication challenge.
    ///
    /// The challenge scheme must match the request scheme.
    pub fn with_challenge(
        mut self,
        challenge: AuthenticationChallenge,
    ) -> Result<Self> {
        if challenge.scheme() != &self.scheme {
            return Err(authentication_error(
                ToolchainErrorKind::ValidationFailed,
                "authentication challenge scheme does not match \
                 the requested authentication scheme",
            ));
        }

        self.challenge = Some(challenge);

        Ok(self)
    }

    /// Return the target registry identity.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the requested authentication scheme.
    #[must_use]
    pub fn scheme(&self) -> &AuthenticationSchemeId {
        &self.scheme
    }

    /// Return the opaque credential reference.
    #[must_use]
    pub fn credential_reference(&self) -> &CredentialReference {
        &self.credential
    }

    /// Return the intended registry operation.
    #[must_use]
    pub fn operation(&self) -> &RegistryOperationId {
        &self.operation
    }

    /// Return the optional audience.
    #[must_use]
    pub fn audience(&self) -> Option<&str> {
        self.audience.as_deref()
    }

    /// Return the optional authentication challenge.
    #[must_use]
    pub fn challenge(&self) -> Option<&AuthenticationChallenge> {
        self.challenge.as_ref()
    }

    /// Validate request invariants.
    pub fn validate(&self) -> Result<()> {
        if let Some(challenge) = &self.challenge {
            if challenge.scheme() != &self.scheme {
                return Err(authentication_error(
                    ToolchainErrorKind::ValidationFailed,
                    "authentication challenge scheme does not match \
                     the requested authentication scheme",
                ));
            }
        }

        if let Some(audience) = &self.audience {
            validate_nonempty_text(audience, "authentication audience")?;
        }

        Ok(())
    }
}

impl fmt::Debug for AuthenticationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticationRequest")
            .field("registry_id", &self.registry_id)
            .field("scheme", &self.scheme)
            .field("credential", &"[REDACTED]")
            .field("operation", &self.operation)
            .field("audience", &self.audience)
            .field("challenge", &self.challenge)
            .finish()
    }
}

// ============================================================================
// AUTHENTICATION PROVIDER DESCRIPTOR
// ============================================================================

/// Immutable description of an authentication provider.
///
/// The descriptor declares which authentication schemes an implementation
/// supports. It does not establish that credentials are available, that
/// authentication will succeed, or that an authenticated identity is
/// trusted or authorized.
#[derive(Debug, Clone)]
pub struct AuthenticationProviderDescriptor {
    provider_id: ProviderId,
    display_name: String,
    schemes: BTreeSet<AuthenticationSchemeId>,
}

impl AuthenticationProviderDescriptor {
    /// Construct and validate an authentication provider descriptor.
    ///
    /// At least one authentication scheme must be declared.
    pub fn new(
        provider_id: ProviderId,
        display_name: impl Into<String>,
        schemes: BTreeSet<AuthenticationSchemeId>,
    ) -> Result<Self> {
        let descriptor = Self {
            provider_id,
            display_name: display_name.into(),
            schemes,
        };

        descriptor.validate()?;

        Ok(descriptor)
    }

    /// Validate descriptor invariants.
    pub fn validate(&self) -> Result<()> {
        if self.display_name.trim().is_empty() {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "authentication provider display name must not be empty",
            ));
        }

        if self.schemes.is_empty() {
            return Err(authentication_error(
                ToolchainErrorKind::InvalidInput,
                "authentication provider must declare at least one scheme",
            ));
        }

        Ok(())
    }

    /// Return the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Return the descriptive provider name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Return all declared authentication schemes.
    #[must_use]
    pub fn schemes(&self) -> &BTreeSet<AuthenticationSchemeId> {
        &self.schemes
    }

    /// Determine whether the provider declares a scheme.
    ///
    /// This establishes declared support only. It does not establish
    /// operational readiness or successful authentication.
    #[must_use]
    pub fn supports_scheme(
        &self,
        scheme: &AuthenticationSchemeId,
    ) -> bool {
        self.schemes.contains(scheme)
    }

    /// Validate an authentication request against this descriptor.
    pub fn validate_request(
        &self,
        request: &AuthenticationRequest,
    ) -> Result<()> {
        self.validate()?;
        request.validate()?;

        if !self.supports_scheme(request.scheme()) {
            return Err(authentication_error(
                ToolchainErrorKind::UnsupportedOperation,
                "authentication provider does not declare the requested scheme",
            )
            .with_subject(self.provider_id.to_string()));
        }

        Ok(())
    }
}

// ============================================================================
// AUTHENTICATED SESSION METADATA
// ============================================================================

/// Non-secret metadata describing an authenticated session.
///
/// The session identifier and subject are descriptive identities.
/// They are not credentials and MUST NOT be accepted as proof of
/// authentication without the associated opaque session handle.
///
/// `expires_at` is optional because some authentication mechanisms do
/// not expose a reliable expiration time. In that case the concrete
/// session implementation remains responsible for determining validity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticationSessionMetadata {
    registry_id: RegistryId,
    provider_id: ProviderId,
    scheme: AuthenticationSchemeId,
    subject: Option<Arc<str>>,
    session_id: Option<Arc<str>>,
    expires_at: Option<SystemTime>,
}

impl AuthenticationSessionMetadata {
    /// Construct authenticated session metadata.
    ///
    /// This constructor is public so external authentication providers
    /// can implement the contract without depending on internal helpers.
    pub fn new(
        registry_id: RegistryId,
        provider_id: ProviderId,
        scheme: AuthenticationSchemeId,
        subject: Option<String>,
        session_id: Option<String>,
        expires_at: Option<SystemTime>,
    ) -> Result<Self> {
        if let Some(subject) = &subject {
            validate_nonempty_text(subject, "authenticated subject")?;
        }

        if let Some(session_id) = &session_id {
            validate_nonempty_text(session_id, "authentication session ID")?;
        }

        Ok(Self {
            registry_id,
            provider_id,
            scheme,
            subject: subject.map(Arc::from),
            session_id: session_id.map(Arc::from),
            expires_at,
        })
    }

    /// Return the registry identity associated with this session.
    #[must_use]
    pub fn registry_id(&self) -> &RegistryId {
        &self.registry_id
    }

    /// Return the authentication provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Return the authenticated scheme.
    #[must_use]
    pub fn scheme(&self) -> &AuthenticationSchemeId {
        &self.scheme
    }

    /// Return the authenticated subject, if supplied.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Return the provider-defined session identifier, if supplied.
    #[must_use]
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Return the session expiration time, if known.
    #[must_use]
    pub fn expires_at(&self) -> Option<SystemTime> {
        self.expires_at
    }

    /// Check whether the metadata's known expiration has passed.
    ///
    /// This check does not replace validation by the concrete session
    /// implementation. Revocation, server-side expiration and other
    /// protocol-specific invalidation may occur earlier.
    #[must_use]
    pub fn is_expired_at(&self, now: SystemTime) -> bool {
        self.expires_at
            .is_some_and(|expires_at| now >= expires_at)
    }
}

// ============================================================================
// AUTHENTICATION SESSION
// ============================================================================

/// Opaque authenticated session implemented by an authentication provider.
///
/// The concrete implementation may retain protocol-specific state,
/// connection state or references to a protected credential authority.
///
/// The session MUST NOT expose secrets through Debug output, diagnostics
/// or ordinary metadata accessors.
///
/// The session implementation is responsible for validating its actual
/// authentication state, including revocation or server-side invalidation
/// when the protocol supports those checks.
///
/// Transport-specific registry providers should consume this session
/// through their integration adapter. This trait deliberately does not
/// prescribe how credentials are attached to HTTP, RPC, local or other
/// transports.
pub trait RegistryAuthenticationSession: Send + Sync {
    /// Return non-secret session metadata.
    fn metadata(&self) -> &AuthenticationSessionMetadata;

    /// Validate that the session remains usable at the supplied time.
    ///
    /// Implementations should check local expiration and any applicable
    /// provider-specific validity conditions. They must not claim that
    /// a session is valid merely because metadata exists.
    fn validate(&self, now: SystemTime) -> Result<()>;
}

/// Shared opaque authentication session.
pub type SharedAuthenticationSession =
    Arc<dyn RegistryAuthenticationSession>;

/// Authenticated session returned by an authentication provider.
///
/// The wrapper validates that the session metadata belongs to the
/// registry, provider and scheme requested by the caller.
#[derive(Clone)]
pub struct AuthenticatedSession {
    metadata: AuthenticationSessionMetadata,
    session: SharedAuthenticationSession,
}

impl AuthenticatedSession {
    /// Construct and validate an authenticated session wrapper.
    ///
    /// The supplied session's metadata must match the separately supplied
    /// metadata exactly. This prevents a provider from returning a session
    /// for a different registry, provider or authentication scheme.
    pub fn new(
        metadata: AuthenticationSessionMetadata,
        session: SharedAuthenticationSession,
    ) -> Result<Self> {
        let actual = session.metadata();

        if actual != &metadata {
            return Err(authentication_error(
                ToolchainErrorKind::ProviderFailed,
                "authentication session metadata does not match \
                 the returned session handle",
            ));
        }

        Ok(Self { metadata, session })
    }

    /// Return non-secret session metadata.
    #[must_use]
    pub fn metadata(&self) -> &AuthenticationSessionMetadata {
        &self.metadata
    }

    /// Borrow the opaque session implementation.
    ///
    /// Concrete registry integration adapters use this handle to interact
    /// with their authentication implementation. Callers must not assume
    /// that the handle itself is a credential or authorization grant.
    #[must_use]
    pub fn session(&self) -> &SharedAuthenticationSession {
        &self.session
    }

    /// Validate the session against its concrete implementation.
    pub fn validate(&self, now: SystemTime) -> Result<()> {
        if self.metadata.is_expired_at(now) {
            return Err(authentication_error(
                ToolchainErrorKind::AuthenticationFailed,
                "authentication session has expired",
            ));
        }

        self.session.validate(now)
    }
}

impl fmt::Debug for AuthenticatedSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticatedSession")
            .field("metadata", &self.metadata)
            .field("session", &"[OPAQUE]")
            .finish()
    }
}

// ============================================================================
// AUTHENTICATION PROVIDER CONTRACT
// ============================================================================

/// Provider-neutral authentication implementation contract.
///
/// Implementations may use local credentials, enterprise identity
/// services, challenge-response protocols, certificates, delegated
/// identity systems or future authentication mechanisms.
///
/// Implementations MUST:
///
/// - Return an immutable validated descriptor.
/// - Authenticate only schemes declared by that descriptor.
/// - Bind the resulting session to the requested registry.
/// - Bind the resulting session to the selected provider.
/// - Bind the resulting session to the requested authentication scheme.
/// - Avoid exposing credentials through errors or diagnostics.
/// - Return an actual authenticated session, not a fabricated success.
/// - Preserve provider-specific failures as structured causal errors.
///
/// Implementations MUST NOT interpret successful authentication as
/// authorization to publish, download, modify or administer packages.
///
/// Credential lookup, challenge validation, protocol exchange and
/// session creation remain the responsibility of the implementation.
pub trait RegistryAuthenticationProvider: Send + Sync {
    /// Return the immutable descriptor for this implementation.
    fn descriptor(&self) -> &AuthenticationProviderDescriptor;

    /// Authenticate a registry operation.
    ///
    /// The returned future is tied to the provider borrow and does not
    /// require a specific asynchronous runtime.
    fn authenticate<'a>(
        &'a self,
        request: AuthenticationRequest,
    ) -> AuthenticationFuture<'a>;
}

// ============================================================================
// COMMON AUTHENTICATION EXECUTION
// ============================================================================

/// Authenticate a request with common provider and session validation.
///
/// This function:
///
/// 1. Validates the provider descriptor.
/// 2. Validates the authentication request.
/// 3. Checks that the requested scheme is declared.
/// 4. Invokes the authentication provider.
/// 5. Verifies the returned registry identity.
/// 6. Verifies the returned provider identity.
/// 7. Verifies the returned authentication scheme.
/// 8. Checks the returned session's current validity.
///
/// This function does not authorize the requested registry operation.
/// Authorization MUST be performed separately by the applicable security
/// policy before registry operation dispatch.
pub async fn authenticate_checked(
    provider: &dyn RegistryAuthenticationProvider,
    request: AuthenticationRequest,
) -> Result<AuthenticatedSession> {
    provider
        .descriptor()
        .validate_request(&request)?;

    let expected_registry = request.registry_id().clone();
    let expected_scheme = request.scheme().clone();
    let expected_provider = provider.descriptor().provider_id().clone();

    let session = provider.authenticate(request).await?;

    let metadata = session.metadata();

    if metadata.registry_id() != &expected_registry {
        return Err(authentication_error(
            ToolchainErrorKind::ProviderFailed,
            "authentication provider returned a session for \
             a different registry",
        )
        .with_subject(expected_provider.to_string()));
    }

    if metadata.provider_id() != &expected_provider {
        return Err(authentication_error(
            ToolchainErrorKind::ProviderFailed,
            "authentication provider returned a session associated \
             with a different provider",
        )
        .with_subject(expected_provider.to_string()));
    }

    if metadata.scheme() != &expected_scheme {
        return Err(authentication_error(
            ToolchainErrorKind::ProviderFailed,
            "authentication provider returned a session using \
             an unexpected authentication scheme",
        )
        .with_subject(expected_provider.to_string()));
    }

    session.validate(SystemTime::now())?;

    Ok(session)
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_nonempty_text(
    value: &str,
    description: &str,
) -> Result<()> {
    if value.trim().is_empty() {
        return Err(authentication_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(authentication_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not contain control characters"),
        ));
    }

    Ok(())
}

fn authentication_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(OPERATION_AUTHENTICATION)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn scheme(value: &str) -> AuthenticationSchemeId {
        AuthenticationSchemeId::new(value)
            .expect("valid authentication scheme")
    }

    #[test]
    fn scheme_identifier_accepts_open_world_names() {
        assert!(scheme("custom.vendor.identity").as_str()
            == "custom.vendor.identity");

        assert!(AuthenticationSchemeId::new("identity/v2").is_ok());
        assert!(AuthenticationSchemeId::new("token_bearer").is_ok());
    }

    #[test]
    fn scheme_identifier_rejects_invalid_values() {
        assert!(AuthenticationSchemeId::new("").is_err());
        assert!(AuthenticationSchemeId::new(".scheme").is_err());
        assert!(AuthenticationSchemeId::new("scheme.").is_err());
        assert!(AuthenticationSchemeId::new("scheme..v2").is_err());
        assert!(AuthenticationSchemeId::new("scheme with spaces").is_err());
        assert!(AuthenticationSchemeId::new("身份").is_err());
    }

    #[test]
    fn credential_reference_rejects_empty_and_control_values() {
        assert!(CredentialReference::new("").is_err());
        assert!(CredentialReference::new(" \t ").is_err());
        assert!(CredentialReference::new("credential\nreference").is_err());
    }

    #[test]
    fn credential_reference_debug_is_redacted() {
        let reference = CredentialReference::new(
            "account.production.registry",
        )
        .expect("valid reference");

        let debug = format!("{reference:?}");

        assert!(!debug.contains("account.production.registry"));
        assert!(debug.contains("REDACTED"));
    }

    #[test]
    fn challenge_debug_does_not_expose_payload() {
        let challenge = AuthenticationChallenge::new(
            scheme("challenge.response"),
            Arc::<[u8]>::from(&b"private-challenge"[..]),
        );

        let debug = format!("{challenge:?}");

        assert!(!debug.contains("private-challenge"));
        assert!(debug.contains("REDACTED"));
    }

    #[test]
    fn request_rejects_mismatched_challenge_scheme() {
        let registry = RegistryId::new(
            crate::toolchain::ids::IdNamespace::new("zamani.registry")
                .expect("valid namespace"),
            1,
        )
        .expect("valid registry ID");

        let operation = RegistryOperationId::new("packages.download")
            .expect("valid operation");

        let request = AuthenticationRequest::new(
            registry,
            scheme("token.bearer"),
            CredentialReference::new("credential.production")
                .expect("valid reference"),
            operation,
        );

        let challenge = AuthenticationChallenge::new(
            scheme("certificate.client"),
            Arc::<[u8]>::from(&b"challenge"[..]),
        );

        assert!(request.with_challenge(challenge).is_err());
    }

    #[test]
    fn session_expiration_is_checked() {
        let registry = RegistryId::new(
            crate::toolchain::ids::IdNamespace::new("zamani.registry")
                .expect("valid namespace"),
            1,
        )
        .expect("valid registry ID");

        let provider = ProviderId::new(
            crate::toolchain::ids::IdNamespace::new("zamani.provider")
                .expect("valid namespace"),
            1,
        )
        .expect("valid provider ID");

        let expiration = SystemTime::UNIX_EPOCH;

        let metadata = AuthenticationSessionMetadata::new(
            registry,
            provider,
            scheme("token.bearer"),
            Some("test-subject".to_owned()),
            None,
            Some(expiration),
        )
        .expect("valid metadata");

        assert!(metadata.is_expired_at(SystemTime::now()));
    }

    #[test]
    fn metadata_rejects_empty_subject_and_session_id() {
        let registry = RegistryId::new(
            crate::toolchain::ids::IdNamespace::new("zamani.registry")
                .expect("valid namespace"),
            1,
        )
        .expect("valid registry ID");

        let provider = ProviderId::new(
            crate::toolchain::ids::IdNamespace::new("zamani.provider")
                .expect("valid namespace"),
            1,
        )
        .expect("valid provider ID");

        assert!(AuthenticationSessionMetadata::new(
            registry.clone(),
            provider.clone(),
            scheme("token.bearer"),
            Some(String::new()),
            None,
            None,
        ).is_err());

        assert!(AuthenticationSessionMetadata::new(
            registry,
            provider,
            scheme("token.bearer"),
            None,
            Some("".to_owned()),
            None,
        ).is_err());
    }
}