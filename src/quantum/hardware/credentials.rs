//! Zamani Quantum — Credential References and Credential Policy
//!
//! Production-grade, provider-independent credential boundary for the Zamani
//! Quantum Hardware Abstraction Layer.
//!
//! # Responsibility
//!
//! This module defines the safe representation of HOW quantum hardware
//! credentials are referenced and governed.
//!
//! It owns:
//!
//! - credential identity;
//! - credential kind;
//! - credential source;
//! - credential scope;
//! - provider/account/backend binding;
//! - authentication-purpose binding;
//! - environment-variable references;
//! - OS keychain references;
//! - external secret-manager references;
//! - HSM references;
//! - credential-reference validation;
//! - credential-reference normalization;
//! - credential policy;
//! - expiration policy;
//! - rotation policy;
//! - secret-material detection;
//! - safe redaction;
//! - deterministic equality and ordering;
//! - provider-neutral integration contracts.
//!
//! It deliberately does NOT own:
//!
//! - API keys;
//! - bearer tokens;
//! - passwords;
//! - private keys;
//! - certificates containing private key material;
//! - session cookies;
//! - OAuth access tokens;
//! - OAuth refresh tokens;
//! - secret-manager SDKs;
//! - OS keychain APIs;
//! - HSM APIs;
//! - network communication;
//! - authentication sessions;
//! - credential retrieval;
//! - credential caching;
//! - credential rotation execution;
//! - provider SDKs;
//! - provider-specific authentication protocols;
//! - backend execution;
//! - backend discovery;
//! - backend configuration;
//! - benchmarking;
//! - Danga.
//!
//! Those responsibilities belong to `authentication.rs`, provider adapters,
//! secret-management integrations, and the relevant execution boundaries.
//!
//! # Critical security invariant
//!
//! This module MUST NEVER contain actual secret material.
//!
//! A credential reference is metadata describing where a credential can be
//! obtained. It is not the credential itself.
//!
//! For example, this is valid:
//!
//! ```text
//! source = environment
//! reference = ZAMANI_IBM_API_KEY
//! ```
//!
//! This is NOT valid:
//!
//! ```text
//! reference = sk-actual-secret-value
//! ```
//!
//! Likewise, this module must never expose APIs such as:
//!
//! ```text
//! get_secret()
//! secret_value()
//! api_key()
//! token()
//! password()
//! ```
//!
//! Actual secret resolution belongs to the authentication layer.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                                |
//!                                v
//!                      compatibility / routing
//!                                |
//!                                v
//!                              backend
//!                                |
//!                                v
//!                         backend_config
//!                                |
//!                                v
//!                         execution / job
//!                                |
//!                                v
//!                        provider adapter
//!                                |
//!                         authentication
//!                                |
//!                    +-----------+-----------+
//!                    |           |           |
//!                    v           v           v
//!                 env/keychain secret manager HSM
//!                    |           |           |
//!                    +-----------+-----------+
//!                                |
//!                                v
//!                       authenticated transport
//!                                |
//!                                v
//!                              QPU
//! ```
//!
//! `credentials.rs` sits BEFORE secret resolution.
//!
//! # Integration contract
//!
//! The following modules consume this contract:
//!
//! - `backend_config.rs`
//! - `authentication.rs`
//! - `provider.rs`
//! - `provider_registry.rs`
//! - `device_registry.rs`
//! - `discovery.rs`
//! - provider adapters;
//! - `execution.rs`;
//! - `job.rs`;
//! - Danga.
//!
//! These modules MUST use `CredentialReference` rather than storing secrets.
//!
//! `backend_config.rs` may contain a credential reference, but must never
//! contain a credential value.
//!
//! `authentication.rs` is responsible for resolving a `CredentialReference`
//! into an authentication mechanism. It MUST NOT change the semantics of this
//! file.
//!
//! Provider adapters may define provider-specific credential requirements,
//! but provider-specific secret values must remain outside this module.
//!
//! # No-re-edit rule
//!
//! This file intentionally depends only on the Rust standard library.
//!
//! It does not depend on:
//!
//! - `backend.rs`;
//! - `backend_config.rs`;
//! - `authentication.rs`;
//! - `provider.rs`;
//! - provider SDKs;
//! - networking;
//! - serialization;
//! - operating-system APIs.
//!
//! This allows this file to be completed and frozen before those modules are
//! implemented.
//!
//! Later modules must adapt to this stable contract rather than requiring this
//! file to be modified.
//!
//! # Serialization boundary
//!
//! This module intentionally does not implement Serde serialization.
//!
//! `serialization.rs` owns external serialization.
//!
//! A serialized credential reference may contain metadata such as:
//!
//! ```text
//! credential_id
//! kind
//! source
//! provider
//! account
//! backend
//! purpose
//! scope
//! ```
//!
//! It MUST NEVER contain a resolved secret.
//!
//! # Logging boundary
//!
//! `CredentialReference` implements safe `Debug` and `Display` behaviour.
//!
//! No method in this module returns secret material.
//!
//! Future authentication implementations MUST preserve this invariant when
//! logging authentication failures.
//!
//! # Determinism
//!
//! Credential references are deterministic values.
//!
//! They contain no:
//!
//! - system clock reads;
//! - randomness;
//! - network state;
//! - process state;
//! - environment reads;
//! - secret-manager calls.
//!
//! Environment variables are represented by NAME, but this module never reads
//! the variable.
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
//! # Design principles
//!
//! 1. References, not secrets.
//! 2. Authentication, not credentials, resolves secrets.
//! 3. Provider-specific authentication stays in adapters.
//! 4. Backend configuration stores references, never values.
//! 5. Serialization stores references, never values.
//! 6. Logs contain identifiers and policy information, never secret material.
//! 7. Credential scope must be explicit.
//! 8. Credential purpose must be explicit.
//! 9. Expiration and rotation policy must be explicit.
//! 10. Invalid or suspicious references are rejected before authentication.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for credential references.
pub const CREDENTIAL_SCHEMA_ID: &str = "zamani.quantum.hardware.credentials";

/// Semantic schema version.
///
/// Increment only when the meaning of the public credential-reference
/// contract changes incompatibly.
pub const CREDENTIAL_SCHEMA_VERSION: u16 = 1;

/// Maximum credential identifier length.
pub const MAX_CREDENTIAL_ID_LENGTH: usize = 256;

/// Maximum provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum account/project/workspace reference length.
pub const MAX_ACCOUNT_REFERENCE_LENGTH: usize = 512;

/// Maximum scope reference length.
pub const MAX_SCOPE_REFERENCE_LENGTH: usize = 512;

/// Maximum environment-variable name length.
pub const MAX_ENVIRONMENT_VARIABLE_LENGTH: usize = 256;

/// Maximum keychain service length.
pub const MAX_KEYCHAIN_SERVICE_LENGTH: usize = 256;

/// Maximum keychain account length.
pub const MAX_KEYCHAIN_ACCOUNT_LENGTH: usize = 512;

/// Maximum secret-manager reference length.
pub const MAX_SECRET_MANAGER_REFERENCE_LENGTH: usize = 2048;

/// Maximum HSM reference length.
pub const MAX_HSM_REFERENCE_LENGTH: usize = 2048;

/// Maximum credential label length.
pub const MAX_CREDENTIAL_LABEL_LENGTH: usize = 256;

/// Maximum credential description length.
pub const MAX_CREDENTIAL_DESCRIPTION_LENGTH: usize = 2048;

/// Maximum scope count.
pub const MAX_CREDENTIAL_SCOPES: usize = 64;

/// Maximum authentication-purpose count.
pub const MAX_CREDENTIAL_PURPOSES: usize = 32;

/// Maximum credential lifetime.
pub const MAX_CREDENTIAL_LIFETIME_SECONDS: u64 = 365 * 24 * 60 * 60;

/// Maximum rotation interval.
pub const MAX_ROTATION_INTERVAL_SECONDS: u64 = 365 * 24 * 60 * 60;

/// Maximum grace period.
pub const MAX_ROTATION_GRACE_PERIOD_SECONDS: u64 = 30 * 24 * 60 * 60;

// =============================================================================
// Credential kind
// =============================================================================

/// Type of credential expected by an authentication implementation.
///
/// This describes the authentication material conceptually. It does NOT
/// contain the material itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CredentialKind {
    /// Static API key.
    ApiKey,

    /// Bearer/access token.
    AccessToken,

    /// OAuth2 access/refresh credential pair.
    OAuth2,

    /// OpenID Connect-derived authentication.
    Oidc,

    /// Username/password credential.
    ///
    /// The password is never stored by this module.
    UsernamePassword,

    /// TLS client certificate/private-key credential.
    ///
    /// The certificate/private key material is never stored here.
    Mtls,

    /// SSH-style key authentication.
    ///
    /// Private-key material is never stored here.
    SshKey,

    /// Cloud provider IAM role/session identity.
    CloudIam,

    /// Hardware security module-backed identity.
    Hsm,

    /// Workload identity.
    WorkloadIdentity,

    /// Provider-specific credential mechanism.
    Custom,
}

impl CredentialKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApiKey => "api_key",
            Self::AccessToken => "access_token",
            Self::OAuth2 => "oauth2",
            Self::Oidc => "oidc",
            Self::UsernamePassword => "username_password",
            Self::Mtls => "mtls",
            Self::SshKey => "ssh_key",
            Self::CloudIam => "cloud_iam",
            Self::Hsm => "hsm",
            Self::WorkloadIdentity => "workload_identity",
            Self::Custom => "custom",
        }
    }

    /// Returns true when this kind normally contains a bearer-style secret.
    pub const fn is_bearer_like(self) -> bool {
        matches!(
            self,
            Self::ApiKey | Self::AccessToken | Self::OAuth2 | Self::Oidc
        )
    }

    /// Returns true when this kind normally contains private key material.
    pub const fn is_key_based(self) -> bool {
        matches!(self, Self::Mtls | Self::SshKey | Self::Hsm)
    }
}

impl fmt::Display for CredentialKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Credential source
// =============================================================================

/// Location from which an authentication implementation may resolve a
/// credential.
///
/// This enum contains only references. It never reads from these sources.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CredentialSource {
    /// Environment variable reference.
    Environment {
        /// Environment variable name.
        variable: String,
    },

    /// Operating-system keychain reference.
    OsKeychain {
        /// Keychain service name.
        service: String,

        /// Keychain account/item name.
        account: String,
    },

    /// External secret-manager reference.
    SecretManager {
        /// Provider-neutral secret reference.
        reference: String,
    },

    /// Hardware-security-module reference.
    Hsm {
        /// Provider-neutral HSM object/reference.
        reference: String,
    },

    /// Cloud workload identity.
    WorkloadIdentity {
        /// Identity/provider reference.
        reference: String,
    },

    /// Explicitly external credential reference.
    External {
        /// Provider-neutral external reference.
        reference: String,
    },
}

impl CredentialSource {
    /// Returns a stable machine-readable source identifier.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Environment { .. } => "environment",
            Self::OsKeychain { .. } => "os_keychain",
            Self::SecretManager { .. } => "secret_manager",
            Self::Hsm { .. } => "hsm",
            Self::WorkloadIdentity { .. } => "workload_identity",
            Self::External { .. } => "external",
        }
    }

    /// Returns whether this source is expected to be local to the execution
    /// environment.
    pub const fn is_local(&self) -> bool {
        matches!(
            self,
            Self::Environment { .. } | Self::OsKeychain { .. }
        )
    }

    /// Returns a safe human-readable reference without resolving anything.
    pub fn safe_reference(&self) -> String {
        match self {
            Self::Environment { variable } => {
                format!("environment:{variable}")
            }
            Self::OsKeychain { service, account } => {
                format!("os_keychain:{service}:{account}")
            }
            Self::SecretManager { reference } => {
                format!("secret_manager:{reference}")
            }
            Self::Hsm { reference } => {
                format!("hsm:{reference}")
            }
            Self::WorkloadIdentity { reference } => {
                format!("workload_identity:{reference}")
            }
            Self::External { reference } => {
                format!("external:{reference}")
            }
        }
    }

    /// Validates the credential source.
    pub fn validate(&self) -> Result<(), CredentialError> {
        match self {
            Self::Environment { variable } => {
                validate_environment_variable(variable)
            }

            Self::OsKeychain { service, account } => {
                validate_bounded_string(
                    "source.service",
                    service,
                    MAX_KEYCHAIN_SERVICE_LENGTH,
                    false,
                )?;

                validate_bounded_string(
                    "source.account",
                    account,
                    MAX_KEYCHAIN_ACCOUNT_LENGTH,
                    false,
                )?;

                reject_secret_material("source.account", account)?;

                Ok(())
            }

            Self::SecretManager { reference } => {
                validate_bounded_string(
                    "source.reference",
                    reference,
                    MAX_SECRET_MANAGER_REFERENCE_LENGTH,
                    false,
                )?;

                reject_secret_material("source.reference", reference)
            }

            Self::Hsm { reference } => {
                validate_bounded_string(
                    "source.reference",
                    reference,
                    MAX_HSM_REFERENCE_LENGTH,
                    false,
                )?;

                reject_secret_material("source.reference", reference)
            }

            Self::WorkloadIdentity { reference } => {
                validate_bounded_string(
                    "source.reference",
                    reference,
                    MAX_SECRET_MANAGER_REFERENCE_LENGTH,
                    false,
                )?;

                reject_secret_material("source.reference", reference)
            }

            Self::External { reference } => {
                validate_bounded_string(
                    "source.reference",
                    reference,
                    MAX_SECRET_MANAGER_REFERENCE_LENGTH,
                    false,
                )?;

                reject_secret_material("source.reference", reference)
            }
        }
    }
}

// =============================================================================
// Credential scope
// =============================================================================

/// Scope restricting where a credential may be used.
///
/// Scope is authorization metadata, not the credential itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CredentialScope {
    /// Credential is restricted to a provider.
    Provider(String),

    /// Credential is restricted to a backend.
    Backend(String),

    /// Credential is restricted to an account/project/workspace.
    Account(String),

    /// Credential is restricted to a named region.
    Region(String),

    /// Credential is restricted to a provider-defined scope.
    Custom(String),
}

impl CredentialScope {
    /// Returns a stable machine-readable identifier.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Provider(_) => "provider",
            Self::Backend(_) => "backend",
            Self::Account(_) => "account",
            Self::Region(_) => "region",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns the referenced value.
    pub fn value(&self) -> &str {
        match self {
            Self::Provider(value)
            | Self::Backend(value)
            | Self::Account(value)
            | Self::Region(value)
            | Self::Custom(value) => value,
        }
    }

    /// Validates the scope.
    pub fn validate(&self) -> Result<(), CredentialError> {
        validate_bounded_string(
            "scope",
            self.value(),
            MAX_SCOPE_REFERENCE_LENGTH,
            false,
        )?;

        reject_secret_material("scope", self.value())
    }
}

// =============================================================================
// Authentication purpose
// =============================================================================

/// Intended purpose for a credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuthenticationPurpose {
    /// Backend API access.
    BackendApi,

    /// Backend discovery.
    Discovery,

    /// Job submission.
    JobSubmission,

    /// Job status polling.
    JobStatus,

    /// Result retrieval.
    ResultRetrieval,

    /// Job cancellation.
    Cancellation,

    /// Health checking.
    HealthCheck,

    /// Calibration retrieval.
    Calibration,

    /// Queue information.
    QueueInformation,

    /// Provider account/project access.
    AccountManagement,

    /// Provider-defined purpose.
    Custom,
}

impl AuthenticationPurpose {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackendApi => "backend_api",
            Self::Discovery => "discovery",
            Self::JobSubmission => "job_submission",
            Self::JobStatus => "job_status",
            Self::ResultRetrieval => "result_retrieval",
            Self::Cancellation => "cancellation",
            Self::HealthCheck => "health_check",
            Self::Calibration => "calibration",
            Self::QueueInformation => "queue_information",
            Self::AccountManagement => "account_management",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for AuthenticationPurpose {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Credential ID
// =============================================================================

/// Stable logical identifier for a credential reference.
///
/// This is an identifier only. It is never a secret.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CredentialId(String);

impl CredentialId {
    /// Creates a validated credential ID.
    pub fn new(value: impl Into<String>) -> Result<Self, CredentialError> {
        let value = value.into();

        validate_identifier(
            "credential_id",
            &value,
            MAX_CREDENTIAL_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for CredentialId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for CredentialId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for CredentialId {
    type Err = CredentialError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// =============================================================================
// Provider reference
// =============================================================================

/// Provider identifier used by a credential reference.
///
/// This intentionally remains a plain validated string so that
/// `credentials.rs` does not depend on `provider.rs` or `identity.rs`.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProviderReference(String);

impl ProviderReference {
    /// Creates a validated provider reference.
    pub fn new(value: impl Into<String>) -> Result<Self, CredentialError> {
        let value = value.into();

        validate_identifier(
            "provider",
            &value,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the provider reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ProviderReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ProviderReference")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for ProviderReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Backend reference
// =============================================================================

/// Backend identifier used by a credential reference.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendReference(String);

impl BackendReference {
    /// Creates a validated backend reference.
    pub fn new(value: impl Into<String>) -> Result<Self, CredentialError> {
        let value = value.into();

        validate_identifier(
            "backend",
            &value,
            MAX_BACKEND_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the backend reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BackendReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BackendReference")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for BackendReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Credential reference
// =============================================================================

/// Complete provider-neutral reference to authentication material.
///
/// This structure contains metadata only.
///
/// It MUST NEVER contain:
///
/// - API keys;
/// - passwords;
/// - bearer tokens;
/// - private keys;
/// - client secrets;
/// - cookies;
/// - certificate private-key material.
///
/// The authentication layer resolves this reference into actual
/// authentication material without changing this structure.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CredentialReference {
    /// Stable logical credential identifier.
    credential_id: CredentialId,

    /// Credential material category.
    kind: CredentialKind,

    /// Where the credential can be resolved.
    source: CredentialSource,

    /// Optional provider restriction.
    provider: Option<ProviderReference>,

    /// Optional backend restriction.
    backend: Option<BackendReference>,

    /// Optional account/project/workspace restriction.
    account: Option<String>,

    /// Allowed authentication purposes.
    purposes: Vec<AuthenticationPurpose>,

    /// Additional usage scopes.
    scopes: Vec<CredentialScope>,

    /// Optional human-readable label.
    label: Option<String>,

    /// Optional non-secret description.
    description: Option<String>,

    /// Credential policy.
    policy: CredentialPolicy,
}

impl fmt::Debug for CredentialReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialReference")
            .field("credential_id", &self.credential_id)
            .field("kind", &self.kind)
            .field("source", &self.source.safe_reference())
            .field("provider", &self.provider)
            .field("backend", &self.backend)
            .field("account", &self.account)
            .field("purposes", &self.purposes)
            .field("scopes", &self.scopes)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("policy", &self.policy)
            .finish()
    }
}

impl fmt::Display for CredentialReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "credential:{} ({})",
            self.credential_id,
            self.kind
        )
    }
}

impl CredentialReference {
    /// Creates a credential reference with the minimum required fields.
    pub fn new(
        credential_id: CredentialId,
        kind: CredentialKind,
        source: CredentialSource,
    ) -> Result<Self, CredentialError> {
        let reference = Self {
            credential_id,
            kind,
            source,
            provider: None,
            backend: None,
            account: None,
            purposes: Vec::new(),
            scopes: Vec::new(),
            label: None,
            description: None,
            policy: CredentialPolicy::default(),
        };

        reference.validate()?;

        Ok(reference)
    }

    /// Adds a provider restriction.
    pub fn with_provider(
        mut self,
        provider: ProviderReference,
    ) -> Result<Self, CredentialError> {
        self.provider = Some(provider);
        self.validate()?;
        Ok(self)
    }

    /// Adds a backend restriction.
    pub fn with_backend(
        mut self,
        backend: BackendReference,
    ) -> Result<Self, CredentialError> {
        self.backend = Some(backend);
        self.validate()?;
        Ok(self)
    }

    /// Adds an account/project/workspace restriction.
    pub fn with_account(
        mut self,
        account: impl Into<String>,
    ) -> Result<Self, CredentialError> {
        let account = account.into();

        validate_bounded_string(
            "account",
            &account,
            MAX_ACCOUNT_REFERENCE_LENGTH,
            false,
        )?;

        reject_secret_material("account", &account)?;

        self.account = Some(account);
        self.validate()?;

        Ok(self)
    }

    /// Adds an authentication purpose.
    pub fn with_purpose(
        mut self,
        purpose: AuthenticationPurpose,
    ) -> Result<Self, CredentialError> {
        if !self.purposes.contains(&purpose) {
            if self.purposes.len() >= MAX_CREDENTIAL_PURPOSES {
                return Err(CredentialError::TooManyEntries {
                    field: "purposes",
                    maximum: MAX_CREDENTIAL_PURPOSES,
                });
            }

            self.purposes.push(purpose);
            self.purposes.sort();
        }

        self.validate()?;
        Ok(self)
    }

    /// Adds a usage scope.
    pub fn with_scope(
        mut self,
        scope: CredentialScope,
    ) -> Result<Self, CredentialError> {
        scope.validate()?;

        if !self.scopes.contains(&scope) {
            if self.scopes.len() >= MAX_CREDENTIAL_SCOPES {
                return Err(CredentialError::TooManyEntries {
                    field: "scopes",
                    maximum: MAX_CREDENTIAL_SCOPES,
                });
            }

            self.scopes.push(scope);
            self.scopes.sort();
        }

        self.validate()?;
        Ok(self)
    }

    /// Adds a non-secret label.
    pub fn with_label(
        mut self,
        label: impl Into<String>,
    ) -> Result<Self, CredentialError> {
        let label = label.into();

        validate_bounded_string(
            "label",
            &label,
            MAX_CREDENTIAL_LABEL_LENGTH,
            true,
        )?;

        reject_secret_material("label", &label)?;

        self.label = Some(label);
        self.validate()?;

        Ok(self)
    }

    /// Adds a non-secret description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, CredentialError> {
        let description = description.into();

        validate_bounded_string(
            "description",
            &description,
            MAX_CREDENTIAL_DESCRIPTION_LENGTH,
            true,
        )?;

        reject_secret_material("description", &description)?;

        self.description = Some(description);
        self.validate()?;

        Ok(self)
    }

    /// Replaces the credential policy.
    pub fn with_policy(
        mut self,
        policy: CredentialPolicy,
    ) -> Result<Self, CredentialError> {
        policy.validate()?;
        self.policy = policy;
        self.validate()?;
        Ok(self)
    }

    /// Returns the stable credential identifier.
    pub fn credential_id(&self) -> &CredentialId {
        &self.credential_id
    }

    /// Returns the credential kind.
    pub const fn kind(&self) -> CredentialKind {
        self.kind
    }

    /// Returns the credential source.
    pub fn source(&self) -> &CredentialSource {
        &self.source
    }

    /// Returns the optional provider restriction.
    pub fn provider(&self) -> Option<&ProviderReference> {
        self.provider.as_ref()
    }

    /// Returns the optional backend restriction.
    pub fn backend(&self) -> Option<&BackendReference> {
        self.backend.as_ref()
    }

    /// Returns the optional account reference.
    pub fn account(&self) -> Option<&str> {
        self.account.as_deref()
    }

    /// Returns allowed authentication purposes.
    pub fn purposes(&self) -> &[AuthenticationPurpose] {
        &self.purposes
    }

    /// Returns credential scopes.
    pub fn scopes(&self) -> &[CredentialScope] {
        &self.scopes
    }

    /// Returns the optional label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the optional description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the credential policy.
    pub const fn policy(&self) -> &CredentialPolicy {
        &self.policy
    }

    /// Validates the complete credential reference.
    pub fn validate(&self) -> Result<(), CredentialError> {
        self.source.validate()?;
        self.policy.validate()?;

        if let Some(provider) = &self.provider {
            validate_identifier(
                "provider",
                provider.as_str(),
                MAX_PROVIDER_ID_LENGTH,
            )?;
        }

        if let Some(backend) = &self.backend {
            validate_identifier(
                "backend",
                backend.as_str(),
                MAX_BACKEND_ID_LENGTH,
            )?;
        }

        if let Some(account) = &self.account {
            validate_bounded_string(
                "account",
                account,
                MAX_ACCOUNT_REFERENCE_LENGTH,
                false,
            )?;

            reject_secret_material("account", account)?;
        }

        if self.purposes.len() > MAX_CREDENTIAL_PURPOSES {
            return Err(CredentialError::TooManyEntries {
                field: "purposes",
                maximum: MAX_CREDENTIAL_PURPOSES,
            });
        }

        if self.scopes.len() > MAX_CREDENTIAL_SCOPES {
            return Err(CredentialError::TooManyEntries {
                field: "scopes",
                maximum: MAX_CREDENTIAL_SCOPES,
            });
        }

        for scope in &self.scopes {
            scope.validate()?;
        }

        if let Some(label) = &self.label {
            validate_bounded_string(
                "label",
                label,
                MAX_CREDENTIAL_LABEL_LENGTH,
                true,
            )?;

            reject_secret_material("label", label)?;
        }

        if let Some(description) = &self.description {
            validate_bounded_string(
                "description",
                description,
                MAX_CREDENTIAL_DESCRIPTION_LENGTH,
                true,
            )?;

            reject_secret_material("description", description)?;
        }

        Ok(())
    }

    /// Returns whether this credential is explicitly restricted to a provider.
    pub fn is_provider_scoped(&self) -> bool {
        self.provider.is_some()
    }

    /// Returns whether this credential is explicitly restricted to a backend.
    pub fn is_backend_scoped(&self) -> bool {
        self.backend.is_some()
    }

    /// Returns whether the credential can be used for the supplied purpose.
    ///
    /// An empty purpose list means "no explicit purpose restriction".
    pub fn allows_purpose(&self, purpose: AuthenticationPurpose) -> bool {
        self.purposes.is_empty() || self.purposes.contains(&purpose)
    }

    /// Returns whether the reference matches a provider.
    pub fn matches_provider(&self, provider: &str) -> bool {
        match &self.provider {
            Some(expected) => expected.as_str() == provider,
            None => true,
        }
    }

    /// Returns whether the reference matches a backend.
    pub fn matches_backend(&self, backend: &str) -> bool {
        match &self.backend {
            Some(expected) => expected.as_str() == backend,
            None => true,
        }
    }

    /// Returns a safe summary suitable for logs and diagnostics.
    ///
    /// This method never resolves credentials.
    pub fn safe_summary(&self) -> String {
        format!(
            "credential_id={}, kind={}, source={}, provider={}, backend={}",
            self.credential_id,
            self.kind,
            self.source.safe_reference(),
            self.provider
                .as_ref()
                .map(|value| value.as_str())
                .unwrap_or("<any>"),
            self.backend
                .as_ref()
                .map(|value| value.as_str())
                .unwrap_or("<any>")
        )
    }
}

// =============================================================================
// Credential policy
// =============================================================================

/// Credential lifecycle and usage policy.
///
/// This structure governs credentials without containing their values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CredentialPolicy {
    /// Whether the authentication layer may use the credential for remote
    /// provider access.
    pub allow_remote_use: bool,

    /// Whether use from local development environments is permitted.
    pub allow_local_use: bool,

    /// Whether authentication may cache resolved credentials.
    ///
    /// This is a policy signal to `authentication.rs`; this module does not
    /// perform caching.
    pub allow_authentication_cache: bool,

    /// Maximum lifetime for a resolved authentication session.
    pub max_session_lifetime: Duration,

    /// Expected credential rotation interval.
    pub rotation_interval: Duration,

    /// Allowed grace period during credential rotation.
    pub rotation_grace_period: Duration,

    /// Whether expiration must be enforced strictly.
    pub enforce_expiration: bool,

    /// Whether rotation metadata must be enforced.
    pub enforce_rotation: bool,
}

impl Default for CredentialPolicy {
    fn default() -> Self {
        Self {
            allow_remote_use: true,
            allow_local_use: true,
            allow_authentication_cache: false,
            max_session_lifetime: Duration::from_secs(60 * 60),
            rotation_interval: Duration::from_secs(90 * 24 * 60 * 60),
            rotation_grace_period: Duration::from_secs(24 * 60 * 60),
            enforce_expiration: true,
            enforce_rotation: false,
        }
    }
}

impl CredentialPolicy {
    /// Creates the strictest generally useful policy.
    pub const fn strict() -> Self {
        Self {
            allow_remote_use: true,
            allow_local_use: false,
            allow_authentication_cache: false,
            max_session_lifetime: Duration::from_secs(15 * 60),
            rotation_interval: Duration::from_secs(30 * 24 * 60 * 60),
            rotation_grace_period: Duration::from_secs(0),
            enforce_expiration: true,
            enforce_rotation: true,
        }
    }

    /// Creates a policy intended for local-only execution.
    pub const fn local_only() -> Self {
        Self {
            allow_remote_use: false,
            allow_local_use: true,
            allow_authentication_cache: false,
            max_session_lifetime: Duration::from_secs(60 * 60),
            rotation_interval: Duration::from_secs(90 * 24 * 60 * 60),
            rotation_grace_period: Duration::from_secs(24 * 60 * 60),
            enforce_expiration: true,
            enforce_rotation: false,
        }
    }

    /// Validates policy bounds.
    pub fn validate(&self) -> Result<(), CredentialError> {
        validate_duration(
            "policy.max_session_lifetime",
            self.max_session_lifetime,
            Duration::from_secs(1),
            Duration::from_secs(MAX_CREDENTIAL_LIFETIME_SECONDS),
        )?;

        validate_duration(
            "policy.rotation_interval",
            self.rotation_interval,
            Duration::from_secs(1),
            Duration::from_secs(MAX_ROTATION_INTERVAL_SECONDS),
        )?;

        validate_duration(
            "policy.rotation_grace_period",
            self.rotation_grace_period,
            Duration::from_secs(0),
            Duration::from_secs(MAX_ROTATION_GRACE_PERIOD_SECONDS),
        )?;

        if self.rotation_grace_period > self.rotation_interval {
            return Err(CredentialError::InvalidPolicy(
                "rotation grace period cannot exceed rotation interval"
                    .to_string(),
            ));
        }

        if !self.allow_remote_use && !self.allow_local_use {
            return Err(CredentialError::InvalidPolicy(
                "credential policy disables both remote and local use"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Credential requirement
// =============================================================================

/// Provider-neutral authentication requirement for a workload.
///
/// This is useful to `backend_config.rs`, `execution.rs`, and provider
/// adapters when determining what authentication is needed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CredentialRequirement {
    /// Required credential kind.
    pub kind: CredentialKind,

    /// Required authentication purpose.
    pub purpose: AuthenticationPurpose,

    /// Optional required provider.
    pub provider: Option<ProviderReference>,

    /// Optional required backend.
    pub backend: Option<BackendReference>,

    /// Whether an explicitly scoped credential is required.
    pub require_scoped_reference: bool,
}

impl CredentialRequirement {
    /// Creates a requirement.
    pub fn new(
        kind: CredentialKind,
        purpose: AuthenticationPurpose,
    ) -> Self {
        Self {
            kind,
            purpose,
            provider: None,
            backend: None,
            require_scoped_reference: false,
        }
    }

    /// Restricts the requirement to a provider.
    pub fn for_provider(
        mut self,
        provider: ProviderReference,
    ) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Restricts the requirement to a backend.
    pub fn for_backend(
        mut self,
        backend: BackendReference,
    ) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Requires explicit scope.
    pub const fn requiring_scope(mut self) -> Self {
        self.require_scoped_reference = true;
        self
    }

    /// Determines whether a credential reference satisfies the requirement.
    pub fn matches(&self, reference: &CredentialReference) -> bool {
        if reference.kind != self.kind {
            return false;
        }

        if !reference.allows_purpose(self.purpose) {
            return false;
        }

        if let Some(provider) = &self.provider {
            if !reference.matches_provider(provider.as_str()) {
                return false;
            }
        }

        if let Some(backend) = &self.backend {
            if !reference.matches_backend(backend.as_str()) {
                return false;
            }
        }

        if self.require_scoped_reference
            && !reference.is_provider_scoped()
            && !reference.is_backend_scoped()
        {
            return false;
        }

        true
    }
}

// =============================================================================
// Credential status
// =============================================================================

/// Provider-neutral lifecycle status.
///
/// This is metadata supplied by the authentication/credential-management
/// layer. `credentials.rs` does not query providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CredentialStatus {
    /// Status has not been established.
    Unknown,

    /// Credential is known to be usable.
    Active,

    /// Credential is approaching rotation/expiration.
    ExpiringSoon,

    /// Credential has expired.
    Expired,

    /// Credential has been explicitly revoked.
    Revoked,

    /// Credential is disabled.
    Disabled,

    /// Credential is unavailable at its configured source.
    Unavailable,

    /// Credential metadata is malformed.
    Invalid,
}

impl CredentialStatus {
    /// Returns true when the credential should normally be usable.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Active | Self::ExpiringSoon)
    }

    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Disabled => "disabled",
            Self::Unavailable => "unavailable",
            Self::Invalid => "invalid",
        }
    }
}

impl fmt::Display for CredentialStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Credential validation
// =============================================================================

/// Validates a credential reference against an authentication requirement.
pub fn validate_credential_reference(
    reference: &CredentialReference,
    requirement: &CredentialRequirement,
) -> Result<(), CredentialError> {
    reference.validate()?;

    if !requirement.matches(reference) {
        return Err(CredentialError::RequirementNotSatisfied {
            credential_id: reference.credential_id().as_str().to_string(),
            required_kind: requirement.kind,
            required_purpose: requirement.purpose,
        });
    }

    Ok(())
}

// =============================================================================
// Secret-material detection
// =============================================================================

/// Detects strings that appear to contain actual credential material.
///
/// This is deliberately conservative. It is a defence-in-depth mechanism, not
/// a cryptographic proof that a string is secret-free.
///
/// The function rejects common patterns such as:
///
/// - `Bearer ...`
/// - `Basic ...`
/// - `api_key=...`
/// - `token=...`
/// - `password=...`
/// - `secret=...`
/// - PEM private-key headers
/// - common API-key prefixes.
///
/// References should therefore use names/paths rather than actual values.
pub fn contains_secret_material(value: &str) -> bool {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return false;
    }

    let lower = trimmed.to_ascii_lowercase();

    const SECRET_MARKERS: &[&str] = &[
        "bearer ",
        "basic ",
        "api_key=",
        "apikey=",
        "access_token=",
        "refresh_token=",
        "token=",
        "password=",
        "passwd=",
        "secret=",
        "client_secret=",
        "private_key=",
        "authorization=",
        "proxy-authorization=",
        "set-cookie:",
        "cookie:",
        "-----begin private key-----",
        "-----begin rsa private key-----",
        "-----begin ec private key-----",
        "-----begin openssh private key-----",
        "-----begin encrypted private key-----",
    ];

    if SECRET_MARKERS.iter().any(|marker| lower.contains(marker)) {
        return true;
    }

    if looks_like_common_api_key(trimmed) {
        return true;
    }

    false
}

fn looks_like_common_api_key(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    let prefixes = [
        "sk-",
        "pk-",
        "ghp_",
        "gho_",
        "github_pat_",
        "xoxb-",
        "xoxp-",
        "AIza",
        "AKIA",
    ];

    if prefixes.iter().any(|prefix| value.starts_with(prefix)) {
        return true;
    }

    if lower.starts_with("ssh-rsa ")
        || lower.starts_with("ssh-ed25519 ")
        || lower.starts_with("ecdsa-sha2-")
    {
        return true;
    }

    false
}

/// Rejects suspected secret material.
fn reject_secret_material(
    field: &'static str,
    value: &str,
) -> Result<(), CredentialError> {
    if contains_secret_material(value) {
        return Err(CredentialError::SecretMaterialDetected { field });
    }

    Ok(())
}

// =============================================================================
// General validation
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), CredentialError> {
    validate_bounded_string(field, value, maximum, false)?;

    if value.contains(char::is_whitespace) {
        return Err(CredentialError::InvalidIdentifier {
            field,
            value: value.to_string(),
            reason: "whitespace is not permitted".to_string(),
        });
    }

    if value.contains('\0') {
        return Err(CredentialError::InvalidIdentifier {
            field,
            value: value.to_string(),
            reason: "NUL characters are not permitted".to_string(),
        });
    }

    Ok(())
}

fn validate_bounded_string(
    field: &'static str,
    value: &str,
    maximum: usize,
    allow_empty: bool,
) -> Result<(), CredentialError> {
    if !allow_empty && value.trim().is_empty() {
        return Err(CredentialError::EmptyValue { field });
    }

    if value.len() > maximum {
        return Err(CredentialError::TooLong {
            field,
            maximum,
            actual: value.len(),
        });
    }

    if value.contains('\0') {
        return Err(CredentialError::InvalidValue {
            field,
            message: "NUL characters are not permitted".to_string(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(CredentialError::InvalidValue {
            field,
            message: "control characters are not permitted".to_string(),
        });
    }

    Ok(())
}

fn validate_environment_variable(
    variable: &str,
) -> Result<(), CredentialError> {
    validate_bounded_string(
        "source.variable",
        variable,
        MAX_ENVIRONMENT_VARIABLE_LENGTH,
        false,
    )?;

    if variable.contains(char::is_whitespace) {
        return Err(CredentialError::InvalidEnvironmentVariable {
            value: variable.to_string(),
            reason: "whitespace is not permitted".to_string(),
        });
    }

    let mut characters = variable.chars();

    match characters.next() {
        Some(first) if first == '_' || first.is_ascii_alphabetic() => {}
        _ => {
            return Err(CredentialError::InvalidEnvironmentVariable {
                value: variable.to_string(),
                reason:
                    "must begin with an ASCII letter or underscore"
                        .to_string(),
            });
        }
    }

    if !characters.all(|character| {
        character == '_'
            || character.is_ascii_alphanumeric()
    }) {
        return Err(CredentialError::InvalidEnvironmentVariable {
            value: variable.to_string(),
            reason:
                "may contain only ASCII letters, digits and underscores"
                    .to_string(),
        });
    }

    if contains_secret_material(variable) {
        return Err(CredentialError::SecretMaterialDetected {
            field: "source.variable",
        });
    }

    Ok(())
}

fn validate_duration(
    field: &'static str,
    value: Duration,
    minimum: Duration,
    maximum: Duration,
) -> Result<(), CredentialError> {
    if value < minimum || value > maximum {
        return Err(CredentialError::DurationOutOfRange {
            field,
            minimum,
            maximum,
            actual: value,
        });
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by credential-reference validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialError {
    /// A required value was empty.
    EmptyValue {
        /// Field name.
        field: &'static str,
    },

    /// A value exceeded its maximum length.
    TooLong {
        /// Field name.
        field: &'static str,

        /// Maximum allowed length.
        maximum: usize,

        /// Actual length.
        actual: usize,
    },

    /// Generic invalid value.
    InvalidValue {
        /// Field name.
        field: &'static str,

        /// Explanation.
        message: String,
    },

    /// Invalid identifier.
    InvalidIdentifier {
        /// Field name.
        field: &'static str,

        /// Invalid value.
        value: String,

        /// Explanation.
        reason: String,
    },

    /// Invalid environment-variable name.
    InvalidEnvironmentVariable {
        /// Invalid environment variable.
        value: String,

        /// Explanation.
        reason: String,
    },

    /// Suspected secret material was supplied where only a reference is
    /// permitted.
    SecretMaterialDetected {
        /// Field where secret material was detected.
        field: &'static str,
    },

    /// Too many collection entries.
    TooManyEntries {
        /// Field name.
        field: &'static str,

        /// Maximum allowed entries.
        maximum: usize,
    },

    /// Invalid policy.
    InvalidPolicy(String),

    /// Duration outside the supported range.
    DurationOutOfRange {
        /// Field name.
        field: &'static str,

        /// Minimum accepted value.
        minimum: Duration,

        /// Maximum accepted value.
        maximum: Duration,

        /// Actual value.
        actual: Duration,
    },

    /// A credential does not satisfy an authentication requirement.
    RequirementNotSatisfied {
        /// Credential identifier.
        credential_id: String,

        /// Required credential kind.
        required_kind: CredentialKind,

        /// Required authentication purpose.
        required_purpose: AuthenticationPurpose,
    },
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::TooLong {
                field,
                maximum,
                actual,
            } => write!(
                formatter,
                "{field} exceeds maximum length {maximum} (actual {actual})"
            ),

            Self::InvalidValue { field, message } => {
                write!(formatter, "invalid {field}: {message}")
            }

            Self::InvalidIdentifier {
                field,
                value,
                reason,
            } => write!(
                formatter,
                "invalid {field} `{value}`: {reason}"
            ),

            Self::InvalidEnvironmentVariable { value, reason } => {
                write!(
                    formatter,
                    "invalid environment variable `{value}`: {reason}"
                )
            }

            Self::SecretMaterialDetected { field } => write!(
                formatter,
                "secret material detected in `{field}`; \
                 credential references must never contain resolved secrets"
            ),

            Self::TooManyEntries { field, maximum } => write!(
                formatter,
                "{field} contains more than the maximum allowed \
                 number of entries ({maximum})"
            ),

            Self::InvalidPolicy(message) => {
                write!(formatter, "invalid credential policy: {message}")
            }

            Self::DurationOutOfRange {
                field,
                minimum,
                maximum,
                actual,
            } => write!(
                formatter,
                "{field} duration {:?} is outside allowed range \
                 [{:?}, {:?}]",
                actual,
                minimum,
                maximum
            ),

            Self::RequirementNotSatisfied {
                credential_id,
                required_kind,
                required_purpose,
            } => write!(
                formatter,
                "credential `{credential_id}` does not satisfy \
                 required kind `{required_kind}` and purpose \
                 `{required_purpose}`"
            ),
        }
    }
}

impl Error for CredentialError {}

// =============================================================================
// Safe redaction
// =============================================================================

/// Safe redaction utility for values that MUST NOT be logged.
///
/// This type is deliberately simple: it never attempts to recover or inspect
/// a secret. It merely provides a stable redacted representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Redacted;

impl Redacted {
    /// Stable redacted representation.
    pub const VALUE: &'static str = "<redacted>";

    /// Returns the redacted representation.
    pub const fn as_str(self) -> &'static str {
        Self::VALUE
    }
}

impl fmt::Display for Redacted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(Self::VALUE)
    }
}

// =============================================================================
// Integration helpers
// =============================================================================

/// Determines whether a credential reference can be used for a particular
/// authentication operation.
///
/// This helper intentionally performs no secret resolution.
pub fn credential_satisfies(
    reference: &CredentialReference,
    requirement: &CredentialRequirement,
) -> bool {
    reference.validate().is_ok() && requirement.matches(reference)
}

/// Returns a safe diagnostic identifier.
///
/// This is preferable to logging provider-specific authentication material.
pub fn safe_credential_identifier(
    reference: &CredentialReference,
) -> &str {
    reference.credential_id().as_str()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn environment_reference() -> CredentialReference {
        CredentialReference::new(
            CredentialId::new("ibm-production").unwrap(),
            CredentialKind::ApiKey,
            CredentialSource::Environment {
                variable: "ZAMANI_IBM_API_KEY".to_string(),
            },
        )
        .unwrap()
    }

    #[test]
    fn credential_reference_contains_no_secret_value() {
        let reference = environment_reference();

        assert_eq!(reference.credential_id().as_str(), "ibm-production");
        assert_eq!(reference.kind(), CredentialKind::ApiKey);
        assert_eq!(
            reference.source().kind(),
            "environment"
        );
    }

    #[test]
    fn environment_variable_name_is_validated() {
        let source = CredentialSource::Environment {
            variable: "ZAMANI_IBM_API_KEY".to_string(),
        };

        assert!(source.validate().is_ok());
    }

    #[test]
    fn invalid_environment_variable_is_rejected() {
        let source = CredentialSource::Environment {
            variable: "1_INVALID".to_string(),
        };

        assert!(matches!(
            source.validate(),
            Err(CredentialError::InvalidEnvironmentVariable { .. })
        ));
    }

    #[test]
    fn whitespace_in_environment_variable_is_rejected() {
        let source = CredentialSource::Environment {
            variable: "ZAMANI API KEY".to_string(),
        };

        assert!(matches!(
            source.validate(),
            Err(CredentialError::InvalidEnvironmentVariable { .. })
        ));
    }

    #[test]
    fn actual_api_key_material_is_rejected() {
        assert!(contains_secret_material(
            "Bearer abcdefghijklmnop"
        ));

        assert!(contains_secret_material(
            "api_key=real-secret-value"
        ));

        assert!(contains_secret_material(
            "-----BEGIN PRIVATE KEY-----"
        ));
    }

    #[test]
    fn ordinary_reference_is_not_detected_as_secret() {
        assert!(!contains_secret_material(
            "ZAMANI_IBM_API_KEY"
        ));

        assert!(!contains_secret_material(
            "projects/prod/secrets/ibm-api-key"
        ));
    }

    #[test]
    fn keychain_reference_is_validated() {
        let source = CredentialSource::OsKeychain {
            service: "zamani.quantum".to_string(),
            account: "ibm-production".to_string(),
        };

        assert!(source.validate().is_ok());
    }

    #[test]
    fn secret_manager_reference_is_validated() {
        let source = CredentialSource::SecretManager {
            reference:
                "projects/prod/secrets/ibm-api-key".to_string(),
        };

        assert!(source.validate().is_ok());
    }

    #[test]
    fn credential_kind_is_stable() {
        assert_eq!(CredentialKind::ApiKey.as_str(), "api_key");
        assert_eq!(
            CredentialKind::AccessToken.as_str(),
            "access_token"
        );
        assert_eq!(
            CredentialKind::CloudIam.as_str(),
            "cloud_iam"
        );
    }

    #[test]
    fn credential_policy_default_is_valid() {
        assert!(CredentialPolicy::default().validate().is_ok());
    }

    #[test]
    fn strict_policy_is_valid() {
        assert!(CredentialPolicy::strict().validate().is_ok());
    }

    #[test]
    fn local_only_policy_is_valid() {
        assert!(CredentialPolicy::local_only().validate().is_ok());
    }

    #[test]
    fn policy_cannot_disable_all_usage() {
        let policy = CredentialPolicy {
            allow_remote_use: false,
            allow_local_use: false,
            ..CredentialPolicy::default()
        };

        assert!(matches!(
            policy.validate(),
            Err(CredentialError::InvalidPolicy(_))
        ));
    }

    #[test]
    fn policy_grace_period_cannot_exceed_rotation_interval() {
        let policy = CredentialPolicy {
            rotation_interval: Duration::from_secs(10),
            rotation_grace_period: Duration::from_secs(20),
            ..CredentialPolicy::default()
        };

        assert!(matches!(
            policy.validate(),
            Err(CredentialError::InvalidPolicy(_))
        ));
    }

    #[test]
    fn credential_purpose_matching_works() {
        let reference = environment_reference();

        let requirement = CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::JobSubmission,
        );

        assert!(requirement.matches(&reference));

        let restricted = reference
            .clone()
            .with_purpose(AuthenticationPurpose::Discovery)
            .unwrap();

        assert!(!CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::JobSubmission,
        )
        .matches(&restricted));

        assert!(CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::Discovery,
        )
        .matches(&restricted));
    }

    #[test]
    fn provider_scope_matching_works() {
        let reference = environment_reference()
            .with_provider(
                ProviderReference::new("ibm").unwrap()
            )
            .unwrap();

        assert!(reference.matches_provider("ibm"));
        assert!(!reference.matches_provider("ionq"));
    }

    #[test]
    fn backend_scope_matching_works() {
        let reference = environment_reference()
            .with_backend(
                BackendReference::new("ibm_torino").unwrap()
            )
            .unwrap();

        assert!(reference.matches_backend("ibm_torino"));
        assert!(!reference.matches_backend("other_backend"));
    }

    #[test]
    fn requirement_can_require_explicit_scope() {
        let unscoped = environment_reference();

        let requirement = CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::BackendApi,
        )
        .requiring_scope();

        assert!(!requirement.matches(&unscoped));

        let scoped = unscoped
            .with_provider(
                ProviderReference::new("ibm").unwrap()
            )
            .unwrap();

        assert!(requirement.matches(&scoped));
    }

    #[test]
    fn source_safe_reference_contains_no_secret() {
        let source = CredentialSource::Environment {
            variable: "ZAMANI_IONQ_API_KEY".to_string(),
        };

        let safe = source.safe_reference();

        assert_eq!(
            safe,
            "environment:ZAMANI_IONQ_API_KEY"
        );
        assert!(!contains_secret_material(&safe));
    }

    #[test]
    fn debug_output_contains_reference_metadata_only() {
        let reference = environment_reference();

        let debug = format!("{reference:?}");

        assert!(debug.contains("ibm-production"));
        assert!(debug.contains("ZAMANI_IBM_API_KEY"));
        assert!(!debug.contains("Bearer "));
    }

    #[test]
    fn display_output_is_safe() {
        let reference = environment_reference();

        let display = reference.to_string();

        assert_eq!(
            display,
            "credential:ibm-production (api_key)"
        );
    }

    #[test]
    fn redacted_value_is_stable() {
        assert_eq!(Redacted::VALUE, "<redacted>");
        assert_eq!(Redacted.as_str(), "<redacted>");
        assert_eq!(
            format!("{}", Redacted),
            "<redacted>"
        );
    }

    #[test]
    fn credential_id_rejects_whitespace() {
        assert!(CredentialId::new("invalid id").is_err());
    }

    #[test]
    fn credential_id_rejects_empty_value() {
        assert!(CredentialId::new("").is_err());
    }

    #[test]
    fn credential_id_accepts_stable_identifier() {
        let id = CredentialId::new("ibm-production").unwrap();

        assert_eq!(id.as_str(), "ibm-production");
    }

    #[test]
    fn full_reference_validation_is_deterministic() {
        let reference = environment_reference()
            .with_provider(
                ProviderReference::new("ibm").unwrap()
            )
            .unwrap()
            .with_backend(
                BackendReference::new("ibm_torino").unwrap()
            )
            .unwrap()
            .with_account("production")
            .unwrap()
            .with_purpose(
                AuthenticationPurpose::JobSubmission
            )
            .unwrap()
            .with_purpose(
                AuthenticationPurpose::ResultRetrieval
            )
            .unwrap()
            .with_scope(
                CredentialScope::Provider("ibm".to_string())
            )
            .unwrap();

        assert!(reference.validate().is_ok());
    }

    #[test]
    fn credential_satisfies_helper_works() {
        let reference = environment_reference();

        let requirement = CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::BackendApi,
        );

        assert!(credential_satisfies(
            &reference,
            &requirement
        ));
    }

    #[test]
    fn invalid_secret_manager_reference_is_rejected() {
        let source = CredentialSource::SecretManager {
            reference:
                "api_key=actual-secret".to_string(),
        };

        assert!(matches!(
            source.validate(),
            Err(CredentialError::SecretMaterialDetected { .. })
        ));
    }

    #[test]
    fn private_key_material_is_rejected() {
        let source = CredentialSource::External {
            reference:
                "-----BEGIN PRIVATE KEY-----secret"
                    .to_string(),
        };

        assert!(matches!(
            source.validate(),
            Err(CredentialError::SecretMaterialDetected { .. })
        ));
    }

    #[test]
    fn credential_reference_is_orderable() {
        let first = environment_reference();

        let second = CredentialReference::new(
            CredentialId::new("ionq-production").unwrap(),
            CredentialKind::ApiKey,
            CredentialSource::Environment {
                variable: "ZAMANI_IONQ_API_KEY".to_string(),
            },
        )
        .unwrap();

        assert!(first < second);
    }

    #[test]
    fn credential_requirement_matches_provider_and_backend() {
        let reference = environment_reference()
            .with_provider(
                ProviderReference::new("ibm").unwrap()
            )
            .unwrap()
            .with_backend(
                BackendReference::new("ibm_torino").unwrap()
            )
            .unwrap();

        let requirement = CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::JobSubmission,
        )
        .for_provider(
            ProviderReference::new("ibm").unwrap()
        )
        .for_backend(
            BackendReference::new("ibm_torino").unwrap()
        );

        assert!(requirement.matches(&reference));
    }

    #[test]
    fn wrong_provider_does_not_match() {
        let reference = environment_reference()
            .with_provider(
                ProviderReference::new("ibm").unwrap()
            )
            .unwrap();

        let requirement = CredentialRequirement::new(
            CredentialKind::ApiKey,
            AuthenticationPurpose::JobSubmission,
        )
        .for_provider(
            ProviderReference::new("ionq").unwrap()
        );

        assert!(!requirement.matches(&reference));
    }

    #[test]
    fn wrong_kind_does_not_match() {
        let reference = environment_reference();

        let requirement = CredentialRequirement::new(
            CredentialKind::AccessToken,
            AuthenticationPurpose::JobSubmission,
        );

        assert!(!requirement.matches(&reference));
    }

    #[test]
    fn no_purpose_restriction_allows_any_purpose() {
        let reference = environment_reference();

        assert!(reference.allows_purpose(
            AuthenticationPurpose::BackendApi
        ));

        assert!(reference.allows_purpose(
            AuthenticationPurpose::JobSubmission
        ));

        assert!(reference.allows_purpose(
            AuthenticationPurpose::ResultRetrieval
        ));
    }
}