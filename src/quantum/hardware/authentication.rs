//! Zamani Quantum Hardware — Authentication
//!
//! Production-grade, provider-neutral authentication contracts for the
//! quantum hardware abstraction layer.
//!
//! # Responsibility
//!
//! This module owns authentication semantics at the hardware boundary.
//!
//! It defines:
//!
//! - authentication mechanisms;
//! - authentication requirements;
//! - credential-material classifications;
//! - authentication context;
//! - authentication state;
//! - authentication results;
//! - authenticated request metadata;
//! - credential redaction;
//! - authentication policy;
//! - retryability classification;
//! - authentication error taxonomy;
//! - provider-neutral authentication traits;
//! - deterministic authentication fingerprints;
//! - security invariants;
//! - audit-safe representations.
//!
//! It deliberately does NOT own:
//!
//! - persistent credential storage;
//! - operating-system keychains;
//! - secret-manager APIs;
//! - provider SDKs;
//! - HTTP clients;
//! - TLS implementation;
//! - OAuth browser flows;
//! - cloud IAM SDKs;
//! - API-token acquisition;
//! - private-key storage;
//! - provider-specific request formats.
//!
//! Those responsibilities belong to `credentials.rs`, provider adapters,
//! transport implementations, or external authentication systems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Quantum IR
//!       |
//!       v
//! Hardware compatibility
//!       |
//!       v
//! QuantumBackend
//!       |
//!       v
//! Provider adapter
//!       |
//!       +-------------------+
//!       |                   |
//!       v                   v
//! authentication       credentials
//!       |                   |
//!       +---------+---------+
//!                 |
//!                 v
//!          provider transport
//!                 |
//!                 v
//!              QPU/API
//! ```
//!
//! Authentication is therefore a boundary between provider-neutral hardware
//! execution and provider-specific credential acquisition/transport.
//!
//! # Critical security rule
//!
//! This module may represent sensitive credential material transiently because
//! an authenticated request must eventually be constructed, but it MUST NOT:
//!
//! - persist credentials;
//! - print credentials;
//! - log credentials;
//! - serialize secret values;
//! - include secrets in `Debug` output;
//! - include secrets in `Display` output;
//! - include secrets in authentication errors;
//! - include secrets in audit records;
//! - expose secrets through backend metadata;
//! - place secrets into backend identifiers.
//!
//! Secret persistence belongs to `credentials.rs` or an external secret
//! provider.
//!
//! # Integration contract
//!
//! Future modules consume this file as follows:
//!
//! - `credentials.rs` supplies credential material;
//! - `backend_config.rs` supplies authentication policy references;
//! - `provider.rs` declares provider authentication requirements;
//! - `adapters/*` implement provider-specific authentication;
//! - `backend_trait.rs` exposes authentication-independent backend operations;
//! - `health.rs` may use authentication checks;
//! - `execution.rs` may require an authenticated provider session;
//! - `errors.rs` may map provider failures into the hardware error hierarchy;
//! - `telemetry.rs` records authentication-safe metrics;
//! - `device_registry.rs` and `discovery.rs` may authenticate discovery calls.
//!
//! No later module needs to modify this file to add a provider.
//!
//! # Provider independence
//!
//! Provider adapters should implement:
//!
//! ```text
//! AuthenticationProvider
//! ```
//!
//! and translate their provider-specific mechanisms into this module's
//! provider-neutral model.
//!
//! Adding a new provider must therefore be possible without changing the
//! authentication core.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Stability
//!
//! This module intentionally provides a stable semantic contract rather than
//! binding Zamani to one provider's authentication protocol.
//!
//! Provider-specific authentication should always be implemented by an
//! adapter.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for authentication objects.
pub const AUTHENTICATION_SCHEMA_ID: &str = "zamani.quantum.hardware.authentication";

/// Semantic schema version.
pub const AUTHENTICATION_SCHEMA_VERSION: u16 = 1;

/// Maximum provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum authentication scheme name length.
pub const MAX_SCHEME_NAME_LENGTH: usize = 128;

/// Maximum authentication scope length.
pub const MAX_SCOPE_LENGTH: usize = 512;

/// Maximum authentication scope count.
pub const MAX_SCOPES: usize = 256;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum authentication metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 1024;

/// Maximum authentication mechanism count in a policy.
pub const MAX_AUTHENTICATION_MECHANISMS: usize = 64;

/// Maximum credential reference length.
pub const MAX_CREDENTIAL_REFERENCE_LENGTH: usize = 2048;

/// Maximum fingerprint length in hexadecimal characters.
pub const AUTHENTICATION_FINGERPRINT_LENGTH: usize = 64;

// =============================================================================
// Authentication mechanism
// =============================================================================

/// Provider-neutral authentication mechanism.
///
/// Provider adapters translate their concrete authentication systems into one
/// of these semantic categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AuthenticationMethod {
    /// No authentication is required.
    Anonymous,

    /// Static API key authentication.
    ApiKey,

    /// Bearer/access-token authentication.
    BearerToken,

    /// OAuth 2.x/OIDC-derived access token.
    OAuth2,

    /// OpenID Connect authentication.
    Oidc,

    /// Cloud-provider IAM authentication.
    CloudIam,

    /// Mutual TLS authentication.
    MutualTls,

    /// Client certificate authentication.
    ClientCertificate,

    /// Signed request authentication.
    SignedRequest,

    /// SSH-style key authentication.
    SshKey,

    /// Hardware security module backed authentication.
    HardwareSecurityModule,

    /// Provider-specific authentication mechanism.
    Custom,
}

impl AuthenticationMethod {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anonymous => "anonymous",
            Self::ApiKey => "api_key",
            Self::BearerToken => "bearer_token",
            Self::OAuth2 => "oauth2",
            Self::Oidc => "oidc",
            Self::CloudIam => "cloud_iam",
            Self::MutualTls => "mutual_tls",
            Self::ClientCertificate => "client_certificate",
            Self::SignedRequest => "signed_request",
            Self::SshKey => "ssh_key",
            Self::HardwareSecurityModule => "hardware_security_module",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the mechanism normally carries secret material.
    pub const fn is_secret_bearing(self) -> bool {
        !matches!(self, Self::Anonymous)
    }

    /// Returns whether the mechanism normally requires an external
    /// credential/identity provider.
    pub const fn requires_external_identity(self) -> bool {
        matches!(
            self,
            Self::OAuth2
                | Self::Oidc
                | Self::CloudIam
                | Self::MutualTls
                | Self::ClientCertificate
                | Self::SignedRequest
                | Self::SshKey
                | Self::HardwareSecurityModule
        )
    }
}

impl fmt::Display for AuthenticationMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Authentication state
// =============================================================================

/// Lifecycle state of authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AuthenticationState {
    /// Authentication has not been attempted.
    Unauthenticated,

    /// Authentication is being established.
    Authenticating,

    /// Authentication succeeded.
    Authenticated,

    /// Authentication is valid but requires refresh.
    RefreshRequired,

    /// Authentication failed.
    Failed,

    /// Authentication has expired.
    Expired,

    /// Authentication has been explicitly revoked.
    Revoked,
}

impl AuthenticationState {
    /// Returns true when an authenticated request may normally be attempted.
    pub const fn is_authenticated(self) -> bool {
        matches!(self, Self::Authenticated)
    }

    /// Returns true when authentication can potentially be refreshed.
    pub const fn requires_refresh(self) -> bool {
        matches!(self, Self::RefreshRequired | Self::Expired)
    }

    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unauthenticated => "unauthenticated",
            Self::Authenticating => "authenticating",
            Self::Authenticated => "authenticated",
            Self::RefreshRequired => "refresh_required",
            Self::Failed => "failed",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
        }
    }
}

impl fmt::Display for AuthenticationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Authentication requirement
// =============================================================================

/// Authentication requirement advertised by a provider/backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationRequirement {
    /// Accepted authentication mechanisms.
    pub methods: Vec<AuthenticationMethod>,

    /// Whether authentication is mandatory.
    pub required: bool,

    /// Whether authentication must be refreshed before expiration.
    pub proactive_refresh: bool,

    /// Whether anonymous operation is explicitly allowed.
    pub anonymous_allowed: bool,

    /// Required scopes.
    pub scopes: Vec<String>,

    /// Provider-defined scheme name.
    pub scheme: Option<String>,
}

impl Default for AuthenticationRequirement {
    fn default() -> Self {
        Self {
            methods: vec![AuthenticationMethod::Anonymous],
            required: false,
            proactive_refresh: false,
            anonymous_allowed: true,
            scopes: Vec::new(),
            scheme: None,
        }
    }
}

impl AuthenticationRequirement {
    /// Creates a mandatory requirement for one authentication mechanism.
    pub fn required(method: AuthenticationMethod) -> Self {
        Self {
            methods: vec![method],
            required: true,
            proactive_refresh: true,
            anonymous_allowed: false,
            scopes: Vec::new(),
            scheme: None,
        }
    }

    /// Creates an optional authentication requirement.
    pub fn optional(method: AuthenticationMethod) -> Self {
        Self {
            methods: vec![method],
            required: false,
            proactive_refresh: false,
            anonymous_allowed: true,
            scopes: Vec::new(),
            scheme: None,
        }
    }

    /// Adds an accepted authentication mechanism.
    pub fn with_method(mut self, method: AuthenticationMethod) -> Self {
        if !self.methods.contains(&method) {
            self.methods.push(method);
        }

        self
    }

    /// Adds a required scope.
    pub fn with_scope(mut self, scope: impl Into<String>) -> Result<Self, AuthenticationError> {
        let scope = validate_scope(scope.into())?;

        if !self.scopes.contains(&scope) {
            if self.scopes.len() >= MAX_SCOPES {
                return Err(AuthenticationError::PolicyLimitExceeded {
                    field: "scopes",
                    limit: MAX_SCOPES,
                });
            }

            self.scopes.push(scope);
            self.scopes.sort();
        }

        Ok(self)
    }

    /// Validates the requirement.
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        if self.methods.is_empty() {
            return Err(AuthenticationError::InvalidRequirement(
                "at least one authentication method is required".to_owned(),
            ));
        }

        if self.methods.len() > MAX_AUTHENTICATION_MECHANISMS {
            return Err(AuthenticationError::PolicyLimitExceeded {
                field: "methods",
                limit: MAX_AUTHENTICATION_MECHANISMS,
            });
        }

        if self.required && self.anonymous_allowed {
            return Err(AuthenticationError::InvalidRequirement(
                "required authentication cannot simultaneously allow anonymous access"
                    .to_owned(),
            ));
        }

        for scope in &self.scopes {
            validate_scope(scope.clone())?;
        }

        if let Some(scheme) = &self.scheme {
            validate_text_field(
                "scheme",
                scheme,
                MAX_SCHEME_NAME_LENGTH,
                false,
            )?;
        }

        Ok(())
    }

    /// Returns whether a method satisfies this requirement.
    pub fn accepts(&self, method: AuthenticationMethod) -> bool {
        self.methods.contains(&method)
    }
}

// =============================================================================
// Credential reference
// =============================================================================

/// Opaque reference to externally managed credential material.
///
/// This is intentionally NOT the credential itself.
///
/// Examples:
///
/// ```text
/// env://ZAMANI_IBM_TOKEN
/// keychain://zamani/ibm/prod
/// secret-manager://project/quantum/ibm
/// hsm://slot/7/key/quantum
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CredentialReference(String);

impl CredentialReference {
    /// Creates a validated credential reference.
    pub fn new(value: impl Into<String>) -> Result<Self, AuthenticationError> {
        let value = value.into();

        validate_text_field(
            "credential_reference",
            &value,
            MAX_CREDENTIAL_REFERENCE_LENGTH,
            false,
        )?;

        if contains_secret_like_value(&value) {
            return Err(AuthenticationError::InsecureCredentialReference);
        }

        Ok(Self(value))
    }

    /// Returns the opaque reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for CredentialReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialReference")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for CredentialReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Sensitive value
// =============================================================================

/// Transient secret value.
///
/// This type deliberately redacts `Debug` and `Display` output.
///
/// It must be treated as short-lived authentication material and must not be
/// persisted by the hardware layer.
#[derive(Clone)]
pub struct SensitiveValue(Arc<str>);

impl SensitiveValue {
    /// Creates a sensitive value.
    ///
    /// Empty values are rejected because an empty credential is almost always
    /// a configuration error and should never be silently interpreted as a
    /// valid credential.
    pub fn new(value: impl Into<String>) -> Result<Self, AuthenticationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(AuthenticationError::EmptyCredential);
        }

        if value.chars().any(char::is_control) {
            return Err(AuthenticationError::InvalidCredential(
                "credential contains control characters".to_owned(),
            ));
        }

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Provides controlled access to the secret.
    ///
    /// Callers must not log, persist, clone into long-lived structures, or
    /// expose the returned value.
    pub fn expose(&self) -> &str {
        &self.0
    }

    /// Returns the length without revealing the value.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the secret is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for SensitiveValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SensitiveValue(REDACTED)")
    }
}

impl fmt::Display for SensitiveValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

// =============================================================================
// Credential material
// =============================================================================

/// Transient credential material supplied to an authentication provider.
///
/// `credentials.rs` should normally construct these values from a secure
/// credential store and should avoid retaining them longer than required.
#[derive(Clone)]
pub enum CredentialMaterial {
    /// No credential.
    Anonymous,

    /// API key.
    ApiKey(SensitiveValue),

    /// Bearer/access token.
    BearerToken(SensitiveValue),

    /// OAuth2 access token.
    OAuth2AccessToken(SensitiveValue),

    /// OIDC access token.
    OidcAccessToken(SensitiveValue),

    /// Cloud IAM token/signature material.
    CloudIam(SensitiveValue),

    /// Client certificate/key reference.
    ClientCertificate {
        /// Opaque reference to the certificate.
        certificate: CredentialReference,

        /// Opaque reference to the private key.
        private_key: CredentialReference,
    },

    /// Mutual TLS identity references.
    MutualTls {
        /// Opaque certificate reference.
        certificate: CredentialReference,

        /// Opaque private-key reference.
        private_key: CredentialReference,
    },

    /// Signed request key material.
    SignedRequest(SensitiveValue),

    /// SSH-style key reference.
    SshKey(CredentialReference),

    /// Hardware security module key reference.
    HardwareSecurityModule(CredentialReference),

    /// Provider-specific credential material.
    Custom {
        /// Provider-specific mechanism identifier.
        mechanism: String,

        /// Opaque sensitive value.
        value: SensitiveValue,
    },
}

impl fmt::Debug for CredentialMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => formatter.write_str("CredentialMaterial::Anonymous"),
            Self::ApiKey(_) => formatter.write_str("CredentialMaterial::ApiKey(REDACTED)"),
            Self::BearerToken(_) => {
                formatter.write_str("CredentialMaterial::BearerToken(REDACTED)")
            }
            Self::OAuth2AccessToken(_) => {
                formatter.write_str("CredentialMaterial::OAuth2AccessToken(REDACTED)")
            }
            Self::OidcAccessToken(_) => {
                formatter.write_str("CredentialMaterial::OidcAccessToken(REDACTED)")
            }
            Self::CloudIam(_) => formatter.write_str("CredentialMaterial::CloudIam(REDACTED)"),
            Self::ClientCertificate { .. } => {
                formatter.write_str("CredentialMaterial::ClientCertificate(REDACTED)")
            }
            Self::MutualTls { .. } => {
                formatter.write_str("CredentialMaterial::MutualTls(REDACTED)")
            }
            Self::SignedRequest(_) => {
                formatter.write_str("CredentialMaterial::SignedRequest(REDACTED)")
            }
            Self::SshKey(_) => formatter.write_str("CredentialMaterial::SshKey(REDACTED)"),
            Self::HardwareSecurityModule(_) => {
                formatter.write_str("CredentialMaterial::HardwareSecurityModule(REDACTED)")
            }
            Self::Custom { mechanism, .. } => formatter
                .debug_struct("CredentialMaterial::Custom")
                .field("mechanism", mechanism)
                .field("value", &"REDACTED")
                .finish(),
        }
    }
}

impl CredentialMaterial {
    /// Returns the semantic authentication method.
    pub fn method(&self) -> AuthenticationMethod {
        match self {
            Self::Anonymous => AuthenticationMethod::Anonymous,
            Self::ApiKey(_) => AuthenticationMethod::ApiKey,
            Self::BearerToken(_) => AuthenticationMethod::BearerToken,
            Self::OAuth2AccessToken(_) => AuthenticationMethod::OAuth2,
            Self::OidcAccessToken(_) => AuthenticationMethod::Oidc,
            Self::CloudIam(_) => AuthenticationMethod::CloudIam,
            Self::ClientCertificate { .. } => AuthenticationMethod::ClientCertificate,
            Self::MutualTls { .. } => AuthenticationMethod::MutualTls,
            Self::SignedRequest(_) => AuthenticationMethod::SignedRequest,
            Self::SshKey(_) => AuthenticationMethod::SshKey,
            Self::HardwareSecurityModule(_) => AuthenticationMethod::HardwareSecurityModule,
            Self::Custom { .. } => AuthenticationMethod::Custom,
        }
    }

    /// Returns whether this material contains an actual secret.
    pub fn is_secret_bearing(&self) -> bool {
        !matches!(self, Self::Anonymous)
    }

    /// Validates the credential material.
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        match self {
            Self::Anonymous => Ok(()),

            Self::ApiKey(value)
            | Self::BearerToken(value)
            | Self::OAuth2AccessToken(value)
            | Self::OidcAccessToken(value)
            | Self::CloudIam(value)
            | Self::SignedRequest(value) => {
                if value.is_empty() {
                    Err(AuthenticationError::EmptyCredential)
                } else {
                    Ok(())
                }
            }

            Self::ClientCertificate {
                certificate,
                private_key,
            }
            | Self::MutualTls {
                certificate,
                private_key,
            } => {
                if certificate == private_key {
                    return Err(AuthenticationError::InvalidCredential(
                        "certificate and private-key references must differ".to_owned(),
                    ));
                }

                Ok(())
            }

            Self::SshKey(reference)
            | Self::HardwareSecurityModule(reference) => {
                if reference.as_str().is_empty() {
                    Err(AuthenticationError::EmptyCredential)
                } else {
                    Ok(())
                }
            }

            Self::Custom { mechanism, value } => {
                validate_text_field(
                    "custom authentication mechanism",
                    mechanism,
                    MAX_SCHEME_NAME_LENGTH,
                    false,
                )?;
                value.validate()?;
                Ok(())
            }
        }
    }
}

// =============================================================================
// Authentication context
// =============================================================================

/// Provider-neutral authentication context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationContext {
    /// Provider identifier.
    pub provider_id: String,

    /// Backend identifier, if authentication targets a specific backend.
    pub backend_id: Option<String>,

    /// Authentication requirement.
    pub requirement: AuthenticationRequirement,

    /// Optional credential reference.
    ///
    /// This is an opaque locator and never contains the secret itself.
    pub credential_reference: Option<CredentialReference>,

    /// Arbitrary non-sensitive provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl AuthenticationContext {
    /// Creates a new authentication context.
    pub fn new(
        provider_id: impl Into<String>,
        requirement: AuthenticationRequirement,
    ) -> Result<Self, AuthenticationError> {
        let provider_id = provider_id.into();

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        requirement.validate()?;

        Ok(Self {
            provider_id,
            backend_id: None,
            requirement,
            credential_reference: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Associates the context with a backend.
    pub fn with_backend(
        mut self,
        backend_id: impl Into<String>,
    ) -> Result<Self, AuthenticationError> {
        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Associates an opaque credential reference.
    pub fn with_credential_reference(
        mut self,
        reference: CredentialReference,
    ) -> Self {
        self.credential_reference = Some(reference);
        self
    }

    /// Adds safe metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AuthenticationError> {
        insert_safe_metadata(
            &mut self.metadata,
            key.into(),
            value.into(),
        )?;
        Ok(self)
    }

    /// Validates the context.
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        validate_identifier(
            "provider_id",
            &self.provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        if let Some(backend_id) = &self.backend_id {
            validate_identifier(
                "backend_id",
                backend_id,
                MAX_BACKEND_ID_LENGTH,
            )?;
        }

        self.requirement.validate()?;

        validate_metadata(&self.metadata)?;

        Ok(())
    }
}

// =============================================================================
// Authentication policy
// =============================================================================

/// Policy controlling how authentication is performed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationPolicy {
    /// Whether authentication failures should be retried.
    pub retry_authentication_failures: bool,

    /// Maximum number of authentication attempts.
    pub max_attempts: u32,

    /// Whether refresh should happen before expiration.
    pub proactive_refresh: bool,

    /// Whether anonymous authentication is allowed when the provider permits
    /// it.
    pub allow_anonymous: bool,

    /// Whether experimental/custom authentication mechanisms are permitted.
    pub allow_custom_methods: bool,

    /// Whether authentication metadata may be emitted into telemetry.
    ///
    /// Secret values are NEVER allowed regardless of this setting.
    pub allow_safe_metadata_telemetry: bool,

    /// Whether provider redirects are permitted.
    ///
    /// This is informational at the hardware boundary; the actual transport
    /// layer must enforce redirect policy.
    pub allow_redirects: bool,
}

impl Default for AuthenticationPolicy {
    fn default() -> Self {
        Self {
            retry_authentication_failures: false,
            max_attempts: 1,
            proactive_refresh: true,
            allow_anonymous: false,
            allow_custom_methods: false,
            allow_safe_metadata_telemetry: true,
            allow_redirects: false,
        }
    }
}

impl AuthenticationPolicy {
    /// Validates the policy.
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        if self.max_attempts == 0 {
            return Err(AuthenticationError::InvalidPolicy(
                "max_attempts must be greater than zero".to_owned(),
            ));
        }

        if self.max_attempts > 16 {
            return Err(AuthenticationError::PolicyLimitExceeded {
                field: "max_attempts",
                limit: 16,
            });
        }

        if self.retry_authentication_failures && self.max_attempts == 1 {
            return Err(AuthenticationError::InvalidPolicy(
                "retry_authentication_failures requires max_attempts > 1".to_owned(),
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Authentication request
// =============================================================================

/// Provider-neutral authentication request.
#[derive(Debug, Clone)]
pub struct AuthenticationRequest {
    /// Authentication context.
    pub context: AuthenticationContext,

    /// Credential material supplied transiently.
    pub credential: CredentialMaterial,

    /// Authentication policy.
    pub policy: AuthenticationPolicy,

    /// Request correlation identifier.
    pub request_id: String,
}

impl AuthenticationRequest {
    /// Creates a request.
    pub fn new(
        context: AuthenticationContext,
        credential: CredentialMaterial,
        request_id: impl Into<String>,
    ) -> Result<Self, AuthenticationError> {
        let request_id = request_id.into();

        validate_identifier(
            "request_id",
            &request_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        context.validate()?;
        credential.validate()?;

        let policy = AuthenticationPolicy::default();
        policy.validate()?;

        Ok(Self {
            context,
            credential,
            policy,
            request_id,
        })
    }

    /// Replaces the authentication policy.
    pub fn with_policy(
        mut self,
        policy: AuthenticationPolicy,
    ) -> Result<Self, AuthenticationError> {
        policy.validate()?;
        self.policy = policy;
        Ok(self)
    }

    /// Validates the complete request.
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        self.context.validate()?;
        self.credential.validate()?;
        self.policy.validate()?;

        validate_identifier(
            "request_id",
            &self.request_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        let method = self.credential.method();

        if !self.context.requirement.accepts(method) {
            return Err(AuthenticationError::UnsupportedMethod {
                method,
                provider_id: self.context.provider_id.clone(),
            });
        }

        if method == AuthenticationMethod::Anonymous
            && !self.context.requirement.anonymous_allowed
        {
            return Err(AuthenticationError::AnonymousNotAllowed);
        }

        if method == AuthenticationMethod::Custom
            && !self.policy.allow_custom_methods
        {
            return Err(AuthenticationError::CustomMethodDisabled);
        }

        if method == AuthenticationMethod::Anonymous
            && !self.policy.allow_anonymous
            && self.context.requirement.required
        {
            return Err(AuthenticationError::AnonymousNotAllowed);
        }

        Ok(())
    }
}

// =============================================================================
// Authenticated request
// =============================================================================

/// Provider-neutral result of successful authentication.
///
/// This structure deliberately does not contain secret material.
///
/// Provider adapters use the result to establish their authenticated
/// transport/session using provider-specific logic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedSession {
    /// Authentication state.
    pub state: AuthenticationState,

    /// Provider identifier.
    pub provider_id: String,

    /// Backend identifier.
    pub backend_id: Option<String>,

    /// Method used.
    pub method: AuthenticationMethod,

    /// Stable non-secret authentication fingerprint.
    pub fingerprint: String,

    /// Whether the session can be refreshed.
    pub refreshable: bool,

    /// Whether proactive refresh is recommended.
    pub refresh_required: bool,

    /// Safe provider metadata.
    pub metadata: BTreeMap<String, String>,
}

impl AuthenticatedSession {
    /// Creates an authenticated session.
    pub fn new(
        provider_id: impl Into<String>,
        backend_id: Option<String>,
        method: AuthenticationMethod,
        fingerprint: String,
        refreshable: bool,
    ) -> Result<Self, AuthenticationError> {
        let provider_id = provider_id.into();

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        if let Some(backend_id) = &backend_id {
            validate_identifier(
                "backend_id",
                backend_id,
                MAX_BACKEND_ID_LENGTH,
            )?;
        }

        validate_fingerprint(&fingerprint)?;

        Ok(Self {
            state: AuthenticationState::Authenticated,
            provider_id,
            backend_id,
            method,
            fingerprint,
            refreshable,
            refresh_required: false,
            metadata: BTreeMap::new(),
        })
    }

    /// Marks the session as requiring refresh.
    pub fn requiring_refresh(mut self) -> Self {
        self.state = AuthenticationState::RefreshRequired;
        self.refresh_required = true;
        self
    }

    /// Adds safe metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AuthenticationError> {
        insert_safe_metadata(
            &mut self.metadata,
            key.into(),
            value.into(),
        )?;
        Ok(self)
    }

    /// Returns whether the session is usable without refresh.
    pub fn is_usable(&self) -> bool {
        self.state.is_authenticated() && !self.refresh_required
    }

    /// Validates the session.
    pub fn validate(&self) -> Result<(), AuthenticationError> {
        if !self.state.is_authenticated()
            && !matches!(self.state, AuthenticationState::RefreshRequired)
        {
            return Err(AuthenticationError::InvalidSession(
                "session is not authenticated".to_owned(),
            ));
        }

        validate_identifier(
            "provider_id",
            &self.provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        if let Some(backend_id) = &self.backend_id {
            validate_identifier(
                "backend_id",
                backend_id,
                MAX_BACKEND_ID_LENGTH,
            )?;
        }

        validate_fingerprint(&self.fingerprint)?;
        validate_metadata(&self.metadata)?;

        Ok(())
    }
}

// =============================================================================
// Authentication result
// =============================================================================

/// Provider-neutral authentication outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationOutcome {
    /// Authentication succeeded.
    Authenticated(AuthenticatedSession),

    /// Authentication requires refresh/re-authentication.
    RefreshRequired {
        /// Stable authentication fingerprint.
        fingerprint: String,
    },

    /// Authentication failed.
    Failed(AuthenticationError),
}

impl AuthenticationOutcome {
    /// Returns the resulting state.
    pub const fn state(&self) -> AuthenticationState {
        match self {
            Self::Authenticated(session) => session.state,
            Self::RefreshRequired { .. } => AuthenticationState::RefreshRequired,
            Self::Failed(_) => AuthenticationState::Failed,
        }
    }

    /// Returns the successful session.
    pub fn session(&self) -> Option<&AuthenticatedSession> {
        match self {
            Self::Authenticated(session) => Some(session),
            Self::RefreshRequired { .. } | Self::Failed(_) => None,
        }
    }

    /// Converts the outcome into a `Result`.
    pub fn into_result(self) -> Result<AuthenticatedSession, AuthenticationError> {
        match self {
            Self::Authenticated(session) => Ok(session),
            Self::RefreshRequired { fingerprint } => {
                Err(AuthenticationError::RefreshRequired { fingerprint })
            }
            Self::Failed(error) => Err(error),
        }
    }
}

// =============================================================================
// Authentication provider trait
// =============================================================================

/// Provider-neutral authentication implementation.
///
/// Each provider adapter implements this trait.
///
/// The trait deliberately does not expose HTTP, OAuth SDKs, cloud SDKs, or
/// provider-specific request types.
pub trait AuthenticationProvider: Send + Sync {
    /// Returns the stable provider identifier.
    fn provider_id(&self) -> &str;

    /// Returns supported authentication requirements.
    fn authentication_requirement(
        &self,
    ) -> Result<AuthenticationRequirement, AuthenticationError>;

    /// Authenticates using transient credential material.
    fn authenticate(
        &self,
        request: &AuthenticationRequest,
    ) -> Result<AuthenticatedSession, AuthenticationError>;

    /// Refreshes an existing session when the provider supports refresh.
    fn refresh(
        &self,
        _session: &AuthenticatedSession,
        _request: &AuthenticationRequest,
    ) -> Result<AuthenticatedSession, AuthenticationError> {
        Err(AuthenticationError::RefreshUnsupported)
    }

    /// Revokes a session when the provider supports explicit revocation.
    fn revoke(
        &self,
        _session: &AuthenticatedSession,
    ) -> Result<(), AuthenticationError> {
        Err(AuthenticationError::RevocationUnsupported)
    }

    /// Performs a provider-neutral authentication health check.
    fn check(
        &self,
        session: &AuthenticatedSession,
    ) -> Result<(), AuthenticationError> {
        session.validate()
    }
}

// =============================================================================
// Authentication manager
// =============================================================================

/// Provider-neutral authentication orchestrator.
///
/// This type owns orchestration only. It does not persist credentials or
/// sessions.
#[derive(Default)]
pub struct AuthenticationManager {
    providers: BTreeMap<String, Arc<dyn AuthenticationProvider>>,
}

impl AuthenticationManager {
    /// Creates an empty authentication manager.
    pub fn new() -> Self {
        Self {
            providers: BTreeMap::new(),
        }
    }

    /// Registers an authentication provider.
    pub fn register(
        &mut self,
        provider: Arc<dyn AuthenticationProvider>,
    ) -> Result<(), AuthenticationError> {
        let provider_id = provider.provider_id();

        validate_identifier(
            "provider_id",
            provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        if self.providers.contains_key(provider_id) {
            return Err(AuthenticationError::ProviderAlreadyRegistered(
                provider_id.to_owned(),
            ));
        }

        let requirement = provider.authentication_requirement()?;
        requirement.validate()?;

        self.providers
            .insert(provider_id.to_owned(), provider);

        Ok(())
    }

    /// Removes a provider.
    pub fn unregister(
        &mut self,
        provider_id: &str,
    ) -> Result<(), AuthenticationError> {
        if self.providers.remove(provider_id).is_none() {
            return Err(AuthenticationError::ProviderNotRegistered(
                provider_id.to_owned(),
            ));
        }

        Ok(())
    }

    /// Gets a provider.
    pub fn provider(
        &self,
        provider_id: &str,
    ) -> Result<&Arc<dyn AuthenticationProvider>, AuthenticationError> {
        self.providers
            .get(provider_id)
            .ok_or_else(|| {
                AuthenticationError::ProviderNotRegistered(
                    provider_id.to_owned(),
                )
            })
    }

    /// Authenticates through a registered provider.
    pub fn authenticate(
        &self,
        request: &AuthenticationRequest,
    ) -> Result<AuthenticatedSession, AuthenticationError> {
        request.validate()?;

        let provider = self.provider(&request.context.provider_id)?;

        let expected = provider.authentication_requirement()?;

        if !expected.accepts(request.credential.method()) {
            return Err(AuthenticationError::UnsupportedMethod {
                method: request.credential.method(),
                provider_id: request.context.provider_id.clone(),
            });
        }

        let session = provider.authenticate(request)?;
        session.validate()?;

        Ok(session)
    }

    /// Refreshes a session.
    pub fn refresh(
        &self,
        request: &AuthenticationRequest,
        session: &AuthenticatedSession,
    ) -> Result<AuthenticatedSession, AuthenticationError> {
        request.validate()?;

        session.validate()?;

        if session.provider_id != request.context.provider_id {
            return Err(AuthenticationError::ProviderMismatch {
                expected: request.context.provider_id.clone(),
                actual: session.provider_id.clone(),
            });
        }

        let provider = self.provider(&session.provider_id)?;
        let refreshed = provider.refresh(session, request)?;

        refreshed.validate()?;

        Ok(refreshed)
    }

    /// Revokes a session.
    pub fn revoke(
        &self,
        session: &AuthenticatedSession,
    ) -> Result<(), AuthenticationError> {
        session.validate()?;

        let provider = self.provider(&session.provider_id)?;
        provider.revoke(session)
    }

    /// Performs an authentication health check.
    pub fn check(
        &self,
        session: &AuthenticatedSession,
    ) -> Result<(), AuthenticationError> {
        session.validate()?;

        let provider = self.provider(&session.provider_id)?;
        provider.check(session)
    }

    /// Returns registered provider IDs in deterministic order.
    pub fn provider_ids(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Returns the number of registered authentication providers.
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether no providers are registered.
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}

// =============================================================================
// Authentication fingerprint
// =============================================================================

/// Creates a deterministic, non-secret authentication fingerprint.
///
/// The fingerprint identifies an authentication context without exposing
/// credential material.
///
/// It must never be treated as a credential.
pub fn authentication_fingerprint(
    provider_id: &str,
    backend_id: Option<&str>,
    method: AuthenticationMethod,
    credential_reference: Option<&CredentialReference>,
) -> Result<String, AuthenticationError> {
    validate_identifier(
        "provider_id",
        provider_id,
        MAX_PROVIDER_ID_LENGTH,
    )?;

    if let Some(backend_id) = backend_id {
        validate_identifier(
            "backend_id",
            backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;
    }

    let mut hasher = Sha256::new();

    hasher.update(AUTHENTICATION_SCHEMA_ID.as_bytes());
    hasher.update([0]);
    hasher.update(AUTHENTICATION_SCHEMA_VERSION.to_be_bytes());
    hasher.update([0]);
    hasher.update(provider_id.as_bytes());
    hasher.update([0]);

    if let Some(backend_id) = backend_id {
        hasher.update(backend_id.as_bytes());
    }

    hasher.update([0]);
    hasher.update(method.as_str().as_bytes());
    hasher.update([0]);

    if let Some(reference) = credential_reference {
        hasher.update(reference.as_str().as_bytes());
    }

    let digest = hasher.finalize();
    let fingerprint = hex::encode(digest);

    validate_fingerprint(&fingerprint)?;

    Ok(fingerprint)
}

// =============================================================================
// Redaction
// =============================================================================

/// Redacts a string that may contain authentication material.
///
/// This is intentionally conservative.
///
/// It is designed for logs, diagnostics and audit messages.
pub fn redact_sensitive_text(input: &str) -> String {
    let lower = input.to_ascii_lowercase();

    let sensitive_markers = [
        "authorization:",
        "proxy-authorization:",
        "api-key:",
        "api_key:",
        "apikey:",
        "access-token:",
        "access_token:",
        "bearer ",
        "token=",
        "password=",
        "passwd=",
        "secret=",
        "private-key:",
        "private_key:",
    ];

    if sensitive_markers
        .iter()
        .any(|marker| lower.contains(marker))
    {
        return "[REDACTED]".to_owned();
    }

    input.to_owned()
}

// =============================================================================
// Authentication errors
// =============================================================================

/// Provider-neutral authentication error taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationError {
    /// Invalid authentication requirement.
    InvalidRequirement(String),

    /// Invalid authentication policy.
    InvalidPolicy(String),

    /// Authentication method is not accepted.
    UnsupportedMethod {
        /// Attempted mechanism.
        method: AuthenticationMethod,

        /// Provider rejecting it.
        provider_id: String,
    },

    /// Anonymous authentication is forbidden.
    AnonymousNotAllowed,

    /// Custom mechanisms are disabled.
    CustomMethodDisabled,

    /// Credential was empty.
    EmptyCredential,

    /// Credential itself was malformed.
    InvalidCredential(String),

    /// Credential reference was unsafe.
    InsecureCredentialReference,

    /// Credential reference cannot be found.
    CredentialNotFound,

    /// Credential has expired.
    CredentialExpired,

    /// Credential has been revoked.
    CredentialRevoked,

    /// Authentication token expired.
    TokenExpired,

    /// Authentication requires refresh.
    RefreshRequired {
        /// Non-secret session fingerprint.
        fingerprint: String,
    },

    /// Provider does not support refresh.
    RefreshUnsupported,

    /// Provider does not support explicit revocation.
    RevocationUnsupported,

    /// Provider rejected credentials.
    AuthenticationRejected,

    /// Provider rejected authorization after successful authentication.
    AuthorizationDenied,

    /// Provider requested authentication again.
    ReauthenticationRequired,

    /// Provider rate limit.
    RateLimited,

    /// Authentication service unavailable.
    ServiceUnavailable,

    /// Network/transport error.
    Transport(String),

    /// TLS failure.
    TlsFailure(String),

    /// Provider API incompatibility.
    ProtocolMismatch(String),

    /// Provider ID mismatch.
    ProviderMismatch {
        /// Expected provider.
        expected: String,

        /// Actual provider.
        actual: String,
    },

    /// Provider is already registered.
    ProviderAlreadyRegistered(String),

    /// Provider was not registered.
    ProviderNotRegistered(String),

    /// Authentication session is invalid.
    InvalidSession(String),

    /// Authentication policy/resource limit exceeded.
    PolicyLimitExceeded {
        /// Policy field.
        field: &'static str,

        /// Maximum allowed value.
        limit: usize,
    },

    /// Metadata is invalid or unsafe.
    InvalidMetadata(String),

    /// Invalid identifier.
    InvalidIdentifier(String),

    /// Invalid fingerprint.
    InvalidFingerprint,

    /// Authentication operation timed out.
    Timeout,

    /// Authentication operation was cancelled.
    Cancelled,

    /// Internal authentication error.
    Internal(String),
}

impl AuthenticationError {
    /// Returns true when retrying without changing credentials is generally
    /// appropriate.
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited
                | Self::ServiceUnavailable
                | Self::Transport(_)
                | Self::TlsFailure(_)
                | Self::Timeout
        )
    }

    /// Returns true when credentials should normally be refreshed/replaced.
    pub const fn requires_credential_refresh(&self) -> bool {
        matches!(
            self,
            Self::CredentialExpired
                | Self::CredentialRevoked
                | Self::TokenExpired
                | Self::RefreshRequired { .. }
                | Self::ReauthenticationRequired
        )
    }

    /// Returns true when retrying the same request without changing the
    /// authorization context should not be attempted.
    pub const fn is_permanent_for_current_credentials(&self) -> bool {
        matches!(
            self,
            Self::AuthenticationRejected
                | Self::AuthorizationDenied
                | Self::CredentialExpired
                | Self::CredentialRevoked
                | Self::InvalidCredential(_)
                | Self::CredentialNotFound
                | Self::AnonymousNotAllowed
                | Self::UnsupportedMethod { .. }
                | Self::CustomMethodDisabled
        )
    }

    /// Stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidRequirement(_) => "AUTH_INVALID_REQUIREMENT",
            Self::InvalidPolicy(_) => "AUTH_INVALID_POLICY",
            Self::UnsupportedMethod { .. } => "AUTH_UNSUPPORTED_METHOD",
            Self::AnonymousNotAllowed => "AUTH_ANONYMOUS_NOT_ALLOWED",
            Self::CustomMethodDisabled => "AUTH_CUSTOM_METHOD_DISABLED",
            Self::EmptyCredential => "AUTH_EMPTY_CREDENTIAL",
            Self::InvalidCredential(_) => "AUTH_INVALID_CREDENTIAL",
            Self::InsecureCredentialReference => "AUTH_INSECURE_CREDENTIAL_REFERENCE",
            Self::CredentialNotFound => "AUTH_CREDENTIAL_NOT_FOUND",
            Self::CredentialExpired => "AUTH_CREDENTIAL_EXPIRED",
            Self::CredentialRevoked => "AUTH_CREDENTIAL_REVOKED",
            Self::TokenExpired => "AUTH_TOKEN_EXPIRED",
            Self::RefreshRequired { .. } => "AUTH_REFRESH_REQUIRED",
            Self::RefreshUnsupported => "AUTH_REFRESH_UNSUPPORTED",
            Self::RevocationUnsupported => "AUTH_REVOCATION_UNSUPPORTED",
            Self::AuthenticationRejected => "AUTH_REJECTED",
            Self::AuthorizationDenied => "AUTHORIZATION_DENIED",
            Self::ReauthenticationRequired => "AUTH_REAUTHENTICATION_REQUIRED",
            Self::RateLimited => "AUTH_RATE_LIMITED",
            Self::ServiceUnavailable => "AUTH_SERVICE_UNAVAILABLE",
            Self::Transport(_) => "AUTH_TRANSPORT",
            Self::TlsFailure(_) => "AUTH_TLS_FAILURE",
            Self::ProtocolMismatch(_) => "AUTH_PROTOCOL_MISMATCH",
            Self::ProviderMismatch { .. } => "AUTH_PROVIDER_MISMATCH",
            Self::ProviderAlreadyRegistered(_) => "AUTH_PROVIDER_ALREADY_REGISTERED",
            Self::ProviderNotRegistered(_) => "AUTH_PROVIDER_NOT_REGISTERED",
            Self::InvalidSession(_) => "AUTH_INVALID_SESSION",
            Self::PolicyLimitExceeded { .. } => "AUTH_POLICY_LIMIT",
            Self::InvalidMetadata(_) => "AUTH_INVALID_METADATA",
            Self::InvalidIdentifier(_) => "AUTH_INVALID_IDENTIFIER",
            Self::InvalidFingerprint => "AUTH_INVALID_FINGERPRINT",
            Self::Timeout => "AUTH_TIMEOUT",
            Self::Cancelled => "AUTH_CANCELLED",
            Self::Internal(_) => "AUTH_INTERNAL",
        }
    }

    /// Returns a safe human-readable message.
    ///
    /// Secret values are intentionally excluded.
    pub fn safe_message(&self) -> String {
        match self {
            Self::InvalidRequirement(message)
            | Self::InvalidPolicy(message)
            | Self::InvalidCredential(message)
            | Self::InvalidSession(message)
            | Self::InvalidMetadata(message)
            | Self::InvalidIdentifier(message)
            | Self::Transport(message)
            | Self::TlsFailure(message)
            | Self::ProtocolMismatch(message)
            | Self::Internal(message) => redact_sensitive_text(message),

            Self::UnsupportedMethod {
                method,
                provider_id,
            } => format!(
                "authentication method `{method}` is not accepted by provider `{}`",
                redact_identifier(provider_id)
            ),

            Self::AnonymousNotAllowed => {
                "anonymous authentication is not allowed".to_owned()
            }

            Self::CustomMethodDisabled => {
                "custom authentication methods are disabled".to_owned()
            }

            Self::EmptyCredential => "credential is empty".to_owned(),

            Self::InsecureCredentialReference => {
                "credential reference is unsafe".to_owned()
            }

            Self::CredentialNotFound => {
                "credential could not be resolved".to_owned()
            }

            Self::CredentialExpired => "credential has expired".to_owned(),

            Self::CredentialRevoked => "credential has been revoked".to_owned(),

            Self::TokenExpired => "authentication token has expired".to_owned(),

            Self::RefreshRequired { fingerprint } => format!(
                "authentication refresh is required for session `{}`",
                redact_identifier(fingerprint)
            ),

            Self::RefreshUnsupported => {
                "provider does not support authentication refresh".to_owned()
            }

            Self::RevocationUnsupported => {
                "provider does not support authentication revocation".to_owned()
            }

            Self::AuthenticationRejected => {
                "provider rejected authentication".to_owned()
            }

            Self::AuthorizationDenied => {
                "provider denied authorization".to_owned()
            }

            Self::ReauthenticationRequired => {
                "provider requires re-authentication".to_owned()
            }

            Self::RateLimited => {
                "authentication request was rate limited".to_owned()
            }

            Self::ServiceUnavailable => {
                "authentication service is unavailable".to_owned()
            }

            Self::ProviderMismatch { expected, actual } => format!(
                "authentication provider mismatch: expected `{}`, received `{}`",
                redact_identifier(expected),
                redact_identifier(actual)
            ),

            Self::ProviderAlreadyRegistered(provider) => format!(
                "authentication provider `{}` is already registered",
                redact_identifier(provider)
            ),

            Self::ProviderNotRegistered(provider) => format!(
                "authentication provider `{}` is not registered",
                redact_identifier(provider)
            ),

            Self::PolicyLimitExceeded { field, limit } => format!(
                "authentication policy field `{field}` exceeds limit {limit}"
            ),

            Self::InvalidFingerprint => {
                "authentication fingerprint is invalid".to_owned()
            }

            Self::Timeout => "authentication operation timed out".to_owned(),

            Self::Cancelled => "authentication operation was cancelled".to_owned(),
        }
    }
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.safe_message())
    }
}

impl std::error::Error for AuthenticationError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum_length: usize,
) -> Result<(), AuthenticationError> {
    if value.is_empty() {
        return Err(AuthenticationError::InvalidIdentifier(format!(
            "{field} must not be empty"
        )));
    }

    if value.len() > maximum_length {
        return Err(AuthenticationError::InvalidIdentifier(format!(
            "{field} exceeds maximum length of {maximum_length}"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(AuthenticationError::InvalidIdentifier(format!(
            "{field} contains control characters"
        )));
    }

    if value.trim() != value {
        return Err(AuthenticationError::InvalidIdentifier(format!(
            "{field} contains leading or trailing whitespace"
        )));
    }

    Ok(())
}

fn validate_text_field(
    field: &'static str,
    value: &str,
    maximum_length: usize,
    allow_empty: bool,
) -> Result<(), AuthenticationError> {
    if !allow_empty && value.is_empty() {
        return Err(AuthenticationError::InvalidMetadata(
            format!("{field} must not be empty"),
        ));
    }

    if value.len() > maximum_length {
        return Err(AuthenticationError::InvalidMetadata(
            format!("{field} exceeds maximum length of {maximum_length}"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(AuthenticationError::InvalidMetadata(
            format!("{field} contains control characters"),
        ));
    }

    Ok(())
}

fn validate_scope(scope: String) -> Result<String, AuthenticationError> {
    validate_text_field(
        "scope",
        &scope,
        MAX_SCOPE_LENGTH,
        false,
    )?;

    if scope.chars().any(char::is_whitespace) {
        return Err(AuthenticationError::InvalidRequirement(
            "authentication scopes must not contain whitespace".to_owned(),
        ));
    }

    Ok(scope)
}

fn validate_fingerprint(
    fingerprint: &str,
) -> Result<(), AuthenticationError> {
    if fingerprint.len() != AUTHENTICATION_FINGERPRINT_LENGTH {
        return Err(AuthenticationError::InvalidFingerprint);
    }

    if !fingerprint
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(AuthenticationError::InvalidFingerprint);
    }

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> Result<(), AuthenticationError> {
    if metadata.len() > MAX_METADATA_ENTRIES {
        return Err(AuthenticationError::PolicyLimitExceeded {
            field: "metadata",
            limit: MAX_METADATA_ENTRIES,
        });
    }

    for (key, value) in metadata {
        validate_text_field(
            "metadata key",
            key,
            MAX_METADATA_KEY_LENGTH,
            false,
        )?;

        validate_text_field(
            "metadata value",
            value,
            MAX_METADATA_VALUE_LENGTH,
            true,
        )?;

        if is_sensitive_metadata_key(key)
            || contains_secret_like_value(value)
        {
            return Err(AuthenticationError::InvalidMetadata(
                format!(
                    "metadata field `{}` appears to contain sensitive authentication material",
                    redact_identifier(key)
                ),
            ));
        }
    }

    Ok(())
}

fn insert_safe_metadata(
    metadata: &mut BTreeMap<String, String>,
    key: String,
    value: String,
) -> Result<(), AuthenticationError> {
    if metadata.len() >= MAX_METADATA_ENTRIES
        && !metadata.contains_key(&key)
    {
        return Err(AuthenticationError::PolicyLimitExceeded {
            field: "metadata",
            limit: MAX_METADATA_ENTRIES,
        });
    }

    validate_metadata_key(&key)?;
    validate_text_field(
        "metadata value",
        &value,
        MAX_METADATA_VALUE_LENGTH,
        true,
    )?;

    if contains_secret_like_value(&value) {
        return Err(AuthenticationError::InvalidMetadata(
            "metadata value appears to contain secret material".to_owned(),
        ));
    }

    metadata.insert(key, value);
    Ok(())
}

fn validate_metadata_key(key: &str) -> Result<(), AuthenticationError> {
    validate_text_field(
        "metadata key",
        key,
        MAX_METADATA_KEY_LENGTH,
        false,
    )?;

    if is_sensitive_metadata_key(key) {
        return Err(AuthenticationError::InvalidMetadata(
            "sensitive metadata keys are forbidden".to_owned(),
        ));
    }

    Ok(())
}

fn is_sensitive_metadata_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();

    [
        "authorization",
        "proxy_authorization",
        "api_key",
        "apikey",
        "access_token",
        "refresh_token",
        "token",
        "password",
        "passwd",
        "secret",
        "private_key",
        "privatekey",
        "client_secret",
        "credential",
        "credentials",
        "cookie",
        "set_cookie",
    ]
    .iter()
    .any(|marker| key == *marker || key.contains(marker))
}

fn contains_secret_like_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    [
        "bearer ",
        "authorization:",
        "api-key:",
        "api_key:",
        "apikey:",
        "access-token:",
        "access_token:",
        "password=",
        "passwd=",
        "secret=",
        "private-key:",
        "private_key:",
        "-----begin",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn redact_identifier(value: &str) -> String {
    if value.len() <= 8 {
        return "[REDACTED]".to_owned();
    }

    let prefix = &value[..4];
    let suffix = &value[value.len() - 4..];

    format!("{prefix}…{suffix}")
}

// =============================================================================
// Stable fingerprint hashing
// =============================================================================

/// Wrapper used when an authentication fingerprint needs deterministic
/// `Hash` semantics without hashing secret material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticationIdentity {
    /// Provider identifier.
    pub provider_id: String,

    /// Backend identifier.
    pub backend_id: Option<String>,

    /// Authentication mechanism.
    pub method: AuthenticationMethod,

    /// Credential reference, if any.
    pub credential_reference: Option<CredentialReference>,
}

impl AuthenticationIdentity {
    /// Builds an authentication identity.
    pub fn new(
        provider_id: impl Into<String>,
        backend_id: Option<String>,
        method: AuthenticationMethod,
        credential_reference: Option<CredentialReference>,
    ) -> Result<Self, AuthenticationError> {
        let provider_id = provider_id.into();

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        if let Some(backend_id) = &backend_id {
            validate_identifier(
                "backend_id",
                backend_id,
                MAX_BACKEND_ID_LENGTH,
            )?;
        }

        Ok(Self {
            provider_id,
            backend_id,
            method,
            credential_reference,
        })
    }

    /// Computes a deterministic non-secret fingerprint.
    pub fn fingerprint(&self) -> Result<String, AuthenticationError> {
        authentication_fingerprint(
            &self.provider_id,
            self.backend_id.as_deref(),
            self.method,
            self.credential_reference.as_ref(),
        )
    }
}

impl Hash for AuthenticationIdentity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.provider_id.hash(state);
        self.backend_id.hash(state);
        self.method.hash(state);
        self.credential_reference.hash(state);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct AnonymousProvider;

    impl AuthenticationProvider for AnonymousProvider {
        fn provider_id(&self) -> &str {
            "local"
        }

        fn authentication_requirement(
            &self,
        ) -> Result<AuthenticationRequirement, AuthenticationError> {
            Ok(AuthenticationRequirement {
                methods: vec![AuthenticationMethod::Anonymous],
                required: false,
                proactive_refresh: false,
                anonymous_allowed: true,
                scopes: Vec::new(),
                scheme: None,
            })
        }

        fn authenticate(
            &self,
            request: &AuthenticationRequest,
        ) -> Result<AuthenticatedSession, AuthenticationError> {
            request.validate()?;

            let fingerprint = authentication_fingerprint(
                &request.context.provider_id,
                request.context.backend_id.as_deref(),
                request.credential.method(),
                request.context.credential_reference.as_ref(),
            )?;

            AuthenticatedSession::new(
                request.context.provider_id.clone(),
                request.context.backend_id.clone(),
                AuthenticationMethod::Anonymous,
                fingerprint,
                false,
            )
        }
    }

    #[test]
    fn authentication_method_identifiers_are_stable() {
        assert_eq!(
            AuthenticationMethod::Anonymous.as_str(),
            "anonymous"
        );
        assert_eq!(
            AuthenticationMethod::ApiKey.as_str(),
            "api_key"
        );
        assert_eq!(
            AuthenticationMethod::BearerToken.as_str(),
            "bearer_token"
        );
        assert_eq!(
            AuthenticationMethod::MutualTls.as_str(),
            "mutual_tls"
        );
    }

    #[test]
    fn sensitive_values_are_redacted() {
        let value = SensitiveValue::new("super-secret-token")
            .expect("secret should be accepted");

        assert_eq!(value.to_string(), "[REDACTED]");
        assert_eq!(
            format!("{value:?}"),
            "SensitiveValue(REDACTED)"
        );
        assert_eq!(value.expose(), "super-secret-token");
    }

    #[test]
    fn empty_sensitive_values_are_rejected() {
        assert!(matches!(
            SensitiveValue::new(""),
            Err(AuthenticationError::EmptyCredential)
        ));
    }

    #[test]
    fn credential_references_are_opaque() {
        let reference = CredentialReference::new(
            "env://ZAMANI_QUANTUM_TOKEN",
        )
        .expect("reference should be valid");

        assert_eq!(
            reference.as_str(),
            "env://ZAMANI_QUANTUM_TOKEN"
        );
    }

    #[test]
    fn credential_references_cannot_contain_secrets() {
        assert!(matches!(
            CredentialReference::new(
                "env://token=super-secret"
            ),
            Err(AuthenticationError::InsecureCredentialReference)
        ));
    }

    #[test]
    fn default_requirement_allows_anonymous() {
        let requirement = AuthenticationRequirement::default();

        assert!(requirement.validate().is_ok());
        assert!(requirement.accepts(AuthenticationMethod::Anonymous));
        assert!(requirement.anonymous_allowed);
    }

    #[test]
    fn required_authentication_rejects_anonymous() {
        let requirement =
            AuthenticationRequirement::required(
                AuthenticationMethod::ApiKey,
            );

        assert!(requirement.validate().is_ok());
        assert!(!requirement.anonymous_allowed);
        assert!(!requirement.accepts(
            AuthenticationMethod::Anonymous
        ));
    }

    #[test]
    fn required_authentication_cannot_allow_anonymous() {
        let requirement = AuthenticationRequirement {
            methods: vec![AuthenticationMethod::ApiKey],
            required: true,
            proactive_refresh: true,
            anonymous_allowed: true,
            scopes: Vec::new(),
            scheme: None,
        };

        assert!(matches!(
            requirement.validate(),
            Err(AuthenticationError::InvalidRequirement(_))
        ));
    }

    #[test]
    fn scopes_are_normalized() {
        let requirement =
            AuthenticationRequirement::default()
                .with_scope("quantum.execute")
                .expect("scope should be valid");

        assert_eq!(
            requirement.scopes,
            vec!["quantum.execute".to_owned()]
        );
    }

    #[test]
    fn scope_whitespace_is_rejected() {
        let result =
            AuthenticationRequirement::default()
                .with_scope("quantum execute");

        assert!(result.is_err());
    }

    #[test]
    fn sensitive_metadata_is_rejected() {
        let context = AuthenticationContext::new(
            "local",
            AuthenticationRequirement::default(),
        )
        .expect("context should be valid");

        let result = context.with_metadata(
            "access_token",
            "secret",
        );

        assert!(matches!(
            result,
            Err(AuthenticationError::InvalidMetadata(_))
        ));
    }

    #[test]
    fn sensitive_metadata_values_are_rejected() {
        let context = AuthenticationContext::new(
            "local",
            AuthenticationRequirement::default(),
        )
        .expect("context should be valid");

        let result = context.with_metadata(
            "provider_status",
            "Authorization: Bearer secret",
        );

        assert!(matches!(
            result,
            Err(AuthenticationError::InvalidMetadata(_))
        ));
    }

    #[test]
    fn authentication_fingerprint_is_deterministic() {
        let first = authentication_fingerprint(
            "provider",
            Some("backend"),
            AuthenticationMethod::ApiKey,
            None,
        )
        .expect("fingerprint should succeed");

        let second = authentication_fingerprint(
            "provider",
            Some("backend"),
            AuthenticationMethod::ApiKey,
            None,
        )
        .expect("fingerprint should succeed");

        assert_eq!(first, second);
        assert_eq!(
            first.len(),
            AUTHENTICATION_FINGERPRINT_LENGTH
        );
    }

    #[test]
    fn different_authentication_contexts_have_different_fingerprints() {
        let first = authentication_fingerprint(
            "provider-a",
            Some("backend"),
            AuthenticationMethod::ApiKey,
            None,
        )
        .expect("fingerprint should succeed");

        let second = authentication_fingerprint(
            "provider-b",
            Some("backend"),
            AuthenticationMethod::ApiKey,
            None,
        )
        .expect("fingerprint should succeed");

        assert_ne!(first, second);
    }

    #[test]
    fn credential_material_reports_method() {
        let credential = CredentialMaterial::ApiKey(
            SensitiveValue::new("secret")
                .expect("secret should be valid"),
        );

        assert_eq!(
            credential.method(),
            AuthenticationMethod::ApiKey
        );
        assert!(credential.is_secret_bearing());
    }

    #[test]
    fn anonymous_credential_is_not_secret_bearing() {
        let credential = CredentialMaterial::Anonymous;

        assert_eq!(
            credential.method(),
            AuthenticationMethod::Anonymous
        );
        assert!(!credential.is_secret_bearing());
        assert!(credential.validate().is_ok());
    }

    #[test]
    fn policy_rejects_zero_attempts() {
        let policy = AuthenticationPolicy {
            max_attempts: 0,
            ..AuthenticationPolicy::default()
        };

        assert!(matches!(
            policy.validate(),
            Err(AuthenticationError::InvalidPolicy(_))
        ));
    }

    #[test]
    fn policy_rejects_retry_with_one_attempt() {
        let policy = AuthenticationPolicy {
            retry_authentication_failures: true,
            max_attempts: 1,
            ..AuthenticationPolicy::default()
        };

        assert!(matches!(
            policy.validate(),
            Err(AuthenticationError::InvalidPolicy(_))
        ));
    }

    #[test]
    fn authentication_error_codes_are_stable() {
        assert_eq!(
            AuthenticationError::AuthenticationRejected.code(),
            "AUTH_REJECTED"
        );

        assert_eq!(
            AuthenticationError::CredentialExpired.code(),
            "AUTH_CREDENTIAL_EXPIRED"
        );

        assert_eq!(
            AuthenticationError::AuthorizationDenied.code(),
            "AUTHORIZATION_DENIED"
        );
    }

    #[test]
    fn authentication_errors_do_not_leak_secrets() {
        let error = AuthenticationError::InvalidCredential(
            "password=super-secret".to_owned(),
        );

        let rendered = error.to_string();

        assert!(!rendered.contains("super-secret"));
        assert!(!rendered.contains("password=super-secret"));
        assert!(rendered.contains("[REDACTED]"));
    }

    #[test]
    fn redact_sensitive_text_is_conservative() {
        assert_eq!(
            redact_sensitive_text(
                "Authorization: Bearer very-secret-token"
            ),
            "[REDACTED]"
        );

        assert_eq!(
            redact_sensitive_text("ordinary provider message"),
            "ordinary provider message"
        );
    }

    #[test]
    fn authenticated_session_validates() {
        let fingerprint = authentication_fingerprint(
            "local",
            None,
            AuthenticationMethod::Anonymous,
            None,
        )
        .expect("fingerprint should succeed");

        let session = AuthenticatedSession::new(
            "local",
            None,
            AuthenticationMethod::Anonymous,
            fingerprint,
            false,
        )
        .expect("session should be valid");

        assert!(session.validate().is_ok());
        assert!(session.is_usable());
    }

    #[test]
    fn refresh_state_is_not_usable() {
        let fingerprint = authentication_fingerprint(
            "local",
            None,
            AuthenticationMethod::Anonymous,
            None,
        )
        .expect("fingerprint should succeed");

        let session = AuthenticatedSession::new(
            "local",
            None,
            AuthenticationMethod::Anonymous,
            fingerprint,
            false,
        )
        .expect("session should be valid")
        .requiring_refresh();

        assert!(!session.is_usable());
        assert!(session.refresh_required);
    }

    #[test]
    fn anonymous_provider_can_authenticate() {
        let mut manager = AuthenticationManager::new();

        manager
            .register(Arc::new(AnonymousProvider))
            .expect("provider should register");

        let context = AuthenticationContext::new(
            "local",
            AuthenticationRequirement::default(),
        )
        .expect("context should be valid");

        let request = AuthenticationRequest::new(
            context,
            CredentialMaterial::Anonymous,
            "request-1",
        )
        .expect("request should be valid")
        .with_policy(AuthenticationPolicy {
            allow_anonymous: true,
            ..AuthenticationPolicy::default()
        })
        .expect("policy should be valid");

        let session = manager
            .authenticate(&request)
            .expect("authentication should succeed");

        assert_eq!(
            session.provider_id,
            "local"
        );
        assert_eq!(
            session.method,
            AuthenticationMethod::Anonymous
        );
        assert!(session.is_usable());
    }

    #[test]
    fn duplicate_provider_registration_is_rejected() {
        let mut manager = AuthenticationManager::new();

        manager
            .register(Arc::new(AnonymousProvider))
            .expect("first registration should succeed");

        let result =
            manager.register(Arc::new(AnonymousProvider));

        assert!(matches!(
            result,
            Err(AuthenticationError::ProviderAlreadyRegistered(_))
        ));
    }

    #[test]
    fn missing_provider_is_rejected() {
        let manager = AuthenticationManager::new();

        assert!(matches!(
            manager.provider("missing"),
            Err(AuthenticationError::ProviderNotRegistered(_))
        ));
    }

    #[test]
    fn provider_ids_are_deterministic() {
        struct Provider {
            id: &'static str,
        }

        impl AuthenticationProvider for Provider {
            fn provider_id(&self) -> &str {
                self.id
            }

            fn authentication_requirement(
                &self,
            ) -> Result<
                AuthenticationRequirement,
                AuthenticationError,
            > {
                Ok(AuthenticationRequirement::default())
            }

            fn authenticate(
                &self,
                _request: &AuthenticationRequest,
            ) -> Result<
                AuthenticatedSession,
                AuthenticationError,
            > {
                Err(AuthenticationError::AuthenticationRejected)
            }
        }

        let mut manager = AuthenticationManager::new();

        manager
            .register(Arc::new(Provider { id: "z-provider" }))
            .expect("registration should succeed");

        manager
            .register(Arc::new(Provider { id: "a-provider" }))
            .expect("registration should succeed");

        assert_eq!(
            manager.provider_ids(),
            vec![
                "a-provider".to_owned(),
                "z-provider".to_owned()
            ]
        );
    }

    #[test]
    fn retryability_is_classified() {
        assert!(
            AuthenticationError::RateLimited.is_retryable()
        );

        assert!(
            AuthenticationError::ServiceUnavailable
                .is_retryable()
        );

        assert!(
            !AuthenticationError::AuthenticationRejected
                .is_retryable()
        );
    }

    #[test]
    fn credential_refresh_is_classified() {
        assert!(
            AuthenticationError::TokenExpired
                .requires_credential_refresh()
        );

        assert!(
            AuthenticationError::CredentialExpired
                .requires_credential_refresh()
        );

        assert!(
            !AuthenticationError::AuthenticationRejected
                .requires_credential_refresh()
        );
    }

    #[test]
    fn schema_constants_are_stable() {
        assert_eq!(
            AUTHENTICATION_SCHEMA_ID,
            "zamani.quantum.hardware.authentication"
        );

        assert_eq!(
            AUTHENTICATION_SCHEMA_VERSION,
            1
        );
    }
}