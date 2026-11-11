
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Key Management
//!
//! Path: `src/stdlib/security/key_management.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Defines portable, provider-backed key-management contracts for:
//!
//! - key generation;
//! - protected key import;
//! - key derivation;
//! - key metadata and version inspection;
//! - key rotation;
//! - key activation and suspension;
//! - key revocation;
//! - key destruction;
//! - provider capability validation;
//! - authorization integration.
//!
//! ## Architectural ownership
//!
//! This module owns the standard-library key-management API and its
//! provider-facing contracts.
//!
//! It does not own:
//!
//! - cryptographic algorithm implementations;
//! - cryptographic signatures or hashing;
//! - identity verification;
//! - authorization policy implementation;
//! - trust evaluation;
//! - secret storage implementation;
//! - hardware discovery;
//! - HSM, TPM, enclave, or QPU implementation;
//! - physical resource allocation;
//! - runtime scheduling;
//! - compiler lowering;
//! - quantum IR or hardware IR.
//!
//! Cryptographic identifiers, protected references, and structured
//! cryptographic errors are reused from `cryptography.rs`.
//!
//! Authorization decisions are delegated to an explicitly supplied
//! authorization provider. The key-management provider must also enforce
//! its own security boundary.
//!
//! ## Security boundary
//!
//! Raw key material is never returned by this API.
//!
//! Imported key material is streamed directly to a provider.
//!
//! Key references identify managed objects. They are not key material.
//!
//! A provider declaration is not proof of cryptographic security.
//!
//! No operation succeeds merely because it is unsupported or simulated.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - keys;
//! - key versions;
//! - algorithms;
//! - providers;
//! - key purposes;
//! - key usages;
//! - operations;
//! - consumers;
//! - devices;
//! - distributed participants.
//!
//! Actual limits are determined by provider capabilities, algorithm
//! specifications, available resources, and explicit operational policy.

use std::collections::BTreeSet;
use std::fmt;
use std::io::Read;
use std::sync::Arc;

use super::cryptography::{
    CryptoAlgorithmId,
    CryptoError,
    CryptoErrorKind,
    CryptoOperationId,
    CryptoPropertyId,
    CryptoProviderId,
    CryptoResult,
    KeyReference,
};

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn valid_symbol(value: &str) -> bool {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return false;
    }

    value.split("::").all(|part| {
        let mut bytes = part.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        (first.is_ascii_alphanumeric() || first == b'_')
            && bytes.all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'_' | b'-' | b'.')
            })
    })
}

fn valid_reference(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_control)
        && !value.contains('\0')
}

// ============================================================================
// OPEN-WORLD KEY IDENTIFIERS
// ============================================================================

/// An open-world key purpose.
///
/// Examples include signing, encryption, authentication, key agreement,
/// wrapping, and domain-specific purposes.
///
/// This type does not enumerate a closed set of algorithms or purposes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyPurpose(String);

impl KeyPurpose {
    /// Creates a validated key-purpose identifier.
    pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
        let value = value.into();

        if !valid_symbol(&value) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical purpose identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for KeyPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An open-world key-usage identifier.
///
/// Usage names are semantic identifiers, not a fixed enumeration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyUsage(String);

impl KeyUsage {
    /// Creates a validated key-usage identifier.
    pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
        let value = value.into();

        if !valid_symbol(&value) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical usage identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An open-world key lifecycle state.
///
/// Providers define the operational meaning of their lifecycle states.
/// The standard library does not impose a closed lifecycle enumeration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyLifecycleState(String);

impl KeyLifecycleState {
    /// Creates a validated lifecycle-state identifier.
    pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
        let value = value.into();

        if !valid_symbol(&value) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the canonical state identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// CANONICAL OPERATION IDENTIFIERS
// ============================================================================

/// Key-management operation namespace.
pub const KEY_MANAGEMENT_NAMESPACE: &str = "security::key_management";

/// Creates an operation identifier within the key-management namespace.
fn operation_id(name: &str) -> CryptoResult<CryptoOperationId> {
    CryptoOperationId::new(format!(
        "{KEY_MANAGEMENT_NAMESPACE}::{name}"
    ))
}

/// Key generation operation.
pub const OPERATION_GENERATE: &str =
    "security::key_management::generate";

/// Key import operation.
pub const OPERATION_IMPORT: &str =
    "security::key_management::import";

/// Key derivation operation.
pub const OPERATION_DERIVE: &str =
    "security::key_management::derive";

/// Key metadata inspection operation.
pub const OPERATION_INSPECT: &str =
    "security::key_management::inspect";

/// Key rotation operation.
pub const OPERATION_ROTATE: &str =
    "security::key_management::rotate";

/// Key lifecycle transition operation.
pub const OPERATION_TRANSITION: &str =
    "security::key_management::transition";

/// Key revocation operation.
pub const OPERATION_REVOKE: &str =
    "security::key_management::revoke";

/// Key destruction operation.
pub const OPERATION_DESTROY: &str =
    "security::key_management::destroy";

// ============================================================================
// KEY SPECIFICATION
// ============================================================================

/// Describes the requested properties of a managed key.
///
/// This structure contains metadata only. It never contains secret bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeySpec {
    algorithm: CryptoAlgorithmId,
    purpose: KeyPurpose,
    usages: BTreeSet<KeyUsage>,
    required_properties: BTreeSet<CryptoPropertyId>,
    extractable: bool,
}

impl KeySpec {
    /// Creates a key specification.
    pub fn new(
        algorithm: CryptoAlgorithmId,
        purpose: KeyPurpose,
    ) -> Self {
        Self {
            algorithm,
            purpose,
            usages: BTreeSet::new(),
            required_properties: BTreeSet::new(),
            extractable: false,
        }
    }

    /// Adds an allowed key usage.
    #[must_use]
    pub fn with_usage(mut self, usage: KeyUsage) -> Self {
        self.usages.insert(usage);
        self
    }

    /// Adds a mandatory security property.
    #[must_use]
    pub fn requiring_property(
        mut self,
        property: CryptoPropertyId,
    ) -> Self {
        self.required_properties.insert(property);
        self
    }

    /// Sets whether the provider may permit raw key export.
    ///
    /// The standard library never exports raw key material. This field
    /// communicates the requested key policy to the provider.
    #[must_use]
    pub fn extractable(mut self, extractable: bool) -> Self {
        self.extractable = extractable;
        self
    }

    /// Returns the algorithm identity.
    #[must_use]
    pub fn algorithm(&self) -> &CryptoAlgorithmId {
        &self.algorithm
    }

    /// Returns the key purpose.
    #[must_use]
    pub fn purpose(&self) -> &KeyPurpose {
        &self.purpose
    }

    /// Returns permitted usages.
    #[must_use]
    pub fn usages(&self) -> &BTreeSet<KeyUsage> {
        &self.usages
    }

    /// Returns mandatory security properties.
    #[must_use]
    pub fn required_properties(
        &self,
    ) -> &BTreeSet<CryptoPropertyId> {
        &self.required_properties
    }

    /// Returns the requested extractability policy.
    #[must_use]
    pub const fn is_extractable(&self) -> bool {
        self.extractable
    }
}

// ============================================================================
// KEY METADATA
// ============================================================================

/// Public metadata describing a managed key.
///
/// This structure must never contain private or secret key material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyMetadata {
    reference: KeyReference,
    provider: CryptoProviderId,
    algorithm: CryptoAlgorithmId,
    purpose: KeyPurpose,
    usages: BTreeSet<KeyUsage>,
    properties: BTreeSet<CryptoPropertyId>,
    state: KeyLifecycleState,
    version: String,
    extractable: bool,
}

impl KeyMetadata {
    /// Constructs provider-returned key metadata.
    ///
    /// Providers should use this constructor only after the corresponding
    /// key operation has actually succeeded.
    pub fn new(
        reference: KeyReference,
        provider: CryptoProviderId,
        algorithm: CryptoAlgorithmId,
        purpose: KeyPurpose,
        usages: BTreeSet<KeyUsage>,
        properties: BTreeSet<CryptoPropertyId>,
        state: KeyLifecycleState,
        version: impl Into<String>,
        extractable: bool,
    ) -> CryptoResult<Self> {
        let version = version.into();

        if !valid_reference(&version) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidMetadata,
            ));
        }

        if !properties.is_superset(&BTreeSet::new()) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self {
            reference,
            provider,
            algorithm,
            purpose,
            usages,
            properties,
            state,
            version,
            extractable,
        })
    }

    /// Returns the protected key reference.
    #[must_use]
    pub fn reference(&self) -> &KeyReference {
        &self.reference
    }

    /// Returns the managing provider.
    #[must_use]
    pub fn provider(&self) -> &CryptoProviderId {
        &self.provider
    }

    /// Returns the algorithm identity.
    #[must_use]
    pub fn algorithm(&self) -> &CryptoAlgorithmId {
        &self.algorithm
    }

    /// Returns the key purpose.
    #[must_use]
    pub fn purpose(&self) -> &KeyPurpose {
        &self.purpose
    }

    /// Returns the permitted usages.
    #[must_use]
    pub fn usages(&self) -> &BTreeSet<KeyUsage> {
        &self.usages
    }

    /// Returns the provider-declared security properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeSet<CryptoPropertyId> {
        &self.properties
    }

    /// Returns the provider-reported lifecycle state.
    #[must_use]
    pub fn state(&self) -> &KeyLifecycleState {
        &self.state
    }

    /// Returns the opaque provider-defined version identifier.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the provider-reported extractability policy.
    #[must_use]
    pub const fn is_extractable(&self) -> bool {
        self.extractable
    }
}

// ============================================================================
// ACCESS CONTEXT AND AUTHORIZATION BOUNDARY
// ============================================================================

/// Identifies the principal requesting a key-management operation.
///
/// This is an opaque identity reference, not proof of authentication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyAccessContext {
    principal: String,
    correlation_id: Option<String>,
}

impl KeyAccessContext {
    /// Creates an access context for an authenticated principal reference.
    ///
    /// Authentication itself must be performed by the identity subsystem.
    pub fn new(principal: impl Into<String>) -> CryptoResult<Self> {
        let principal = principal.into();

        if !valid_reference(&principal) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidRequest,
            ));
        }

        Ok(Self {
            principal,
            correlation_id: None,
        })
    }

    /// Associates a non-secret correlation identifier.
    pub fn with_correlation_id(
        mut self,
        correlation_id: impl Into<String>,
    ) -> CryptoResult<Self> {
        let correlation_id = correlation_id.into();

        if !valid_reference(&correlation_id) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidRequest,
            ));
        }

        self.correlation_id = Some(correlation_id);
        Ok(self)
    }

    /// Returns the opaque principal reference.
    #[must_use]
    pub fn principal(&self) -> &str {
        &self.principal
    }

    /// Returns the correlation identifier, if supplied.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// Describes an attempted key-management action for authorization.
///
/// It contains references and metadata only, never secret material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyAccessRequest {
    principal: String,
    action: CryptoOperationId,
    key: Option<KeyReference>,
    algorithm: Option<CryptoAlgorithmId>,
    purpose: Option<KeyPurpose>,
    correlation_id: Option<String>,
}

impl KeyAccessRequest {
    /// Creates an authorization request.
    fn new(
        context: &KeyAccessContext,
        action: CryptoOperationId,
        key: Option<KeyReference>,
        algorithm: Option<CryptoAlgorithmId>,
        purpose: Option<KeyPurpose>,
    ) -> Self {
        Self {
            principal: context.principal.clone(),
            action,
            key,
            algorithm,
            purpose,
            correlation_id: context.correlation_id.clone(),
        }
    }

    /// Returns the principal reference.
    #[must_use]
    pub fn principal(&self) -> &str {
        &self.principal
    }

    /// Returns the requested action.
    #[must_use]
    pub fn action(&self) -> &CryptoOperationId {
        &self.action
    }

    /// Returns the key reference, if applicable.
    #[must_use]
    pub fn key(&self) -> Option<&KeyReference> {
        self.key.as_ref()
    }

    /// Returns the algorithm, if applicable.
    #[must_use]
    pub fn algorithm(&self) -> Option<&CryptoAlgorithmId> {
        self.algorithm.as_ref()
    }

    /// Returns the purpose, if applicable.
    #[must_use]
    pub fn purpose(&self) -> Option<&KeyPurpose> {
        self.purpose.as_ref()
    }

    /// Returns the correlation identifier.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// Authorization boundary supplied by the authoritative security-policy
/// and authorization subsystem.
///
/// Implementations must deny by default. This trait does not itself define
/// policy semantics or authenticate principals.
pub trait KeyAccessAuthorizer: Send + Sync {
    /// Authorizes a key-management action.
    ///
    /// Return `Ok(())` only when the action is authorized.
    /// Return `CryptoErrorKind::Denied` when access is denied.
    fn authorize(&self, request: &KeyAccessRequest) -> CryptoResult<()>;
}

// ============================================================================
// PROVIDER CAPABILITIES
// ============================================================================

/// Declares one provider-supported key-management operation.
///
/// A declaration is a provider claim, not an independent security
/// certification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyManagementCapability {
    operation: CryptoOperationId,
    algorithm: Option<CryptoAlgorithmId>,
    properties: BTreeSet<CryptoPropertyId>,
}

impl KeyManagementCapability {
    /// Creates an operation capability.
    #[must_use]
    pub fn new(operation: CryptoOperationId) -> Self {
        Self {
            operation,
            algorithm: None,
            properties: BTreeSet::new(),
        }
    }

    /// Associates an algorithm with this capability.
    #[must_use]
    pub fn with_algorithm(
        mut self,
        algorithm: CryptoAlgorithmId,
    ) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Adds a provider-declared security property.
    #[must_use]
    pub fn with_property(
        mut self,
        property: CryptoPropertyId,
    ) -> Self {
        self.properties.insert(property);
        self
    }

    /// Returns the operation identifier.
    #[must_use]
    pub fn operation(&self) -> &CryptoOperationId {
        &self.operation
    }

    /// Returns the associated algorithm, if any.
    #[must_use]
    pub fn algorithm(&self) -> Option<&CryptoAlgorithmId> {
        self.algorithm.as_ref()
    }

    /// Returns declared security properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeSet<CryptoPropertyId> {
        &self.properties
    }
}

// ============================================================================
// IMPORT AND DERIVATION OPTIONS
// ============================================================================

/// Describes the requested handling of imported key material.
///
/// The provider is responsible for securely ingesting the stream.
/// The standard library does not buffer the complete key material.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KeyImportOptions {
    require_non_extractable: bool,
}

impl KeyImportOptions {
    /// Creates default import options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requires the imported key to remain non-extractable.
    #[must_use]
    pub fn require_non_extractable(mut self) -> Self {
        self.require_non_extractable = true;
        self
    }

    /// Returns whether non-extractability is mandatory.
    #[must_use]
    pub const fn non_extractable_required(&self) -> bool {
        self.require_non_extractable
    }
}

/// Describes a key-destruction outcome.
///
/// `Pending` does not mean the key has already been destroyed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum KeyDestructionStatus {
    /// The provider confirms destruction has completed.
    Destroyed,

    /// The provider accepted the request but destruction is not complete.
    Pending,

    /// The provider has scheduled destruction for a later time.
    Scheduled,
}

/// Receipt describing a provider-reported destruction operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDestructionReceipt {
    reference: KeyReference,
    provider: CryptoProviderId,
    status: KeyDestructionStatus,
    evidence: Option<String>,
}

impl KeyDestructionReceipt {
    /// Creates a destruction receipt.
    pub fn new(
        reference: KeyReference,
        provider: CryptoProviderId,
        status: KeyDestructionStatus,
        evidence: Option<String>,
    ) -> CryptoResult<Self> {
        if evidence
            .as_ref()
            .is_some_and(|value| !valid_reference(value))
        {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidMetadata,
            ));
        }

        Ok(Self {
            reference,
            provider,
            status,
            evidence,
        })
    }

    /// Returns the affected key reference.
    #[must_use]
    pub fn reference(&self) -> &KeyReference {
        &self.reference
    }

    /// Returns the provider.
    #[must_use]
    pub fn provider(&self) -> &CryptoProviderId {
        &self.provider
    }

    /// Returns the provider-reported destruction status.
    #[must_use]
    pub fn status(&self) -> &KeyDestructionStatus {
        &self.status
    }

    /// Returns optional provider evidence.
    #[must_use]
    pub fn evidence(&self) -> Option<&str> {
        self.evidence.as_deref()
    }

    /// Returns whether the provider reports completed destruction.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self.status, KeyDestructionStatus::Destroyed)
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Provider interface for protected key management.
///
/// Implementations must use genuine protected storage and actual
/// cryptographic mechanisms. A provider must never simulate a successful
/// operation.
///
/// Providers are responsible for:
///
/// - secure key generation and ingestion;
/// - secure storage and access control;
/// - algorithm and parameter validation;
/// - key-purpose and usage enforcement;
/// - key versioning;
/// - concurrency-safe rotation;
/// - revocation semantics;
/// - destruction semantics;
/// - audit evidence;
/// - provider-specific resource accounting;
/// - protecting secret material from ordinary application memory.
///
/// A provider may use software protection, an HSM, a TPM, an enclave,
/// a remote KMS, or another implementation. Those choices are not encoded
/// in the portable API.
pub trait KeyManagementProvider: Send + Sync {
    /// Returns the provider identity.
    fn id(&self) -> &CryptoProviderId;

    /// Returns declared key-management capabilities.
    fn capabilities(&self) -> &[KeyManagementCapability];

    /// Generates a key inside the provider's protected boundary.
    fn generate_key(&self, spec: &KeySpec) -> CryptoResult<KeyMetadata>;

    /// Imports key material from a stream.
    ///
    /// The provider must ingest the stream securely and must not expose
    /// the material through returned metadata or diagnostic errors.
    fn import_key(
        &self,
        spec: &KeySpec,
        input: &mut dyn Read,
        options: &KeyImportOptions,
    ) -> CryptoResult<KeyMetadata>;

    /// Derives a new protected key from an existing protected key.
    ///
    /// The base key must never be returned to application memory.
    fn derive_key(
        &self,
        base: &KeyReference,
        spec: &KeySpec,
        context: &[u8],
    ) -> CryptoResult<KeyMetadata>;

    /// Retrieves metadata for a managed key.
    fn inspect_key(
        &self,
        reference: &KeyReference,
    ) -> CryptoResult<KeyMetadata>;

    /// Rotates a key.
    ///
    /// Providers must define atomicity, version progression, old-key
    /// retention, and failure behavior. A successful result identifies
    /// the newly active key version.
    fn rotate_key(
        &self,
        reference: &KeyReference,
        replacement: &KeySpec,
    ) -> CryptoResult<KeyMetadata>;

    /// Transitions a key between provider-defined lifecycle states.
    ///
    /// Providers must perform this as an atomic state transition or return
    /// an error. `expected_state` prevents unnoticed concurrent updates.
    fn transition_key(
        &self,
        reference: &KeyReference,
        expected_state: &KeyLifecycleState,
        target_state: &KeyLifecycleState,
    ) -> CryptoResult<KeyMetadata>;

    /// Revokes a managed key.
    ///
    /// Revocation must prevent future use according to the provider's
    /// documented consistency and propagation guarantees.
    fn revoke_key(
        &self,
        reference: &KeyReference,
        reason: &str,
    ) -> CryptoResult<KeyMetadata>;

    /// Requests destruction of a managed key.
    ///
    /// The returned receipt distinguishes completed destruction from
    /// scheduled or pending destruction.
    fn destroy_key(
        &self,
        reference: &KeyReference,
    ) -> CryptoResult<KeyDestructionReceipt>;
}

// ============================================================================
// KEY MANAGER
// ============================================================================

/// Provider-backed key-management facade.
///
/// The manager does not own a provider registry. Provider discovery and
/// selection belong to the shared provider/registry infrastructure.
///
/// A manager requires an explicit authorizer. There is no permissive
/// default authorization implementation.
pub struct KeyManager {
    provider: Arc<dyn KeyManagementProvider>,
    authorizer: Arc<dyn KeyAccessAuthorizer>,
}

impl Clone for KeyManager {
    fn clone(&self) -> Self {
        Self {
            provider: Arc::clone(&self.provider),
            authorizer: Arc::clone(&self.authorizer),
        }
    }
}

impl KeyManager {
    /// Creates a manager using an explicit provider and authorizer.
    #[must_use]
    pub fn new(
        provider: Arc<dyn KeyManagementProvider>,
        authorizer: Arc<dyn KeyAccessAuthorizer>,
    ) -> Self {
        Self {
            provider,
            authorizer,
        }
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &CryptoProviderId {
        self.provider.id()
    }

    fn authorize(
        &self,
        context: &KeyAccessContext,
        operation: &str,
        key: Option<KeyReference>,
        algorithm: Option<CryptoAlgorithmId>,
        purpose: Option<KeyPurpose>,
    ) -> CryptoResult<()> {
        let action = operation_id(operation)?;

        let request = KeyAccessRequest::new(
            context,
            action,
            key,
            algorithm,
            purpose,
        );

        self.authorizer.authorize(&request)
    }

    fn require_capability(
        &self,
        operation: &str,
        algorithm: Option<&CryptoAlgorithmId>,
    ) -> CryptoResult<()> {
        let operation = operation_id(operation)?;

        let supported = self.provider.capabilities().iter().any(
            |capability| {
                capability.operation == operation
                    && capability.algorithm.as_ref() == algorithm
            },
        );

        if !supported {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::CapabilityUnavailable,
                )
                .with_provider(self.provider.id().clone())
                .with_operation(operation),
            );
        }

        Ok(())
    }

    fn validate_metadata(
        &self,
        metadata: &KeyMetadata,
        requested: &KeySpec,
    ) -> CryptoResult<()> {
        if metadata.provider != *self.provider.id()
            || metadata.algorithm != requested.algorithm
            || metadata.purpose != requested.purpose
            || metadata.extractable != requested.extractable
            || !metadata.usages.is_superset(&requested.usages)
            || !metadata
                .properties
                .is_superset(&requested.required_properties)
        {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::InvalidProviderResult,
                )
                .with_provider(self.provider.id().clone()),
            );
        }

        Ok(())
    }

    /// Generates a key within the provider's protected boundary.
    pub fn generate(
        &self,
        context: &KeyAccessContext,
        spec: &KeySpec,
    ) -> CryptoResult<KeyMetadata> {
        self.authorize(
            context,
            "generate",
            None,
            Some(spec.algorithm.clone()),
            Some(spec.purpose.clone()),
        )?;

        self.require_capability(
            "generate",
            Some(&spec.algorithm),
        )?;

        let metadata = self.provider.generate_key(spec)?;

        self.validate_metadata(&metadata, spec)?;

        Ok(metadata)
    }

    /// Imports key material through a provider-controlled stream.
    ///
    /// The manager does not read or buffer the complete input.
    pub fn import(
        &self,
        context: &KeyAccessContext,
        spec: &KeySpec,
        input: &mut dyn Read,
        options: &KeyImportOptions,
    ) -> CryptoResult<KeyMetadata> {
        self.authorize(
            context,
            "import",
            None,
            Some(spec.algorithm.clone()),
            Some(spec.purpose.clone()),
        )?;

        self.require_capability(
            "import",
            Some(&spec.algorithm),
        )?;

        if options.require_non_extractable && spec.extractable {
            return Err(CryptoError::new(
                CryptoErrorKind::RequirementUnsatisfied,
            ));
        }

        let metadata = self.provider.import_key(
            spec,
            input,
            options,
        )?;

        self.validate_metadata(&metadata, spec)?;

        if options.require_non_extractable && metadata.extractable {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::InvalidProviderResult,
                )
                .with_provider(self.provider.id().clone()),
            );
        }

        Ok(metadata)
    }

    /// Derives a protected key without exposing the base key.
    pub fn derive(
        &self,
        context: &KeyAccessContext,
        base: &KeyReference,
        spec: &KeySpec,
        derivation_context: &[u8],
    ) -> CryptoResult<KeyMetadata> {
        self.authorize(
            context,
            "derive",
            Some(base.clone()),
            Some(spec.algorithm.clone()),
            Some(spec.purpose.clone()),
        )?;

        self.require_capability(
            "derive",
            Some(&spec.algorithm),
        )?;

        let metadata = self.provider.derive_key(
            base,
            spec,
            derivation_context,
        )?;

        self.validate_metadata(&metadata, spec)?;

        Ok(metadata)
    }

    /// Inspects a key without retrieving its secret material.
    pub fn inspect(
        &self,
        context: &KeyAccessContext,
        reference: &KeyReference,
    ) -> CryptoResult<KeyMetadata> {
        self.authorize(
            context,
            "inspect",
            Some(reference.clone()),
            None,
            None,
        )?;

        self.require_capability("inspect", None)?;

        let metadata = self.provider.inspect_key(reference)?;

        if metadata.provider != *self.provider.id()
            || metadata.reference != *reference
        {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::InvalidProviderResult,
                )
                .with_provider(self.provider.id().clone()),
            );
        }

        Ok(metadata)
    }

    /// Rotates a managed key.
    pub fn rotate(
        &self,
        context: &KeyAccessContext,
        reference: &KeyReference,
        replacement: &KeySpec,
    ) -> CryptoResult<KeyMetadata> {
        self.authorize(
            context,
            "rotate",
            Some(reference.clone()),
            Some(replacement.algorithm.clone()),
            Some(replacement.purpose.clone()),
        )?;

        self.require_capability(
            "rotate",
            Some(&replacement.algorithm),
        )?;

        let metadata = self.provider.rotate_key(
            reference,
            replacement,
        )?;

        self.validate_metadata(&metadata, replacement)?;

        Ok(metadata)
    }

    /// Performs an optimistic, provider-controlled lifecycle transition.
    pub fn transition(
        &self,
        context: &KeyAccessContext,
        reference: &KeyReference,
        expected_state: &KeyLifecycleState,
        target_state: &KeyLifecycleState,
    ) -> CryptoResult<KeyMetadata> {
        self.authorize(
            context,
            "transition",
            Some(reference.clone()),
            None,
            None,
        )?;

        self.require_capability("transition", None)?;

        let metadata = self.provider.transition_key(
            reference,
            expected_state,
            target_state,
        )?;

        if metadata.provider != *self.provider.id()
            || metadata.reference != *reference
            || metadata.state != *target_state
        {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::InvalidProviderResult,
                )
                .with_provider(self.provider.id().clone()),
            );
        }

        Ok(metadata)
    }

    /// Revokes a managed key.
    pub fn revoke(
        &self,
        context: &KeyAccessContext,
        reference: &KeyReference,
        reason: &str,
    ) -> CryptoResult<KeyMetadata> {
        if !valid_reference(reason) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidRequest,
            ));
        }

        self.authorize(
            context,
            "revoke",
            Some(reference.clone()),
            None,
            None,
        )?;

        self.require_capability("revoke", None)?;

        let metadata = self.provider.revoke_key(
            reference,
            reason,
        )?;

        if metadata.provider != *self.provider.id()
            || metadata.reference != *reference
        {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::InvalidProviderResult,
                )
                .with_provider(self.provider.id().clone()),
            );
        }

        Ok(metadata)
    }

    /// Requests destruction of a managed key.
    ///
    /// A pending or scheduled receipt is not represented as completed
    /// destruction.
    pub fn destroy(
        &self,
        context: &KeyAccessContext,
        reference: &KeyReference,
    ) -> CryptoResult<KeyDestructionReceipt> {
        self.authorize(
            context,
            "destroy",
            Some(reference.clone()),
            None,
            None,
        )?;

        self.require_capability("destroy", None)?;

        let receipt = self.provider.destroy_key(reference)?;

        if receipt.provider != *self.provider.id()
            || receipt.reference != *reference
        {
            return Err(
                CryptoError::new(
                    CryptoErrorKind::InvalidProviderResult,
                )
                .with_provider(self.provider.id().clone()),
            );
        }

        Ok(receipt)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn algorithm() -> CryptoAlgorithmId {
        CryptoAlgorithmId::new(
            "cryptography::encryption::example",
        )
        .unwrap()
    }

    fn provider_id() -> CryptoProviderId {
        CryptoProviderId::new(
            "zamani::test::key_provider",
        )
        .unwrap()
    }

    fn purpose() -> KeyPurpose {
        KeyPurpose::new("application::encryption").unwrap()
    }

    fn context() -> KeyAccessContext {
        KeyAccessContext::new("identity::test_principal").unwrap()
    }

    fn spec() -> KeySpec {
        KeySpec::new(algorithm(), purpose())
            .with_usage(
                KeyUsage::new("encrypt").unwrap(),
            )
    }

    #[test]
    fn accepts_open_world_purposes_and_usages() {
        assert!(
            KeyPurpose::new(
                "future::cryptographic::purpose",
            )
            .is_ok()
        );

        assert!(
            KeyUsage::new(
                "organization::custom::usage",
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_invalid_identifiers() {
        assert!(KeyPurpose::new("").is_err());
        assert!(KeyUsage::new("invalid::").is_err());
        assert!(KeyLifecycleState::new(" invalid").is_err());
    }

    #[test]
    fn key_spec_does_not_expose_secret_material() {
        let value = spec();

        assert_eq!(value.algorithm(), &algorithm());
        assert!(!value.is_extractable());
    }

    #[test]
    fn access_context_rejects_invalid_principals() {
        assert!(KeyAccessContext::new("").is_err());
        assert!(KeyAccessContext::new("invalid\nprincipal").is_err());
    }

    #[test]
    fn import_options_can_require_non_extractability() {
        let options = KeyImportOptions::new()
            .require_non_extractable();

        assert!(options.non_extractable_required());
    }

    #[test]
    fn destruction_receipt_distinguishes_pending_from_complete() {
        let reference = KeyReference::new(
            "provider::key::one",
        )
        .unwrap();

        let pending = KeyDestructionReceipt::new(
            reference.clone(),
            provider_id(),
            KeyDestructionStatus::Pending,
            None,
        )
        .unwrap();

        assert!(!pending.is_complete());

        let completed = KeyDestructionReceipt::new(
            reference,
            provider_id(),
            KeyDestructionStatus::Destroyed,
            Some("provider-evidence::receipt".to_string()),
        )
        .unwrap();

        assert!(completed.is_complete());
    }

    #[test]
    fn provider_capabilities_are_open_world() {
        let capability = KeyManagementCapability::new(
            operation_id("generate").unwrap(),
        )
        .with_algorithm(algorithm());

        assert_eq!(
            capability.algorithm(),
            Some(&algorithm()),
        );
    }

    // A test authorizer that explicitly denies every operation.
    struct DenyAll;

    impl KeyAccessAuthorizer for DenyAll {
        fn authorize(
            &self,
            _request: &KeyAccessRequest,
        ) -> CryptoResult<()> {
            Err(CryptoError::new(CryptoErrorKind::Denied))
        }
    }

    // This provider is deliberately not implemented. It proves that the
    // manager cannot bypass authorization to reach a provider operation.
    struct UnusedProvider {
        id: CryptoProviderId,
        capabilities: Vec<KeyManagementCapability>,
    }

    impl KeyManagementProvider for UnusedProvider {
        fn id(&self) -> &CryptoProviderId {
            &self.id
        }

        fn capabilities(&self) -> &[KeyManagementCapability] {
            &self.capabilities
        }

        fn generate_key(
            &self,
            _spec: &KeySpec,
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn import_key(
            &self,
            _spec: &KeySpec,
            _input: &mut dyn Read,
            _options: &KeyImportOptions,
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn derive_key(
            &self,
            _base: &KeyReference,
            _spec: &KeySpec,
            _context: &[u8],
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn inspect_key(
            &self,
            _reference: &KeyReference,
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn rotate_key(
            &self,
            _reference: &KeyReference,
            _replacement: &KeySpec,
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn transition_key(
            &self,
            _reference: &KeyReference,
            _expected_state: &KeyLifecycleState,
            _target_state: &KeyLifecycleState,
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn revoke_key(
            &self,
            _reference: &KeyReference,
            _reason: &str,
        ) -> CryptoResult<KeyMetadata> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }

        fn destroy_key(
            &self,
            _reference: &KeyReference,
        ) -> CryptoResult<KeyDestructionReceipt> {
            Err(CryptoError::new(
                CryptoErrorKind::ProviderFailure,
            ))
        }
    }

    #[test]
    fn authorization_denial_prevents_provider_operation() {
        let operation = operation_id("generate").unwrap();

        let provider = UnusedProvider {
            id: provider_id(),
            capabilities: vec![
                KeyManagementCapability::new(operation)
                    .with_algorithm(algorithm()),
            ],
        };

        let manager = KeyManager::new(
            Arc::new(provider),
            Arc::new(DenyAll),
        );

        let result = manager.generate(&context(), &spec());

        assert!(matches!(
            result,
            Err(error) if error.kind() == CryptoErrorKind::Denied
        ));
    }
}
