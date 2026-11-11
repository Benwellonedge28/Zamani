
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Networking Security
//!
//! Path: `src/stdlib/networking/security.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable contracts for applying security requirements to
//! networking operations.
//!
//! This module connects networking intent to authoritative identity,
//! authentication, cryptography, policy, transport and runtime providers.
//!
//! It does not implement cryptographic algorithms, establish operating
//! system connections, resolve credentials, or make authorization decisions.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Open-world security-property identifiers.
//! - Authentication-mechanism identifiers.
//! - Credential and identity references.
//! - Peer-authentication request contracts.
//! - Authentication-provider interfaces.
//! - Secure-channel requirement descriptions.
//! - Secure-channel capability descriptions.
//! - Secure-channel provider interfaces.
//! - Established secure-channel metadata.
//! - Security-specific validation and provider errors.
//!
//! This module does not own:
//!
//! - Canonical security language semantics.
//! - Identity issuance or identity proof verification algorithms.
//! - Cryptographic algorithms.
//! - Cryptographic key management.
//! - Authorization.
//! - Policy evaluation or enforcement.
//! - Addresses or endpoint declarations.
//! - Protocol declarations.
//! - Transport implementation.
//! - Socket, stream or datagram operations.
//! - Service discovery.
//! - Routing or topology.
//! - Distributed scheduling.
//! - Hardware discovery or allocation.
//! - Runtime execution.
//!
//! ## Grammar authorities
//!
//! Networking security adapter:
//! `grammar/networking/security.g4`
//!
//! Canonical security grammar:
//! `grammar/security/security.g4`
//!
//! Networking composition:
//! `grammar/networking/networking.g4`
//!
//! Normative networking specification:
//! `grammar/spec/networking.md`
//!
//! The frontend and semantic layer remain responsible for parsing,
//! resolving and validating source-language security declarations.
//!
//! ## POCO-REAF
//!
//! Security identities and mechanism names are open-world identifiers.
//!
//! New authentication mechanisms, cryptographic technologies, trust
//! facilities, secure transports and hardware security providers must
//! be introducible without adding closed Rust enum variants.
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Security properties.
//! - Authentication mechanisms.
//! - Credential references.
//! - Identity references.
//! - Security requirements.
//! - Providers.
//! - Supported capabilities.
//! - Secure communication sessions.
//! - Participants.
//! - Nodes.
//! - Devices.
//! - Computational domains.
//!
//! Actual capacity is constrained by available resources, provider
//! capabilities, operational budgets and physical reality.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Secret handling
//!
//! Actual passwords, private keys, secret keys, bearer tokens and other
//! secret material must never be placed in these public request structures.
//!
//! Use opaque references resolved by an appropriately protected identity,
//! credential or key-management provider.
//!
//! Debug output must not disclose secret material.
//!
//! ## Security invariants
//!
//! - Discovery does not establish trust.
//! - Authentication does not grant authorization.
//! - A secure channel does not prove that an application operation is allowed.
//! - A provider declaration does not prove that a security guarantee was met.
//! - Missing mandatory security properties must fail closed.
//! - Indeterminate security capability information must not be treated as
//!   proof that a requirement is satisfied.
//! - An established channel must report the actual properties established
//!   by its provider.
//! - A failed or incomplete security operation must never be reported as
//!   successful establishment.
//!
//! ## Integration
//!
//! Address contracts:
//! `crate::stdlib::networking::addresses`
//!
//! Transport contracts:
//! `crate::stdlib::networking::transports`
//!
//! Protocol contracts:
//! `crate::stdlib::networking::protocols`
//!
//! Cryptographic providers:
//! `crate::stdlib::crypto` and authoritative cryptographic subsystems.
//!
//! Distributed identity and authentication:
//! `crate::distributed::security`
//!
//! Runtime and physical transport:
//! `crate::runtime`, `crate::distributed`, HAL and provider implementations.
//!
//! This module is intentionally independent of concrete implementations
//! in those subsystems. Integration is performed through explicit adapters.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::time::Duration;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn is_valid_qualified_identifier(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    if value.chars().any(char::is_control) {
        return false;
    }

    value.split("::").all(|component| {
        let mut bytes = component.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        (first.is_ascii_alphanumeric() || first == b'_')
            && bytes.all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
                    || byte == b'.'
            })
    })
}

fn is_valid_reference(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_control)
        && !value.contains('\0')
}

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of networking-security failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SecurityErrorKind {
    /// A security-property identifier is invalid.
    InvalidProperty,

    /// An authentication-mechanism identifier is invalid.
    InvalidMechanism,

    /// A credential reference is invalid.
    InvalidCredentialReference,

    /// An identity reference is invalid.
    InvalidIdentityReference,

    /// A provider identifier is invalid.
    InvalidProvider,

    /// A security request is structurally invalid.
    InvalidRequest,

    /// A security capability declaration is invalid.
    InvalidCapability,

    /// A mandatory security property is unavailable.
    MissingRequiredProperty,

    /// A forbidden security property is present.
    ForbiddenProperty,

    /// The selected provider cannot satisfy the request.
    UnsupportedRequirement,

    /// The selected provider is unavailable.
    ProviderUnavailable,

    /// A provider operation failed.
    ProviderFailure,

    /// Authentication failed.
    AuthenticationFailed,

    /// The peer identity does not satisfy the expected identity.
    IdentityMismatch,

    /// Authentication evidence is invalid.
    InvalidAuthenticationEvidence,

    /// Authentication evidence is expired.
    ExpiredAuthenticationEvidence,

    /// Authentication evidence has been replayed.
    ReplayedAuthenticationEvidence,

    /// The authentication operation was cancelled.
    Cancelled,

    /// The authentication operation timed out.
    TimedOut,

    /// The security operation was denied by an authoritative policy.
    Denied,

    /// The policy decision was indeterminate.
    Indeterminate,

    /// A required policy obligation was not fulfilled.
    UnfulfilledObligation,

    /// A provider returned metadata inconsistent with the request.
    InvalidProviderResult,

    /// The secure channel is closed.
    Closed,

    /// The requested operation is invalid in the current state.
    InvalidState,
}

impl SecurityErrorKind {
    /// Returns the stable textual classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidProperty => "invalid_property",
            Self::InvalidMechanism => "invalid_mechanism",
            Self::InvalidCredentialReference => "invalid_credential_reference",
            Self::InvalidIdentityReference => "invalid_identity_reference",
            Self::InvalidProvider => "invalid_provider",
            Self::InvalidRequest => "invalid_request",
            Self::InvalidCapability => "invalid_capability",
            Self::MissingRequiredProperty => "missing_required_property",
            Self::ForbiddenProperty => "forbidden_property",
            Self::UnsupportedRequirement => "unsupported_requirement",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderFailure => "provider_failure",
            Self::AuthenticationFailed => "authentication_failed",
            Self::IdentityMismatch => "identity_mismatch",
            Self::InvalidAuthenticationEvidence => "invalid_authentication_evidence",
            Self::ExpiredAuthenticationEvidence => "expired_authentication_evidence",
            Self::ReplayedAuthenticationEvidence => "replayed_authentication_evidence",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Denied => "denied",
            Self::Indeterminate => "indeterminate",
            Self::UnfulfilledObligation => "unfulfilled_obligation",
            Self::InvalidProviderResult => "invalid_provider_result",
            Self::Closed => "closed",
            Self::InvalidState => "invalid_state",
        }
    }
}

impl fmt::Display for SecurityErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured networking-security error.
#[derive(Debug)]
pub struct SecurityError {
    kind: SecurityErrorKind,
    message: String,
    related: Vec<String>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl SecurityError {
    /// Creates a structured security error.
    #[must_use]
    pub fn new(
        kind: SecurityErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            related: Vec::new(),
            source: None,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> SecurityErrorKind {
        self.kind
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns related non-secret diagnostic references.
    #[must_use]
    pub fn related(&self) -> &[String] {
        &self.related
    }

    /// Adds a non-secret diagnostic reference.
    #[must_use]
    pub fn with_related(mut self, value: impl Into<String>) -> Self {
        self.related.push(value.into());
        self
    }

    /// Attaches an underlying provider error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for SecurityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "networking security error [{}]: {}",
            self.kind,
            self.message
        )
    }
}

impl StdError for SecurityError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for networking-security operations.
pub type SecurityResult<T> = Result<T, SecurityError>;

fn security_error(
    kind: SecurityErrorKind,
    message: impl Into<String>,
) -> SecurityError {
    SecurityError::new(kind, message)
}

// ============================================================================
// OPEN-WORLD IDENTITIES
// ============================================================================

/// Open-world security-property identity.
///
/// Examples include `security::confidentiality`,
/// `security::integrity`, `security::authentication`,
/// and future provider-defined properties.
///
/// This is not a closed list of security guarantees.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SecurityPropertyId(String);

impl SecurityPropertyId {
    /// Creates a validated security-property identity.
    pub fn new(value: impl Into<String>) -> SecurityResult<Self> {
        let value = value.into();

        if !is_valid_qualified_identifier(&value) {
            return Err(security_error(
                SecurityErrorKind::InvalidProperty,
                "invalid security-property identifier",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the complete identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SecurityPropertyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Open-world authentication-mechanism identity.
///
/// Examples include `security::mutual_authentication`,
/// `security::certificate_authentication`, and provider-defined
/// mechanisms.
///
/// The identity does not imply that the mechanism is implemented or secure.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AuthenticationMechanismId(String);

impl AuthenticationMechanismId {
    /// Creates a validated authentication-mechanism identity.
    pub fn new(value: impl Into<String>) -> SecurityResult<Self> {
        let value = value.into();

        if !is_valid_qualified_identifier(&value) {
            return Err(security_error(
                SecurityErrorKind::InvalidMechanism,
                "invalid authentication-mechanism identifier",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the complete identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuthenticationMechanismId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque reference to a credential managed by a trusted provider.
///
/// This contains a reference, not the credential or secret itself.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CredentialReference(String);

impl CredentialReference {
    /// Creates a validated credential reference.
    pub fn new(value: impl Into<String>) -> SecurityResult<Self> {
        let value = value.into();

        if !is_valid_reference(&value) {
            return Err(security_error(
                SecurityErrorKind::InvalidCredentialReference,
                "invalid credential reference",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for CredentialReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CredentialReference([REDACTED])")
    }
}

/// Opaque reference to a canonical identity declaration or identity record.
///
/// The reference is not proof that the identity is authentic or trusted.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdentityReference(String);

impl IdentityReference {
    /// Creates a validated identity reference.
    pub fn new(value: impl Into<String>) -> SecurityResult<Self> {
        let value = value.into();

        if !is_valid_reference(&value) {
            return Err(security_error(
                SecurityErrorKind::InvalidIdentityReference,
                "invalid identity reference",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque identity reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Open-world security-provider identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SecurityProviderId(String);

impl SecurityProviderId {
    /// Creates a validated provider identity.
    pub fn new(value: impl Into<String>) -> SecurityResult<Self> {
        let value = value.into();

        if !is_valid_qualified_identifier(&value) {
            return Err(security_error(
                SecurityErrorKind::InvalidProvider,
                "invalid security-provider identifier",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// SECURITY PROPERTY REQUIREMENTS
// ============================================================================

/// A mandatory security property.
///
/// A property may carry an opaque qualifier interpreted by its
/// authoritative semantic or security provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredSecurityProperty {
    id: SecurityPropertyId,
    qualifier: Option<String>,
}

impl RequiredSecurityProperty {
    /// Creates a required security property.
    #[must_use]
    pub fn new(id: SecurityPropertyId) -> Self {
        Self {
            id,
            qualifier: None,
        }
    }

    /// Adds an opaque, non-secret qualifier.
    pub fn with_qualifier(
        mut self,
        qualifier: impl Into<String>,
    ) -> SecurityResult<Self> {
        let qualifier = qualifier.into();

        if !is_valid_reference(&qualifier) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "invalid security-property qualifier",
            ));
        }

        self.qualifier = Some(qualifier);
        Ok(self)
    }

    /// Returns the property identity.
    #[must_use]
    pub fn id(&self) -> &SecurityPropertyId {
        &self.id
    }

    /// Returns the optional qualifier.
    #[must_use]
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }
}

/// A security property that must not be present.
///
/// This is useful for expressing explicit security restrictions.
/// The semantic layer remains responsible for validating whether
/// such a restriction is meaningful and consistent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenSecurityProperty {
    id: SecurityPropertyId,
}

impl ForbiddenSecurityProperty {
    /// Creates a forbidden security property.
    #[must_use]
    pub fn new(id: SecurityPropertyId) -> Self {
        Self { id }
    }

    /// Returns the property identity.
    #[must_use]
    pub fn id(&self) -> &SecurityPropertyId {
        &self.id
    }
}

// ============================================================================
// PEER AUTHENTICATION
// ============================================================================

/// Request to authenticate a communication peer.
///
/// The request contains references and intent only. It never contains
/// passwords, private keys, bearer tokens or raw secret material.
///
/// Authentication providers must verify the supplied evidence and
/// establish the identity through their authoritative trust mechanism.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticationRequest {
    mechanism: AuthenticationMechanismId,
    expected_identity: Option<IdentityReference>,
    credential: Option<CredentialReference>,
    challenge_reference: Option<String>,
    timeout: Option<Duration>,
}

impl AuthenticationRequest {
    /// Creates an authentication request.
    #[must_use]
    pub fn new(mechanism: AuthenticationMechanismId) -> Self {
        Self {
            mechanism,
            expected_identity: None,
            credential: None,
            challenge_reference: None,
            timeout: None,
        }
    }

    /// Sets the identity expected from the peer.
    #[must_use]
    pub fn expecting_identity(
        mut self,
        identity: IdentityReference,
    ) -> Self {
        self.expected_identity = Some(identity);
        self
    }

    /// Sets an opaque local credential reference.
    #[must_use]
    pub fn using_credential(
        mut self,
        credential: CredentialReference,
    ) -> Self {
        self.credential = Some(credential);
        self
    }

    /// Sets an opaque challenge reference.
    ///
    /// Challenge generation, freshness and replay protection belong
    /// to the authentication provider.
    pub fn with_challenge_reference(
        mut self,
        reference: impl Into<String>,
    ) -> SecurityResult<Self> {
        let reference = reference.into();

        if !is_valid_reference(&reference) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "invalid challenge reference",
            ));
        }

        self.challenge_reference = Some(reference);
        Ok(self)
    }

    /// Sets an optional operational timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Returns the requested mechanism.
    #[must_use]
    pub fn mechanism(&self) -> &AuthenticationMechanismId {
        &self.mechanism
    }

    /// Returns the expected identity, if specified.
    #[must_use]
    pub fn expected_identity(&self) -> Option<&IdentityReference> {
        self.expected_identity.as_ref()
    }

    /// Returns the credential reference, if specified.
    #[must_use]
    pub fn credential(&self) -> Option<&CredentialReference> {
        self.credential.as_ref()
    }

    /// Returns the challenge reference, if specified.
    #[must_use]
    pub fn challenge_reference(&self) -> Option<&str> {
        self.challenge_reference.as_deref()
    }

    /// Returns the optional timeout.
    #[must_use]
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

/// An identity established by an authentication provider.
///
/// Construction is public because external providers must be able to
/// return their verified result. Therefore, this value is trusted only
/// when it is received from an explicitly trusted provider boundary.
///
/// It is not a cryptographic proof by itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticatedPeer {
    identity: IdentityReference,
    mechanism: AuthenticationMechanismId,
    provider: SecurityProviderId,
    evidence_reference: Option<String>,
}

impl AuthenticatedPeer {
    /// Constructs an authenticated-peer result.
    ///
    /// Call this only after the provider has actually verified the
    /// peer's authentication evidence and trust requirements.
    pub fn verified(
        identity: IdentityReference,
        mechanism: AuthenticationMechanismId,
        provider: SecurityProviderId,
        evidence_reference: Option<String>,
    ) -> SecurityResult<Self> {
        if evidence_reference
            .as_deref()
            .is_some_and(|value| !is_valid_reference(value))
        {
            return Err(security_error(
                SecurityErrorKind::InvalidAuthenticationEvidence,
                "invalid authentication evidence reference",
            ));
        }

        Ok(Self {
            identity,
            mechanism,
            provider,
            evidence_reference,
        })
    }

    /// Returns the authenticated identity reference.
    #[must_use]
    pub fn identity(&self) -> &IdentityReference {
        &self.identity
    }

    /// Returns the authentication mechanism.
    #[must_use]
    pub fn mechanism(&self) -> &AuthenticationMechanismId {
        &self.mechanism
    }

    /// Returns the provider that produced this result.
    #[must_use]
    pub fn provider(&self) -> &SecurityProviderId {
        &self.provider
    }

    /// Returns the optional non-secret evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }
}

/// Provider contract for peer authentication.
///
/// Implementations must perform real verification. Returning an
/// `AuthenticatedPeer` without verifying the evidence violates this
/// contract.
///
/// Authentication does not authorize an application operation.
pub trait AuthenticationProvider: Send + Sync {
    /// Returns the provider identity.
    fn provider_id(&self) -> &SecurityProviderId;

    /// Authenticates a peer.
    ///
    /// # Errors
    ///
    /// Returns an explicit error for invalid evidence, failed trust
    /// verification, expiry, replay, cancellation, timeout or provider
    /// failure.
    fn authenticate(
        &self,
        request: &AuthenticationRequest,
    ) -> SecurityResult<AuthenticatedPeer>;
}

// ============================================================================
// SECURE-CHANNEL REQUIREMENTS
// ============================================================================

/// Portable requirements for establishing a secure communication channel.
///
/// These requirements describe desired security properties, not a
/// particular cryptographic algorithm or transport technology.
///
/// A transport adapter must verify the actual established properties
/// before returning a successful channel.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SecureChannelRequirements {
    required: Vec<RequiredSecurityProperty>,
    forbidden: Vec<ForbiddenSecurityProperty>,
    mechanisms: Vec<AuthenticationMechanismId>,
    expected_peer: Option<IdentityReference>,
    policy_references: Vec<String>,
    timeout: Option<Duration>,
}

impl SecureChannelRequirements {
    /// Creates an empty requirement set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a mandatory security property.
    pub fn require(
        &mut self,
        property: RequiredSecurityProperty,
    ) -> SecurityResult<()> {
        if self.required.iter().any(|existing| existing.id == property.id) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "duplicate required security property",
            )
            .with_related(property.id.to_string()));
        }

        if self.forbidden.iter().any(|existing| existing.id == property.id) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "a security property cannot be both required and forbidden",
            )
            .with_related(property.id.to_string()));
        }

        self.required.push(property);
        Ok(())
    }

    /// Adds a property that must not be present.
    pub fn forbid(
        &mut self,
        property: ForbiddenSecurityProperty,
    ) -> SecurityResult<()> {
        if self.forbidden.iter().any(|existing| existing.id == property.id) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "duplicate forbidden security property",
            )
            .with_related(property.id.to_string()));
        }

        if self.required.iter().any(|existing| existing.id == property.id) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "a security property cannot be both required and forbidden",
            )
            .with_related(property.id.to_string()));
        }

        self.forbidden.push(property);
        Ok(())
    }

    /// Adds an acceptable authentication mechanism.
    pub fn allow_mechanism(
        &mut self,
        mechanism: AuthenticationMechanismId,
    ) -> SecurityResult<()> {
        if self.mechanisms.contains(&mechanism) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "duplicate authentication mechanism",
            )
            .with_related(mechanism.to_string()));
        }

        self.mechanisms.push(mechanism);
        Ok(())
    }

    /// Sets the expected peer identity.
    #[must_use]
    pub fn expecting_peer(
        mut self,
        identity: IdentityReference,
    ) -> Self {
        self.expected_peer = Some(identity);
        self
    }

    /// Adds a reference to a canonical security policy.
    ///
    /// This module does not evaluate the policy.
    pub fn reference_policy(
        &mut self,
        reference: impl Into<String>,
    ) -> SecurityResult<()> {
        let reference = reference.into();

        if !is_valid_reference(&reference) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "invalid security-policy reference",
            ));
        }

        if self.policy_references.contains(&reference) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "duplicate security-policy reference",
            ));
        }

        self.policy_references.push(reference);
        Ok(())
    }

    /// Sets an optional operational timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Returns mandatory security properties.
    #[must_use]
    pub fn required_properties(&self) -> &[RequiredSecurityProperty] {
        &self.required
    }

    /// Returns forbidden security properties.
    #[must_use]
    pub fn forbidden_properties(&self) -> &[ForbiddenSecurityProperty] {
        &self.forbidden
    }

    /// Returns acceptable authentication mechanisms.
    #[must_use]
    pub fn mechanisms(&self) -> &[AuthenticationMechanismId] {
        &self.mechanisms
    }

    /// Returns the expected peer identity.
    #[must_use]
    pub fn expected_peer(&self) -> Option<&IdentityReference> {
        self.expected_peer.as_ref()
    }

    /// Returns policy references.
    #[must_use]
    pub fn policy_references(&self) -> &[String] {
        &self.policy_references
    }

    /// Returns the optional timeout.
    #[must_use]
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Validates local consistency of the requirements.
    ///
    /// This does not resolve policy references or establish feasibility.
    pub fn validate(&self) -> SecurityResult<()> {
        let required: BTreeSet<&str> = self
            .required
            .iter()
            .map(|property| property.id.as_str())
            .collect();

        for forbidden in &self.forbidden {
            if required.contains(forbidden.id.as_str()) {
                return Err(security_error(
                    SecurityErrorKind::InvalidRequest,
                    "a property is both required and forbidden",
                )
                .with_related(forbidden.id.to_string()));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SECURITY CAPABILITIES
// ============================================================================

/// A provider's declaration that it can establish a security property.
///
/// A capability declaration is descriptive. It is not proof that the
/// property was actually established for a particular channel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecurityCapability {
    property: SecurityPropertyId,
    qualifier: Option<String>,
}

impl SecurityCapability {
    /// Creates a capability declaration.
    #[must_use]
    pub fn new(property: SecurityPropertyId) -> Self {
        Self {
            property,
            qualifier: None,
        }
    }

    /// Adds an opaque capability qualifier.
    pub fn with_qualifier(
        mut self,
        qualifier: impl Into<String>,
    ) -> SecurityResult<Self> {
        let qualifier = qualifier.into();

        if !is_valid_reference(&qualifier) {
            return Err(security_error(
                SecurityErrorKind::InvalidCapability,
                "invalid capability qualifier",
            ));
        }

        self.qualifier = Some(qualifier);
        Ok(self)
    }

    /// Returns the property identity.
    #[must_use]
    pub fn property(&self) -> &SecurityPropertyId {
        &self.property
    }

    /// Returns the optional qualifier.
    #[must_use]
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }
}

/// Security capabilities advertised by a provider.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SecurityProviderCapabilities {
    properties: Vec<SecurityCapability>,
    mechanisms: Vec<AuthenticationMechanismId>,
}

impl SecurityProviderCapabilities {
    /// Creates an empty capability description.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a security capability.
    pub fn add_property(
        &mut self,
        capability: SecurityCapability,
    ) -> SecurityResult<()> {
        if self.properties.contains(&capability) {
            return Err(security_error(
                SecurityErrorKind::InvalidCapability,
                "duplicate security capability",
            )
            .with_related(capability.property.to_string()));
        }

        self.properties.push(capability);
        Ok(())
    }

    /// Adds a supported authentication mechanism.
    pub fn add_mechanism(
        &mut self,
        mechanism: AuthenticationMechanismId,
    ) -> SecurityResult<()> {
        if self.mechanisms.contains(&mechanism) {
            return Err(security_error(
                SecurityErrorKind::InvalidCapability,
                "duplicate authentication mechanism capability",
            )
            .with_related(mechanism.to_string()));
        }

        self.mechanisms.push(mechanism);
        Ok(())
    }

    /// Returns declared security properties.
    #[must_use]
    pub fn properties(&self) -> &[SecurityCapability] {
        &self.properties
    }

    /// Returns supported authentication mechanisms.
    #[must_use]
    pub fn mechanisms(&self) -> &[AuthenticationMechanismId] {
        &self.mechanisms
    }
}

/// Checks whether declared capabilities cover the mandatory property IDs.
///
/// Qualifiers require provider-specific interpretation and therefore are
/// not treated as equivalent merely because their property IDs match.
pub fn validate_declared_capabilities(
    requirements: &SecureChannelRequirements,
    capabilities: &SecurityProviderCapabilities,
) -> SecurityResult<()> {
    requirements.validate()?;

    for required in &requirements.required {
        let matched = capabilities.properties.iter().any(|capability| {
            capability.property == required.id
                && capability.qualifier == required.qualifier
        });

        if !matched {
            return Err(security_error(
                SecurityErrorKind::UnsupportedRequirement,
                "provider has not declared a required security capability",
            )
            .with_related(required.id.to_string()));
        }
    }

    if !requirements.mechanisms.is_empty() {
        let has_compatible_mechanism = requirements
            .mechanisms
            .iter()
            .any(|mechanism| capabilities.mechanisms.contains(mechanism));

        if !has_compatible_mechanism {
            return Err(security_error(
                SecurityErrorKind::UnsupportedRequirement,
                "provider has no acceptable authentication mechanism",
            ));
        }
    }

    Ok(())
}

// ============================================================================
// SECURE-CHANNEL REQUEST
// ============================================================================

/// Request to establish a secure channel over a separately selected
/// networking transport.
///
/// The transport itself is supplied by an adapter. This request does not
/// contain a socket, address, connection or transport implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecureChannelRequest {
    requirements: SecureChannelRequirements,
    authentication: Option<AuthenticationRequest>,
    operation_reference: Option<String>,
}

impl SecureChannelRequest {
    /// Creates a secure-channel request.
    #[must_use]
    pub fn new(requirements: SecureChannelRequirements) -> Self {
        Self {
            requirements,
            authentication: None,
            operation_reference: None,
        }
    }

    /// Attaches an authentication request.
    #[must_use]
    pub fn with_authentication(
        mut self,
        authentication: AuthenticationRequest,
    ) -> Self {
        self.authentication = Some(authentication);
        self
    }

    /// Adds a non-secret reference to the operation being protected.
    pub fn with_operation_reference(
        mut self,
        reference: impl Into<String>,
    ) -> SecurityResult<Self> {
        let reference = reference.into();

        if !is_valid_reference(&reference) {
            return Err(security_error(
                SecurityErrorKind::InvalidRequest,
                "invalid protected-operation reference",
            ));
        }

        self.operation_reference = Some(reference);
        Ok(self)
    }

    /// Returns the security requirements.
    #[must_use]
    pub fn requirements(&self) -> &SecureChannelRequirements {
        &self.requirements
    }

    /// Returns the optional authentication request.
    #[must_use]
    pub fn authentication(&self) -> Option<&AuthenticationRequest> {
        self.authentication.as_ref()
    }

    /// Returns the optional protected-operation reference.
    #[must_use]
    pub fn operation_reference(&self) -> Option<&str> {
        self.operation_reference.as_deref()
    }

    /// Validates local request consistency.
    pub fn validate(&self) -> SecurityResult<()> {
        self.requirements.validate()?;

        if let (Some(expected), Some(authentication)) = (
            self.requirements.expected_peer(),
            self.authentication.as_ref(),
        ) {
            if let Some(authentication_expected) =
                authentication.expected_identity()
            {
                if expected != authentication_expected {
                    return Err(security_error(
                        SecurityErrorKind::IdentityMismatch,
                        "channel and authentication requests specify different peer identities",
                    ));
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// ESTABLISHED CHANNEL METADATA
// ============================================================================

/// Evidence that a security property was established for a channel.
///
/// This metadata must be produced by the trusted channel provider after
/// successful establishment, not copied from the original request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstablishedSecurityProperty {
    property: SecurityPropertyId,
    qualifier: Option<String>,
    evidence_reference: Option<String>,
}

impl EstablishedSecurityProperty {
    /// Creates metadata for an actually established property.
    pub fn new(
        property: SecurityPropertyId,
        qualifier: Option<String>,
        evidence_reference: Option<String>,
    ) -> SecurityResult<Self> {
        if qualifier
            .as_deref()
            .is_some_and(|value| !is_valid_reference(value))
        {
            return Err(security_error(
                SecurityErrorKind::InvalidProviderResult,
                "invalid established-property qualifier",
            ));
        }

        if evidence_reference
            .as_deref()
            .is_some_and(|value| !is_valid_reference(value))
        {
            return Err(security_error(
                SecurityErrorKind::InvalidProviderResult,
                "invalid security evidence reference",
            ));
        }

        Ok(Self {
            property,
            qualifier,
            evidence_reference,
        })
    }

    /// Returns the property identity.
    #[must_use]
    pub fn property(&self) -> &SecurityPropertyId {
        &self.property
    }

    /// Returns the optional qualifier.
    #[must_use]
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }

    /// Returns the optional non-secret evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }
}

/// Metadata describing a successfully established secure channel.
///
/// This structure does not own the underlying transport or connection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstablishedSecurity {
    provider: SecurityProviderId,
    properties: Vec<EstablishedSecurityProperty>,
    authenticated_peer: Option<AuthenticatedPeer>,
}

impl EstablishedSecurity {
    /// Creates established-security metadata.
    ///
    /// The caller must be a trusted provider adapter that has completed
    /// the actual security operation.
    pub fn new(
        provider: SecurityProviderId,
        properties: Vec<EstablishedSecurityProperty>,
        authenticated_peer: Option<AuthenticatedPeer>,
    ) -> SecurityResult<Self> {
        let mut seen = BTreeSet::new();

        for property in &properties {
            let key = (
                property.property.as_str(),
                property.qualifier.as_deref(),
            );

            if !seen.insert(key) {
                return Err(security_error(
                    SecurityErrorKind::InvalidProviderResult,
                    "provider returned duplicate established security properties",
                )
                .with_related(property.property.to_string()));
            }
        }

        Ok(Self {
            provider,
            properties,
            authenticated_peer,
        })
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider(&self) -> &SecurityProviderId {
        &self.provider
    }

    /// Returns the properties actually established.
    #[must_use]
    pub fn properties(&self) -> &[EstablishedSecurityProperty] {
        &self.properties
    }

    /// Returns the authenticated peer, if authentication occurred.
    #[must_use]
    pub fn authenticated_peer(&self) -> Option<&AuthenticatedPeer> {
        self.authenticated_peer.as_ref()
    }
}

// ============================================================================
// ESTABLISHED SECURE CHANNEL
// ============================================================================

/// A provider-owned secure communication channel.
///
/// The trait deliberately does not define send, receive, stream, datagram,
/// or socket methods. Those belong to the networking transport authority.
///
/// A transport adapter may retain its own connection internally and expose
/// only its security lifecycle through this interface.
pub trait SecureChannel: Send + Sync {
    /// Returns the security metadata established for this channel.
    fn security(&self) -> &EstablishedSecurity;

    /// Returns whether the channel is currently open.
    fn is_open(&self) -> bool;

    /// Closes the security session and its provider-owned resources.
    ///
    /// The implementation must document its concurrent-close semantics.
    fn close(&self) -> SecurityResult<()>;
}

// ============================================================================
// SECURE-CHANNEL PROVIDER
// ============================================================================

/// Provider contract for establishing secure channels.
///
/// Implementations integrate with the selected transport, authentication,
/// cryptographic, identity, key-management and policy providers.
///
/// A successful result means the provider has actually established the
/// requested security properties and returned verifiable metadata.
///
/// Merely declaring a capability is not sufficient.
///
/// The provider must fail closed when mandatory security properties cannot
/// be established.
pub trait SecureChannelProvider: Send + Sync {
    /// Returns the provider identity.
    fn provider_id(&self) -> &SecurityProviderId;

    /// Returns declared capabilities.
    fn capabilities(&self) -> &SecurityProviderCapabilities;

    /// Establishes a secure channel using the provider's transport adapter.
    ///
    /// # Errors
    ///
    /// Returns an explicit error if requirements cannot be satisfied,
    /// authentication fails, policy denies establishment, the provider
    /// fails, or the operation is cancelled or times out.
    fn establish(
        &self,
        request: &SecureChannelRequest,
    ) -> SecurityResult<Box<dyn SecureChannel>>;
}

// ============================================================================
// POST-ESTABLISHMENT VALIDATION
// ============================================================================

/// Validates the actual security metadata returned by a provider.
///
/// This function checks mandatory and forbidden properties against the
/// established result. It does not verify cryptographic evidence itself;
/// that remains the responsibility of the canonical security provider.
pub fn validate_established_security(
    request: &SecureChannelRequest,
    established: &EstablishedSecurity,
) -> SecurityResult<()> {
    request.validate()?;

    for required in request.requirements.required_properties() {
        let found = established.properties.iter().any(|actual| {
            actual.property == required.id
                && actual.qualifier == required.qualifier
        });

        if !found {
            return Err(security_error(
                SecurityErrorKind::MissingRequiredProperty,
                "provider did not establish a mandatory security property",
            )
            .with_related(required.id.to_string()));
        }
    }

    for forbidden in request.requirements.forbidden_properties() {
        let found = established
            .properties
            .iter()
            .any(|actual| actual.property == forbidden.id);

        if found {
            return Err(security_error(
                SecurityErrorKind::ForbiddenProperty,
                "provider established a forbidden security property",
            )
            .with_related(forbidden.id.to_string()));
        }
    }

    if let Some(expected) = request.requirements.expected_peer() {
        let actual = established.authenticated_peer.as_ref().ok_or_else(|| {
            security_error(
                SecurityErrorKind::AuthenticationFailed,
                "the request requires an authenticated peer but no authenticated identity was returned",
            )
        })?;

        if actual.identity() != expected {
            return Err(security_error(
                SecurityErrorKind::IdentityMismatch,
                "authenticated peer does not match the expected identity",
            )
            .with_related(expected.as_str().to_owned())
            .with_related(actual.identity().as_str().to_owned()));
        }
    }

    if let Some(authentication) = request.authentication() {
        let actual = established.authenticated_peer.as_ref().ok_or_else(|| {
            security_error(
                SecurityErrorKind::AuthenticationFailed,
                "authentication was requested but the provider returned no authenticated peer",
            )
        })?;

        if actual.mechanism() != authentication.mechanism() {
            return Err(security_error(
                SecurityErrorKind::InvalidProviderResult,
                "provider returned an unexpected authentication mechanism",
            )
            .with_related(actual.mechanism().to_string())
            .with_related(authentication.mechanism().to_string()));
        }
    }

    Ok(())
}

// ============================================================================
// PROVIDER REGISTRY CONTRACT
// ============================================================================

/// An immutable snapshot of a registered secure-channel provider.
///
/// The registry itself belongs to the composition/runtime layer. This
/// module supplies a provider-neutral description for that registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecurityProviderDescriptor {
    id: SecurityProviderId,
    capabilities: SecurityProviderCapabilities,
}

impl SecurityProviderDescriptor {
    /// Creates a provider descriptor.
    #[must_use]
    pub fn new(
        id: SecurityProviderId,
        capabilities: SecurityProviderCapabilities,
    ) -> Self {
        Self { id, capabilities }
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn id(&self) -> &SecurityProviderId {
        &self.id
    }

    /// Returns the declared capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &SecurityProviderCapabilities {
        &self.capabilities
    }

    /// Checks whether the descriptor declares the required properties.
    pub fn check(
        &self,
        requirements: &SecureChannelRequirements,
    ) -> SecurityResult<()> {
        validate_declared_capabilities(
            requirements,
            &self.capabilities,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn property(value: &str) -> SecurityPropertyId {
        SecurityPropertyId::new(value).expect("valid property")
    }

    fn mechanism(value: &str) -> AuthenticationMechanismId {
        AuthenticationMechanismId::new(value).expect("valid mechanism")
    }

    fn identity(value: &str) -> IdentityReference {
        IdentityReference::new(value).expect("valid identity")
    }

    fn provider(value: &str) -> SecurityProviderId {
        SecurityProviderId::new(value).expect("valid provider")
    }

    #[test]
    fn accepts_open_world_security_properties() {
        let value = property("future::quantum::integrity");

        assert_eq!(
            value.as_str(),
            "future::quantum::integrity"
        );
    }

    #[test]
    fn rejects_invalid_security_property_names() {
        assert!(SecurityPropertyId::new("").is_err());
        assert!(SecurityPropertyId::new("security::::integrity").is_err());
        assert!(SecurityPropertyId::new(" security::integrity").is_err());
    }

    #[test]
    fn accepts_open_world_authentication_mechanisms() {
        let value = mechanism("vendor::future_authentication");

        assert_eq!(
            value.as_str(),
            "vendor::future_authentication"
        );
    }

    #[test]
    fn credential_debug_output_is_redacted() {
        let credential = CredentialReference::new("secret-store::credential-1")
            .expect("valid reference");

        assert_eq!(
            format!("{credential:?}"),
            "CredentialReference([REDACTED])"
        );
    }

    #[test]
    fn creates_authentication_requests_without_secret_material() {
        let request = AuthenticationRequest::new(
            mechanism("security::mutual_authentication"),
        )
        .expecting_identity(identity("principals::service"))
        .using_credential(
            CredentialReference::new("credential-store::service")
                .expect("valid credential reference"),
        );

        assert_eq!(
            request.expected_identity().unwrap().as_str(),
            "principals::service"
        );

        assert_eq!(
            request.credential().unwrap().as_str(),
            "credential-store::service"
        );
    }

    #[test]
    fn rejects_invalid_challenge_references() {
        let result = AuthenticationRequest::new(
            mechanism("security::challenge"),
        )
        .with_challenge_reference("invalid\nchallenge");

        assert_eq!(
            result.unwrap_err().kind(),
            SecurityErrorKind::InvalidRequest
        );
    }

    #[test]
    fn rejects_conflicting_required_and_forbidden_properties() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::integrity"),
            ))
            .unwrap();

        let error = requirements
            .forbid(ForbiddenSecurityProperty::new(
                property("security::integrity"),
            ))
            .unwrap_err();

        assert_eq!(
            error.kind(),
            SecurityErrorKind::InvalidRequest
        );
    }

    #[test]
    fn accepts_distinct_security_requirements() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::confidentiality"),
            ))
            .unwrap();

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::integrity"),
            ))
            .unwrap();

        requirements
            .allow_mechanism(mechanism("security::mutual_authentication"))
            .unwrap();

        assert!(requirements.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_required_properties() {
        let mut requirements = SecureChannelRequirements::new();

        let required = RequiredSecurityProperty::new(
            property("security::integrity"),
        );

        requirements.require(required.clone()).unwrap();

        assert_eq!(
            requirements.require(required).unwrap_err().kind(),
            SecurityErrorKind::InvalidRequest
        );
    }

    #[test]
    fn validates_provider_capability_declarations() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::integrity"),
            ))
            .unwrap();

        let mut capabilities = SecurityProviderCapabilities::new();

        capabilities
            .add_property(SecurityCapability::new(
                property("security::integrity"),
            ))
            .unwrap();

        assert!(
            validate_declared_capabilities(
                &requirements,
                &capabilities,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_missing_provider_capabilities() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::confidentiality"),
            ))
            .unwrap();

        let capabilities = SecurityProviderCapabilities::new();

        assert_eq!(
            validate_declared_capabilities(
                &requirements,
                &capabilities,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::UnsupportedRequirement
        );
    }

    #[test]
    fn rejects_unavailable_authentication_mechanisms() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .allow_mechanism(mechanism("security::mutual_authentication"))
            .unwrap();

        let capabilities = SecurityProviderCapabilities::new();

        assert_eq!(
            validate_declared_capabilities(
                &requirements,
                &capabilities,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::UnsupportedRequirement
        );
    }

    #[test]
    fn validates_channel_request_identity_consistency() {
        let expected = identity("principals::server");

        let requirements = SecureChannelRequirements::new()
            .expecting_peer(expected.clone());

        let authentication = AuthenticationRequest::new(
            mechanism("security::mutual_authentication"),
        )
        .expecting_identity(expected);

        let request = SecureChannelRequest::new(requirements)
            .with_authentication(authentication);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn rejects_conflicting_channel_and_authentication_identities() {
        let requirements = SecureChannelRequirements::new()
            .expecting_peer(identity("principals::server"));

        let authentication = AuthenticationRequest::new(
            mechanism("security::mutual_authentication"),
        )
        .expecting_identity(identity("principals::different_server"));

        let request = SecureChannelRequest::new(requirements)
            .with_authentication(authentication);

        assert_eq!(
            request.validate().unwrap_err().kind(),
            SecurityErrorKind::IdentityMismatch
        );
    }

    #[test]
    fn creates_authenticated_peer_metadata() {
        let peer = AuthenticatedPeer::verified(
            identity("principals::remote"),
            mechanism("security::mutual_authentication"),
            provider("provider::test"),
            Some("evidence::record-1".to_owned()),
        )
        .unwrap();

        assert_eq!(
            peer.identity().as_str(),
            "principals::remote"
        );

        assert_eq!(
            peer.mechanism().as_str(),
            "security::mutual_authentication"
        );
    }

    #[test]
    fn rejects_invalid_authentication_evidence_reference() {
        let result = AuthenticatedPeer::verified(
            identity("principals::remote"),
            mechanism("security::mutual_authentication"),
            provider("provider::test"),
            Some("invalid\nreference".to_owned()),
        );

        assert_eq!(
            result.unwrap_err().kind(),
            SecurityErrorKind::InvalidAuthenticationEvidence
        );
    }

    #[test]
    fn validates_established_required_properties() {
        let requirements = SecureChannelRequirements::new();

        let mut requirements = requirements;

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::integrity"),
            ))
            .unwrap();

        let request = SecureChannelRequest::new(requirements);

        let established = EstablishedSecurity::new(
            provider("provider::test"),
            vec![
                EstablishedSecurityProperty::new(
                    property("security::integrity"),
                    None,
                    Some("evidence::integrity".to_owned()),
                )
                .unwrap(),
            ],
            None,
        )
        .unwrap();

        assert!(
            validate_established_security(
                &request,
                &established,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_unestablished_required_properties() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .require(RequiredSecurityProperty::new(
                property("security::confidentiality"),
            ))
            .unwrap();

        let request = SecureChannelRequest::new(requirements);

        let established = EstablishedSecurity::new(
            provider("provider::test"),
            Vec::new(),
            None,
        )
        .unwrap();

        assert_eq!(
            validate_established_security(
                &request,
                &established,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::MissingRequiredProperty
        );
    }

    #[test]
    fn rejects_established_forbidden_properties() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .forbid(ForbiddenSecurityProperty::new(
                property("security::legacy_mode"),
            ))
            .unwrap();

        let request = SecureChannelRequest::new(requirements);

        let established = EstablishedSecurity::new(
            provider("provider::test"),
            vec![
                EstablishedSecurityProperty::new(
                    property("security::legacy_mode"),
                    None,
                    None,
                )
                .unwrap(),
            ],
            None,
        )
        .unwrap();

        assert_eq!(
            validate_established_security(
                &request,
                &established,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::ForbiddenProperty
        );
    }

    #[test]
    fn rejects_missing_peer_authentication() {
        let requirements = SecureChannelRequirements::new()
            .expecting_peer(identity("principals::server"));

        let request = SecureChannelRequest::new(requirements);

        let established = EstablishedSecurity::new(
            provider("provider::test"),
            Vec::new(),
            None,
        )
        .unwrap();

        assert_eq!(
            validate_established_security(
                &request,
                &established,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::AuthenticationFailed
        );
    }

    #[test]
    fn rejects_authenticated_peer_identity_mismatch() {
        let requirements = SecureChannelRequirements::new()
            .expecting_peer(identity("principals::expected"));

        let request = SecureChannelRequest::new(requirements);

        let peer = AuthenticatedPeer::verified(
            identity("principals::actual"),
            mechanism("security::mutual_authentication"),
            provider("provider::test"),
            None,
        )
        .unwrap();

        let established = EstablishedSecurity::new(
            provider("provider::test"),
            Vec::new(),
            Some(peer),
        )
        .unwrap();

        assert_eq!(
            validate_established_security(
                &request,
                &established,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::IdentityMismatch
        );
    }

    #[test]
    fn rejects_duplicate_established_properties() {
        let property_id = property("security::integrity");

        let first = EstablishedSecurityProperty::new(
            property_id.clone(),
            None,
            None,
        )
        .unwrap();

        let second = EstablishedSecurityProperty::new(
            property_id,
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            EstablishedSecurity::new(
                provider("provider::test"),
                vec![first, second],
                None,
            )
            .unwrap_err()
            .kind(),
            SecurityErrorKind::InvalidProviderResult
        );
    }

    #[test]
    fn preserves_security_policy_references() {
        let mut requirements = SecureChannelRequirements::new();

        requirements
            .reference_policy("policies::network::protected_service")
            .unwrap();

        assert_eq!(
            requirements.policy_references(),
            &["policies::network::protected_service".to_owned()]
        );
    }

    #[test]
    fn rejects_invalid_policy_references() {
        let mut requirements = SecureChannelRequirements::new();

        assert_eq!(
            requirements
                .reference_policy("invalid\npolicy")
                .unwrap_err()
                .kind(),
            SecurityErrorKind::InvalidRequest
        );
    }

    #[test]
    fn accepts_large_open_world_reference_values() {
        let value = format!("identity::{}", "x".repeat(32_768));

        let identity = IdentityReference::new(value.clone())
            .expect("no arbitrary small identity-reference ceiling");

        assert_eq!(identity.as_str(), value);
    }

    #[test]
    fn error_classifications_have_stable_names() {
        assert_eq!(
            SecurityErrorKind::AuthenticationFailed.as_str(),
            "authentication_failed"
        );

        assert_eq!(
            SecurityErrorKind::MissingRequiredProperty.as_str(),
            "missing_required_property"
        );
    }
}
