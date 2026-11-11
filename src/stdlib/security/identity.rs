
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Identity
//!
//! Path: `src/stdlib/security/identity.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! ## Purpose
//!
//! Defines portable identity and principal contracts for Zamani programs.
//!
//! This module represents identities and their relationships without
//! authenticating subjects, evaluating trust, granting permissions,
//! storing credentials, or selecting physical execution targets.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Open-world identity identifiers.
//! - Open-world principal identifiers.
//! - Open-world identity-kind identifiers.
//! - Authority and namespace references.
//! - Portable identity records.
//! - Non-secret identity metadata.
//! - Identity bindings.
//! - Identity lifecycle descriptions.
//! - Identity provider contracts.
//! - Structured identity errors.
//!
//! It does not own:
//!
//! - Authentication.
//! - Authorization.
//! - Trust evaluation.
//! - Cryptographic execution.
//! - Private-key storage.
//! - Credential storage.
//! - Permission definitions.
//! - Capability definitions.
//! - Semantic identity validation.
//! - Hardware discovery.
//! - Distributed consensus.
//! - Runtime scheduling.
//!
//! ## Integration boundaries
//!
//! Source-level identity declarations are owned by:
//!
//!     grammar/security/identifiers.g4
//!
//! The singular grammar integration façade is:
//!
//!     grammar/security/identity.g4
//!
//! Security-wide grammar composition is owned by:
//!
//!     grammar/security/security.g4
//!
//! Trust evaluation is owned by the security trust subsystem.
//!
//! Authentication and authorization are owned by their respective
//! security/runtime authorities.
//!
//! Cryptographic key lifecycle is owned by:
//!
//!     crate::stdlib::security::key_management
//!
//! Cryptographic operations are owned by:
//!
//!     crate::stdlib::security::cryptography
//!
//! Semantic identity validation remains owned by:
//!
//!     crate::frontend::semantic
//!
//! ## Security invariants
//!
//! - An identity record is not proof of a real-world identity.
//! - Registering an identity does not authenticate its subject.
//! - Resolving an identity does not establish trust.
//! - A principal does not automatically possess permissions.
//! - An identity binding does not prove that two subjects are equivalent.
//! - Identity metadata must not contain passwords, private keys, tokens,
//!   secret seeds, or other credential material.
//! - Providers must enforce their own storage and access protections.
//! - Errors must not disclose credential or secret material.
//!
//! ## Scalability
//!
//! This API imposes no language-level maximum on:
//!
//! - Identities.
//! - Principals.
//! - Authorities.
//! - Namespaces.
//! - Bindings.
//! - Metadata entries.
//! - Providers.
//! - Distributed subjects.
//! - Devices.
//! - Services.
//! - Nodes.
//!
//! Actual limits are determined by the provider, available memory,
//! storage, operational budgets, and the execution environment.
//!
//! No finite implementation promises literal infinite storage or execution.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::Arc;

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates an open-world qualified identifier.
///
/// Identifier components use the same general shape as the repository's
/// security and cryptography identifier contracts.
///
/// This is structural validation only. Semantic name resolution remains
/// the responsibility of the frontend semantic authority.
fn valid_identifier(value: &str) -> bool {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
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

/// Validates an opaque external reference.
///
/// External references may contain URI-like punctuation and namespace
/// separators. They are not interpreted or dereferenced by this module.
fn valid_reference(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_control)
        && !value.contains('\0')
}

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of identity failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum IdentityErrorKind {
    /// An identity identifier is invalid.
    InvalidIdentity,

    /// A principal identifier is invalid.
    InvalidPrincipal,

    /// An identity kind is invalid.
    InvalidIdentityKind,

    /// An authority identifier is invalid.
    InvalidAuthority,

    /// A namespace identifier is invalid.
    InvalidNamespace,

    /// An external identity reference is invalid.
    InvalidReference,

    /// An identity record is invalid.
    InvalidRecord,

    /// Identity metadata is invalid.
    InvalidMetadata,

    /// An identity binding is invalid.
    InvalidBinding,

    /// A required identity was not found.
    NotFound,

    /// An identity already exists.
    AlreadyExists,

    /// An identity is in an incompatible lifecycle state.
    InvalidState,

    /// The provider does not support the requested operation.
    UnsupportedOperation,

    /// The required provider capability is unavailable.
    CapabilityUnavailable,

    /// The provider is unavailable.
    ProviderUnavailable,

    /// The provider returned an invalid result.
    InvalidProviderResult,

    /// The provider operation failed.
    ProviderFailure,

    /// The operation was denied by the provider.
    Denied,

    /// The operation was cancelled.
    Cancelled,

    /// The operation timed out.
    TimedOut,

    /// A concurrency or version conflict occurred.
    Conflict,
}

impl IdentityErrorKind {
    /// Returns a stable machine-readable error identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentity => "invalid_identity",
            Self::InvalidPrincipal => "invalid_principal",
            Self::InvalidIdentityKind => "invalid_identity_kind",
            Self::InvalidAuthority => "invalid_authority",
            Self::InvalidNamespace => "invalid_namespace",
            Self::InvalidReference => "invalid_reference",
            Self::InvalidRecord => "invalid_record",
            Self::InvalidMetadata => "invalid_metadata",
            Self::InvalidBinding => "invalid_binding",
            Self::NotFound => "not_found",
            Self::AlreadyExists => "already_exists",
            Self::InvalidState => "invalid_state",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::InvalidProviderResult => "invalid_provider_result",
            Self::ProviderFailure => "provider_failure",
            Self::Denied => "denied",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Conflict => "conflict",
        }
    }
}

impl fmt::Display for IdentityErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Structured identity error.
///
/// The error deliberately excludes identity credentials and secret material.
pub struct IdentityError {
    kind: IdentityErrorKind,
    provider: Option<IdentityProviderId>,
    identity: Option<IdentityId>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl IdentityError {
    /// Creates an identity error.
    #[must_use]
    pub fn new(kind: IdentityErrorKind) -> Self {
        Self {
            kind,
            provider: None,
            identity: None,
            source: None,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> IdentityErrorKind {
        self.kind
    }

    /// Returns the associated provider, if known.
    #[must_use]
    pub fn provider(&self) -> Option<&IdentityProviderId> {
        self.provider.as_ref()
    }

    /// Returns the associated identity, if known.
    #[must_use]
    pub fn identity(&self) -> Option<&IdentityId> {
        self.identity.as_ref()
    }

    /// Associates a provider with the error.
    #[must_use]
    pub fn with_provider(mut self, provider: IdentityProviderId) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Associates an identity with the error.
    #[must_use]
    pub fn with_identity(mut self, identity: IdentityId) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Attaches a non-secret diagnostic source.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Debug for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityError")
            .field("kind", &self.kind)
            .field("provider", &self.provider)
            .field("identity", &self.identity)
            .field(
                "source",
                &self.source.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "identity operation failed: {}", self.kind)?;

        if let Some(identity) = &self.identity {
            write!(f, " (identity: {identity})")?;
        }

        if let Some(provider) = &self.provider {
            write!(f, " (provider: {provider})")?;
        }

        Ok(())
    }
}

impl StdError for IdentityError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for identity operations.
pub type IdentityResult<T> = Result<T, IdentityError>;

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

/// Defines a validated open-world identity identifier.
macro_rules! identity_identifier {
    ($name:ident, $error:expr) => {
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            pub fn new(value: impl Into<String>) -> IdentityResult<Self> {
                let value = value.into();

                if !valid_identifier(&value) {
                    return Err(IdentityError::new($error));
                }

                Ok(Self(value))
            }

            /// Returns the identifier text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

identity_identifier!(IdentityId, IdentityErrorKind::InvalidIdentity);
identity_identifier!(PrincipalId, IdentityErrorKind::InvalidPrincipal);
identity_identifier!(IdentityKind, IdentityErrorKind::InvalidIdentityKind);
identity_identifier!(IdentityAuthorityId, IdentityErrorKind::InvalidAuthority);
identity_identifier!(IdentityNamespace, IdentityErrorKind::InvalidNamespace);
identity_identifier!(IdentityProviderId, IdentityErrorKind::InvalidReference);

// ============================================================================
// OPAQUE EXTERNAL REFERENCES
// ============================================================================

/// Opaque reference to an identity managed by an external authority.
///
/// This is an identifier, not a credential or proof of authentication.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExternalIdentityReference(String);

impl ExternalIdentityReference {
    /// Creates an opaque external identity reference.
    pub fn new(value: impl Into<String>) -> IdentityResult<Self> {
        let value = value.into();

        if !valid_reference(&value) {
            return Err(IdentityError::new(
                IdentityErrorKind::InvalidReference,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the external reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ExternalIdentityReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ExternalIdentityReference([REDACTED])")
    }
}

impl fmt::Display for ExternalIdentityReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[EXTERNAL IDENTITY REFERENCE]")
    }
}

// ============================================================================
// IDENTITY LIFECYCLE DESCRIPTION
// ============================================================================

/// Describes the lifecycle state reported by an identity provider.
///
/// These states describe provider-managed identity records. They do not
/// establish authentication, authorization, or trust.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum IdentityStatus {
    /// The identity is active according to its provider.
    Active,

    /// The identity is temporarily suspended.
    Suspended,

    /// The identity has been deactivated.
    Deactivated,
}

// ============================================================================
// NON-SECRET METADATA
// ============================================================================

/// A validated identity metadata key.
///
/// Metadata keys are open-world identifiers. They are not a fixed catalogue
/// of identity attributes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdentityMetadataKey(String);

impl IdentityMetadataKey {
    /// Creates a metadata key.
    pub fn new(value: impl Into<String>) -> IdentityResult<Self> {
        let value = value.into();

        if !valid_identifier(&value) {
            return Err(IdentityError::new(
                IdentityErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the metadata key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A non-secret identity metadata value.
///
/// This type is intended for descriptive, non-sensitive metadata only.
/// It must never be used for passwords, tokens, private keys, secret seeds,
/// credentials, or other authentication material.
#[derive(Clone, Eq, PartialEq)]
pub struct IdentityMetadataValue(String);

impl IdentityMetadataValue {
    /// Creates a non-secret metadata value.
    pub fn new(value: impl Into<String>) -> IdentityResult<Self> {
        let value = value.into();

        if value.chars().any(char::is_control) {
            return Err(IdentityError::new(
                IdentityErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the metadata value.
    ///
    /// Callers must not place secrets in identity metadata.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for IdentityMetadataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IdentityMetadataValue([REDACTED])")
    }
}

/// Non-secret metadata associated with an identity.
///
/// The map has no language-level maximum size. Provider and memory limits
/// remain operational constraints.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IdentityMetadata {
    entries: BTreeMap<IdentityMetadataKey, IdentityMetadataValue>,
}

impl IdentityMetadata {
    /// Creates empty identity metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a metadata entry.
    pub fn insert(
        &mut self,
        key: IdentityMetadataKey,
        value: IdentityMetadataValue,
    ) -> Option<IdentityMetadataValue> {
        self.entries.insert(key, value)
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(
        &self,
        key: &IdentityMetadataKey,
    ) -> Option<&IdentityMetadataValue> {
        self.entries.get(key)
    }

    /// Removes a metadata entry.
    pub fn remove(
        &mut self,
        key: &IdentityMetadataKey,
    ) -> Option<IdentityMetadataValue> {
        self.entries.remove(key)
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates over metadata entries in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (&IdentityMetadataKey, &IdentityMetadataValue),
    > {
        self.entries.iter()
    }
}

// ============================================================================
// IDENTITY BINDINGS
// ============================================================================

/// A binding between a Zamani identity and an external identity namespace.
///
/// A binding is a reference relationship only. It does not prove that the
/// external subject has been authenticated or that the binding is trusted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityBinding {
    namespace: IdentityNamespace,
    external_identity: ExternalIdentityReference,
}

impl IdentityBinding {
    /// Creates an identity binding.
    #[must_use]
    pub fn new(
        namespace: IdentityNamespace,
        external_identity: ExternalIdentityReference,
    ) -> Self {
        Self {
            namespace,
            external_identity,
        }
    }

    /// Returns the external namespace.
    #[must_use]
    pub fn namespace(&self) -> &IdentityNamespace {
        &self.namespace
    }

    /// Returns the external identity reference.
    #[must_use]
    pub fn external_identity(&self) -> &ExternalIdentityReference {
        &self.external_identity
    }
}

// ============================================================================
// IDENTITY RECORD
// ============================================================================

/// A portable identity record.
///
/// The record contains descriptive identity information and references only.
/// It never contains private keys, passwords, tokens, or credential bytes.
///
/// The record is not proof that the identity is authentic or trusted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityRecord {
    id: IdentityId,
    kind: IdentityKind,
    authority: IdentityAuthorityId,
    status: IdentityStatus,
    metadata: IdentityMetadata,
    bindings: Vec<IdentityBinding>,
    revision: Option<String>,
}

impl IdentityRecord {
    /// Creates an identity record.
    ///
    /// The authority is a semantic/provider reference. This constructor does
    /// not contact or validate that authority.
    #[must_use]
    pub fn new(
        id: IdentityId,
        kind: IdentityKind,
        authority: IdentityAuthorityId,
    ) -> Self {
        Self {
            id,
            kind,
            authority,
            status: IdentityStatus::Active,
            metadata: IdentityMetadata::new(),
            bindings: Vec::new(),
            revision: None,
        }
    }

    /// Returns the identity identifier.
    #[must_use]
    pub fn id(&self) -> &IdentityId {
        &self.id
    }

    /// Returns the open-world identity kind.
    #[must_use]
    pub fn kind(&self) -> &IdentityKind {
        &self.kind
    }

    /// Returns the identity authority.
    #[must_use]
    pub fn authority(&self) -> &IdentityAuthorityId {
        &self.authority
    }

    /// Returns the provider-reported lifecycle state.
    #[must_use]
    pub const fn status(&self) -> IdentityStatus {
        self.status
    }

    /// Returns non-secret metadata.
    #[must_use]
    pub fn metadata(&self) -> &IdentityMetadata {
        &self.metadata
    }

    /// Returns identity bindings.
    #[must_use]
    pub fn bindings(&self) -> &[IdentityBinding] {
        &self.bindings
    }

    /// Returns the provider revision or concurrency token.
    ///
    /// The value is opaque. It must not be interpreted as a timestamp,
    /// monotonic counter, or globally ordered version.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }

    /// Replaces non-secret metadata.
    pub fn set_metadata(&mut self, metadata: IdentityMetadata) {
        self.metadata = metadata;
    }

    /// Adds an external identity binding.
    pub fn add_binding(&mut self, binding: IdentityBinding) {
        self.bindings.push(binding);
    }

    /// Removes all bindings for a namespace.
    ///
    /// Returns the number of removed bindings.
    pub fn remove_bindings(
        &mut self,
        namespace: &IdentityNamespace,
    ) -> usize {
        let previous = self.bindings.len();

        self.bindings
            .retain(|binding| binding.namespace() != namespace);

        previous - self.bindings.len()
    }

    /// Sets a provider revision token.
    ///
    /// Providers should use opaque concurrency tokens to prevent lost
    /// updates when multiple writers modify an identity.
    pub fn set_revision(
        &mut self,
        revision: Option<String>,
    ) -> IdentityResult<()> {
        if revision
            .as_deref()
            .is_some_and(|value| !valid_reference(value))
        {
            return Err(IdentityError::new(
                IdentityErrorKind::InvalidRecord,
            ));
        }

        self.revision = revision;
        Ok(())
    }

    /// Updates the lifecycle description.
    ///
    /// This changes only the local record value. Persisting a transition
    /// requires a successful provider operation.
    pub fn set_status(&mut self, status: IdentityStatus) {
        self.status = status;
    }
}

// ============================================================================
// PRINCIPAL DESCRIPTION
// ============================================================================

/// A principal reference.
///
/// A principal identifies a subject that may participate in authorization.
/// It does not grant permissions or establish authentication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Principal {
    id: PrincipalId,
    identity: IdentityId,
    authority: IdentityAuthorityId,
}

impl Principal {
    /// Creates a principal reference.
    #[must_use]
    pub fn new(
        id: PrincipalId,
        identity: IdentityId,
        authority: IdentityAuthorityId,
    ) -> Self {
        Self {
            id,
            identity,
            authority,
        }
    }

    /// Returns the principal identifier.
    #[must_use]
    pub fn id(&self) -> &PrincipalId {
        &self.id
    }

    /// Returns the associated identity identifier.
    #[must_use]
    pub fn identity(&self) -> &IdentityId {
        &self.identity
    }

    /// Returns the authority reference.
    #[must_use]
    pub fn authority(&self) -> &IdentityAuthorityId {
        &self.authority
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Provider-neutral identity storage and resolution contract.
///
/// Implementations may use local databases, remote identity services,
/// operating-system identity facilities, distributed directories, or
/// hardware-backed identity systems.
///
/// This trait does not make any particular implementation trusted.
///
/// Provider implementations are responsible for:
///
/// - Authenticating and authorizing administrative operations.
/// - Validating identity authority and uniqueness.
/// - Protecting stored identity metadata.
/// - Enforcing concurrency and revision semantics.
/// - Returning accurate lifecycle state.
/// - Avoiding secret material in identity records.
/// - Applying their own audit and retention policies.
/// - Handling distributed consistency according to their documented model.
///
/// Provider operations must not claim success when persistence or resolution
/// has failed.
pub trait IdentityProvider: Send + Sync {
    /// Returns the provider identity.
    fn id(&self) -> &IdentityProviderId;

    /// Resolves an identity by its canonical identifier.
    ///
    /// Returns `NotFound` when no such identity exists.
    fn resolve(
        &self,
        identity: &IdentityId,
    ) -> IdentityResult<IdentityRecord>;

    /// Registers a new identity.
    ///
    /// Implementations must enforce uniqueness within their declared
    /// authority and consistency scope.
    fn register(
        &self,
        record: IdentityRecord,
    ) -> IdentityResult<IdentityRecord>;

    /// Updates an existing identity.
    ///
    /// Implementations should use the record's opaque revision token to
    /// detect conflicting updates where the underlying store supports it.
    fn update(
        &self,
        record: IdentityRecord,
    ) -> IdentityResult<IdentityRecord>;

    /// Deactivates an identity.
    ///
    /// This is an identity-record lifecycle operation. It does not revoke
    /// cryptographic keys, credentials, permissions, or trust relationships.
    fn deactivate(
        &self,
        identity: &IdentityId,
        expected_revision: Option<&str>,
    ) -> IdentityResult<IdentityRecord>;
}

// ============================================================================
// DIRECTORY FACADE
// ============================================================================

/// A lightweight identity directory backed by an explicit provider.
///
/// The directory does not contain a second identity database or registry.
/// It delegates all persistent operations to the supplied provider.
///
/// Authorization must be enforced by the provider or by a higher-level
/// security service before exposing administrative operations to callers.
#[derive(Clone)]
pub struct IdentityDirectory {
    provider: Arc<dyn IdentityProvider>,
}

impl IdentityDirectory {
    /// Creates a directory backed by an identity provider.
    #[must_use]
    pub fn new(provider: Arc<dyn IdentityProvider>) -> Self {
        Self { provider }
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &IdentityProviderId {
        self.provider.id()
    }

    /// Resolves an identity.
    pub fn resolve(
        &self,
        identity: &IdentityId,
    ) -> IdentityResult<IdentityRecord> {
        self.provider.resolve(identity).map_err(|error| {
            attach_provider(error, self.provider.id())
        })
    }

    /// Registers an identity.
    pub fn register(
        &self,
        record: IdentityRecord,
    ) -> IdentityResult<IdentityRecord> {
        self.provider.register(record).map_err(|error| {
            attach_provider(error, self.provider.id())
        })
    }

    /// Updates an identity.
    pub fn update(
        &self,
        record: IdentityRecord,
    ) -> IdentityResult<IdentityRecord> {
        self.provider.update(record).map_err(|error| {
            attach_provider(error, self.provider.id())
        })
    }

    /// Deactivates an identity.
    pub fn deactivate(
        &self,
        identity: &IdentityId,
        expected_revision: Option<&str>,
    ) -> IdentityResult<IdentityRecord> {
        self.provider
            .deactivate(identity, expected_revision)
            .map_err(|error| attach_provider(error, self.provider.id()))
    }
}

/// Adds provider context to an error without exposing provider internals.
fn attach_provider(
    mut error: IdentityError,
    provider: &IdentityProviderId,
) -> IdentityError {
    if error.provider.is_none() {
        error.provider = Some(provider.clone());
    }

    error
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_id() -> IdentityId {
        IdentityId::new("application::worker").unwrap()
    }

    fn identity_kind() -> IdentityKind {
        IdentityKind::new("infrastructure::service").unwrap()
    }

    fn authority_id() -> IdentityAuthorityId {
        IdentityAuthorityId::new("authority::zamani").unwrap()
    }

    #[test]
    fn accepts_open_world_identity_kinds() {
        let current = IdentityKind::new("application::user").unwrap();
        let future = IdentityKind::new("future::computational_subject").unwrap();

        assert_ne!(current, future);
    }

    #[test]
    fn rejects_malformed_identifiers() {
        assert!(IdentityId::new("").is_err());
        assert!(IdentityId::new(" application::user").is_err());
        assert!(IdentityId::new("application::").is_err());
        assert!(IdentityId::new("application::bad value").is_err());
    }

    #[test]
    fn external_references_are_opaque_in_debug() {
        let reference = ExternalIdentityReference::new(
            "directory://tenant/private-subject",
        )
        .unwrap();

        assert!(!format!("{reference:?}").contains("private-subject"));
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = IdentityMetadata::new();

        let z = IdentityMetadataKey::new("profile::zone").unwrap();
        let a = IdentityMetadataKey::new("profile::alias").unwrap();

        metadata.insert(
            z.clone(),
            IdentityMetadataValue::new("zone-a").unwrap(),
        );

        metadata.insert(
            a.clone(),
            IdentityMetadataValue::new("worker").unwrap(),
        );

        let keys: Vec<&str> = metadata
            .iter()
            .map(|(key, _)| key.as_str())
            .collect();

        assert_eq!(keys, vec!["profile::alias", "profile::zone"]);
    }

    #[test]
    fn metadata_debug_does_not_print_values() {
        let value = IdentityMetadataValue::new("sensitive-looking-value").unwrap();

        assert!(!format!("{value:?}").contains("sensitive-looking-value"));
    }

    #[test]
    fn bindings_are_references_not_trust_assertions() {
        let binding = IdentityBinding::new(
            IdentityNamespace::new("external::directory").unwrap(),
            ExternalIdentityReference::new("subject:123").unwrap(),
        );

        assert_eq!(
            binding.namespace().as_str(),
            "external::directory"
        );
    }

    #[test]
    fn identity_record_has_no_credential_fields() {
        let record = IdentityRecord::new(
            identity_id(),
            identity_kind(),
            authority_id(),
        );

        assert_eq!(record.status(), IdentityStatus::Active);
        assert!(record.metadata().is_empty());
        assert!(record.bindings().is_empty());
        assert!(record.revision().is_none());
    }

    #[test]
    fn binding_removal_is_namespace_scoped() {
        let mut record = IdentityRecord::new(
            identity_id(),
            identity_kind(),
            authority_id(),
        );

        let first_namespace =
            IdentityNamespace::new("directory::primary").unwrap();

        let second_namespace =
            IdentityNamespace::new("directory::secondary").unwrap();

        record.add_binding(IdentityBinding::new(
            first_namespace.clone(),
            ExternalIdentityReference::new("subject:first").unwrap(),
        ));

        record.add_binding(IdentityBinding::new(
            second_namespace,
            ExternalIdentityReference::new("subject:second").unwrap(),
        ));

        assert_eq!(record.remove_bindings(&first_namespace), 1);
        assert_eq!(record.bindings().len(), 1);
    }

    #[test]
    fn revision_tokens_are_validated() {
        let mut record = IdentityRecord::new(
            identity_id(),
            identity_kind(),
            authority_id(),
        );

        assert!(record.set_revision(Some("revision-1".to_owned())).is_ok());
        assert!(record.set_revision(Some("bad\nrevision".to_owned())).is_err());
    }

    #[test]
    fn principal_does_not_contain_permissions() {
        let principal = Principal::new(
            PrincipalId::new("principal::worker").unwrap(),
            identity_id(),
            authority_id(),
        );

        assert_eq!(principal.id().as_str(), "principal::worker");
        assert_eq!(principal.identity().as_str(), "application::worker");
    }

    #[test]
    fn error_classifications_are_stable() {
        let error = IdentityError::new(IdentityErrorKind::NotFound);

        assert_eq!(error.kind().as_str(), "not_found");
    }

    #[test]
    fn error_debug_does_not_expose_source_diagnostics() {
        #[derive(Debug)]
        struct PrivateDiagnostic;

        impl fmt::Display for PrivateDiagnostic {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("private provider diagnostic")
            }
        }

        impl StdError for PrivateDiagnostic {}

        let error = IdentityError::new(IdentityErrorKind::ProviderFailure)
            .with_source(PrivateDiagnostic);

        assert!(!format!("{error:?}").contains("private provider diagnostic"));
    }
}
