
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Security
//!
//! Path: `src/stdlib/security/mod.rs`
//!
//! Language: Zamani
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## 1. Purpose
//!
//! This module is the canonical standard-library security aggregation
//! boundary.
//!
//! It exposes portable security contracts and provider-independent
//! interfaces for Zamani programs operating across:
//!
//! - Classical computing.
//! - Quantum computing.
//! - Hardware description and hardware execution.
//! - Hybrid computation.
//! - Distributed computation.
//! - Artificial intelligence and machine learning.
//! - Embedded and edge computing.
//! - Networking and communication.
//! - Future computational domains.
//!
//! The module provides a unified public security namespace without
//! combining the implementation responsibilities of its constituent
//! security subsystems.
//!
//! ## 2. Architectural ownership
//!
//! This module owns:
//!
//! - Security module composition.
//! - Public security module exports.
//! - Security API aggregation.
//! - Stable security-facing integration paths.
//!
//! The individual modules retain their own implementation authority.
//!
//! This module does not independently implement:
//!
//! - Cryptographic algorithms.
//! - Hash algorithms.
//! - Digital signature algorithms.
//! - Authentication.
//! - Identity verification.
//! - Trust evaluation.
//! - Authorization policy evaluation.
//! - Secret storage.
//! - Key storage.
//! - Hardware security modules.
//! - Secure enclave implementations.
//! - Quantum error correction.
//! - Quantum hardware execution.
//! - Hardware discovery.
//! - Resource allocation.
//! - Runtime scheduling.
//! - Distributed consensus.
//! - Network transport.
//! - Compiler lowering.
//!
//! Those responsibilities belong to their designated authorities.
//!
//! ## 3. Security module hierarchy
//!
//! The public security architecture is:
//!
//! security
//! |
//! +-- cryptography
//! |   +-- Algorithm identities
//! |   +-- Cryptographic requirements
//! |   +-- Cryptographic capabilities
//! |   +-- Cryptographic constraints
//! |   +-- Cryptographic preferences
//! |   +-- Cryptographic hints
//! |   +-- Provider contracts
//! |   +-- Cryptographic requests
//! |   +-- Operation errors
//! |
//! +-- hashing
//! |   +-- Hash algorithms
//! |   +-- Streaming hashing
//! |   +-- Digest representations
//! |   +-- Digest verification
//! |   +-- Hash providers
//! |
//! +-- signatures
//! |   +-- Signing contracts
//! |   +-- Verification contracts
//! |   +-- Signature metadata
//! |   +-- Provider-backed operations
//! |
//! +-- key_management
//! |   +-- Key generation
//! |   +-- Protected key import
//! |   +-- Key derivation
//! |   +-- Key inspection
//! |   +-- Key rotation
//! |   +-- Key revocation
//! |   +-- Key destruction
//! |   +-- Key authorization
//! |
//! +-- identity
//! |   +-- Identity identifiers
//! |   +-- Principal identifiers
//! |   +-- Identity records
//! |   +-- Identity bindings
//! |   +-- Identity providers
//! |
//! +-- policy
//!     +-- Policy identifiers
//!     +-- Authorization requests
//!     +-- Authorization decisions
//!     +-- Policy obligations
//!     +-- Audit events
//!     +-- Audit providers
//!
//! ## 4. Single-authority rule
//!
//! Every security concept must have one authoritative implementation.
//!
//! Specifically:
//!
//! - cryptography owns cryptographic operation contracts.
//! - hashing owns hashing APIs.
//! - signatures owns signing and verification APIs.
//! - key_management owns managed-key lifecycle contracts.
//! - identity owns portable identity representations.
//! - policy owns standard-library authorization and audit contracts.
//!
//! The modules may depend on one another through explicit interfaces.
//! They must not create competing definitions of the same concepts.
//!
//! ## 5. Existing repository integration
//!
//! This module integrates with:
//!
//! - `crate::stdlib::crypto`:
//!   Legacy cryptographic implementation during migration.
//!
//! - `crate::stdlib::capability`:
//!   Standard-library capability descriptions.
//!
//! - `crate::stdlib::requirement`:
//!   Standard-library requirement contracts.
//!
//! - `crate::stdlib::constraint`:
//!   Standard-library constraint contracts.
//!
//! - `crate::stdlib::preference`:
//!   Standard-library preference contracts.
//!
//! - `crate::stdlib::hint`:
//!   Standard-library hint contracts.
//!
//! - `crate::stdlib::feasibility`:
//!   Standard-library feasibility contracts.
//!
//! - `crate::frontend::semantic`:
//!   Canonical source-language semantic authority.
//!
//! - `crate::quantum`:
//!   Quantum computation and quantum IR.
//!
//! - `crate::hdl`:
//!   Hardware-description and hardware semantics.
//!
//! - `crate::ai`:
//!   AI computation and model semantics.
//!
//! - `crate::distributed`:
//!   Distributed computation and coordination.
//!
//! - `crate::runtime`:
//!   Runtime execution and security enforcement.
//!
//! - HAL and security providers:
//!   Actual platform-specific security implementations.
//!
//! These integrations must use explicit adapters.
//!
//! This module must not make the compiler, semantic layer, quantum IR,
//! HDL IR or runtime depend on a duplicate security implementation.
//!
//! ## 6. Security invariants
//!
//! The following rules apply across the security namespace:
//!
//! 1. Unsupported security operations must fail explicitly.
//! 2. Invalid cryptographic operations must never report success.
//! 3. Authentication failures must never produce accepted plaintext.
//! 4. Private keys and secret material must not be exposed through
//!    ordinary public identity or operation records.
//! 5. Key references must not be confused with actual key material.
//! 6. Identity registration must not imply authentication.
//! 7. Identity resolution must not imply trust.
//! 8. Trust must not automatically imply authorization.
//! 9. Authorization must be deny-by-default.
//! 10. Indeterminate authorization decisions must not authorize.
//! 11. Mandatory audit failures must not silently be ignored.
//! 12. Provider declarations must not be treated as proof of security.
//! 13. Cryptographic algorithm metadata must accurately describe
//!     the operation that was actually performed.
//! 14. Security failures must not disclose secret material.
//! 15. Security requirements must not silently become preferences.
//! 16. Preferences and hints must not override mandatory constraints.
//! 17. Provider substitution must preserve required security properties.
//! 18. Unsupported post-quantum operations must not use fabricated
//!     cryptographic results.
//!
//! ## 7. POCO-REAF and portability
//!
//! Security contracts are independent of the physical machine.
//!
//! This module must not impose arbitrary universal limits on:
//!
//! - Cryptographic providers.
//! - Algorithms.
//! - Keys.
//! - Identities.
//! - Principals.
//! - Security policies.
//! - Authorization requests.
//! - Distributed participants.
//! - Devices.
//! - Processors.
//! - Memory.
//! - Computational domains.
//!
//! Open-world identifiers and extensible provider contracts are preferred
//! over closed hardware, vendor or algorithm enumerations.
//!
//! Actual algorithm limits, provider capabilities, operational budgets
//! and available resources remain explicit implementation facts.
//!
//! Literal infinite storage or computation is not promised.
//!
//! The architecture must remain capable of scaling subject to available
//! resources without introducing arbitrary language-level ceilings.
//!
//! ## 8. Trust and security-domain boundaries
//!
//! The repository contains canonical security grammar for:
//!
//! - Identity.
//! - Trust.
//! - Authorization.
//! - Cryptography.
//! - Key management.
//! - Permissions.
//! - Privacy.
//! - Secrets.
//! - Security constraints.
//! - Provenance.
//! - Secure computation.
//! - Zero-knowledge proofs.
//! - Auditing.
//!
//! Grammar defines source-level syntax and intent.
//!
//! Semantic interpretation belongs to `crate::frontend::semantic`.
//!
//! Actual security operations require implemented and verified providers.
//!
//! The existence of a grammar rule must never be interpreted as evidence
//! that its corresponding runtime operation is implemented.
//!
//! Trust evaluation must remain a distinct responsibility. It must not
//! be fabricated by identity registration or policy module initialization.
//!
//! ## 9. Lifecycle
//!
//! This module deliberately does not implement a global security
//! initialization or shutdown sequence.
//!
//! Its child modules define contracts and provider-backed operations.
//!
//! Providers are supplied explicitly to the relevant APIs.
//!
//! Their lifecycle belongs to the provider, runtime and standard-library
//! lifecycle authorities.
//!
//! This prevents:
//!
//! - Unconditional initialization of every security subsystem.
//! - Hidden global cryptographic state.
//! - Uncontrolled private-key initialization.
//! - Duplicate initialization of legacy crypto modules.
//! - Printing success messages without successful initialization.
//!
//! ## 10. Dependency direction
//!
//! The dependency direction is:
//!
//! Zamani source
//!      |
//!      v
//! Canonical grammar
//!      |
//!      v
//! Frontend AST
//!      |
//!      v
//! Semantic security validation
//!      |
//!      v
//! Standard-library security contracts
//!      |
//!      v
//! Compiler / IR / execution planning
//!      |
//!      v
//! Runtime / HAL / security providers
//!      |
//!      v
//! Actual security operations
//!
//! No downstream component may redefine source-language security meaning.
//!
//! No security module may become a competing compiler, semantic,
//! hardware-management or runtime authority.
//!
//! ## 11. Compatibility
//!
//! Existing legacy APIs under `crate::stdlib::crypto` may remain available
//! during migration.
//!
//! They are not automatically re-exported here.
//!
//! Legacy operations must be audited and replaced before being exposed
//! through the stable security API.
//!
//! The public paths under `crate::stdlib::security` are the canonical
//! standard-library security integration paths.
//!
//! ## 12. Completion criteria
//!
//! This module is ready for integration when:
//!
//! - All six declared child modules exist.
//! - All child modules compile independently.
//! - Their public contracts agree on shared types.
//! - The parent standard-library module exposes `security`.
//! - No duplicate security authority is introduced.
//! - No unsafe Rust is introduced.
//! - Existing security tests pass.
//! - Cross-module security integration tests pass.
//! - All unsupported operations fail explicitly.

/// Portable cryptographic operation contracts and provider interfaces.
///
/// This module owns cryptographic operation descriptions, requirements,
/// capabilities, constraints, preferences, hints and provider contracts.
///
/// It does not implement the underlying algorithms.
pub mod cryptography;

/// Cryptographic hashing APIs and digest operations.
///
/// This module owns hashing operations, digest representations,
/// streaming interfaces and hashing-provider integration.
pub mod hashing;

/// Portable identity and principal representations.
///
/// This module does not authenticate identities, establish trust,
/// grant permissions or store credentials.
pub mod identity;

/// Protected key-management contracts.
///
/// This module defines managed-key operations and provider interfaces.
/// Actual secret storage and key protection belong to trusted providers.
pub mod key_management;

/// Security-policy, authorization and audit contracts.
///
/// This module provides fail-closed authorization interfaces and
/// audit-provider integration.
pub mod policy;

/// Provider-backed digital signing and signature verification.
///
/// This module does not expose private-key material or implement
/// cryptographic signing algorithms directly.
pub mod signatures;

// ============================================================================
// PUBLIC SECURITY API
// ============================================================================

// Cryptographic operation contracts.

pub use cryptography::{
    CryptoAlgorithmId,
    CryptoCapability,
    CryptoConstraint,
    CryptoError,
    CryptoErrorKind,
    CryptoFeasibility,
    CryptoHint,
    CryptoOperationId,
    CryptoPreference,
    CryptoPropertyId,
    CryptoProvider,
    CryptoProviderId,
    CryptoRequest,
    CryptoRequirement,
    CryptoResult,
    KeyReference,
    SecretReference,
};

// Hashing contracts.

pub use hashing::{
    BuiltinHashProvider,
    HashAlgorithm,
    HashDigest,
    Hasher,
};

// Identity contracts.

pub use identity::{
    IdentityDirectory,
    IdentityProvider,
    IdentityRecord,
    IdentityResult,
    Principal,
};

// Key-management contracts.

pub use key_management::{
    KeyAccessAuthorizer,
    KeyManager,
    KeyManagementProvider,
    KeyMetadata,
    KeySpec,
};

// Policy and authorization contracts.

pub use policy::{
    AuditEvent,
    AuditSink,
    AuthorizationReceipt,
    AuthorizationRequest,
    AuthorizationService,
    PolicyDecision,
    PolicyEffect,
    PolicyEvaluator,
};

// Signature operations.

pub use signatures::{
    sign_to,
    verify,
};
