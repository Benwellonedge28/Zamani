
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Security
//!
//! Path: `src/distributed/security/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the public composition boundary for distributed security
//! in the Zamani programming language.
//!
//! The security subsystem establishes the interfaces and composition
//! boundaries required to authenticate distributed participants,
//! authorize operations, evaluate security policies, and protect
//! distributed execution.
//!
//! It supports the POCO-REAF architecture:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Security requirements must remain independent of the physical
//! machines, processors, accelerators, networks, operating systems,
//! execution providers, and deployment environments used to realize
//! a Zamani computation.
//!
//! ## Architectural ownership
//!
//! This module composes four existing security authorities.
//!
//! - `identity`: cryptographic identity and identity proofs.
//! - `authentication`: peer authentication, trust verification,
//!   authentication challenges, sessions, and session lifecycle.
//! - `authorization`: authorization orchestration at protected
//!   distributed operation boundaries.
//! - `policy`: policy requests, policy evaluation, policy decisions,
//!   obligations, and enforcement permits.
//!
//! This module owns:
//!
//! - Security module composition.
//! - Public security module exports.
//! - The stable security subsystem import boundary.
//! - Security ownership documentation.
//! - Cross-module security integration contracts.
//!
//! This module does NOT own:
//!
//! - Source-language security syntax.
//! - AST construction or semantic analysis.
//! - Semantic identity resolution.
//! - Security requirement interpretation.
//! - Capability discovery.
//! - Physical resource discovery or allocation.
//! - Network transport implementation.
//! - TLS implementation.
//! - Network endpoint discovery.
//! - Placement, partitioning, routing, or scheduling.
//! - Distributed task execution.
//! - Process migration.
//! - Checkpoint persistence.
//! - Recovery execution.
//! - Distributed idempotency.
//! - Backend selection.
//! - ZUIR, ZCIR, quantum::ir, or HDLIR.
//! - Hardware security implementations.
//! - Hardware attestation implementations.
//!
//! Those responsibilities remain with their existing repository
//! authorities and dedicated integration interfaces.
//!
//! ## Security architecture
//!
//! ```text
//!                 ZAMANI SOURCE
//!                       |
//!                       v
//!              FRONTEND / SEMANTICS
//!                       |
//!                       v
//!              SECURITY REQUIREMENTS
//!                       |
//!                       v
//!             DISTRIBUTED EXECUTION PLAN
//!                       |
//!                       v
//!              DISTRIBUTED SECURITY
//!                       |
//!          +------------+------------+
//!          |            |            |
//!          v            v            v
//!       IDENTITY   AUTHENTICATION  POLICY
//!          |            |            |
//!          |            v            |
//!          |       TRUST VERIFICATION|
//!          |            |            |
//!          +------------+------------+
//!                       |
//!                       v
//!                 AUTHORIZATION
//!                       |
//!                       v
//!                PROTECTED OPERATION
//!                       |
//!                       v
//!             DISTRIBUTED TRANSPORT
//!                       |
//!                       v
//!                REMOTE RUNTIME
//! ```
//!
//! Authentication establishes an identity.
//!
//! Trust verification determines whether the identity's credentials
//! are accepted by the configured trust authority.
//!
//! Authorization determines whether the authenticated identity may
//! perform a particular operation on a particular resource.
//!
//! Policy evaluation determines whether the configured policies
//! permit that operation and whether their obligations are fulfilled.
//!
//! These are separate decisions and must remain separate.
//!
//! ## Trust invariants
//!
//! The following invariants apply throughout the distributed subsystem.
//!
//! 1. A discovered endpoint is not a trusted participant.
//! 2. A public identity document is not authentication evidence.
//! 3. A valid cryptographic signature alone is not authorization.
//! 4. A valid authentication session does not imply permission.
//! 5. An authenticated participant must be authorized separately.
//! 6. Authorization must occur at the protected-operation boundary.
//! 7. Policy evaluation must fail closed.
//! 8. An indeterminate policy result must never grant permission.
//! 9. Policy obligations must be fulfilled before protected execution.
//! 10. Revoked or expired sessions must not authorize new operations.
//! 11. Security credentials must never be included in ordinary
//!     distributed message payloads.
//! 12. Authentication and authorization failures must remain
//!     distinguishable from transport and execution failures.
//! 13. A previously issued authorization permit must not be treated
//!     as a permanent or transferable authorization grant.
//! 14. Remote callers must not be allowed to manufacture trusted
//!     authorization context.
//! 15. Security decisions must be made using authoritative runtime
//!     information and configured trust and policy providers.
//!
//! ## Cryptographic identity
//!
//! `identity` owns:
//!
//! - PrincipalId.
//! - PublicKeyFingerprint.
//! - PeerIdentity.
//! - PeerIdentityDocument.
//! - IdentityKeyPair.
//! - IdentityProof.
//! - Identity proof verification.
//!
//! Cryptographic identity does not independently establish trust,
//! credential freshness, authorization, or physical device identity.
//!
//! Those properties require their respective authorities.
//!
//! ## Authentication
//!
//! `authentication` owns:
//!
//! - AuthenticationConfig.
//! - AuthenticationChallenge.
//! - AuthenticationResponse.
//! - PeerTrustStore.
//! - ExternalAuthenticationProvider.
//! - AuthenticationSessionStore.
//! - AuthenticationService.
//! - AuthenticatedSession.
//! - SessionId.
//!
//! Authentication providers must verify their protocol-specific
//! evidence before returning an authenticated identity.
//!
//! Session storage must be selected according to deployment needs.
//!
//! InMemorySessionStore is process-local. A multi-process or
//! multi-machine deployment must inject an appropriately protected
//! shared session store where shared session visibility is required.
//!
//! Challenge replay protection must cover the deployment's actual
//! authentication boundary. Independent process-local challenge
//! stores must not be mistaken for cluster-wide replay protection.
//!
//! ## Authorization
//!
//! `authorization` owns:
//!
//! - AuthorizationContextProvider.
//! - EmptyAuthorizationContext.
//! - AuthorizationService.
//! - Authorization-before-execution orchestration.
//!
//! Authorization context must originate from trusted runtime
//! authorities.
//!
//! Caller-controlled attributes must not become trusted merely
//! because they were supplied with an authenticated request.
//!
//! Authorization must be checked for the actual requested operation,
//! resource, and execution context.
//!
//! ## Policy
//!
//! `policy` owns:
//!
//! - PolicyIdentifier.
//! - AuthenticatedSubject.
//! - PolicyAttributes.
//! - PolicyRequest.
//! - PolicyEngine.
//! - PolicyEnforcer.
//! - PolicyPermit.
//! - Policy conditions and combining behavior.
//! - Policy obligations and evaluation outcomes.
//!
//! This subsystem does not introduce a second policy engine.
//!
//! Source-language policy syntax and semantic validation remain
//! owned by the grammar, frontend, and semantic authorities.
//!
//! ## Distributed transport integration
//!
//! The distributed transport subsystem remains responsible for
//! physical communication and authenticated connection lifecycle.
//!
//! Relevant existing interfaces include:
//!
//! - `distributed/transport/traits.rs`:
//!   AuthenticationContext and AuthenticatedPeer.
//! - `distributed/transport/messages.rs`:
//!   Protocol message envelopes.
//! - `distributed/transport/errors.rs`:
//!   Transport error classification.
//! - `distributed/transport/codec.rs`:
//!   Message encoding and decoding.
//!
//! Transport providers must establish the actual peer identity
//! before constructing authenticated-peer evidence.
//!
//! Constructing an AuthenticatedPeer value does not itself perform
//! authentication.
//!
//! Transport authentication must not automatically grant workload
//! authorization.
//!
//! The transport adapter must associate the verified identity with
//! the connection on which it was established. It must not accept
//! an unrelated identity reference supplied by an untrusted caller.
//!
//! ## Membership integration
//!
//! Membership and discovery remain separate from trust.
//!
//! The following existing modules remain authoritative:
//!
//! - `distributed/membership/registry.rs`:
//!   participant membership.
//! - `distributed/membership/discovery.rs`:
//!   participant discovery.
//! - `distributed/membership/health.rs`:
//!   participant health observations.
//!
//! Security must be applied when a discovered participant attempts
//! to establish a trusted communication or execution relationship.
//!
//! Membership registration alone must never grant execution rights.
//!
//! ## Resilience integration
//!
//! The resilience subsystem remains responsible for retry,
//! checkpointing, idempotency, and recovery coordination.
//!
//! Security integration must ensure that:
//!
//! - retries do not bypass authorization;
//! - revoked credentials cannot be reused for new authentication;
//! - expired sessions cannot authorize new operations;
//! - recovery does not bypass workload authorization;
//! - restored execution state is checked against current security policy;
//! - duplicate requests do not bypass authorization;
//! - identity and operation provenance remain available for audit.
//!
//! An operation whose outcome is unknown must not be repeated merely
//! because authentication or authorization succeeded.
//!
//! Idempotency and execution-outcome reconciliation remain owned
//! by the resilience and runtime authorities.
//!
//! ## Runtime integration
//!
//! `runtime/distributed.rs` must consume the distributed security
//! services rather than independently implementing authentication,
//! trust, or authorization.
//!
//! Protected operations include, as applicable:
//!
//! - remote workload submission;
//! - service invocation;
//! - deployment;
//! - participant management;
//! - state access;
//! - state transfer;
//! - checkpoint operations;
//! - administrative operations;
//! - resource access.
//!
//! `runtime/omniversal_kernel.rs` must construct or receive its
//! security dependencies through explicit, fallible initialization.
//!
//! Neither runtime module may introduce an independent global
//! security registry or implicit allow-all authorization.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no application-defined universal maximum on:
//!
//! - principals;
//! - identities;
//! - authentication providers;
//! - trust records;
//! - authenticated sessions;
//! - authorization providers;
//! - policies;
//! - policy rules;
//! - protected resources;
//! - operations;
//! - security attributes;
//! - participants;
//! - machines;
//! - accelerators;
//! - distributed execution contexts.
//!
//! Actual capacity depends on:
//!
//! - available memory;
//! - available storage;
//! - address-space limits;
//! - cryptographic provider capabilities;
//! - configured operational budgets;
//! - external identity and policy services;
//! - the underlying execution environment.
//!
//! Operational capacity limits must be configurable by the relevant
//! provider and must not become arbitrary language-level limits.
//!
//! Cryptographic algorithm properties and their actual representation
//! limits must be documented honestly.
//!
//! Literal infinite physical capacity is not promised.
//!
//! The architecture must permit additional authentication providers,
//! trust stores, policy engines, and deployment environments without
//! redesigning the distributed security ownership model.
//!
//! ## Concurrency and lifecycle
//!
//! This module creates no global mutable state.
//!
//! It does not create internal background threads or task queues.
//!
//! Services and providers are owned and shared explicitly by callers.
//!
//! Collaborators must implement their documented concurrency,
//! synchronization, persistence, and shutdown contracts.
//!
//! ## Failure behavior
//!
//! Security failures must use the canonical distributed error
//! vocabulary from `distributed/error.rs`.
//!
//! In particular:
//!
//! - invalid identities must not be authenticated;
//! - unknown or untrusted peers must be rejected;
//! - revoked credentials must be rejected;
//! - invalid proofs must be rejected;
//! - expired challenges must be rejected;
//! - replayed challenges must be rejected;
//! - expired sessions must be rejected;
//! - revoked sessions must be rejected;
//! - denied authorization requests must not execute;
//! - indeterminate policy results must not execute;
//! - failed policy obligations must not execute;
//! - unavailable security providers must not silently permit work.
//!
//! Errors must preserve their original causes where applicable.
//!
//! Human-readable error messages must not expose credentials,
//! private keys, authentication tokens, or secret policy attributes.
//!
//! ## Public API compatibility
//!
//! Existing concrete security types retain their ownership in their
//! current implementation files.
//!
//! This module exposes those types through their authoritative
//! module paths and selected public re-exports.
//!
//! Moving an existing type into this module or defining a second
//! implementation is prohibited.
//!
//! Changes to existing public security APIs require an explicit
//! compatibility review.
//!
//! ## Safety
//!
//! This module uses safe Rust only.
//!
//! It must not introduce:
//!
//! - unsafe blocks;
//! - unsafe functions;
//! - unsafe trait implementations;
//! - mutable global state;
//! - hidden credential storage;
//! - implicit trust decisions;
//! - implicit authorization grants.

// =============================================================================
// SECURITY MODULES
// =============================================================================

/// Cryptographic principal identities and identity proofs.
pub mod identity;

/// Peer authentication, trust verification, and session management.
pub mod authentication;

/// Authorization orchestration and protected-operation enforcement.
pub mod authorization;

/// Runtime security policies, decisions, and obligations.
pub mod policy;

// =============================================================================
// IDENTITY EXPORTS
// =============================================================================

pub use identity::{
    IdentityKeyPair,
    IdentityProof,
    PeerIdentity,
    PeerIdentityDocument,
    PrincipalId,
    PublicKeyFingerprint,
};

// =============================================================================
// AUTHENTICATION EXPORTS
// =============================================================================

pub use authentication::{
    AuthenticatedSession,
    AuthenticationChallenge,
    AuthenticationConfig,
    AuthenticationResponse,
    AuthenticationService,
    AuthenticationSessionStore,
    ExternalAuthenticationProvider,
    InMemoryPeerTrustStore,
    InMemorySessionStore,
    PeerTrustStore,
    SessionId,
};

// =============================================================================
// AUTHORIZATION EXPORTS
// =============================================================================

pub use authorization::{
    AuthorizationContextProvider,
    AuthorizationService,
    EmptyAuthorizationContext,
};

// =============================================================================
// POLICY EXPORTS
// =============================================================================

pub use policy::{
    AuthenticatedSubject,
    PolicyAttributes,
    PolicyDecision,
    PolicyEffect,
    PolicyEngine,
    PolicyEnforcer,
    PolicyIdentifier,
    PolicyPermit,
    PolicyRequest,
    SubjectSelector,
};

// =============================================================================
// COMPOSITION TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the identity authority is publicly accessible.
    #[test]
    fn identity_authority_is_exported() {
        let principal = PrincipalId::new("zamani.test.participant")
            .expect("valid principal");

        assert_eq!(
            principal.as_str(),
            "zamani.test.participant"
        );
    }

    /// Verifies that authentication configuration is publicly accessible.
    #[test]
    fn authentication_configuration_is_exported() {
        let configuration = AuthenticationConfig::default();

        assert!(configuration.validate().is_ok());
    }

    /// Verifies that explicit empty authorization context is accessible.
    ///
    /// Empty context does not grant permission. The policy enforcer
    /// remains responsible for making the actual authorization decision.
    #[test]
    fn authorization_context_is_exported() {
        let context = EmptyAuthorizationContext;

        let _ = context;
    }

    /// Verifies that policy identifiers are publicly accessible.
    #[test]
    fn policy_identifiers_are_exported() {
        let identifier = PolicyIdentifier::new(
            "distributed.workload.execute"
        )
        .expect("valid policy identifier");

        assert_eq!(
            identifier.as_str(),
            "distributed.workload.execute"
        );
    }

    /// Verifies that authentication and policy identifiers remain
    /// independently accessible through the composition boundary.
    #[test]
    fn security_authorities_remain_separate() {
        let principal = PrincipalId::new("participant.alpha")
            .expect("valid principal");

        let action = PolicyIdentifier::new(
            "workload.execute"
        )
        .expect("valid action");

        assert_eq!(principal.as_str(), "participant.alpha");
        assert_eq!(action.as_str(), "workload.execute");
    }
}
