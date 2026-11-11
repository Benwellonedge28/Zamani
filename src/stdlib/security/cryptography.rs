
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Cryptography
//!
//! Path: `src/stdlib/security/cryptography.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Defines portable, target-independent cryptographic operation contracts.
//!
//! This module provides the stable cryptography-facing API used by Zamani
//! programs and by concrete cryptographic providers.
//!
//! It does not implement cryptographic algorithms, manage physical hardware,
//! store private keys, allocate devices, or schedule computations.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Open-world cryptographic identifiers.
//! - Cryptographic operation descriptions.
//! - Cryptographic requirements and constraints.
//! - Cryptographic preferences and hints.
//! - Provider capability descriptions.
//! - Cryptographic operation requests.
//! - Streaming operation interfaces.
//! - Structured cryptographic errors.
//! - Provider integration contracts.
//!
//! Actual cryptographic algorithms remain owned by their implementations
//! under `crate::stdlib::crypto` or external audited providers.
//!
//! Key storage and secret lifecycle remain the responsibility of protected
//! key-management and runtime security providers.
//!
//! ## Portability
//!
//! The API does not select:
//!
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - QPUs;
//! - hardware security modules;
//! - secure enclaves;
//! - operating systems;
//! - distributed nodes.
//!
//! Those decisions belong to downstream compilation and execution systems.
//!
//! ## Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - algorithms;
//! - providers;
//! - cryptographic operations;
//! - participants;
//! - keys;
//! - security properties;
//! - requirements;
//! - supported devices;
//! - distributed participants.
//!
//! Streaming operations permit providers to process data incrementally
//! rather than requiring an entire input to be held in memory.
//!
//! Physical resource availability and algorithm-specific limits still apply.
//!
//! ## Security
//!
//! - Secret material must not be placed in public request structures.
//! - Key references are not private keys.
//! - Provider declarations are not proof of security.
//! - Unsupported operations must fail explicitly.
//! - Authentication failures must never produce successful plaintext output.
//! - Cryptographic metadata must not claim guarantees that were not achieved.
//! - Algorithm selection must preserve mandatory security requirements.
//! - Provider errors must not expose secret material.
//!
//! This module does not claim that a provider is cryptographically secure
//! merely because it implements the provider trait.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::io::{Read, Write};

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn valid_identifier(value: &str) -> bool {
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

fn valid_reference(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_control)
        && !value.contains('\0')
}

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of cryptographic failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CryptoErrorKind {
    /// Invalid algorithm identity.
    InvalidAlgorithm,

    /// Invalid provider identity.
    InvalidProvider,

    /// Invalid operation identity.
    InvalidOperation,

    /// Invalid security-property identity.
    InvalidProperty,

    /// Invalid key reference.
    InvalidKeyReference,

    /// Invalid secret reference.
    InvalidSecretReference,

    /// Invalid operation request.
    InvalidRequest,

    /// Required cryptographic capability is unavailable.
    CapabilityUnavailable,

    /// Mandatory cryptographic requirement cannot be satisfied.
    RequirementUnsatisfied,

    /// Operation is not implemented by the provider.
    UnsupportedOperation,

    /// Provider is unavailable.
    ProviderUnavailable,

    /// Provider execution failed.
    ProviderFailure,

    /// Cryptographic authentication failed.
    AuthenticationFailed,

    /// Signature verification failed.
    SignatureVerificationFailed,

    /// Key material is invalid.
    InvalidKey,

    /// Cryptographic parameters are invalid.
    InvalidParameters,

    /// Cryptographic metadata is invalid.
    InvalidMetadata,

    /// Input data is invalid.
    InvalidInput,

    /// Output operation failed.
    OutputFailure,

    /// Input operation failed.
    InputFailure,

    /// The operation was cancelled.
    Cancelled,

    /// An operational timeout occurred.
    TimedOut,

    /// A provider returned an invalid result.
    InvalidProviderResult,

    /// A protected operation was denied.
    Denied,

    /// The operation is in an invalid state.
    InvalidState,
}

impl CryptoErrorKind {
    /// Returns the stable machine-readable error identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidAlgorithm => "invalid_algorithm",
            Self::InvalidProvider => "invalid_provider",
            Self::InvalidOperation => "invalid_operation",
            Self::InvalidProperty => "invalid_property",
            Self::InvalidKeyReference => "invalid_key_reference",
            Self::InvalidSecretReference => "invalid_secret_reference",
            Self::InvalidRequest => "invalid_request",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::RequirementUnsatisfied => "requirement_unsatisfied",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderFailure => "provider_failure",
            Self::AuthenticationFailed => "authentication_failed",
            Self::SignatureVerificationFailed => "signature_verification_failed",
            Self::InvalidKey => "invalid_key",
            Self::InvalidParameters => "invalid_parameters",
            Self::InvalidMetadata => "invalid_metadata",
            Self::InvalidInput => "invalid_input",
            Self::OutputFailure => "output_failure",
            Self::InputFailure => "input_failure",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::InvalidProviderResult => "invalid_provider_result",
            Self::Denied => "denied",
            Self::InvalidState => "invalid_state",
        }
    }
}

impl fmt::Display for CryptoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Structured cryptographic error.
///
/// The error intentionally does not contain secret material.
pub struct CryptoError {
    kind: CryptoErrorKind,
    operation: Option<CryptoOperationId>,
    provider: Option<CryptoProviderId>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl CryptoError {
    /// Creates a cryptographic error.
    #[must_use]
    pub fn new(kind: CryptoErrorKind) -> Self {
        Self {
            kind,
            operation: None,
            provider: None,
            source: None,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> CryptoErrorKind {
        self.kind
    }

    /// Returns the operation identity, if available.
    #[must_use]
    pub fn operation(&self) -> Option<&CryptoOperationId> {
        self.operation.as_ref()
    }

    /// Returns the provider identity, if available.
    #[must_use]
    pub fn provider(&self) -> Option<&CryptoProviderId> {
        self.provider.as_ref()
    }

    /// Associates an operation with the error.
    #[must_use]
    pub fn with_operation(mut self, operation: CryptoOperationId) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Associates a provider with the error.
    #[must_use]
    pub fn with_provider(mut self, provider: CryptoProviderId) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Attaches an underlying non-secret diagnostic error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Debug for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CryptoError")
            .field("kind", &self.kind)
            .field("operation", &self.operation)
            .field("provider", &self.provider)
            .field("source", &self.source.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cryptographic operation failed: {}", self.kind)?;

        if let Some(operation) = &self.operation {
            write!(f, " (operation: {operation})")?;
        }

        if let Some(provider) = &self.provider {
            write!(f, " (provider: {provider})")?;
        }

        Ok(())
    }
}

impl StdError for CryptoError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for cryptographic operations.
pub type CryptoResult<T> = Result<T, CryptoError>;

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

/// Defines a validated, open-world cryptographic identifier.
macro_rules! crypto_identifier {
    ($name:ident, $error:expr) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
                let value = value.into();

                if !valid_identifier(&value) {
                    return Err(CryptoError::new($error));
                }

                Ok(Self(value))
            }

            /// Returns the identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

crypto_identifier!(CryptoAlgorithmId, CryptoErrorKind::InvalidAlgorithm);
crypto_identifier!(CryptoOperationId, CryptoErrorKind::InvalidOperation);
crypto_identifier!(CryptoPropertyId, CryptoErrorKind::InvalidProperty);
crypto_identifier!(CryptoProviderId, CryptoErrorKind::InvalidProvider);

// ============================================================================
// PROTECTED REFERENCES
// ============================================================================

/// Opaque reference to a key managed by a trusted key-management provider.
///
/// This is not key material.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KeyReference(String);

impl KeyReference {
    /// Creates a validated key reference.
    pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
        let value = value.into();

        if !valid_reference(&value) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidKeyReference,
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

impl fmt::Debug for KeyReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("KeyReference([REDACTED])")
    }
}

impl fmt::Display for KeyReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[PROTECTED KEY REFERENCE]")
    }
}

/// Opaque reference to a secret managed outside ordinary program memory.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SecretReference(String);

impl SecretReference {
    /// Creates a validated secret reference.
    pub fn new(value: impl Into<String>) -> CryptoResult<Self> {
        let value = value.into();

        if !valid_reference(&value) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidSecretReference,
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

impl fmt::Debug for SecretReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretReference([REDACTED])")
    }
}

// ============================================================================
// REQUIREMENTS, CONSTRAINTS, PREFERENCES AND HINTS
// ============================================================================

/// Mandatory cryptographic security requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CryptoRequirement {
    operation: CryptoOperationId,
    algorithm: Option<CryptoAlgorithmId>,
    properties: BTreeSet<CryptoPropertyId>,
}

impl CryptoRequirement {
    /// Creates a mandatory cryptographic requirement.
    #[must_use]
    pub fn new(operation: CryptoOperationId) -> Self {
        Self {
            operation,
            algorithm: None,
            properties: BTreeSet::new(),
        }
    }

    /// Requires a particular algorithm.
    #[must_use]
    pub fn requiring_algorithm(
        mut self,
        algorithm: CryptoAlgorithmId,
    ) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Adds a mandatory security property.
    #[must_use]
    pub fn requiring_property(
        mut self,
        property: CryptoPropertyId,
    ) -> Self {
        self.properties.insert(property);
        self
    }

    /// Returns the operation.
    #[must_use]
    pub fn operation(&self) -> &CryptoOperationId {
        &self.operation
    }

    /// Returns the required algorithm.
    #[must_use]
    pub fn algorithm(&self) -> Option<&CryptoAlgorithmId> {
        self.algorithm.as_ref()
    }

    /// Returns mandatory properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeSet<CryptoPropertyId> {
        &self.properties
    }
}

/// A desirable but non-mandatory cryptographic choice.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CryptoPreference {
    algorithm: Option<CryptoAlgorithmId>,
    provider: Option<CryptoProviderId>,
}

impl CryptoPreference {
    /// Creates an empty preference.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Prefers an algorithm without making it mandatory.
    #[must_use]
    pub fn preferring_algorithm(
        mut self,
        algorithm: CryptoAlgorithmId,
    ) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Prefers a provider without making it mandatory.
    #[must_use]
    pub fn preferring_provider(
        mut self,
        provider: CryptoProviderId,
    ) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Returns the preferred algorithm.
    #[must_use]
    pub fn algorithm(&self) -> Option<&CryptoAlgorithmId> {
        self.algorithm.as_ref()
    }

    /// Returns the preferred provider.
    #[must_use]
    pub fn provider(&self) -> Option<&CryptoProviderId> {
        self.provider.as_ref()
    }
}

/// Cryptographic restriction on permitted implementations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CryptoConstraint {
    forbidden_algorithms: BTreeSet<CryptoAlgorithmId>,
    forbidden_properties: BTreeSet<CryptoPropertyId>,
}

impl CryptoConstraint {
    /// Creates an empty constraint set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Prohibits an algorithm.
    #[must_use]
    pub fn forbidding_algorithm(
        mut self,
        algorithm: CryptoAlgorithmId,
    ) -> Self {
        self.forbidden_algorithms.insert(algorithm);
        self
    }

    /// Prohibits a security property or mechanism.
    #[must_use]
    pub fn forbidding_property(
        mut self,
        property: CryptoPropertyId,
    ) -> Self {
        self.forbidden_properties.insert(property);
        self
    }

    /// Returns prohibited algorithms.
    #[must_use]
    pub fn forbidden_algorithms(&self) -> &BTreeSet<CryptoAlgorithmId> {
        &self.forbidden_algorithms
    }

    /// Returns prohibited properties.
    #[must_use]
    pub fn forbidden_properties(&self) -> &BTreeSet<CryptoPropertyId> {
        &self.forbidden_properties
    }
}

/// Non-binding implementation hint.
///
/// Hints cannot override requirements or constraints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CryptoHint {
    name: String,
    value: String,
}

impl CryptoHint {
    /// Creates a non-secret implementation hint.
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> CryptoResult<Self> {
        let name = name.into();
        let value = value.into();

        if !valid_identifier(&name) || !valid_reference(&value) {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidRequest,
            ));
        }

        Ok(Self { name, value })
    }

    /// Returns the hint name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the hint value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Result of cryptographic capability evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CryptoFeasibility {
    /// The declared provider capabilities satisfy the requirement.
    Feasible,

    /// A mandatory property is unavailable.
    Infeasible {
        /// Reason for infeasibility.
        reason: CryptoErrorKind,
    },

    /// Available evidence is insufficient to decide.
    Unknown,
}

// ============================================================================
// PROVIDER CAPABILITIES
// ============================================================================

/// Declares one cryptographic capability.
///
/// Capability declarations describe provider claims. They do not constitute
/// independent cryptographic certification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CryptoCapability {
    operation: CryptoOperationId,
    algorithm: Option<CryptoAlgorithmId>,
    properties: BTreeSet<CryptoPropertyId>,
}

impl CryptoCapability {
    /// Creates a capability declaration.
    #[must_use]
    pub fn new(operation: CryptoOperationId) -> Self {
        Self {
            operation,
            algorithm: None,
            properties: BTreeSet::new(),
        }
    }

    /// Associates an algorithm.
    #[must_use]
    pub fn with_algorithm(
        mut self,
        algorithm: CryptoAlgorithmId,
    ) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Adds a claimed security property.
    #[must_use]
    pub fn with_property(
        mut self,
        property: CryptoPropertyId,
    ) -> Self {
        self.properties.insert(property);
        self
    }

    /// Returns the operation.
    #[must_use]
    pub fn operation(&self) -> &CryptoOperationId {
        &self.operation
    }

    /// Returns the algorithm.
    #[must_use]
    pub fn algorithm(&self) -> Option<&CryptoAlgorithmId> {
        self.algorithm.as_ref()
    }

    /// Returns declared properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeSet<CryptoPropertyId> {
        &self.properties
    }

    /// Evaluates the declared capability against a requirement.
    ///
    /// This evaluates declarations only, not actual cryptographic security.
    #[must_use]
    pub fn evaluate(
        &self,
        requirement: &CryptoRequirement,
    ) -> CryptoFeasibility {
        if self.operation != requirement.operation {
            return CryptoFeasibility::Infeasible {
                reason: CryptoErrorKind::UnsupportedOperation,
            };
        }

        if let Some(required) = &requirement.algorithm {
            if self.algorithm.as_ref() != Some(required) {
                return CryptoFeasibility::Infeasible {
                    reason: CryptoErrorKind::CapabilityUnavailable,
                };
            }
        }

        if !requirement.properties.is_subset(&self.properties) {
            return CryptoFeasibility::Infeasible {
                reason: CryptoErrorKind::RequirementUnsatisfied,
            };
        }

        CryptoFeasibility::Feasible
    }
}

// ============================================================================
// REQUEST TYPES
// ============================================================================

/// Generic operation context.
///
/// References identify protected resources; they never contain raw secrets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CryptoRequest {
    operation: CryptoOperationId,
    algorithm: CryptoAlgorithmId,
    key: Option<KeyReference>,
    requirement: CryptoRequirement,
    constraint: CryptoConstraint,
    preference: CryptoPreference,
    hints: Vec<CryptoHint>,
}

impl CryptoRequest {
    /// Creates a cryptographic operation request.
    pub fn new(
        operation: CryptoOperationId,
        algorithm: CryptoAlgorithmId,
        requirement: CryptoRequirement,
    ) -> CryptoResult<Self> {
        if requirement.operation != operation {
            return Err(CryptoError::new(
                CryptoErrorKind::InvalidRequest,
            ));
        }

        if let Some(required) = &requirement.algorithm {
            if required != &algorithm {
                return Err(CryptoError::new(
                    CryptoErrorKind::RequirementUnsatisfied,
                ));
            }
        }

        Ok(Self {
            operation,
            algorithm,
            key: None,
            requirement,
            constraint: CryptoConstraint::new(),
            preference: CryptoPreference::new(),
            hints: Vec::new(),
        })
    }

    /// Associates a protected key reference.
    #[must_use]
    pub fn using_key(mut self, key: KeyReference) -> Self {
        self.key = Some(key);
        self
    }

    /// Associates constraints.
    #[must_use]
    pub fn with_constraint(mut self, constraint: CryptoConstraint) -> Self {
        self.constraint = constraint;
        self
    }

    /// Associates preferences.
    #[must_use]
    pub fn with_preference(mut self, preference: CryptoPreference) -> Self {
        self.preference = preference;
        self
    }

    /// Adds a non-binding hint.
    #[must_use]
    pub fn with_hint(mut self, hint: CryptoHint) -> Self {
        self.hints.push(hint);
        self
    }

    /// Returns the operation.
    #[must_use]
    pub fn operation(&self) -> &CryptoOperationId {
        &self.operation
    }

    /// Returns the algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &CryptoAlgorithmId {
        &self.algorithm
    }

    /// Returns the key reference.
    #[must_use]
    pub fn key(&self) -> Option<&KeyReference> {
        self.key.as_ref()
    }

    /// Returns the mandatory requirement.
    #[must_use]
    pub fn requirement(&self) -> &CryptoRequirement {
        &self.requirement
    }

    /// Returns constraints.
    #[must_use]
    pub fn constraint(&self) -> &CryptoConstraint {
        &self.constraint
    }

    /// Returns preferences.
    #[must_use]
    pub fn preference(&self) -> &CryptoPreference {
        &self.preference
    }

    /// Returns hints.
    #[must_use]
    pub fn hints(&self) -> &[CryptoHint] {
        &self.hints
    }

    /// Validates that a provider capability satisfies the request.
    pub fn validate_capability(
        &self,
        capability: &CryptoCapability,
    ) -> CryptoResult<()> {
        if self.constraint.forbidden_algorithms.contains(&self.algorithm) {
            return Err(CryptoError::new(
                CryptoErrorKind::RequirementUnsatisfied,
            ));
        }

        if self.constraint.forbidden_properties
            .iter()
            .any(|property| capability.properties.contains(property))
        {
            return Err(CryptoError::new(
                CryptoErrorKind::RequirementUnsatisfied,
            ));
        }

        match capability.evaluate(&self.requirement) {
            CryptoFeasibility::Feasible => Ok(()),
            CryptoFeasibility::Infeasible { reason } => {
                Err(CryptoError::new(reason))
            }
            CryptoFeasibility::Unknown => {
                Err(CryptoError::new(
                    CryptoErrorKind::CapabilityUnavailable,
                ))
            }
        }
    }
}

// ============================================================================
// OPERATION METADATA
// ============================================================================

/// Metadata returned by a successful encryption operation.
///
/// The nonce is public algorithm metadata, not secret material.
/// Its interpretation and uniqueness requirements are algorithm-specific.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptionMetadata {
    algorithm: CryptoAlgorithmId,
    key: KeyReference,
    provider: CryptoProviderId,
    nonce: Vec<u8>,
    properties: BTreeSet<CryptoPropertyId>,
}

impl EncryptionMetadata {
    /// Constructs metadata for a completed encryption operation.
    ///
    /// Providers must only include properties actually achieved.
    pub fn new(
        algorithm: CryptoAlgorithmId,
        key: KeyReference,
        provider: CryptoProviderId,
        nonce: Vec<u8>,
        properties: BTreeSet<CryptoPropertyId>,
    ) -> Self {
        Self {
            algorithm,
            key,
            provider,
            nonce,
            properties,
        }
    }

    /// Returns the algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &CryptoAlgorithmId {
        &self.algorithm
    }

    /// Returns the key reference.
    #[must_use]
    pub fn key(&self) -> &KeyReference {
        &self.key
    }

    /// Returns the provider.
    #[must_use]
    pub fn provider(&self) -> &CryptoProviderId {
        &self.provider
    }

    /// Returns the nonce.
    #[must_use]
    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    /// Returns achieved properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeSet<CryptoPropertyId> {
        &self.properties
    }
}

/// Metadata describing a completed signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignatureMetadata {
    algorithm: CryptoAlgorithmId,
    key: KeyReference,
    provider: CryptoProviderId,
}

impl SignatureMetadata {
    /// Creates signature metadata.
    #[must_use]
    pub fn new(
        algorithm: CryptoAlgorithmId,
        key: KeyReference,
        provider: CryptoProviderId,
    ) -> Self {
        Self {
            algorithm,
            key,
            provider,
        }
    }

    /// Returns the algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &CryptoAlgorithmId {
        &self.algorithm
    }

    /// Returns the key reference.
    #[must_use]
    pub fn key(&self) -> &KeyReference {
        &self.key
    }

    /// Returns the provider.
    #[must_use]
    pub fn provider(&self) -> &CryptoProviderId {
        &self.provider
    }
}

/// Result of key encapsulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncapsulationResult {
    ciphertext: Vec<u8>,
    shared_secret: SecretReference,
}

impl EncapsulationResult {
    /// Creates an encapsulation result.
    ///
    /// The shared secret is represented by a protected reference.
    #[must_use]
    pub fn new(
        ciphertext: Vec<u8>,
        shared_secret: SecretReference,
    ) -> Self {
        Self {
            ciphertext,
            shared_secret,
        }
    }

    /// Returns the encapsulation ciphertext.
    #[must_use]
    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    /// Returns the protected shared-secret reference.
    #[must_use]
    pub fn shared_secret(&self) -> &SecretReference {
        &self.shared_secret
    }
}

// ============================================================================
// CRYPTOGRAPHIC PROVIDER CONTRACT
// ============================================================================

/// A cryptographic provider.
///
/// Implementations must use genuine, reviewed cryptographic algorithms.
/// They must not return successful results from simulated operations.
///
/// All data-processing operations use streaming interfaces so that input
/// size is not artificially restricted by this API.
///
/// Providers are responsible for:
///
/// - Correct algorithm implementation.
/// - Parameter validation.
/// - Key authorization.
/// - Key lifecycle.
/// - Nonce uniqueness where required.
/// - Authentication and integrity.
/// - Constant-time operations where required by the algorithm.
/// - Secure error handling.
/// - Resource accounting.
/// - Algorithm-specific limits.
/// - Correct output semantics.
///
/// Provider capability declarations must accurately describe actual support.
pub trait CryptoProvider: Send + Sync {
    /// Returns the provider identity.
    fn id(&self) -> &CryptoProviderId;

    /// Returns declared capabilities.
    fn capabilities(&self) -> &[CryptoCapability];

    /// Computes a digest over a streaming input.
    ///
    /// The provider writes the complete digest to `output`.
    fn hash(
        &self,
        _request: &CryptoRequest,
        _input: &mut dyn Read,
        _output: &mut dyn Write,
    ) -> CryptoResult<()> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }

    /// Encrypts a streaming input.
    ///
    /// The provider must use authenticated encryption when the requested
    /// security properties require authenticity and integrity.
    ///
    /// Authentication metadata and nonce information must be returned.
    fn encrypt(
        &self,
        _request: &CryptoRequest,
        _plaintext: &mut dyn Read,
        _ciphertext: &mut dyn Write,
        _associated_data: &[u8],
    ) -> CryptoResult<EncryptionMetadata> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }

    /// Decrypts a streaming input.
    ///
    /// Implementations must not expose unauthenticated plaintext as
    /// successfully decrypted output.
    ///
    /// Providers must use an appropriate transactional or protected
    /// temporary-output strategy when authentication is only confirmed
    /// after processing the complete ciphertext.
    fn decrypt(
        &self,
        _request: &CryptoRequest,
        _metadata: &EncryptionMetadata,
        _ciphertext: &mut dyn Read,
        _plaintext: &mut dyn Write,
        _associated_data: &[u8],
    ) -> CryptoResult<()> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }

    /// Signs a streaming message.
    ///
    /// Signature output is written to `signature`.
    fn sign(
        &self,
        _request: &CryptoRequest,
        _message: &mut dyn Read,
        _signature: &mut dyn Write,
    ) -> CryptoResult<SignatureMetadata> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }

    /// Verifies a streaming message against a signature.
    ///
    /// A valid signature returns `Ok(())`.
    /// An invalid signature returns `SignatureVerificationFailed`.
    fn verify(
        &self,
        _request: &CryptoRequest,
        _metadata: &SignatureMetadata,
        _message: &mut dyn Read,
        _signature: &mut dyn Read,
    ) -> CryptoResult<()> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }

    /// Performs public-key encapsulation.
    ///
    /// The shared secret must remain protected by the provider or its
    /// associated key-management system.
    fn encapsulate(
        &self,
        _request: &CryptoRequest,
    ) -> CryptoResult<EncapsulationResult> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }

    /// Performs public-key decapsulation.
    ///
    /// Returns an opaque reference to the resulting protected secret.
    fn decapsulate(
        &self,
        _request: &CryptoRequest,
        _ciphertext: &[u8],
    ) -> CryptoResult<SecretReference> {
        Err(CryptoError::new(
            CryptoErrorKind::UnsupportedOperation,
        ))
    }
}

// ============================================================================
// PROVIDER VALIDATION
// ============================================================================

/// Validates a provider's declared support for a request.
///
/// This does not independently verify the implementation's security.
pub fn validate_provider(
    provider: &dyn CryptoProvider,
    request: &CryptoRequest,
) -> CryptoResult<()> {
    let capability = provider
        .capabilities()
        .iter()
        .find(|capability| {
            capability.operation == request.operation
                && capability.algorithm.as_ref() == Some(&request.algorithm)
        })
        .ok_or_else(|| {
            CryptoError::new(CryptoErrorKind::CapabilityUnavailable)
                .with_operation(request.operation.clone())
                .with_provider(provider.id().clone())
        })?;

    request
        .validate_capability(capability)
        .map_err(|error| {
            CryptoError::new(error.kind())
                .with_operation(request.operation.clone())
                .with_provider(provider.id().clone())
        })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn algorithm() -> CryptoAlgorithmId {
        CryptoAlgorithmId::new("cryptography::hash::sha256").unwrap()
    }

    fn operation() -> CryptoOperationId {
        CryptoOperationId::new("cryptography::hash").unwrap()
    }

    fn property() -> CryptoPropertyId {
        CryptoPropertyId::new(
            "cryptography::property::collision_resistance",
        )
        .unwrap()
    }

    #[test]
    fn accepts_open_world_algorithm_identifiers() {
        let current = algorithm();

        let future = CryptoAlgorithmId::new(
            "future::cryptography::new_algorithm",
        )
        .unwrap();

        assert_ne!(current, future);
    }

    #[test]
    fn rejects_invalid_identifiers() {
        assert!(
            CryptoAlgorithmId::new("").is_err()
        );

        assert!(
            CryptoAlgorithmId::new("invalid::").is_err()
        );

        assert!(
            CryptoAlgorithmId::new(" invalid").is_err()
        );
    }

    #[test]
    fn distinguishes_requirements_from_preferences() {
        let required = CryptoRequirement::new(operation())
            .requiring_algorithm(algorithm());

        let preferred = CryptoPreference::new()
            .preferring_algorithm(algorithm());

        assert_eq!(
            required.algorithm(),
            preferred.algorithm()
        );

        // Their types and semantics remain distinct.
    }

    #[test]
    fn capability_evaluation_checks_required_properties() {
        let capability = CryptoCapability::new(operation())
            .with_algorithm(algorithm())
            .with_property(property());

        let requirement = CryptoRequirement::new(operation())
            .requiring_algorithm(algorithm())
            .requiring_property(property());

        assert_eq!(
            capability.evaluate(&requirement),
            CryptoFeasibility::Feasible
        );
    }

    #[test]
    fn missing_property_is_not_feasible() {
        let capability = CryptoCapability::new(operation())
            .with_algorithm(algorithm());

        let requirement = CryptoRequirement::new(operation())
            .requiring_algorithm(algorithm())
            .requiring_property(property());

        assert!(matches!(
            capability.evaluate(&requirement),
            CryptoFeasibility::Infeasible {
                reason: CryptoErrorKind::RequirementUnsatisfied
            }
        ));
    }

    #[test]
    fn forbidden_algorithms_are_rejected() {
        let constraint = CryptoConstraint::new()
            .forbidding_algorithm(algorithm());

        let request = CryptoRequest::new(
            operation(),
            algorithm(),
            CryptoRequirement::new(operation()),
        )
        .unwrap()
        .with_constraint(constraint);

        let capability = CryptoCapability::new(operation())
            .with_algorithm(algorithm());

        assert!(
            request.validate_capability(&capability).is_err()
        );
    }

    #[test]
    fn key_references_do_not_expose_their_values_in_debug() {
        let key = KeyReference::new("kms::production::key").unwrap();

        assert!(
            !format!("{key:?}").contains("production")
        );
    }

    #[test]
    fn secret_references_are_opaque_in_debug() {
        let secret = SecretReference::new(
            "secrets::application::credential",
        )
        .unwrap();

        assert!(
            !format!("{secret:?}").contains("application")
        );
    }

    #[test]
    fn errors_have_stable_classifications() {
        let error = CryptoError::new(
            CryptoErrorKind::AuthenticationFailed,
        );

        assert_eq!(
            error.kind().as_str(),
            "authentication_failed"
        );
    }

    #[test]
    fn request_rejects_inconsistent_mandatory_algorithm() {
        let required = CryptoAlgorithmId::new(
            "cryptography::signature::ed25519",
        )
        .unwrap();

        let actual = CryptoAlgorithmId::new(
            "cryptography::hash::sha256",
        )
        .unwrap();

        let request = CryptoRequest::new(
            operation(),
            actual,
            CryptoRequirement::new(operation())
                .requiring_algorithm(required),
        );

        assert!(request.is_err());
    }
}
