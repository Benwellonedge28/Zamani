
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Security Composition Root
//!
//! Path: `src/zhal/security/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the public security composition root for ZHAL.
//!
//! It integrates the security boundaries required to access,
//! inspect and interact with physical and virtual hardware
//! throughout the Zamani computing architecture.
//!
//! It composes three independently owned security authorities:
//!
//! 1. Provider trust and verification.
//! 2. Resource-scoped hardware authorization.
//! 3. Hardware execution isolation.
//!
//! These authorities remain independently implemented and
//! independently configurable.
//!
//! This module exposes their stable ZHAL integration paths.
//!
//! It does not implement a second security engine.
//!
//! # Supported computational domains
//!
//! The security composition is independent of computational
//! domain and supports hardware providers associated with:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware realization.
//! - FPGA and ASIC devices.
//! - GPUs and other accelerators.
//! - AI and tensor computing.
//! - Embedded computing.
//! - Hybrid computing.
//! - Distributed computing.
//! - Networking.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! New computational domains must integrate through existing
//! security contracts rather than requiring changes to this
//! composition root.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Public security-module composition.
//! - Stable security integration paths.
//! - Public exposure of ZHAL security services.
//! - Public exposure of provider-trust contracts.
//! - Public exposure of hardware authorization integration.
//! - Public exposure of hardware isolation integration.
//! - Security composition documentation.
//!
//! This module does NOT own:
//!
//! - Principal identity.
//! - Authentication.
//! - Credential storage.
//! - Cryptographic algorithms.
//! - Key management.
//! - Trust-anchor persistence.
//! - Authorization policy definitions.
//! - Authorization policy evaluation.
//! - Permission declarations.
//! - Permission persistence.
//! - Security audit persistence.
//! - Isolation implementation.
//! - Process sandbox implementation.
//! - Container implementation.
//! - Virtual-machine implementation.
//! - Enclave implementation.
//! - Hardware discovery.
//! - Provider registration.
//! - Hardware selection.
//! - Capability discovery.
//! - Resource allocation.
//! - Hardware scheduling.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//!
//! Those responsibilities remain with their existing
//! authoritative subsystems.
//!
//! # Canonical repository authorities
//!
//! Provider identity and lifecycle:
//!
//!     crate::stdlib::provider
//!
//! Standard-library capability authority:
//!
//!     crate::stdlib::capability
//!
//! Principal identity:
//!
//!     crate::stdlib::security::identity
//!
//! Authorization requests, evaluation and audit:
//!
//!     crate::stdlib::security::policy
//!
//! Runtime authenticated-session permission enforcement:
//!
//!     crate::zur::security::permissions
//!
//! Runtime security policy enforcement:
//!
//!     crate::zur::security::policy
//!
//! Actual isolation establishment and enforcement:
//!
//!     crate::zur::security::isolation
//!
//! Distributed authentication and authorization:
//!
//!     crate::distributed::security
//!
//! Provider trust integration:
//!
//!     crate::zhal::security::trust
//!
//! Hardware authorization integration:
//!
//!     crate::zhal::security::authorization
//!
//! Hardware isolation integration:
//!
//!     crate::zhal::security::isolation
//!
//! Hardware access:
//!
//!     crate::zhal::devices::access
//!
//! Provider admission:
//!
//!     crate::zhal::providers::validation
//!
//! # Security architecture
//!
//! Security controls are independent.
//!
//! Provider trust does not imply authorization.
//!
//! Authorization does not imply provider trust.
//!
//! Neither trust nor authorization establishes isolation.
//!
//! Isolation does not imply authentication or authorization.
//!
//! Hardware capability does not imply permission.
//!
//! Hardware availability does not imply permission.
//!
//! A successfully registered provider is not automatically
//! trusted, authenticated, authorized or isolated.
//!
//! Every protected operation must satisfy the applicable
//! security requirements established by the canonical
//! semantic and runtime authorities.
//!
//! # Security invariants
//!
//! 1. Security is deny-by-default.
//! 2. Missing security evidence never establishes success.
//! 3. Unknown security status never establishes success.
//! 4. Provider registration never establishes trust.
//! 5. Provider identity never establishes authentication.
//! 6. Trust never implies authorization.
//! 7. Authorization never implies trust.
//! 8. Authorization never implies isolation.
//! 9. Isolation never implies authorization.
//! 10. Capability evidence never grants permission.
//! 11. Resource availability never grants permission.
//! 12. Policy denial never becomes permission through fallback.
//! 13. Indeterminate policy decisions never authorize access.
//! 14. Outstanding mandatory policy obligations prevent access.
//! 15. Required audit failures prevent authorization.
//! 16. Invalid authorization receipts prevent access.
//! 17. Mandatory isolation requirements require affirmative evidence.
//! 18. Failed isolation establishment never becomes success.
//! 19. Security-provider failures remain observable.
//! 20. Security evidence must remain associated with its subject.
//! 21. Provider-side security enforcement remains mandatory.
//! 22. Security services must not silently weaken mandatory controls.
//! 23. No security result is treated as a permanent unrestricted grant.
//! 24. No security subsystem may bypass its canonical authority.
//!
//! # Security evaluation boundaries
//!
//! The intended protected hardware-access flow is:
//!
//! 1. Resolve and validate source-level security intent.
//! 2. Resolve the authenticated principal through canonical identity
//!    and authentication authorities.
//! 3. Verify applicable provider trust requirements.
//! 4. Verify provider admission and readiness.
//! 5. Verify device availability and identity.
//! 6. Construct a resource-scoped authorization request.
//! 7. Obtain an affirmative authorization receipt.
//! 8. Establish mandatory execution isolation.
//! 9. Preserve the resulting isolation lease for the protected
//!    execution lifetime.
//! 10. Obtain provider-managed hardware access.
//! 11. Require the hardware provider to enforce its own security
//!     boundary.
//! 12. Preserve audit, provenance and operation results.
//! 13. Explicitly release security resources and isolation sessions.
//!
//! Not every operation necessarily requires every control.
//! Applicable requirements are determined by the canonical
//! frontend semantic model and runtime security policy.
//!
//! A caller must never interpret an omitted check as permission
//! to bypass a mandatory security requirement.
//!
//! # Grammar integration
//!
//! The following grammar files describe security intent:
//!
//!     grammar/security/security.g4
//!     grammar/security/identity.g4
//!     grammar/security/authorization.g4
//!     grammar/security/permissions.g4
//!     grammar/security/policies.g4
//!     grammar/security/trust.g4
//!     grammar/security/sandbox.g4
//!     grammar/security/security-constraints.g4
//!     grammar/security/cryptography.g4
//!     grammar/security/signatures.g4
//!     grammar/security/audit.g4
//!     grammar/security/provenance.g4
//!     grammar/execution/execution-context.g4
//!
//! The grammar describes source-level declarations and intent.
//!
//! It does not authenticate principals, establish trust,
//! authorize physical access, verify cryptographic signatures,
//! establish isolation or execute hardware operations.
//!
//! The canonical frontend semantic authority must validate
//! and normalize these declarations before runtime enforcement.
//!
//! ZHAL must consume validated runtime contracts.
//!
//! It must not parse grammar files or implement a second
//! semantic security model.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Providers.
//! - Trust authorities.
//! - Trust evidence.
//! - Principals.
//! - Policies.
//! - Permissions.
//! - Authorization requests.
//! - Security obligations.
//! - Isolation providers.
//! - Isolation sessions.
//! - Hardware devices.
//! - Hardware operations.
//! - Computational domains.
//! - Distributed participants.
//! - Machines.
//! - Hardware technologies.
//!
//! Collections and operations scale according to available
//! resources and explicitly configured operational budgets.
//!
//! No fixed hardware counts, memory assumptions, vendor
//! inventories or closed computational-domain lists are
//! introduced here.
//!
//! Literal infinite storage, hardware capacity or computation
//! cannot be guaranteed by a finite implementation.
//!
//! # Concurrency
//!
//! The underlying security services retain their existing
//! concurrency and synchronization contracts.
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No global security singleton.
//! - No competing provider registry.
//! - No local authorization cache.
//! - No local trust cache.
//! - No local isolation-session registry.
//! - No implicit background threads.
//! - No implicit security-provider initialization.
//!
//! Services must be explicitly constructed and injected by
//! their owning runtime or provider composition authority.
//!
//! # Failure handling
//!
//! Security failures must remain explicit.
//!
//! This module does not convert errors into permissive defaults.
//!
//! The underlying modules retain their structured error types,
//! causal information and failure classifications.
//!
//! # Integration
//!
//! The public composition root is consumed by:
//!
//!     crate::zhal::devices::access
//!     crate::zhal::devices::sessions
//!     crate::zhal::providers::validation
//!     crate::zhal::observability
//!     crate::zur
//!
//! The runtime remains responsible for coordinating the
//! complete protected execution lifecycle.
//!
//! Domain-specific adapters must preserve the same security
//! requirements for classical, quantum, HDL, accelerator,
//! embedded, hybrid and distributed hardware.

// ============================================================================
// SECURITY COMPONENTS
// ============================================================================

/// Provider trust evidence, verification and assessment.
///
/// This module does not authenticate providers or authorize
/// hardware operations.
///
/// Trust verification is separate from provider admission,
/// authorization and isolation.
pub mod trust;

/// Resource-scoped hardware authorization integration.
///
/// Delegates authorization evaluation and mandatory audit
/// handling to the canonical standard-library security policy
/// authority.
///
/// This module does not implement authentication, policy
/// evaluation, grant persistence or audit storage.
pub mod authorization;

/// Hardware execution-isolation integration.
///
/// Delegates actual isolation establishment, verification,
/// session management and cleanup to the canonical ZUR
/// isolation authority.
///
/// This module does not implement a competing sandbox,
/// isolation provider registry or isolation manager.
pub mod isolation;

// ============================================================================
// PUBLIC PROVIDER TRUST API
// ============================================================================

/// Provider trust evidence identifier.
pub use trust::TrustEvidenceId;

/// Trust issuer identity.
pub use trust::TrustIssuerId;

/// Opaque external verification-key reference.
pub use trust::TrustKeyReference;

/// Open-world cryptographic signature scheme identifier.
pub use trust::SignatureScheme;

/// Provider trust policy identity.
pub use trust::TrustPolicyId;

/// Explicit provider trust policy.
pub use trust::TrustPolicy;

/// Provider trust evidence.
pub use trust::TrustEvidence;

/// Explicit trust evaluation timestamp.
pub use trust::TrustTimestamp;

/// External cryptographic signature-verification contract.
pub use trust::SignatureVerifier;

/// Signature verification result.
pub use trust::SignatureValidity;

/// External trust revocation-checking contract.
pub use trust::RevocationChecker;

/// Trust evidence revocation status.
pub use trust::RevocationStatus;

/// Provider trust decision.
pub use trust::TrustDecision;

/// Structured trust assessment finding.
pub use trust::TrustFinding;

/// Provider trust assessment.
pub use trust::TrustAssessment;

/// Evaluates provider trust against an explicit trust policy.
///
/// This operation does not authenticate a principal or
/// authorize a hardware operation.
pub use trust::evaluate_trust;

// ============================================================================
// PUBLIC HARDWARE AUTHORIZATION API
// ============================================================================

/// ZHAL adapter for the canonical authorization authority.
///
/// All authorization decisions remain owned by
/// `crate::stdlib::security::policy`.
pub use authorization::HardwareAuthorizationService;

// ============================================================================
// PUBLIC HARDWARE ISOLATION API
// ============================================================================

/// Hardware-facing isolation identifier.
pub use isolation::HardwareIsolationIdentifier;

/// Hardware isolation disposition.
pub use isolation::HardwareIsolationDisposition;

/// Hardware isolation directive.
pub use isolation::HardwareIsolationDirective;

/// Validated hardware isolation request.
pub use isolation::HardwareIsolationRequest;

/// Provider-reported hardware isolation state.
pub use isolation::HardwareIsolationControlState;

/// Provider-reported hardware isolation evidence.
pub use isolation::HardwareIsolationEvidence;

/// Provider-managed hardware isolation session.
pub use isolation::HardwareIsolationSession;

/// Hardware isolation provider contract.
pub use isolation::HardwareIsolationProvider;

/// Verified hardware isolation report.
pub use isolation::HardwareIsolationReport;

/// ZHAL adapter for the canonical ZUR isolation manager.
pub use isolation::HardwareIsolationService;

/// Owned hardware isolation lease.
///
/// The lease must remain alive while protected execution
/// depends on the established isolation boundary.
pub use isolation::HardwareIsolationLease;

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_security_authorities_are_composed() {
        assert_eq!(
            trust::COMPONENT,
            "zhal.security.trust"
        );

        assert_eq!(
            authorization::COMPONENT,
            "zhal.security.authorization"
        );

        assert_eq!(
            isolation::COMPONENT,
            "zhal.security.isolation"
        );
    }

    #[test]
    fn trust_schema_is_exposed() {
        assert!(!trust::TRUST_SCHEMA_VERSION.is_empty());
    }

    #[test]
    fn isolation_schema_is_exposed() {
        assert!(!isolation::ISOLATION_SCHEMA_VERSION.is_empty());
    }

    #[test]
    fn public_trust_contracts_are_available() {
        fn accepts_policy(_: &TrustPolicy) {}
        fn accepts_evidence(_: &TrustEvidence) {}
        fn accepts_assessment(_: &TrustAssessment) {}
        fn accepts_decision(_: TrustDecision) {}
        fn accepts_verifier(_: &dyn SignatureVerifier) {}
        fn accepts_revocation_checker(_: &dyn RevocationChecker) {}

        let _ = (
            accepts_policy,
            accepts_evidence,
            accepts_assessment,
            accepts_decision,
            accepts_verifier,
            accepts_revocation_checker,
        );
    }

    #[test]
    fn public_authorization_service_is_available() {
        fn accepts_service(_: &HardwareAuthorizationService) {}

        let _ = accepts_service;
    }

    #[test]
    fn public_isolation_services_are_available() {
        fn accepts_service(_: &HardwareIsolationService) {}
        fn accepts_lease(_: &HardwareIsolationLease) {}
        fn accepts_request(_: &HardwareIsolationRequest) {}
        fn accepts_report(_: &HardwareIsolationReport) {}

        let _ = (
            accepts_service,
            accepts_lease,
            accepts_request,
            accepts_report,
        );
    }

    #[test]
    fn security_authorities_remain_independent() {
        // Trust, authorization and isolation are separate
        // public modules with separate component identities.
        assert_ne!(trust::COMPONENT, authorization::COMPONENT);
        assert_ne!(trust::COMPONENT, isolation::COMPONENT);
        assert_ne!(authorization::COMPONENT, isolation::COMPONENT);
    }
}
