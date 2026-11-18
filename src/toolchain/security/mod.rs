
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain Security
//!
//! Path: `src/toolchain/security/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for the Zamani toolchain's
//! security subsystem.
//!
//! It coordinates security policy evaluation, trust assessment,
//! digital signatures, content integrity, lifecycle provenance,
//! and security validation.
//!
//! It provides a common security API for all toolchain operations,
//! independently of the computational domain or physical target.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Security subsystem composition.
//! - Public security module declarations.
//! - Deliberate security API exports.
//! - Security subsystem architectural invariants.
//! - Integration boundaries between security authorities.
//!
//! It does not independently implement:
//!
//! - Authentication.
//! - Identity resolution.
//! - Authorization policy evaluation.
//! - Trust-store persistence.
//! - Cryptographic algorithms.
//! - Hashing algorithms.
//! - Key generation or key storage.
//! - Package archive parsing.
//! - Dependency resolution.
//! - Filesystem sandboxing.
//! - External process execution.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime isolation.
//! - Runtime enforcement.
//! - Compiler security analysis.
//! - Semantic interpretation.
//! - Security grammar parsing.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Canonical repository authorities
//!
//! Source-level security semantics:
//!     grammar/spec/security.md
//!
//! Security source grammar:
//!     grammar/security/
//!
//! Universal policy grammar:
//!     grammar/policies/
//!
//! Semantic security interpretation:
//!     crate::frontend::semantic
//!
//! Canonical provider contracts:
//!     crate::stdlib::provider
//!
//! Cryptographic operations:
//!     crate::stdlib::security
//!
//! Hashing:
//!     crate::stdlib::security::hashing
//!
//! Cryptographic providers:
//!     crate::stdlib::security::cryptography
//!
//! Compiler security:
//!     crate::compiler::security
//!
//! Backend security:
//!     crate::backend::security
//!
//! External process execution:
//!     crate::backend::process
//!
//! Hardware security:
//!     crate::zhal::security
//!
//! Runtime security:
//!     crate::zur::security
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Toolchain results:
//!     crate::toolchain::result
//!
//! Package verification:
//!     crate::toolchain::packages::verification
//!
//! Deployment validation:
//!     crate::toolchain::deployment::validation
//!
//! ## Security architecture
//!
//! ```text
//!                 Zamani source
//!                       |
//!                       v
//!              Frontend / Semantic
//!                       |
//!                       v
//!              Security requirements
//!                       |
//!                       v
//!                Toolchain security
//!                       |
//!          +------------+------------+
//!          |            |            |
//!          v            v            v
//!        Policy       Trust      Integrity
//!      evaluation  assessment  verification
//!          |            |            |
//!          +------------+------------+
//!                       |
//!              +--------+--------+
//!              |                 |
//!              v                 v
//!          Signatures        Provenance
//!              |                 |
//!              +--------+--------+
//!                       |
//!                       v
//!              Security validation
//!                       |
//!             +---------+---------+
//!             |                   |
//!             v                   v
//!        Toolchain APIs       Providers
//!             |                   |
//!             +---------+---------+
//!                       |
//!           +-----------+-----------+
//!           |           |           |
//!           v           v           v
//!         Package    Registry   Deployment
//!         services   services    services
//!           |           |           |
//!           +-----------+-----------+
//!                       |
//!                       v
//!                Compiler / Backend
//!                       |
//!                       v
//!                    ZHAL / ZUR
//! ```
//!
//! ## Security decision invariants
//!
//! Every security-sensitive operation must preserve these distinctions:
//!
//! - Authentication is not authorization.
//! - Authorization is not trust.
//! - Trust is not integrity.
//! - Integrity is not authenticity.
//! - Authenticity is not provenance.
//! - Provenance is not authorization.
//! - A registered provider is not automatically trusted.
//! - A valid signature is not automatically a trusted signature.
//! - A successful hash comparison does not establish publisher identity.
//! - A successful build does not automatically establish deployment permission.
//!
//! Security evidence must be evaluated by the authority responsible
//! for that evidence.
//!
//! Missing, invalid, unverified or inconclusive evidence must never
//! be silently converted into successful verification.
//!
//! ## Fail-closed behavior
//!
//! Security validation must distinguish:
//!
//! - Passed: required verification has succeeded.
//! - Blocked: an applicable security requirement has failed.
//! - Unresolved: required evidence or verification is unavailable.
//! - NotEvaluated: the required verification has not been performed.
//!
//! Only the canonical validation authority determines whether an
//! operation may proceed.
//!
//! An absent validation result is never equivalent to permission.
//!
//! An operational budget exhaustion is never equivalent to success.
//!
//! ## POCO-REAF and scalability
//!
//! Security functionality is independent of:
//!
//! - Computational domain.
//! - Processor architecture.
//! - Operating system.
//! - Hardware vendor.
//! - Physical device count.
//! - Distributed topology.
//! - Accelerator implementation.
//! - Quantum hardware implementation.
//! - Deployment environment.
//!
//! No universal maximum is imposed on:
//!
//! - Policies.
//! - Rules.
//! - Principals.
//! - Resources.
//! - Trust relationships.
//! - Evidence records.
//! - Providers.
//! - Packages.
//! - Artifacts.
//! - Provenance records.
//! - Computational domains.
//! - Hardware resources.
//!
//! Operational limits may be introduced by explicit configuration,
//! available implementation resources and security policies.
//!
//! Such limits must remain configurable and must report exhaustion
//! explicitly.
//!
//! Literal infinite computation or storage is not promised.
//!
//! POCO-REAF requires portable security intent, compatible security
//! contracts and honest verification outcomes across realizations.
//!
//! ## Extension policy
//!
//! New security authorities and verification mechanisms must integrate
//! through their appropriate provider or extension contracts.
//!
//! This composition root must not become a closed enumeration of:
//!
//! - Cryptographic algorithms.
//! - Signature schemes.
//! - Hash algorithms.
//! - Authentication systems.
//! - Trust providers.
//! - Security vendors.
//! - Hardware security technologies.
//! - Computational domains.
//!
//! New functionality must not require duplicating existing authorities.
//!
//! ## Module composition
//!
//! The following modules retain independent implementation ownership.

/// Authorization and security policy evaluation.
///
/// Owns policy definitions, rules, selectors, conditions,
/// combining algorithms, evaluation budgets and policy decisions.
///
/// Does not authenticate identities or execute cryptography.
pub mod policy;

/// Trust evidence and trust assessment.
///
/// Owns trust evidence contracts, issuer references, trust policies,
/// signature-verification coordination and revocation-check contracts.
///
/// Uses canonical provider identities and cryptographic verification.
pub mod trust;

/// Digital signature integration.
///
/// Adapts the canonical standard-library cryptographic providers
/// to the toolchain trust and signature contracts.
///
/// Does not implement cryptographic algorithms or key management.
pub mod signatures;

/// Content integrity verification.
///
/// Coordinates canonical hashing providers, streaming content
/// verification, digest comparisons and explicit byte budgets.
///
/// Does not implement hashing algorithms.
pub mod integrity;

/// Lifecycle security provenance.
///
/// Preserves security-relevant lineage across compilation,
/// artifacts, packages, registries, installation and deployment.
///
/// Does not replace the compiler provenance authority.
pub mod provenance;

/// Aggregate security validation.
///
/// Coordinates policy, trust, signature, integrity, provenance
/// and registered security-validation extensions.
///
/// Owns aggregate validation reports and fail-closed acceptance
/// decisions, not the underlying verification implementations.
pub mod validation;

// ============================================================================
// DELIBERATE PUBLIC API
// ============================================================================
//
// The detailed APIs remain available through their owning modules.
//
// Only stable, commonly needed types are re-exported here.
// This avoids wildcard exports, accidental name collisions and
// unnecessary coupling between independent security authorities.

// Policy API

/// Security policy evaluation context.
pub use policy::PolicyContext;

/// Result of a security policy evaluation.
pub use policy::PolicyDecision;

/// Policy evaluation engine.
pub use policy::PolicyEngine;

/// Collection of security policies.
pub use policy::PolicySet;

/// Individual security policy.
pub use policy::SecurityPolicy;

// Trust API

/// Evidence used in a trust assessment.
pub use trust::TrustEvidence;

/// Result of evaluating trust evidence.
pub use trust::TrustAssessment;

/// Trust decision.
pub use trust::TrustDecision;

/// Trust evaluation policy.
pub use trust::TrustPolicy;

// Integrity API

/// Result of content-integrity verification.
pub use integrity::IntegrityReport;

/// Content-integrity verification service.
pub use integrity::IntegrityVerifier;

// Provenance API

/// Immutable lifecycle provenance record.
pub use provenance::ProvenanceRecord;

/// Immutable provenance snapshot.
pub use provenance::ProvenanceSnapshot;

// Aggregate validation API

/// Security validation request.
pub use validation::SecurityValidationRequest;

/// Aggregate security validation report.
pub use validation::SecurityValidationReport;

/// Security validation coordinator.
pub use validation::SecurityValidator;

/// Explicit outcome of a security validation operation.
pub use validation::ValidationStatus;

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================
//
// Every module in this subsystem must preserve:
//
// 1. Safe Rust only.
// 2. One authoritative implementation per security responsibility.
// 3. Default-deny authorization.
// 4. Verified evidence for security-sensitive acceptance.
// 5. Explicit unresolved verification outcomes.
// 6. No implicit trust in registered providers.
// 7. No implicit trust in unsigned or unverifiable artifacts.
// 8. No substitution of integrity for authenticity.
// 9. No substitution of authenticity for trust.
// 10. No substitution of provenance for authorization.
// 11. No cryptographic algorithm duplication.
// 12. No independent hardware security implementation.
// 13. No independent runtime security enforcement.
// 14. No independent package verification implementation.
// 15. No source-language grammar duplication.
// 16. No competing semantic security model.
// 17. No arbitrary hardware capacity ceilings.
// 18. No hard-coded provider or vendor inventory.
// 19. No silent acceptance of unknown evidence.
// 20. No fabricated successful verification.
// 21. No secret material in diagnostics.
// 22. No implicit network or filesystem side effects.
// 23. Explicit operational budgets where required.
// 24. Deterministic results for equivalent validated inputs,
//     where the underlying authority guarantees determinism.
// 25. Explicit version and compatibility contracts.
// 26. Auditable security-sensitive operations.
// 27. Consistent human and AI caller authorization.
// 28. Explicit integration with authoritative security providers.
// 29. Preservation of original subsystem errors.
// 30. Honest reporting of incomplete or unsupported operations.
