
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani distributed authentication.
//!
//! Path: src/distributed/security/authentication.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibilities
//!
//! This module owns the runtime authentication boundary for distributed
//! Zamani participants and services.
//!
//! It provides:
//!
//! - Ed25519 challenge-response authentication;
//! - challenge expiry and single-use enforcement;
//! - replay prevention within the lifetime of the challenge store;
//! - explicit principal-to-public-key trust binding;
//! - authenticated subject construction;
//! - authenticated session creation, validation and revocation;
//! - pluggable external authentication providers;
//! - pluggable session storage.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - source-language identity syntax;
//! - frontend identity resolution;
//! - semantic security analysis;
//! - cryptographic identity representation;
//! - private-key persistence or key generation policy;
//! - certificate-authority infrastructure;
//! - TLS or network transport;
//! - authorization policy evaluation;
//! - policy obligations;
//! - workload execution;
//! - resource placement or scheduling;
//! - physical machine or device identity.
//!
//! # Existing authorities
//!
//! - security/identity.rs owns PrincipalId, PeerIdentity, IdentityProof,
//!   PeerIdentityDocument and IdentityKeyPair.
//! - security/policy.rs owns AuthenticatedSubject and policy evaluation.
//! - distributed/error.rs owns DistributedError and DistributedResult.
//! - networking and transport adapters own secure channel establishment.
//! - external authentication adapters own provider-specific protocol
//!   verification.
//!
//! # Security invariants
//!
//! 1. A public identity document is not proof of identity.
//! 2. A discovered endpoint is not an authenticated principal.
//! 3. A challenge is random, time-limited and single-use.
//! 4. A proof must match the issued challenge and the supplied identity.
//! 5. The public key must be explicitly trusted for the claimed principal.
//! 6. AuthenticatedSubject is constructed only after successful verification.
//! 7. Session lookup fails closed.
//! 8. Authentication does not grant authorization.
//!
//! # Scalability
//!
//! There are no fixed limits on principals, participants, services, machines,
//! devices, authentication providers or sessions.
//!
//! Optional capacity limits are operational budgets supplied through
//! AuthenticationConfig. They are not language or semantic limits.
//!
//! Literal infinite execution is not possible on finite resources. The
//! implementation imposes no arbitrary universal capacity ceiling.
//!
//! # External authentication
//!
//! OAuth, OIDC, mTLS, SSH-like mechanisms, cloud IAM, API-key systems and
//! other provider protocols must be implemented by provider adapters.
//!
//! The adapter must validate its protocol-specific evidence before returning
//! a PeerIdentity. This module then validates the identity and checks its
//! explicit trust binding.
//!
//! Provider adapters must not log or expose credentials, bearer tokens,
//! private keys, passwords or authentication headers.
//!
//! # Session deployment
//!
//! InMemorySessionStore is suitable for local operation and tests. It is
//! process-local and does not provide cross-process persistence.
//!
//! Multi-process and multi-node deployments must inject a shared,
//! appropriately protected AuthenticationSessionStore implementation.
//!
//! # Replay protection
//!
//! Challenge replay prevention is local to the AuthenticationService
//! instance issuing the challenge. A distributed deployment must ensure
//! that a challenge is completed by the same authoritative service, or
//! provide a shared challenge-store implementation in a future extension.
//!
//! Session persistence and challenge persistence are separate concerns.

use core::fmt;
use core::num::NonZeroUsize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::distributed::error::{
    DistributedError,
    DistributedResult,
};

use crate::distributed::security::identity::{
    IdentityKeyPair,
    IdentityProof,
    PeerIdentity,
    PeerIdentityDocument,
    PrincipalId,
};

use crate::distributed::security::policy::{
    AuthenticatedSubject,
    PolicyIdentifier,
};

// ============================================================================
// Configuration
// ============================================================================

/// Runtime authentication configuration.
///
/// Capacity settings are optional operational budgets. `None` means that
/// this layer does not impose an application-level entry limit.
///
/// The operating system, allocator, and available memory remain relevant.
#[derive(Debug, Clone)]
pub struct AuthenticationConfig {
    /// Maximum time allowed to complete a challenge.
    pub challenge_lifetime: Duration,

    /// Lifetime of a successfully authenticated session.
    pub session_lifetime: Duration,

    /// Optional maximum number of simultaneously pending challenges.
    pub max_pending_challenges: Option<NonZeroUsize>,

    /// Optional maximum number of simultaneously active sessions in the
    /// default in-memory session store.
    ///
    /// Custom session stores may enforce their own configured budgets.
    pub max_sessions: Option<NonZeroUsize>,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            challenge_lifetime: Duration::from_secs(30),
            session_lifetime: Duration::from_secs(3600),
            max_pending_challenges: None,
            max_sessions: None,
        }
    }
}

impl AuthenticationConfig {
    /// Validate the configuration.
    ///
    /// A zero duration is rejected because it would make the corresponding
    /// operation expire immediately.
    pub fn validate(&self) -> DistributedResult<()> {
        if self.challenge_lifetime.is_zero() {
            return Err(DistributedError::invalid_configuration(
                "authentication.challenge_lifetime",
                "challenge lifetime must be greater than zero",
            ));
        }

        if self.session_lifetime.is_zero() {
            return Err(DistributedError::invalid_configuration(
                "authentication.session_lifetime",
                "session lifetime must be greater than zero",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Authentication challenge
// ============================================================================

/// A server-issued, time-limited authentication challenge.
///
/// The nonce is public protocol data, not a credential. Its unpredictability
/// and single-use enforcement prevent an old proof from being replayed as a
/// response to a newly issued challenge.
///
/// The challenge is serializable for transport adapters. Deserialized
/// challenges are untrusted until checked against the server-side challenge
/// store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationChallenge {
    nonce: Vec<u8>,
    method: String,
}

impl AuthenticationChallenge {
    /// Return the challenge nonce.
    #[must_use]
    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    /// Return the authentication method identifier.
    #[must_use]
    pub fn method(&self) -> &str {
        &self.method
    }
}

// ============================================================================
// Authentication response
// ============================================================================

/// Response to an issued challenge.
///
/// Contains only public identity information and a signature proof.
/// It must never contain private-key material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    nonce: Vec<u8>,
    identity: PeerIdentityDocument,
    proof: IdentityProof,
}

impl AuthenticationResponse {
    /// Construct a response using an existing identity key pair.
    ///
    /// The private key remains inside IdentityKeyPair. It is not serialized
    /// or returned by this method.
    pub fn create(
        key_pair: &IdentityKeyPair,
        challenge: &AuthenticationChallenge,
    ) -> DistributedResult<Self> {
        if challenge.nonce.is_empty() {
            return Err(DistributedError::invalid_input(
                "authentication challenge nonce cannot be empty",
            ));
        }

        let proof = key_pair.prove_challenge(&challenge.nonce)?;

        Ok(Self {
            nonce: challenge.nonce.clone(),
            identity: key_pair.peer_identity().to_document(),
            proof,
        })
    }

    /// Return the public identity document.
    #[must_use]
    pub fn identity(&self) -> &PeerIdentityDocument {
        &self.identity
    }
}

// ============================================================================
// Peer trust
// ============================================================================

/// Runtime trust-verification interface.
///
/// A trust provider decides whether a public identity and its key binding
/// are currently accepted by the deployment.
///
/// Implementations may use:
///
/// - a locally managed trust registry;
/// - a certificate or trust-anchor system;
/// - a secure identity directory;
/// - a hardware-backed attestation service;
/// - a managed enterprise identity system;
/// - a distributed trust database.
///
/// Returning success means that the exact principal/public-key binding is
/// trusted. It does not grant authorization for any particular operation.
pub trait PeerTrustStore: Send + Sync {
    /// Verify that this principal and public key are trusted.
    fn verify_peer(
        &self,
        identity: &PeerIdentity,
    ) -> DistributedResult<()>;
}

// ============================================================================
// In-memory trust registry
// ============================================================================

/// Explicit in-memory trust registry.
///
/// This implementation is useful for local deployments, tests and
/// controlled environments. Production deployments may inject a persistent
/// or externally managed PeerTrustStore.
///
/// An empty registry trusts nobody.
///
/// Multiple fingerprints may be registered for one principal to support
/// controlled key rotation.
#[derive(Debug, Default)]
pub struct InMemoryPeerTrustStore {
    trusted: RwLock<BTreeMap<PrincipalId, BTreeSet<String>>>,
}

impl InMemoryPeerTrustStore {
    /// Create an empty trust registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Explicitly trust the principal/public-key binding.
    ///
    /// Registering a new key does not remove older keys. Use revoke_key or
    /// revoke_principal to remove trust.
    pub fn trust_peer(
        &self,
        identity: &PeerIdentity,
    ) -> DistributedResult<()> {
        identity.validate()?;

        let mut trusted = self.trusted.write().map_err(|_| {
            authentication_error(
                Some(identity.principal().as_str()),
                "trust registry synchronization failure",
            )
        })?;

        trusted
            .entry(identity.principal().clone())
            .or_default()
            .insert(identity.fingerprint().to_hex());

        Ok(())
    }

    /// Revoke one principal/public-key binding.
    pub fn revoke_key(
        &self,
        principal: &PrincipalId,
        fingerprint: &str,
    ) -> DistributedResult<bool> {
        let mut trusted = self.trusted.write().map_err(|_| {
            authentication_error(
                Some(principal.as_str()),
                "trust registry synchronization failure",
            )
        })?;

        let Some(fingerprints) = trusted.get_mut(principal) else {
            return Ok(false);
        };

        let removed = fingerprints.remove(fingerprint);

        if fingerprints.is_empty() {
            trusted.remove(principal);
        }

        Ok(removed)
    }

    /// Revoke all registered keys for one principal.
    pub fn revoke_principal(
        &self,
        principal: &PrincipalId,
    ) -> DistributedResult<bool> {
        let mut trusted = self.trusted.write().map_err(|_| {
            authentication_error(
                Some(principal.as_str()),
                "trust registry synchronization failure",
            )
        })?;

        Ok(trusted.remove(principal).is_some())
    }
}

impl PeerTrustStore for InMemoryPeerTrustStore {
    fn verify_peer(
        &self,
        identity: &PeerIdentity,
    ) -> DistributedResult<()> {
        identity.validate()?;

        let trusted = self.trusted.read().map_err(|_| {
            authentication_error(
                Some(identity.principal().as_str()),
                "trust registry synchronization failure",
            )
        })?;

        let fingerprint = identity.fingerprint().to_hex();

        let accepted = trusted
            .get(identity.principal())
            .is_some_and(|fingerprints| fingerprints.contains(&fingerprint));

        if !accepted {
            return Err(authentication_error(
                Some(identity.principal().as_str()),
                "principal/public-key binding is not trusted",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// External authentication providers
// ============================================================================

/// Provider-specific authentication adapter.
///
/// Implementations may integrate OAuth, OIDC, mTLS, SSH-like authentication,
/// cloud IAM, API-key systems or other external mechanisms.
///
/// The provider must validate all protocol-specific properties before
/// returning a PeerIdentity. Examples include:
///
/// - issuer and audience validation;
/// - token signature and expiry validation;
/// - nonce and replay validation;
/// - certificate chain and validity validation;
/// - client identity validation;
/// - proof-of-possession requirements;
/// - provider-specific revocation checks.
///
/// The returned identity is still checked against PeerTrustStore.
///
/// The evidence parameter is borrowed so this module does not retain or
/// clone provider credentials or assertions.
///
/// Implementations must not log evidence or include it in error messages.
pub trait ExternalAuthenticationProvider: Send + Sync {
    /// Return the stable identifier for this authentication mechanism.
    fn method(&self) -> &PolicyIdentifier;

    /// Validate provider-specific evidence and return the verified identity.
    fn authenticate(
        &self,
        evidence: &[u8],
    ) -> DistributedResult<PeerIdentity>;
}

// ============================================================================
// Session identifier
// ============================================================================

/// Opaque random session identifier.
///
/// Treat this value as sensitive bearer material when it is used as a
/// session credential. Do not log it or expose it in diagnostics.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId([u8; 32]);

impl SessionId {
    /// Construct a session identifier from its wire representation.
    ///
    /// This validates its representation only. It does not establish that
    /// the session exists or is active.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Return the wire representation.
    ///
    /// Callers must protect this value as a credential.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for SessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SessionId([REDACTED])")
    }
}

// ============================================================================
// Authenticated session
// ============================================================================

/// An authenticated session returned after successful authentication.
///
/// This is not an authorization grant. Authorization must still be evaluated
/// through the policy subsystem for each protected operation.
#[derive(Clone)]
pub struct AuthenticatedSession {
    id: SessionId,
    subject: AuthenticatedSubject,
    expires_at: Instant,
}

impl AuthenticatedSession {
    /// Return the opaque session identifier.
    ///
    /// Treat it as sensitive bearer material.
    #[must_use]
    pub const fn id(&self) -> &SessionId {
        &self.id
    }

    /// Return the authenticated subject.
    #[must_use]
    pub fn subject(&self) -> &AuthenticatedSubject {
        &self.subject
    }

    /// Return whether this local session object has expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }

    /// Return the remaining lifetime.
    #[must_use]
    pub fn remaining_lifetime(&self) -> Duration {
        self.expires_at.saturating_duration_since(Instant::now())
    }
}

impl fmt::Debug for AuthenticatedSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticatedSession")
            .field("id", &"[REDACTED]")
            .field("subject", &self.subject)
            .field("expired", &self.is_expired())
            .finish()
    }
}

// ============================================================================
// Session storage interface
// ============================================================================

/// Storage contract for authenticated sessions.
///
/// Implementations must:
///
/// - treat session IDs as bearer credentials;
/// - enforce expiry;
/// - fail closed on storage errors;
/// - prevent revoked sessions from being returned;
/// - protect stored authentication state;
/// - avoid logging session IDs or credentials.
///
/// A distributed deployment should inject a shared implementation if
/// sessions must be validated across multiple runtime processes.
pub trait AuthenticationSessionStore: Send + Sync {
    /// Persist or register a new session.
    fn insert(
        &self,
        id: SessionId,
        subject: AuthenticatedSubject,
        lifetime: Duration,
    ) -> DistributedResult<()>;

    /// Return the subject for an active session.
    ///
    /// Expired or revoked sessions must return `Ok(None)`.
    fn get(
        &self,
        id: &SessionId,
    ) -> DistributedResult<Option<AuthenticatedSubject>>;

    /// Revoke a session.
    ///
    /// Returns true when an active session was removed.
    fn revoke(
        &self,
        id: &SessionId,
    ) -> DistributedResult<bool>;
}

// ============================================================================
// In-memory session storage
// ============================================================================

struct InMemorySessionRecord {
    subject: AuthenticatedSubject,
    expires_at: Instant,
}

/// Process-local session store.
///
/// This store is not a distributed session database. It is the default
/// implementation for single-process use and testing.
pub struct InMemorySessionStore {
    records: Mutex<HashMap<SessionId, InMemorySessionRecord>>,
    max_sessions: Option<NonZeroUsize>,
}

impl InMemorySessionStore {
    /// Construct an in-memory store with an optional operational capacity.
    #[must_use]
    pub fn new(max_sessions: Option<NonZeroUsize>) -> Self {
        Self {
            records: Mutex::new(HashMap::new()),
            max_sessions,
        }
    }

    fn prune_expired(
        records: &mut HashMap<SessionId, InMemorySessionRecord>,
    ) {
        let now = Instant::now();
        records.retain(|_, record| record.expires_at > now);
    }
}

impl Default for InMemorySessionStore {
    fn default() -> Self {
        Self::new(None)
    }
}

impl AuthenticationSessionStore for InMemorySessionStore {
    fn insert(
        &self,
        id: SessionId,
        subject: AuthenticatedSubject,
        lifetime: Duration,
    ) -> DistributedResult<()> {
        let expires_at = Instant::now()
            .checked_add(lifetime)
            .ok_or_else(|| {
                DistributedError::invalid_configuration(
                    "authentication.session_lifetime",
                    "session expiry cannot be represented by the runtime clock",
                )
            })?;

        let mut records = self.records.lock().map_err(|_| {
            authentication_error(
                Some(subject.principal().as_str()),
                "session store synchronization failure",
            )
        })?;

        Self::prune_expired(&mut records);

        if let Some(limit) = self.max_sessions {
            if records.len() >= limit.get() {
                return Err(DistributedError::resource_unsatisfied(
                    "authentication.active_sessions",
                    Some(records.len().saturating_add(1) as u128),
                    Some(limit.get() as u128),
                    Some("sessions".to_owned()),
                    true,
                ));
            }
        }

        if records.contains_key(&id) {
            return Err(DistributedError::Conflict {
                subject: "authentication.session".to_owned(),
                message: "generated session identifier already exists".to_owned(),
            });
        }

        records.insert(
            id,
            InMemorySessionRecord {
                subject,
                expires_at,
            },
        );

        Ok(())
    }

    fn get(
        &self,
        id: &SessionId,
    ) -> DistributedResult<Option<AuthenticatedSubject>> {
        let mut records = self.records.lock().map_err(|_| {
            authentication_error(
                None,
                "session store synchronization failure",
            )
        })?;

        let now = Instant::now();

        if records
            .get(id)
            .is_some_and(|record| record.expires_at <= now)
        {
            records.remove(id);
            return Ok(None);
        }

        Ok(records.get(id).map(|record| record.subject.clone()))
    }

    fn revoke(
        &self,
        id: &SessionId,
    ) -> DistributedResult<bool> {
        let mut records = self.records.lock().map_err(|_| {
            authentication_error(
                None,
                "session store synchronization failure",
            )
        })?;

        Ok(records.remove(id).is_some())
    }
}

// ============================================================================
// Pending challenge records
// ============================================================================

struct PendingChallenge {
    method: PolicyIdentifier,
    expires_at: Instant,
}

// ============================================================================
// Authentication service
// ============================================================================

/// Runtime authentication service.
///
/// The service is thread-safe and can be shared through Arc.
///
/// The trust store, challenge store and session store are deliberately
/// separate:
///
/// - trust store: which identity/key bindings are accepted;
/// - challenge store: which proofs are currently expected;
/// - session store: which authenticated sessions remain active.
pub struct AuthenticationService {
    config: AuthenticationConfig,
    trust_store: Arc<dyn PeerTrustStore>,
    session_store: Arc<dyn AuthenticationSessionStore>,
    pending_challenges: Mutex<HashMap<[u8; 32], PendingChallenge>>,
}

impl AuthenticationService {
    /// Construct an authentication service using the default in-memory
    /// session store.
    ///
    /// An explicit trust store is mandatory. An empty trust store is
    /// default-deny.
    pub fn new(
        config: AuthenticationConfig,
        trust_store: Arc<dyn PeerTrustStore>,
    ) -> DistributedResult<Self> {
        config.validate()?;

        let session_store = Arc::new(InMemorySessionStore::new(
            config.max_sessions,
        ));

        Self::with_session_store(
            config,
            trust_store,
            session_store,
        )
    }

    /// Construct an authentication service with an injected session store.
    ///
    /// Use this constructor for shared or durable session storage.
    pub fn with_session_store(
        config: AuthenticationConfig,
        trust_store: Arc<dyn PeerTrustStore>,
        session_store: Arc<dyn AuthenticationSessionStore>,
    ) -> DistributedResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            trust_store,
            session_store,
            pending_challenges: Mutex::new(HashMap::new()),
        })
    }

    /// Issue a fresh challenge for the selected authentication method.
    ///
    /// The challenge is recorded locally and expires after the configured
    /// challenge lifetime.
    pub fn issue_challenge(
        &self,
        method: PolicyIdentifier,
    ) -> DistributedResult<AuthenticationChallenge> {
        let expires_at = Instant::now()
            .checked_add(self.config.challenge_lifetime)
            .ok_or_else(|| {
                DistributedError::invalid_configuration(
                    "authentication.challenge_lifetime",
                    "challenge expiry cannot be represented by the runtime clock",
                )
            })?;

        let mut nonce = [0_u8; 32];

        rand::thread_rng()
            .try_fill_bytes(&mut nonce)
            .map_err(|_| {
                authentication_error(
                    None,
                    "cryptographically secure challenge generation failed",
                )
            })?;

        let mut pending = self.pending_challenges.lock().map_err(|_| {
            authentication_error(
                None,
                "challenge store synchronization failure",
            )
        })?;

        let now = Instant::now();
        pending.retain(|_, record| record.expires_at > now);

        if let Some(limit) = self.config.max_pending_challenges {
            if pending.len() >= limit.get() {
                return Err(DistributedError::resource_unsatisfied(
                    "authentication.pending_challenges",
                    Some(pending.len().saturating_add(1) as u128),
                    Some(limit.get() as u128),
                    Some("challenges".to_owned()),
                    true,
                ));
            }
        }

        if pending.contains_key(&nonce) {
            return Err(authentication_error(
                None,
                "challenge nonce collision; issue a new challenge",
            ));
        }

        pending.insert(
            nonce,
            PendingChallenge {
                method: method.clone(),
                expires_at,
            },
        );

        Ok(AuthenticationChallenge {
            nonce: nonce.to_vec(),
            method: method.as_str().to_owned(),
        })
    }

    /// Complete an Ed25519 challenge-response authentication exchange.
    ///
    /// The challenge is consumed before proof verification. Consequently,
    /// failed attempts cannot reuse the same challenge.
    ///
    /// A successful cryptographic proof is not sufficient by itself:
    /// the principal/public-key binding must also be trusted.
    pub fn complete_authentication(
        &self,
        response: AuthenticationResponse,
    ) -> DistributedResult<AuthenticatedSession> {
        let nonce: [u8; 32] = response
            .nonce
            .as_slice()
            .try_into()
            .map_err(|_| {
                authentication_error(
                    None,
                    "authentication response contains an invalid challenge nonce",
                )
            })?;

        let pending = {
            let mut challenges = self.pending_challenges.lock().map_err(|_| {
                authentication_error(
                    None,
                    "challenge store synchronization failure",
                )
            })?;

            challenges.remove(&nonce)
        };

        let pending = pending.ok_or_else(|| {
            authentication_error(
                None,
                "challenge is unknown, expired, or has already been consumed",
            )
        })?;

        if Instant::now() >= pending.expires_at {
            return Err(authentication_error(
                None,
                "authentication challenge has expired",
            ));
        }

        if response.identity.principal.trim().is_empty() {
            return Err(authentication_error(
                None,
                "authentication response contains an empty principal",
            ));
        }

        let identity =
            PeerIdentity::from_document(response.identity)?;

        if response.proof.principal() != identity.principal() {
            return Err(authentication_error(
                Some(identity.principal().as_str()),
                "identity proof principal does not match the identity document",
            ));
        }

        identity.verify(&nonce, &response.proof)?;

        self.trust_store.verify_peer(&identity)?;

        self.establish_session(
            identity,
            pending.method,
        )
    }

    /// Authenticate using an external provider adapter.
    ///
    /// The adapter is responsible for its own protocol-specific freshness,
    /// replay protection, signature, issuer, audience, certificate and
    /// credential checks.
    ///
    /// This service still validates the returned identity and requires an
    /// explicit trusted principal/public-key binding.
    pub fn authenticate_external(
        &self,
        provider: &dyn ExternalAuthenticationProvider,
        evidence: &[u8],
    ) -> DistributedResult<AuthenticatedSession> {
        let identity = provider.authenticate(evidence)?;

        identity.validate()?;

        self.trust_store.verify_peer(&identity)?;

        self.establish_session(
            identity,
            provider.method().clone(),
        )
    }

    /// Resolve an active session into its authenticated subject.
    ///
    /// A missing, expired or revoked session is rejected.
    pub fn authenticate_session(
        &self,
        id: &SessionId,
    ) -> DistributedResult<AuthenticatedSubject> {
        self.session_store
            .get(id)?
            .ok_or_else(|| {
                authentication_error(
                    None,
                    "session is unknown, expired, or revoked",
                )
            })
    }

    /// Revoke a session.
    pub fn revoke_session(
        &self,
        id: &SessionId,
    ) -> DistributedResult<bool> {
        self.session_store.revoke(id)
    }

    /// Return the configured challenge lifetime.
    #[must_use]
    pub const fn challenge_lifetime(&self) -> Duration {
        self.config.challenge_lifetime
    }

    /// Return the configured session lifetime.
    #[must_use]
    pub const fn session_lifetime(&self) -> Duration {
        self.config.session_lifetime
    }

    fn establish_session(
        &self,
        identity: PeerIdentity,
        method: PolicyIdentifier,
    ) -> DistributedResult<AuthenticatedSession> {
        let subject =
            AuthenticatedSubject::from_verified_identity(
                &identity,
                method,
            )?;

        let expires_at = Instant::now()
            .checked_add(self.config.session_lifetime)
            .ok_or_else(|| {
                DistributedError::invalid_configuration(
                    "authentication.session_lifetime",
                    "session expiry cannot be represented by the runtime clock",
                )
            })?;

        let mut session_bytes = [0_u8; 32];

        rand::thread_rng()
            .try_fill_bytes(&mut session_bytes)
            .map_err(|_| {
                authentication_error(
                    Some(identity.principal().as_str()),
                    "cryptographically secure session identifier generation failed",
                )
            })?;

        let session_id = SessionId::from_bytes(session_bytes);

        self.session_store.insert(
            session_id,
            subject.clone(),
            self.config.session_lifetime,
        )?;

        Ok(AuthenticatedSession {
            id: session_id,
            subject,
            expires_at,
        })
    }
}

// ============================================================================
// Error integration
// ============================================================================

fn authentication_error(
    peer: Option<&str>,
    message: impl Into<String>,
) -> DistributedError {
    DistributedError::AuthenticationFailed {
        peer: peer.map(str::to_owned),
        message: message.into(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn method() -> PolicyIdentifier {
        PolicyIdentifier::new("distributed.ed25519.challenge")
            .expect("valid method")
    }

    fn principal(value: &str) -> PrincipalId {
        PrincipalId::new(value).expect("valid principal")
    }

    fn key_pair(name: &str, seed: u8) -> IdentityKeyPair {
        let mut secret = [0_u8; 32];
        secret.fill(seed);

        IdentityKeyPair::from_secret_bytes(
            principal(name),
            &secret,
        )
        .expect("valid key pair")
    }

    fn service(
        key_pair: &IdentityKeyPair,
    ) -> (
        AuthenticationService,
        Arc<InMemoryPeerTrustStore>,
    ) {
        let trust = Arc::new(InMemoryPeerTrustStore::new());

        trust
            .trust_peer(key_pair.peer_identity())
            .expect("trust peer");

        let service = AuthenticationService::new(
            AuthenticationConfig::default(),
            trust.clone(),
        )
        .expect("authentication service");

        (service, trust)
    }

    #[test]
    fn challenge_response_authenticates_trusted_peer() {
        let key = key_pair("service::worker", 7);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        let session = service
            .complete_authentication(response)
            .expect("authenticate peer");

        assert_eq!(
            session.subject().principal(),
            key.peer_identity().principal(),
        );

        assert_eq!(
            session.subject().authentication_method().as_str(),
            "distributed.ed25519.challenge",
        );

        assert!(!session.is_expired());
    }

    #[test]
    fn authenticated_session_can_be_resolved() {
        let key = key_pair("service::worker", 8);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        let session = service
            .complete_authentication(response)
            .expect("authenticate peer");

        let subject = service
            .authenticate_session(session.id())
            .expect("resolve session");

        assert_eq!(
            subject.principal(),
            key.peer_identity().principal(),
        );
    }

    #[test]
    fn consumed_challenge_cannot_be_replayed() {
        let key = key_pair("service::worker", 9);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        let replay = response.clone();

        service
            .complete_authentication(response)
            .expect("first authentication");

        assert!(
            service.complete_authentication(replay).is_err(),
            "a consumed challenge must not be accepted again",
        );
    }

    #[test]
    fn untrusted_identity_is_rejected() {
        let key = key_pair("service::untrusted", 10);

        let trust = Arc::new(InMemoryPeerTrustStore::new());

        let service = AuthenticationService::new(
            AuthenticationConfig::default(),
            trust,
        )
        .expect("authentication service");

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        assert!(
            service.complete_authentication(response).is_err(),
            "unknown principal/key bindings must be rejected",
        );
    }

    #[test]
    fn a_proof_for_another_challenge_is_rejected() {
        let key = key_pair("service::worker", 11);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let mut response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        response.nonce[0] ^= 0x01;

        assert!(
            service.complete_authentication(response).is_err(),
        );
    }

    #[test]
    fn revoked_session_cannot_be_resolved() {
        let key = key_pair("service::worker", 12);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        let session = service
            .complete_authentication(response)
            .expect("authenticate peer");

        assert!(
            service
                .revoke_session(session.id())
                .expect("revoke session"),
        );

        assert!(
            service.authenticate_session(session.id()).is_err(),
        );
    }

    #[test]
    fn trust_can_be_revoked_for_one_key() {
        let key = key_pair("service::worker", 13);
        let (service, trust) = service(&key);

        let fingerprint = key.peer_identity().fingerprint().to_hex();

        assert!(
            trust
                .revoke_key(
                    key.peer_identity().principal(),
                    &fingerprint,
                )
                .expect("revoke key"),
        );

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        assert!(service.complete_authentication(response).is_err());
    }

    #[test]
    fn configuration_rejects_zero_lifetimes() {
        let mut config = AuthenticationConfig::default();
        config.challenge_lifetime = Duration::ZERO;

        assert!(config.validate().is_err());

        let mut config = AuthenticationConfig::default();
        config.session_lifetime = Duration::ZERO;

        assert!(config.validate().is_err());
    }

    #[test]
    fn session_identifier_debug_output_is_redacted() {
        let id = SessionId::from_bytes([0xA5; 32]);

        let output = format!("{id:?}");

        assert!(output.contains("REDACTED"));
        assert!(!output.contains("a5a5"));
    }

    #[test]
    fn challenge_serialization_round_trip() {
        let key = key_pair("service::worker", 14);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let encoded = serde_json::to_vec(&challenge)
            .expect("serialize challenge");

        let decoded: AuthenticationChallenge =
            serde_json::from_slice(&encoded)
                .expect("deserialize challenge");

        assert_eq!(challenge, decoded);
    }

    #[test]
    fn response_serialization_round_trip() {
        let key = key_pair("service::worker", 15);
        let (service, _) = service(&key);

        let challenge = service
            .issue_challenge(method())
            .expect("issue challenge");

        let response = AuthenticationResponse::create(
            &key,
            &challenge,
        )
        .expect("create response");

        let encoded = serde_json::to_vec(&response)
            .expect("serialize response");

        let decoded: AuthenticationResponse =
            serde_json::from_slice(&encoded)
                .expect("deserialize response");

        service
            .complete_authentication(decoded)
            .expect("authenticate deserialized response");
    }
}
