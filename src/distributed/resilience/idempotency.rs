
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Distributed Resilience — Idempotency
//!
//! Path: src/distributed/resilience/idempotency.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides provider-independent contracts for durable distributed
//! operation deduplication, execution ownership, fencing, completion
//! persistence, and unknown-outcome reconciliation.
//!
//! # Fundamental guarantee
//!
//! A repeated request using the same logical operation identity and
//! identical request fingerprint must not independently execute an
//! already completed operation.
//!
//! Reusing an operation identity with different request content is
//! a conflict.
//!
//! A request identifier alone does not establish exactly-once execution.
//!
//! The durable provider must atomically implement the transitions
//! documented by IdempotencyStore.
//!
//! # Ownership
//!
//! This module owns:
//! - Idempotency keys.
//! - Request fingerprints.
//! - Stored result integrity.
//! - Execution leases.
//! - Fencing-token contracts.
//! - Idempotency record states.
//! - Claim outcomes.
//! - Reconciliation decisions.
//! - Provider-independent idempotency coordination.
//!
//! This module does not own:
//! - Distributed operation identity allocation.
//! - Transport message identity.
//! - Request-response correlation.
//! - Execution lifecycle.
//! - Retry policy.
//! - Checkpoint persistence.
//! - Recovery execution.
//! - Networking.
//! - Authentication.
//! - Authorization.
//! - Backend execution.
//! - Domain-specific serialization.
//! - Source-language semantics.
//! - ZUIR, ZCIR, quantum::ir, or HDLIR.
//! - Physical resource allocation.
//!
//! # Required authorities
//!
//! - distributed::ids owns OperationId.
//! - distributed::error owns DistributedError.
//! - distributed::config owns RetryPolicy.
//! - distributed::resilience::retry owns retry evaluation.
//! - distributed::execution owns actual execution.
//! - distributed::transport owns message delivery.
//! - distributed::resilience::checkpoint owns checkpointing.
//! - distributed::resilience::recovery owns recovery.
//! - The injected durable storage provider owns persistent records.
//!
//! # Durability
//!
//! IdempotencyStore is the mandatory persistence boundary.
//!
//! A production implementation MUST provide atomic, durable state
//! transitions. An ordinary process-local mutex or hash map is not
//! a production substitute for durable storage.
//!
//! # Scalability
//!
//! This module imposes no universal maximum on operations, participants,
//! workers, machines, computational domains, or execution contexts.
//!
//! Concrete storage, transaction, memory, and concurrency budgets
//! remain provider-controlled operational constraints.
//!
//! # Safety
//!
//! Safe Rust only.

use core::fmt;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedResult,
};

use crate::distributed::ids::OperationId;

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable identifier for the distributed idempotency contract.
pub const IDEMPOTENCY_SCHEMA_ID: &str =
    "zamani.distributed.resilience.idempotency";

/// Version of the idempotency record contract.
pub const IDEMPOTENCY_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// VALIDATION
// ============================================================================

fn invalid_input(message: impl Into<String>) -> DistributedError {
    DistributedError::invalid_input(message)
}

fn conflict(
    subject: impl Into<String>,
    message: impl Into<String>,
) -> DistributedError {
    DistributedError::Conflict {
        subject: subject.into(),
        message: message.into(),
    }
}

fn unknown_outcome(
    operation_id: &OperationId,
    message: impl Into<String>,
) -> DistributedError {
    DistributedError::outcome_unknown(
        Some(operation_id.to_string()),
        DeliveryState::Unknown,
        message,
    )
}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> DistributedResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(format!(
            "{field} must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// OPERATION IDENTITY
// ============================================================================

/// Logical idempotency key.
///
/// The key combines an application-defined scope with the existing
/// distributed OperationId.
///
/// The scope distinguishes independent logical operation namespaces,
/// such as services, tenants, workflows, or application boundaries.
///
/// Scope is not an authentication identity.
///
/// Authorization must be performed by the security subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdempotencyKey {
    scope: String,
    operation_id: OperationId,
}

impl IdempotencyKey {
    /// Creates a validated idempotency key.
    ///
    /// The supplied OperationId must remain stable across retries
    /// and recovery attempts belonging to the same logical operation.
    pub fn new(
        scope: impl Into<String>,
        operation_id: OperationId,
    ) -> DistributedResult<Self> {
        let scope = scope.into();

        validate_identifier("idempotency.scope", &scope)?;

        Ok(Self {
            scope,
            operation_id,
        })
    }

    /// Returns the logical operation scope.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Returns the authoritative distributed operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Validates the key again after deserialization.
    pub fn validate(&self) -> DistributedResult<()> {
        validate_identifier("idempotency.scope", &self.scope)
    }
}

// ============================================================================
// REQUEST FINGERPRINT
// ============================================================================

/// Cryptographic fingerprint of the canonical logical request.
///
/// The fingerprint prevents an application from accidentally or
/// maliciously reusing an operation identity for different content.
///
/// The caller MUST calculate it from a canonical representation whose
/// semantics are defined by the owning application/domain contract.
///
/// This module does not serialize arbitrary Zamani values.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct OperationFingerprint {
    sha256: [u8; 32],
}

impl OperationFingerprint {
    /// Constructs a fingerprint from canonical request bytes.
    ///
    /// Canonicalization must occur before this method is called.
    #[must_use]
    pub fn from_canonical_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);

        let mut sha256 = [0_u8; 32];
        sha256.copy_from_slice(&digest);

        Self { sha256 }
    }

    /// Constructs a fingerprint from an externally computed digest.
    ///
    /// The caller is responsible for verifying that the digest represents
    /// the correct canonical request.
    #[must_use]
    pub const fn from_digest(sha256: [u8; 32]) -> Self {
        Self { sha256 }
    }

    /// Returns the SHA-256 digest.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.sha256
    }

    /// Returns the lowercase hexadecimal representation.
    #[must_use]
    pub fn to_hex(&self) -> String {
        let mut output = String::with_capacity(64);

        for byte in self.sha256 {
            use core::fmt::Write;

            let _ = write!(&mut output, "{byte:02x}");
        }

        output
    }
}

// ============================================================================
// STORED RESULT
// ============================================================================

/// A completed operation result stored by the idempotency authority.
///
/// The payload is opaque to this module. Its schema and interpretation
/// remain with the execution provider and canonical domain representation.
///
/// The digest protects against accidental storage corruption.
/// It does not authenticate the storage provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredResult {
    format: String,
    format_version: u32,
    payload: Vec<u8>,
    digest: [u8; 32],
}

impl StoredResult {
    /// Creates a stored result from an opaque encoded payload.
    ///
    /// The caller must supply a stable format identifier and version.
    pub fn new(
        format: impl Into<String>,
        format_version: u32,
        payload: Vec<u8>,
    ) -> DistributedResult<Self> {
        let format = format.into();

        validate_identifier("result.format", &format)?;

        let digest = Self::calculate_digest(
            &format,
            format_version,
            &payload,
        );

        Ok(Self {
            format,
            format_version,
            payload,
            digest,
        })
    }

    fn calculate_digest(
        format: &str,
        version: u32,
        payload: &[u8],
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update((format.len() as u128).to_be_bytes());
        hasher.update(format.as_bytes());
        hasher.update(version.to_be_bytes());
        hasher.update((payload.len() as u128).to_be_bytes());
        hasher.update(payload);

        let result = hasher.finalize();

        let mut digest = [0_u8; 32];
        digest.copy_from_slice(&result);

        digest
    }

    /// Returns the result format identifier.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Returns the result format version.
    #[must_use]
    pub const fn format_version(&self) -> u32 {
        self.format_version
    }

    /// Returns the opaque result payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Returns the integrity digest.
    #[must_use]
    pub const fn digest(&self) -> &[u8; 32] {
        &self.digest
    }

    /// Revalidates result metadata and integrity.
    pub fn verify(&self) -> DistributedResult<()> {
        validate_identifier("result.format", &self.format)?;

        let expected = Self::calculate_digest(
            &self.format,
            self.format_version,
            &self.payload,
        );

        if expected != self.digest {
            return Err(DistributedError::RecoveryFailed {
                subject: "idempotency.result_integrity".to_owned(),
                message: "stored result integrity verification failed"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// EXECUTION OWNERSHIP
// ============================================================================

/// Globally distinguishable logical execution-owner identity.
///
/// This is an ownership label, not proof of authentication.
///
/// The security subsystem must authenticate the actual participant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExecutionOwner {
    identity: String,
}

impl ExecutionOwner {
    /// Creates a validated execution-owner identity.
    pub fn new(identity: impl Into<String>) -> DistributedResult<Self> {
        let identity = identity.into();

        validate_identifier("idempotency.owner", &identity)?;

        Ok(Self { identity })
    }

    /// Returns the logical owner identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

// ============================================================================
// FENCING
// ============================================================================

/// Monotonically increasing ownership generation.
///
/// Every successful takeover must receive a fencing token greater
/// than all previous tokens for the same key.
///
/// A storage provider and the actual side-effecting resource must
/// enforce fencing where concurrent stale owners could otherwise
/// produce duplicate effects.
///
/// A fencing token is not a substitute for transactional side effects.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct FencingToken(u128);

impl FencingToken {
    /// Constructs a non-zero fencing token.
    pub fn new(value: u128) -> DistributedResult<Self> {
        if value == 0 {
            return Err(invalid_input(
                "fencing token must be non-zero",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the numeric token.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

/// Ownership lease issued by the authoritative idempotency store.
///
/// Lease validity is checked by the store, not by the caller's local clock.
///
/// A lease may become stale even while a process still holds this value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLease {
    key: IdempotencyKey,
    fingerprint: OperationFingerprint,
    owner: ExecutionOwner,
    fencing_token: FencingToken,
}

impl ExecutionLease {
    /// Returns the logical operation key.
    #[must_use]
    pub fn key(&self) -> &IdempotencyKey {
        &self.key
    }

    /// Returns the request fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> OperationFingerprint {
        self.fingerprint
    }

    /// Returns the owner identity.
    #[must_use]
    pub fn owner(&self) -> &ExecutionOwner {
        &self.owner
    }

    /// Returns the fencing token.
    #[must_use]
    pub const fn fencing_token(&self) -> FencingToken {
        self.fencing_token
    }
}

// ============================================================================
// RECORD STATE
// ============================================================================

/// Durable state of one idempotency record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IdempotencyState {
    /// The operation is reserved but execution has not started.
    Reserved,

    /// The execution boundary has durably acknowledged that execution
    /// may begin or has begun.
    Executing,

    /// The operation completed and its result was durably committed.
    Completed {
        /// Stored authoritative result.
        result: StoredResult,
    },

    /// The operation may have executed, but its final outcome is unknown.
    ///
    /// It must be reconciled before another submission is permitted.
    OutcomeUnknown {
        /// Diagnostic explanation.
        reason: String,
    },

    /// An authoritative execution provider established that the operation
    /// did not execute and produced no observable side effects.
    NotExecuted {
        /// Diagnostic explanation.
        reason: String,
    },
}

/// Durable snapshot of an idempotency record.
///
/// This is a record of operation deduplication state, not a second
/// execution lifecycle state machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdempotencyRecord {
    key: IdempotencyKey,
    fingerprint: OperationFingerprint,
    owner: Option<ExecutionOwner>,
    fencing_token: FencingToken,
    state: IdempotencyState,
}

impl IdempotencyRecord {
    /// Returns the operation key.
    #[must_use]
    pub fn key(&self) -> &IdempotencyKey {
        &self.key
    }

    /// Returns the request fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> OperationFingerprint {
        self.fingerprint
    }

    /// Returns the current execution owner, if one exists.
    #[must_use]
    pub fn owner(&self) -> Option<&ExecutionOwner> {
        self.owner.as_ref()
    }

    /// Returns the fencing token.
    #[must_use]
    pub const fn fencing_token(&self) -> FencingToken {
        self.fencing_token
    }

    /// Returns the durable idempotency state.
    #[must_use]
    pub fn state(&self) -> &IdempotencyState {
        &self.state
    }
}

// ============================================================================
// CLAIM REQUEST
// ============================================================================

/// Input for an atomic operation claim.
#[derive(Debug, Clone)]
pub struct ClaimRequest {
    key: IdempotencyKey,
    fingerprint: OperationFingerprint,
    owner: ExecutionOwner,
    lease_duration: Duration,
}

impl ClaimRequest {
    /// Constructs a validated claim request.
    ///
    /// The lease duration is an operational policy, not an execution
    /// guarantee. The store evaluates lease expiry using its own
    /// authoritative clock.
    pub fn new(
        key: IdempotencyKey,
        fingerprint: OperationFingerprint,
        owner: ExecutionOwner,
        lease_duration: Duration,
    ) -> DistributedResult<Self> {
        key.validate()?;

        if lease_duration.is_zero() {
            return Err(invalid_input(
                "idempotency lease duration must be positive",
            ));
        }

        Ok(Self {
            key,
            fingerprint,
            owner,
            lease_duration,
        })
    }

    /// Returns the requested key.
    #[must_use]
    pub fn key(&self) -> &IdempotencyKey {
        &self.key
    }

    /// Returns the request fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> OperationFingerprint {
        self.fingerprint
    }

    /// Returns the requested owner.
    #[must_use]
    pub fn owner(&self) -> &ExecutionOwner {
        &self.owner
    }

    /// Returns the requested lease duration.
    #[must_use]
    pub const fn lease_duration(&self) -> Duration {
        self.lease_duration
    }
}

// ============================================================================
// CLAIM OUTCOME
// ============================================================================

/// Result of an atomic idempotency claim.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClaimOutcome {
    /// The caller owns the operation and may begin the execution protocol.
    Acquired {
        /// Authoritative execution lease.
        lease: ExecutionLease,
    },

    /// Another valid owner currently holds the reservation.
    InProgress {
        /// Current record.
        record: IdempotencyRecord,
    },

    /// A completed operation already exists.
    ///
    /// The caller must return the stored result rather than execute again.
    AlreadyCompleted {
        /// Authoritative previously committed result.
        result: StoredResult,
    },

    /// Execution may have occurred and requires reconciliation.
    ReconciliationRequired {
        /// Current record.
        record: IdempotencyRecord,
    },

    /// The same operation identity was reused with different content.
    FingerprintConflict {
        /// Existing request fingerprint.
        existing: OperationFingerprint,
    },
}

// ============================================================================
// RECONCILIATION
// ============================================================================

/// Authoritative evidence supplied by the execution/recovery provider.
///
/// These decisions must never be inferred from a timeout or transport
/// error alone.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReconciliationDecision {
    /// The authoritative provider established that execution never
    /// started and no observable side effect occurred.
    NotExecuted {
        /// Evidence/provenance reference.
        evidence: String,
    },

    /// The provider established successful completion and supplies
    /// the authoritative result.
    Completed {
        /// Authoritative result.
        result: StoredResult,

        /// Evidence/provenance reference.
        evidence: String,
    },

    /// The provider established that execution failed without
    /// producing observable side effects.
    FailedWithoutEffects {
        /// Evidence/provenance reference.
        evidence: String,
    },

    /// The provider cannot establish the operation's final outcome.
    StillUnknown {
        /// Explanation.
        reason: String,
    },
}

// ============================================================================
// STORE CONTRACT
// ============================================================================

/// Durable idempotency storage provider.
///
/// This trait is the critical production boundary.
///
/// Implementations MUST provide atomic operations across all processes
/// that can access the same logical key.
///
/// The following guarantees are mandatory:
///
/// 1. `claim` is atomic for the key and fingerprint.
/// 2. Fingerprint conflicts cannot be bypassed.
/// 3. Fencing tokens are strictly increasing per key.
/// 4. `mark_started` is durable before execution side effects begin.
/// 5. `complete` commits the result before returning success.
/// 6. Stale fencing tokens cannot overwrite newer records.
/// 7. An expired `Reserved` lease may be reclaimed safely.
/// 8. An expired `Executing` lease must become reconciliation-required,
///    never automatically re-executable.
/// 9. Completed records remain available for the configured
///    deduplication-retention period.
/// 10. Provider failures must not be represented as successful commits.
///
/// A database implementation should use transactions, compare-and-set,
/// unique constraints, or an equivalent atomic mechanism.
///
/// A provider must document its crash-consistency and durability level.
///
/// The trait deliberately contains no default implementation because
/// a non-atomic fallback would silently violate its contract.
pub trait IdempotencyStore: Send + Sync + fmt::Debug {
    /// Atomically claims or inspects an operation.
    fn claim(
        &self,
        request: ClaimRequest,
    ) -> DistributedResult<ClaimOutcome>;

    /// Durably transitions a reserved operation into execution.
    ///
    /// This MUST happen before allowing observable side effects.
    ///
    /// The fencing token and owner must match the current reservation.
    fn mark_started(
        &self,
        lease: &ExecutionLease,
    ) -> DistributedResult<()>;

    /// Atomically commits the completed result.
    ///
    /// A successful return means the result is durably stored according
    /// to the provider's documented durability contract.
    fn complete(
        &self,
        lease: &ExecutionLease,
        result: StoredResult,
    ) -> DistributedResult<()>;

    /// Marks an operation's outcome as unknown.
    ///
    /// This is required when the caller cannot establish whether
    /// execution or its side effects completed.
    fn mark_outcome_unknown(
        &self,
        lease: &ExecutionLease,
        reason: String,
    ) -> DistributedResult<()>;

    /// Reconciles an unknown operation using authoritative evidence.
    ///
    /// The provider must reject stale decisions and conflicting updates.
    ///
    /// A NotExecuted or FailedWithoutEffects decision permits a future
    /// claim with a new fencing token.
    ///
    /// Completed permanently protects the operation from re-execution
    /// while its deduplication record is retained.
    fn reconcile(
        &self,
        key: &IdempotencyKey,
        fingerprint: OperationFingerprint,
        decision: ReconciliationDecision,
    ) -> DistributedResult<IdempotencyRecord>;

    /// Retrieves the current durable record.
    fn lookup(
        &self,
        key: &IdempotencyKey,
    ) -> DistributedResult<Option<IdempotencyRecord>>;
}

// ============================================================================
// COORDINATOR
// ============================================================================

/// Provider-independent idempotency coordinator.
///
/// The coordinator owns no database, process-global singleton, scheduler,
/// transport, executor, or retry loop.
///
/// It delegates every authoritative transition to the injected store.
#[derive(Debug, Clone)]
pub struct IdempotencyCoordinator {
    store: Arc<dyn IdempotencyStore>,
}

impl IdempotencyCoordinator {
    /// Constructs a coordinator using an injected store.
    #[must_use]
    pub fn new(store: Arc<dyn IdempotencyStore>) -> Self {
        Self { store }
    }

    /// Atomically claims an operation.
    ///
    /// Callers must inspect every outcome.
    ///
    /// Only `Acquired` permits the caller to continue to the
    /// execution-boundary protocol.
    pub fn claim(
        &self,
        request: ClaimRequest,
    ) -> DistributedResult<ClaimOutcome> {
        self.store.claim(request)
    }

    /// Durably marks the execution boundary as started.
    ///
    /// The executor must not invoke the side-effecting provider until
    /// this method succeeds.
    pub fn mark_started(
        &self,
        lease: &ExecutionLease,
    ) -> DistributedResult<()> {
        self.store.mark_started(lease)
    }

    /// Commits an authoritative successful result.
    ///
    /// The executor must not report a deduplicated successful result
    /// until this method succeeds.
    pub fn complete(
        &self,
        lease: &ExecutionLease,
        result: StoredResult,
    ) -> DistributedResult<()> {
        result.verify()?;

        self.store.complete(lease, result)
    }

    /// Records an uncertain execution outcome.
    ///
    /// This method must be called when the execution boundary cannot
    /// prove either successful completion or absence of side effects.
    pub fn mark_outcome_unknown(
        &self,
        lease: &ExecutionLease,
        reason: impl Into<String>,
    ) -> DistributedResult<()> {
        let reason = reason.into();

        validate_identifier("idempotency.unknown_reason", &reason)?;

        self.store.mark_outcome_unknown(lease, reason)
    }

    /// Applies authoritative reconciliation evidence.
    pub fn reconcile(
        &self,
        key: &IdempotencyKey,
        fingerprint: OperationFingerprint,
        decision: ReconciliationDecision,
    ) -> DistributedResult<IdempotencyRecord> {
        key.validate()?;

        match &decision {
            ReconciliationDecision::NotExecuted { evidence }
            | ReconciliationDecision::Completed { evidence, .. }
            | ReconciliationDecision::FailedWithoutEffects { evidence } => {
                validate_identifier(
                    "idempotency.reconciliation_evidence",
                    evidence,
                )?;
            }

            ReconciliationDecision::StillUnknown { reason } => {
                validate_identifier(
                    "idempotency.reconciliation_reason",
                    reason,
                )?;
            }
        }

        if let ReconciliationDecision::Completed { result, .. } = &decision {
            result.verify()?;
        }

        self.store.reconcile(key, fingerprint, decision)
    }

    /// Looks up an existing operation record.
    pub fn lookup(
        &self,
        key: &IdempotencyKey,
    ) -> DistributedResult<Option<IdempotencyRecord>> {
        key.validate()?;

        self.store.lookup(key)
    }
}

// ============================================================================
// EXECUTION RESULT CLASSIFICATION
// ============================================================================

/// Classification of an execution provider's final report.
///
/// This type prevents a generic execution error from being interpreted
/// as proof that no side effects occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExecutionDisposition {
    /// The provider confirms no execution and no observable effects.
    NotExecuted {
        /// Diagnostic explanation.
        reason: String,
    },

    /// The provider confirms successful completion.
    Completed {
        /// Authoritative result.
        result: StoredResult,
    },

    /// Execution may have started or side effects may have occurred.
    Unknown {
        /// Diagnostic explanation.
        reason: String,
    },
}

impl ExecutionDisposition {
    /// Returns whether execution definitely produced no side effects.
    #[must_use]
    pub fn definitely_no_effects(&self) -> bool {
        matches!(self, Self::NotExecuted { .. })
    }

    /// Returns whether the operation has an authoritative completed result.
    #[must_use]
    pub fn completed_result(&self) -> Option<&StoredResult> {
        match self {
            Self::Completed { result } => Some(result),
            _ => None,
        }
    }
}

// ============================================================================
// DOCUMENTATION HELPERS
// ============================================================================

/// Describes the guarantee available from an idempotency provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IdempotencyGuarantee {
    /// Only process-local duplicate detection is available.
    ///
    /// This is insufficient for production distributed execution.
    ProcessLocal,

    /// Durable operation records and atomic claims are available,
    /// but external side effects may require separate reconciliation.
    DurableDeduplication,

    /// The provider integrates operation records and side effects
    /// through a documented atomic transaction boundary.
    TransactionallyCoupledEffects,
}

impl IdempotencyGuarantee {
    /// Returns whether the guarantee is sufficient for durable
    /// duplicate-request protection.
    #[must_use]
    pub const fn is_durable(self) -> bool {
        matches!(
            self,
            Self::DurableDeduplication
                | Self::TransactionallyCoupledEffects
        )
    }

    /// Returns whether the guarantee establishes transactionally
    /// coupled side effects.
    #[must_use]
    pub const fn couples_effects(self) -> bool {
        matches!(
            self,
            Self::TransactionallyCoupledEffects
        )
    }
}

/// Capability description for a concrete idempotency provider.
///
/// The execution/runtime integration must verify these capabilities
/// against the required execution guarantee before advertising a
/// production feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyCapabilities {
    /// Provider identity.
    pub provider: String,

    /// Available idempotency guarantee.
    pub guarantee: IdempotencyGuarantee,

    /// Whether fencing is enforced by the provider.
    pub fencing_supported: bool,

    /// Whether completion records survive provider process restart.
    pub durable_records: bool,

    /// Whether the provider supports authoritative reconciliation.
    pub reconciliation_supported: bool,

    /// Whether the provider supports atomic side-effect transactions.
    pub transactional_effects: bool,
}

impl IdempotencyCapabilities {
    /// Validates provider capability consistency.
    pub fn validate(&self) -> DistributedResult<()> {
        validate_identifier(
            "idempotency.provider",
            &self.provider,
        )?;

        if self.guarantee.is_durable() && !self.durable_records {
            return Err(DistributedError::InvalidConfiguration {
                setting: "idempotency.durable_records".to_owned(),
                message: "a durable guarantee requires durable records"
                    .to_owned(),
            });
        }

        if self.transactional_effects
            && !self.guarantee.couples_effects()
        {
            return Err(DistributedError::InvalidConfiguration {
                setting: "idempotency.transactional_effects".to_owned(),
                message: "transactional effects require a matching guarantee"
                    .to_owned(),
            });
        }

        if self.guarantee.couples_effects()
            && !self.transactional_effects
        {
            return Err(DistributedError::InvalidConfiguration {
                setting: "idempotency.guarantee".to_owned(),
                message: "transactionally coupled guarantees require effect transactions"
                    .to_owned(),
            });
        }

        if self.fencing_supported && !self.durable_records {
            return Err(DistributedError::InvalidConfiguration {
                setting: "idempotency.fencing_supported".to_owned(),
                message: "durable fencing requires persistent ownership records"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// PROVIDER ERROR CONTEXT
// ============================================================================

/// Converts a provider failure into a distributed error without losing
/// its original source.
///
/// The caller supplies the subsystem identity; it must not contain
/// credentials or sensitive connection details.
pub fn provider_error<E>(
    operation: &'static str,
    error: E,
) -> DistributedError
where
    E: Error + Send + Sync + 'static,
{
    DistributedError::external(
        "distributed.idempotency",
        operation,
        error,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::OperationId;

    fn operation_id() -> OperationId {
        OperationId::new(17, 29).expect("valid operation identity")
    }

    fn key() -> IdempotencyKey {
        IdempotencyKey::new("test.application", operation_id())
            .expect("valid idempotency key")
    }

    #[test]
    fn operation_identity_is_preserved() {
        let key = key();

        assert_eq!(key.scope(), "test.application");
        assert_eq!(key.operation_id(), operation_id());
    }

    #[test]
    fn rejects_invalid_scope() {
        assert!(IdempotencyKey::new("", operation_id()).is_err());
        assert!(IdempotencyKey::new("invalid\nscope", operation_id()).is_err());
    }

    #[test]
    fn fingerprints_are_deterministic() {
        let first =
            OperationFingerprint::from_canonical_bytes(b"request");

        let second =
            OperationFingerprint::from_canonical_bytes(b"request");

        let different =
            OperationFingerprint::from_canonical_bytes(b"another request");

        assert_eq!(first, second);
        assert_ne!(first, different);
        assert_eq!(first.to_hex().len(), 64);
    }

    #[test]
    fn stored_result_verifies_integrity() {
        let result = StoredResult::new(
            "zamani.application.result",
            1,
            b"successful-result".to_vec(),
        )
        .expect("valid result");

        assert!(result.verify().is_ok());
        assert_eq!(result.payload(), b"successful-result");
    }

    #[test]
    fn rejects_empty_result_format() {
        assert!(
            StoredResult::new("", 1, Vec::new()).is_err()
        );
    }

    #[test]
    fn fencing_tokens_have_monotonic_order() {
        let first = FencingToken::new(1).unwrap();
        let second = FencingToken::new(2).unwrap();

        assert!(second > first);
        assert!(FencingToken::new(0).is_err());
    }

    #[test]
    fn claim_rejects_zero_lease_duration() {
        let owner = ExecutionOwner::new("worker.example").unwrap();

        assert!(
            ClaimRequest::new(
                key(),
                OperationFingerprint::from_canonical_bytes(b"request"),
                owner,
                Duration::ZERO,
            )
            .is_err()
        );
    }

    #[test]
    fn execution_disposition_does_not_confuse_unknown_with_failure() {
        let unknown = ExecutionDisposition::Unknown {
            reason: "remote response was lost".to_owned(),
        };

        assert!(!unknown.definitely_no_effects());
        assert!(unknown.completed_result().is_none());

        let not_executed = ExecutionDisposition::NotExecuted {
            reason: "rejected before execution".to_owned(),
        };

        assert!(not_executed.definitely_no_effects());
    }

    #[test]
    fn provider_capabilities_reject_false_durability_claims() {
        let capabilities = IdempotencyCapabilities {
            provider: "example.provider".to_owned(),
            guarantee: IdempotencyGuarantee::DurableDeduplication,
            fencing_supported: true,
            durable_records: false,
            reconciliation_supported: true,
            transactional_effects: false,
        };

        assert!(capabilities.validate().is_err());
    }

    #[test]
    fn process_local_guarantee_is_not_durable() {
        assert!(
            !IdempotencyGuarantee::ProcessLocal.is_durable()
        );

        assert!(
            IdempotencyGuarantee::DurableDeduplication.is_durable()
        );
    }
}
