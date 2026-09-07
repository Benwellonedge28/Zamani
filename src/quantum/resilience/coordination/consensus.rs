//! Zamani Quantum Resilience — Distributed Consensus Contract
//!
//! Path:
//!     src/quantum/resilience/coordination/consensus.rs
//!
//! Purpose:
//!     Provider-neutral consensus contracts for distributed quantum-resilience
//!     coordination.
//!
//! Architectural position:
//!
//!     Zamani Quantum Program
//!              |
//!              v
//!     Canonical Quantum IR
//!              |
//!              v
//!     Resilience
//!              |
//!              v
//!     Coordination
//!              |
//!       +------+-------+
//!       |              |
//!       v              v
//!   Ownership        Lease
//!       |              |
//!       +------+-------+
//!              |
//!              v
//!          Consensus
//!              |
//!              v
//!      Distributed Decision
//!
//! This file defines the CONSENSUS CONTRACT.
//!
//! It does NOT implement a particular consensus algorithm.
//!
//! -----------------------------------------------------------------------------
//! RESPONSIBILITIES
//! -----------------------------------------------------------------------------
//!
//! This file owns:
//!
//! - consensus operation identity;
//! - proposal identity;
//! - execution identity;
//! - participant identity;
//! - consensus epochs;
//! - proposal representation;
//! - proposal digests;
//! - participant votes;
//! - vote identity and sequence;
//! - decision representation;
//! - decision evidence;
//! - consensus protocol traits;
//! - quorum/decision-policy contracts;
//! - deterministic validation;
//! - stale-epoch protection;
//! - duplicate-participant detection;
//! - duplicate-vote detection;
//! - decision safety invariants;
//! - protocol capability declarations;
//! - cancellation/deadline contracts;
//! - integration contracts for coordinator, distributed, lease and ownership.
//!
//! This file does NOT own:
//!
//! - network transport;
//! - message delivery;
//! - leader election;
//! - membership discovery;
//! - quorum calculation;
//! - a specific quorum algorithm;
//! - Raft;
//! - Paxos;
//! - PBFT;
//! - HotStuff;
//! - blockchain consensus;
//! - quantum error correction;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum execution;
//! - hardware discovery;
//! - hardware calibration;
//! - mitigation;
//! - recovery planning;
//! - recovery execution;
//! - result verification.
//!
//! Those concerns belong to their owning subsystems.
//!
//! -----------------------------------------------------------------------------
//! IMPORTANT QUANTUM ARCHITECTURE RULE
//! -----------------------------------------------------------------------------
//!
//! Consensus is a control-plane abstraction.
//!
//! It must NOT create a second quantum identity system.
//!
//! If a proposal concerns a logical or physical qubit, the canonical identities
//! remain:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module deliberately does not import those types because consensus
//! should remain generic over the resource being coordinated.
//!
//! The canonical resilience resource model or the owning adapter should carry
//! the quantum resource identity into the consensus proposal.
//!
//! This prevents consensus from becoming coupled to one quantum architecture.
//!
//! -----------------------------------------------------------------------------
//! WRITE ONCE / SCALE EVERYWHERE
//! -----------------------------------------------------------------------------
//!
//! No value in this file represents a machine-size limit.
//!
//! There is deliberately no:
//!
//!     MAX_PARTICIPANTS
//!     MAX_QPUS
//!     MAX_QUBITS
//!     MAX_BACKENDS
//!     DEFAULT_QUORUM
//!     DEFAULT_TIMEOUT
//!     RETRY_COUNT
//!     MAX_VOTES
//!
//! Participant collections grow according to available resources.
//!
//! The actual scalability boundary is imposed by the selected implementation,
//! transport, storage, memory, CPU, communication fabric and policy.
//!
//! "Infinite scalability" therefore means that this contract introduces no
//! artificial quantum-system ceiling.
//!
//! -----------------------------------------------------------------------------
//! SAFETY MODEL
//! -----------------------------------------------------------------------------
//!
//! Consensus success is NOT equivalent to:
//!
//!     one participant succeeded
//!
//! Nor is it equivalent to:
//!
//!     enough participants responded
//!
//! A decision is valid only when the injected decision policy/protocol confirms
//! that all required safety conditions have been satisfied.
//!
//! In particular:
//!
//!     availability != correctness
//!     response count != quorum
//!     vote count != authorization
//!     lease ownership != consensus
//!     consensus != quantum-result verification
//!
//! The final semantic correctness of a quantum result remains the responsibility
//! of the resilience verification subsystem.
//!
//! -----------------------------------------------------------------------------
//! FENCING
//! -----------------------------------------------------------------------------
//!
//! Consensus proposals may carry:
//!
//! - a coordination epoch;
//! - a lease fencing token;
//! - an operation generation;
//! - an implementation-defined authority proof.
//!
//! This module does not interpret those values as hardware identifiers.
//!
//! It validates that a proposal and its votes refer to the same epoch and
//! proposal identity.
//!
//! The lease/ownership subsystem remains authoritative for fencing.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! The contract supports deterministic consensus decisions.
//!
//! A deterministic implementation should produce the same decision when given
//! identical:
//!
//! - proposal;
//! - participant set;
//! - ordered vote set;
//! - policy;
//! - protocol configuration;
//! - epoch;
//! - evidence.
//!
//! This module therefore uses explicit ordering and rejects ambiguous duplicate
//! participant/vote identities.
//!
//! -----------------------------------------------------------------------------
//! IDEMPOTENCY
//! -----------------------------------------------------------------------------
//!
//! Consensus operations must be safely repeatable at the protocol boundary.
//!
//! Implementations should identify a consensus operation using `ConsensusId`.
//!
//! Re-delivery of the same proposal or vote MUST NOT silently create a second
//! semantic operation.
//!
//! Whether a duplicate is:
//!
//! - accepted as an idempotent replay;
//! - rejected;
//! - returned from durable state;
//!
//! is implementation-specific.
//!
//! The contract itself guarantees that a single decision cannot contain
//! conflicting votes from the same participant.
//!
//! -----------------------------------------------------------------------------
//! DISTRIBUTED SYSTEMS RULE
//! -----------------------------------------------------------------------------
//!
//! This module does not assume synchronized clocks.
//!
//! No consensus decision is based on local wall-clock comparison.
//!
//! Deadlines and cancellation are carried as caller-supplied control metadata.
//!
//! Concrete implementations may use:
//!
//! - monotonic clocks;
//! - logical clocks;
//! - hybrid clocks;
//! - externally coordinated time;
//! - transport-level deadlines.
//!
//! -----------------------------------------------------------------------------
//! TWO-PHASE CONTROL-PLANE RULE
//! -----------------------------------------------------------------------------
//!
//! Consensus may be used to coordinate control-plane state such as:
//!
//! - ownership transitions;
//! - resource allocation;
//! - recovery-plan commitment;
//! - migration authorization;
//! - distributed state transitions.
//!
//! It MUST NOT claim that an arbitrary in-flight quantum state can be rolled
//! back transactionally.
//!
//! Quantum state rollback remains subject to the checkpoint/recovery model.
//!
//! -----------------------------------------------------------------------------
//! ERROR CONTRACT
//! -----------------------------------------------------------------------------
//!
//! All fallible public operations use the canonical resilience error boundary:
//!
//!     crate::quantum::resilience::errors::ResilienceError
//!
//!     crate::quantum::resilience::errors::ResilienceResult
//!
//! No second consensus-specific error hierarchy is introduced.
//!
//! -----------------------------------------------------------------------------
//! RUST CONTRACT
//! -----------------------------------------------------------------------------
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

use std::fmt;
use std::num::NonZeroU64;
use std::sync::Arc;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the consensus contract.
pub const CONSENSUS_SCHEMA_ID: &str =
    "zamani.quantum.resilience.coordination.consensus";

/// Semantic contract version.
///
/// This is a protocol/schema version, not a resource limit.
pub const CONSENSUS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Shared immutable string
// =============================================================================

type SharedString = Arc<str>;

// =============================================================================
// Identifier validation
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> ResilienceResult<()> {
    if value.trim().is_empty() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidIdentifier,
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidIdentifier,
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

// =============================================================================
// Consensus identifier
// =============================================================================

/// Globally meaningful identity of a consensus operation.
///
/// The identifier is supplied by the caller. This module deliberately does
/// not generate random IDs so deterministic replay remains possible.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConsensusId(SharedString);

impl ConsensusId {
    /// Creates a consensus operation identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "consensus identifier")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConsensusId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Proposal identifier
// =============================================================================

/// Identity of one proposal within a consensus operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalId(SharedString);

impl ProposalId {
    /// Creates a proposal identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "proposal identifier")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProposalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Execution identifier
// =============================================================================

/// Identifies the quantum execution whose distributed control operation is
/// being coordinated.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(SharedString);

impl ExecutionId {
    /// Creates an execution identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "execution identifier")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Participant identity
// =============================================================================

/// Opaque identity of one consensus participant.
///
/// A participant may be:
///
/// - a runtime;
/// - a process;
/// - a worker;
/// - a QPU controller;
/// - a simulator;
/// - a distributed execution service;
/// - another coordination authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParticipantId(SharedString);

impl ParticipantId {
    /// Creates a participant identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "participant identifier")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ParticipantId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Epoch
// =============================================================================

/// Monotonically increasing consensus epoch.
///
/// Epochs separate generations of authority.
///
/// Zero is deliberately excluded so that an absent/uninitialized epoch cannot
/// accidentally be treated as an established consensus generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConsensusEpoch(NonZeroU64);

impl ConsensusEpoch {
    /// Creates an epoch from a non-zero value.
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates an epoch from a raw integer.
    pub fn try_from_u64(value: u64) -> ResilienceResult<Self> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                    "consensus epoch must be non-zero",
                )
            })
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Returns the next epoch.
    pub fn next(self) -> ResilienceResult<Self> {
        let next = self.get().checked_add(1).ok_or_else(|| {
            ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
                "consensus epoch overflow",
            )
        })?;

        Self::try_from_u64(next)
    }
}

impl fmt::Display for ConsensusEpoch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

// =============================================================================
// Sequence
// =============================================================================

/// Monotonically increasing vote/protocol sequence.
///
/// This is a protocol counter and not a participant/resource limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConsensusSequence(NonZeroU64);

impl ConsensusSequence {
    /// Initial sequence.
    pub const fn initial() -> Self {
        Self(NonZeroU64::MIN)
    }

    /// Creates a sequence from a positive integer.
    pub fn try_from_u64(value: u64) -> ResilienceResult<Self> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                    "consensus sequence must be non-zero",
                )
            })
    }

    /// Returns the sequence value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Advances the sequence.
    pub fn next(self) -> ResilienceResult<Self> {
        let next = self.get().checked_add(1).ok_or_else(|| {
            ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
                "consensus sequence overflow",
            )
        })?;

        Self::try_from_u64(next)
    }
}

impl Default for ConsensusSequence {
    fn default() -> Self {
        Self::initial()
    }
}

// =============================================================================
// Proposal digest
// =============================================================================

/// Opaque digest identifying the semantic content of a proposal.
///
/// The digest algorithm is intentionally not fixed here.
///
/// Implementations may use a cryptographic digest appropriate to the
/// repository's serialization/integrity subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalDigest(SharedString);

impl ProposalDigest {
    /// Creates a proposal digest.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "proposal digest")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the digest representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProposalDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Evidence digest
// =============================================================================

/// Opaque identifier/digest for evidence supporting a vote.
///
/// Evidence itself remains outside this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EvidenceDigest(SharedString);

impl EvidenceDigest {
    /// Creates an evidence digest.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "evidence digest")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EvidenceDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Coordination authority proof
// =============================================================================

/// Optional opaque authority/fencing proof attached to a consensus proposal.
///
/// The consensus layer transports and binds this proof to the proposal but
/// does not interpret its internal representation.
///
/// Lease/ownership implementations remain authoritative for authorization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorityProof(SharedString);

impl AuthorityProof {
    /// Creates an authority proof.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "authority proof")?;

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the proof representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Consensus mode
// =============================================================================

/// Describes the type of decision protocol required.
///
/// This enum does not select a specific consensus algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsensusMode {
    /// One authoritative participant makes the decision.
    Authoritative,

    /// Participants cooperate under an externally supplied decision rule.
    Cooperative,

    /// A distributed state transition requires an explicit agreement rule.
    Agreement,

    /// The implementation chooses the appropriate protocol.
    Automatic,
}

impl Default for ConsensusMode {
    fn default() -> Self {
        Self::Automatic
    }
}

// =============================================================================
// Vote
// =============================================================================

/// Participant vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Vote {
    /// Participant accepts the proposal.
    Accept,

    /// Participant rejects the proposal.
    Reject,

    /// Participant explicitly abstains.
    Abstain,
}

impl Vote {
    /// Returns whether the vote is affirmative.
    #[must_use]
    pub const fn is_accept(self) -> bool {
        matches!(self, Self::Accept)
    }

    /// Returns whether the vote is negative.
    #[must_use]
    pub const fn is_reject(self) -> bool {
        matches!(self, Self::Reject)
    }

    /// Returns whether the vote is an abstention.
    #[must_use]
    pub const fn is_abstain(self) -> bool {
        matches!(self, Self::Abstain)
    }
}

// =============================================================================
// Proposal
// =============================================================================

/// Immutable proposal submitted to a consensus protocol.
///
/// `value_digest` identifies the exact semantic operation/value being proposed.
///
/// The actual operation payload remains owned by the subsystem performing the
/// operation. Consensus should not duplicate quantum programs, IR, schedules or
/// hardware state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusProposal {
    consensus_id: ConsensusId,
    proposal_id: ProposalId,
    execution_id: ExecutionId,
    epoch: ConsensusEpoch,
    mode: ConsensusMode,
    value_digest: ProposalDigest,
    participants: Vec<ParticipantId>,
    authority_proof: Option<AuthorityProof>,
}

impl ConsensusProposal {
    /// Creates a proposal.
    pub fn new(
        consensus_id: ConsensusId,
        proposal_id: ProposalId,
        execution_id: ExecutionId,
        epoch: ConsensusEpoch,
        mode: ConsensusMode,
        value_digest: ProposalDigest,
        participants: Vec<ParticipantId>,
        authority_proof: Option<AuthorityProof>,
    ) -> ResilienceResult<Self> {
        let proposal = Self {
            consensus_id,
            proposal_id,
            execution_id,
            epoch,
            mode,
            value_digest,
            participants,
            authority_proof,
        };

        proposal.validate()?;

        Ok(proposal)
    }

    /// Validates the proposal.
    pub fn validate(&self) -> ResilienceResult<()> {
        if self.participants.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::MissingInformation,
                "consensus proposal must contain at least one participant",
            ));
        }

        for participant in &self.participants {
            validate_identifier(
                participant.as_str(),
                "participant identifier",
            )?;
        }

        for index in 0..self.participants.len() {
            for other in (index + 1)..self.participants.len() {
                if self.participants[index] == self.participants[other] {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::InvalidArgument,
                        "consensus proposal contains duplicate participants",
                    ));
                }
            }
        }

        Ok(())
    }

    /// Consensus operation identity.
    #[must_use]
    pub fn consensus_id(&self) -> &ConsensusId {
        &self.consensus_id
    }

    /// Proposal identity.
    #[must_use]
    pub fn proposal_id(&self) -> &ProposalId {
        &self.proposal_id
    }

    /// Execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Consensus epoch.
    #[must_use]
    pub const fn epoch(&self) -> ConsensusEpoch {
        self.epoch
    }

    /// Consensus mode.
    #[must_use]
    pub const fn mode(&self) -> ConsensusMode {
        self.mode
    }

    /// Semantic value digest.
    #[must_use]
    pub fn value_digest(&self) -> &ProposalDigest {
        &self.value_digest
    }

    /// Participant set.
    #[must_use]
    pub fn participants(&self) -> &[ParticipantId] {
        &self.participants
    }

    /// Optional authority proof.
    #[must_use]
    pub fn authority_proof(&self) -> Option<&AuthorityProof> {
        self.authority_proof.as_ref()
    }

    /// Returns whether the participant is part of the proposal.
    #[must_use]
    pub fn contains_participant(&self, participant: &ParticipantId) -> bool {
        self.participants.iter().any(|candidate| candidate == participant)
    }
}

// =============================================================================
// Participant vote
// =============================================================================

/// A single participant's vote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParticipantVote {
    consensus_id: ConsensusId,
    proposal_id: ProposalId,
    execution_id: ExecutionId,
    participant: ParticipantId,
    epoch: ConsensusEpoch,
    sequence: ConsensusSequence,
    vote: Vote,
    evidence: Option<EvidenceDigest>,
}

impl ParticipantVote {
    /// Creates a participant vote.
    pub fn new(
        consensus_id: ConsensusId,
        proposal_id: ProposalId,
        execution_id: ExecutionId,
        participant: ParticipantId,
        epoch: ConsensusEpoch,
        sequence: ConsensusSequence,
        vote: Vote,
        evidence: Option<EvidenceDigest>,
    ) -> ResilienceResult<Self> {
        Ok(Self {
            consensus_id,
            proposal_id,
            execution_id,
            participant,
            epoch,
            sequence,
            vote,
            evidence,
        })
    }

    /// Validates this vote against a proposal.
    pub fn validate_against(
        &self,
        proposal: &ConsensusProposal,
    ) -> ResilienceResult<()> {
        if self.consensus_id != *proposal.consensus_id() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ConcurrencyConflict,
                "vote belongs to a different consensus operation",
            ));
        }

        if self.proposal_id != *proposal.proposal_id() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ConcurrencyConflict,
                "vote belongs to a different proposal",
            ));
        }

        if self.execution_id != *proposal.execution_id() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ConcurrencyConflict,
                "vote belongs to a different execution",
            ));
        }

        if self.epoch != proposal.epoch() {
            return Err(ResilienceError::ConcurrencyConflict.into());
        }

        if !proposal.contains_participant(&self.participant) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::AuthorizationFailed,
                "voting participant is not part of the proposal",
            ));
        }

        Ok(())
    }

    /// Consensus operation identity.
    #[must_use]
    pub fn consensus_id(&self) -> &ConsensusId {
        &self.consensus_id
    }

    /// Proposal identity.
    #[must_use]
    pub fn proposal_id(&self) -> &ProposalId {
        &self.proposal_id
    }

    /// Execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Participant identity.
    #[must_use]
    pub fn participant(&self) -> &ParticipantId {
        &self.participant
    }

    /// Epoch.
    #[must_use]
    pub const fn epoch(&self) -> ConsensusEpoch {
        self.epoch
    }

    /// Vote sequence.
    #[must_use]
    pub const fn sequence(&self) -> ConsensusSequence {
        self.sequence
    }

    /// Vote.
    #[must_use]
    pub const fn vote(&self) -> Vote {
        self.vote
    }

    /// Optional evidence digest.
    #[must_use]
    pub fn evidence(&self) -> Option<&EvidenceDigest> {
        self.evidence.as_ref()
    }
}

// =============================================================================
// Decision status
// =============================================================================

/// Result of applying a consensus decision policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsensusStatus {
    /// Consensus has been established.
    Committed,

    /// Consensus has definitively rejected the proposal.
    Rejected,

    /// More information is required before a decision can be made.
    Pending,

    /// The proposal is no longer valid for the current protocol generation.
    Stale,

    /// The protocol cannot safely establish a decision.
    Failed,
}

impl ConsensusStatus {
    /// Whether this status represents a final decision.
    #[must_use]
    pub const fn is_final(self) -> bool {
        matches!(
            self,
            Self::Committed | Self::Rejected | Self::Stale | Self::Failed
        )
    }

    /// Whether the proposal may safely be committed.
    #[must_use]
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::Committed)
    }
}

// =============================================================================
// Decision evidence
// =============================================================================

/// Structured summary of evidence used to establish a decision.
///
/// Counts are descriptive only. They do not themselves define a quorum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionEvidence {
    participants_considered: usize,
    accepts: usize,
    rejects: usize,
    abstentions: usize,
    sequence: ConsensusSequence,
}

impl DecisionEvidence {
    /// Creates decision evidence.
    pub fn new(
        participants_considered: usize,
        accepts: usize,
        rejects: usize,
        abstentions: usize,
        sequence: ConsensusSequence,
    ) -> ResilienceResult<Self> {
        let total = accepts
            .checked_add(rejects)
            .and_then(|value| value.checked_add(abstentions))
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "consensus vote-count arithmetic overflow",
                )
            })?;

        if total > participants_considered {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvariantViolation,
                "vote counts exceed participants considered",
            ));
        }

        Ok(Self {
            participants_considered,
            accepts,
            rejects,
            abstentions,
            sequence,
        })
    }

    /// Number of participants considered by the decision policy.
    #[must_use]
    pub const fn participants_considered(&self) -> usize {
        self.participants_considered
    }

    /// Number of affirmative votes.
    #[must_use]
    pub const fn accepts(&self) -> usize {
        self.accepts
    }

    /// Number of negative votes.
    #[must_use]
    pub const fn rejects(&self) -> usize {
        self.rejects
    }

    /// Number of abstentions.
    #[must_use]
    pub const fn abstentions(&self) -> usize {
        self.abstentions
    }

    /// Highest/authoritative protocol sequence represented by the evidence.
    #[must_use]
    pub const fn sequence(&self) -> ConsensusSequence {
        self.sequence
    }
}

// =============================================================================
// Consensus decision
// =============================================================================

/// Immutable consensus decision.
///
/// A committed decision means only that the configured consensus protocol
/// accepted the proposal. It does NOT mean that the quantum operation or its
/// result is semantically correct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusDecision {
    consensus_id: ConsensusId,
    proposal_id: ProposalId,
    execution_id: ExecutionId,
    epoch: ConsensusEpoch,
    status: ConsensusStatus,
    value_digest: ProposalDigest,
    evidence: DecisionEvidence,
}

impl ConsensusDecision {
    /// Creates a decision.
    pub fn new(
        proposal: &ConsensusProposal,
        status: ConsensusStatus,
        evidence: DecisionEvidence,
    ) -> ResilienceResult<Self> {
        if status == ConsensusStatus::Committed
            && evidence.accepts() == 0
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvariantViolation,
                "committed consensus decision requires affirmative evidence",
            ));
        }

        Ok(Self {
            consensus_id: proposal.consensus_id().clone(),
            proposal_id: proposal.proposal_id().clone(),
            execution_id: proposal.execution_id().clone(),
            epoch: proposal.epoch(),
            status,
            value_digest: proposal.value_digest().clone(),
            evidence,
        })
    }

    /// Consensus operation identity.
    #[must_use]
    pub fn consensus_id(&self) -> &ConsensusId {
        &self.consensus_id
    }

    /// Proposal identity.
    #[must_use]
    pub fn proposal_id(&self) -> &ProposalId {
        &self.proposal_id
    }

    /// Execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Consensus epoch.
    #[must_use]
    pub const fn epoch(&self) -> ConsensusEpoch {
        self.epoch
    }

    /// Decision status.
    #[must_use]
    pub const fn status(&self) -> ConsensusStatus {
        self.status
    }

    /// Whether consensus committed the proposal.
    #[must_use]
    pub const fn is_committed(&self) -> bool {
        self.status.is_committed()
    }

    /// Proposal value digest.
    #[must_use]
    pub fn value_digest(&self) -> &ProposalDigest {
        &self.value_digest
    }

    /// Decision evidence.
    #[must_use]
    pub fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }
}

// =============================================================================
// Consensus capabilities
// =============================================================================

/// Capabilities advertised by a concrete consensus implementation.
///
/// These capabilities describe protocol behavior rather than hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsensusCapabilities {
    /// Whether the implementation supports durable/idempotent proposal replay.
    pub idempotent_replay: bool,

    /// Whether the implementation supports explicit epochs.
    pub epoch_fencing: bool,

    /// Whether the implementation supports proposal/value digests.
    pub proposal_integrity: bool,

    /// Whether the implementation can distinguish stale proposals.
    pub stale_proposal_rejection: bool,

    /// Whether the implementation supports explicit cancellation.
    pub cancellation: bool,

    /// Whether the implementation supports caller-supplied deadlines.
    pub deadlines: bool,

    /// Whether the implementation can provide deterministic decisions when
    /// configured for deterministic execution.
    pub deterministic_decision: bool,

    /// Whether the implementation can tolerate participant failure without
    /// violating its configured safety rule.
    pub fault_tolerant_participants: bool,
}

// =============================================================================
// Consensus request
// =============================================================================

/// Request to evaluate a proposal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusRequest {
    proposal: ConsensusProposal,
    votes: Vec<ParticipantVote>,
}

impl ConsensusRequest {
    /// Creates a consensus request.
    pub fn new(
        proposal: ConsensusProposal,
        votes: Vec<ParticipantVote>,
    ) -> ResilienceResult<Self> {
        let request = Self { proposal, votes };

        request.validate()?;

        Ok(request)
    }

    /// Validates the entire request.
    pub fn validate(&self) -> ResilienceResult<()> {
        self.proposal.validate()?;

        for vote in &self.votes {
            vote.validate_against(&self.proposal)?;
        }

        for index in 0..self.votes.len() {
            for other in (index + 1)..self.votes.len() {
                if self.votes[index].participant()
                    == self.votes[other].participant()
                {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::ConcurrencyConflict,
                        "multiple conflicting votes supplied for one participant",
                    ));
                }
            }
        }

        Ok(())
    }

    /// Proposal being evaluated.
    #[must_use]
    pub fn proposal(&self) -> &ConsensusProposal {
        &self.proposal
    }

    /// Supplied votes.
    #[must_use]
    pub fn votes(&self) -> &[ParticipantVote] {
        &self.votes
    }
}

// =============================================================================
// Decision policy
// =============================================================================

/// Pluggable decision rule.
///
/// This is the most important scalability boundary in this file.
///
/// Consensus does NOT assume:
///
/// - simple majority;
/// - two-thirds;
/// - weighted voting;
/// - fixed quorum;
/// - leader-based consensus;
/// - crash fault tolerance;
/// - Byzantine fault tolerance.
///
/// The concrete policy decides what constitutes safe agreement.
///
/// Implementations MUST NOT return `Committed` unless their own safety
/// requirements have been satisfied.
pub trait ConsensusDecisionPolicy: Send + Sync {
    /// Evaluates the supplied proposal and votes.
    fn decide(
        &self,
        request: &ConsensusRequest,
    ) -> ResilienceResult<ConsensusDecision>;
}

// =============================================================================
// Consensus protocol
// =============================================================================

/// Provider-neutral consensus protocol.
///
/// Concrete implementations belong in the distributed coordination layer or
/// an adapter beneath it.
///
/// Examples of possible implementations include:
///
/// - local deterministic consensus;
/// - process-level coordination;
/// - replicated state-machine consensus;
/// - Byzantine-fault-tolerant consensus;
/// - quorum services;
/// - externally managed coordination services.
///
/// No algorithm is embedded in this contract.
pub trait ConsensusProtocol: Send + Sync {
    /// Returns protocol capabilities.
    fn capabilities(&self) -> ConsensusCapabilities;

    /// Starts or registers a proposal.
    ///
    /// This operation must be idempotent with respect to `consensus_id` and
    /// `proposal_id`, according to the implementation's documented semantics.
    fn propose(
        &self,
        proposal: &ConsensusProposal,
    ) -> ResilienceResult<()>;

    /// Records a participant vote.
    ///
    /// Implementations MUST reject stale epochs and votes from participants
    /// that are not authorized for the proposal.
    fn record_vote(
        &self,
        vote: &ParticipantVote,
    ) -> ResilienceResult<()>;

    /// Obtains the current decision.
    ///
    /// `Pending` is a valid result when the protocol cannot yet safely decide.
    fn decide(
        &self,
        consensus_id: &ConsensusId,
        proposal_id: &ProposalId,
        epoch: ConsensusEpoch,
    ) -> ResilienceResult<ConsensusDecision>;

    /// Cancels an undecided consensus operation.
    ///
    /// Cancellation must never transform a committed decision into a
    /// non-committed state.
    fn cancel(
        &self,
        consensus_id: &ConsensusId,
        proposal_id: &ProposalId,
        epoch: ConsensusEpoch,
    ) -> ResilienceResult<()>;
}

// =============================================================================
// Protocol coordinator
// =============================================================================

/// Stateless facade that validates consensus requests before passing them to a
/// concrete protocol.
///
/// This type deliberately does not:
///
/// - calculate quorum;
/// - retry;
/// - communicate with participants;
/// - elect leaders;
/// - mutate quantum state.
pub struct ConsensusCoordinator<P> {
    protocol: P,
}

impl<P> ConsensusCoordinator<P>
where
    P: ConsensusProtocol,
{
    /// Creates a consensus coordinator around a concrete protocol.
    pub fn new(protocol: P) -> Self {
        Self { protocol }
    }

    /// Returns protocol capabilities.
    #[must_use]
    pub fn capabilities(&self) -> ConsensusCapabilities {
        self.protocol.capabilities()
    }

    /// Validates and submits a proposal.
    pub fn propose(
        &self,
        proposal: &ConsensusProposal,
    ) -> ResilienceResult<()> {
        proposal.validate()?;

        self.protocol.propose(proposal)
    }

    /// Validates and records a vote.
    pub fn record_vote(
        &self,
        vote: &ParticipantVote,
        proposal: &ConsensusProposal,
    ) -> ResilienceResult<()> {
        vote.validate_against(proposal)?;

        self.protocol.record_vote(vote)
    }

    /// Obtains the authoritative protocol decision.
    pub fn decide(
        &self,
        consensus_id: &ConsensusId,
        proposal_id: &ProposalId,
        epoch: ConsensusEpoch,
    ) -> ResilienceResult<ConsensusDecision> {
        self.protocol
            .decide(consensus_id, proposal_id, epoch)
    }

    /// Cancels an undecided proposal.
    pub fn cancel(
        &self,
        consensus_id: &ConsensusId,
        proposal_id: &ProposalId,
        epoch: ConsensusEpoch,
    ) -> ResilienceResult<()> {
        self.protocol
            .cancel(consensus_id, proposal_id, epoch)
    }
}

// =============================================================================
// Deterministic vote aggregation helper
// =============================================================================

/// Aggregates votes into deterministic evidence.
///
/// IMPORTANT:
///
/// This helper deliberately does NOT determine whether a quorum exists.
///
/// It only:
///
/// - validates the votes;
/// - rejects duplicate participants;
/// - counts vote categories;
/// - determines the highest supplied sequence.
///
/// A separate `ConsensusDecisionPolicy` must decide whether those votes are
/// sufficient for commitment.
pub fn aggregate_vote_evidence(
    request: &ConsensusRequest,
) -> ResilienceResult<DecisionEvidence> {
    request.validate()?;

    let mut accepts = 0usize;
    let mut rejects = 0usize;
    let mut abstentions = 0usize;
    let mut highest_sequence = ConsensusSequence::initial();

    for vote in request.votes() {
        match vote.vote() {
            Vote::Accept => {
                accepts = accepts.checked_add(1).ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "accept vote count overflow",
                    )
                })?;
            }
            Vote::Reject => {
                rejects = rejects.checked_add(1).ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "reject vote count overflow",
                    )
                })?;
            }
            Vote::Abstain => {
                abstentions = abstentions.checked_add(1).ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "abstention count overflow",
                    )
                })?;
            }
        }

        if vote.sequence() > highest_sequence {
            highest_sequence = vote.sequence();
        }
    }

    DecisionEvidence::new(
        request.proposal().participants().len(),
        accepts,
        rejects,
        abstentions,
        highest_sequence,
    )
}

// =============================================================================
// Proposal compatibility
// =============================================================================

/// Validates that a decision belongs to the expected proposal.
///
/// This helper is intentionally strict because accepting a valid decision for
/// the wrong proposal would be a catastrophic distributed-state error.
pub fn validate_decision_for_proposal(
    decision: &ConsensusDecision,
    proposal: &ConsensusProposal,
) -> ResilienceResult<()> {
    if decision.consensus_id() != proposal.consensus_id() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ConcurrencyConflict,
            "decision belongs to a different consensus operation",
        ));
    }

    if decision.proposal_id() != proposal.proposal_id() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ConcurrencyConflict,
            "decision belongs to a different proposal",
        ));
    }

    if decision.execution_id() != proposal.execution_id() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ConcurrencyConflict,
            "decision belongs to a different execution",
        ));
    }

    if decision.epoch() != proposal.epoch() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ConcurrencyConflict,
            "decision belongs to a different consensus epoch",
        ));
    }

    if decision.value_digest() != proposal.value_digest() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::IntegrityFailure,
            "decision value does not match proposal value",
        ));
    }

    Ok(())
}

// =============================================================================
// Safety validation
// =============================================================================

/// Performs the minimum safety validation required before a committed decision
/// can be passed to another coordination layer.
///
/// This function intentionally does not verify the quantum computation itself.
///
/// The verification subsystem must still validate:
///
/// - program semantics;
/// - canonical IR;
/// - routing;
/// - scheduling;
/// - execution result;
/// - QEC result;
/// - mitigation effects;
/// - provenance.
pub fn validate_committed_decision(
    decision: &ConsensusDecision,
    proposal: &ConsensusProposal,
) -> ResilienceResult<()> {
    validate_decision_for_proposal(decision, proposal)?;

    if !decision.is_committed() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidState,
            "only a committed consensus decision may cross the commit boundary",
        ));
    }

    if decision.evidence().accepts() == 0 {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvariantViolation,
            "committed decision has no affirmative evidence",
        ));
    }

    Ok(())
}

// =============================================================================
// Epoch validation
// =============================================================================

/// Rejects an operation using an older consensus epoch.
pub fn validate_epoch(
    expected: ConsensusEpoch,
    actual: ConsensusEpoch,
) -> ResilienceResult<()> {
    if actual < expected {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ConcurrencyConflict,
            "stale consensus epoch",
        ));
    }

    if actual > expected {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ResourceStateChanged,
            "consensus operation targets an older coordination generation",
        ));
    }

    Ok(())
}

// =============================================================================
// Participant-set validation
// =============================================================================

/// Validates that a supplied participant collection exactly matches the
/// proposal's participant set.
///
/// Ordering does not matter.
///
/// The caller should use this before applying a protocol-specific quorum or
/// membership rule.
pub fn validate_participant_set(
    proposal: &ConsensusProposal,
    participants: &[ParticipantId],
) -> ResilienceResult<()> {
    if participants.len() != proposal.participants().len() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::ConcurrencyConflict,
            "participant set does not match proposal",
        ));
    }

    for participant in proposal.participants() {
        if !participants.iter().any(|candidate| candidate == participant) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ConcurrencyConflict,
                "participant set is missing a proposal participant",
            ));
        }
    }

    for index in 0..participants.len() {
        for other in (index + 1)..participants.len() {
            if participants[index] == participants[other] {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                    "participant set contains duplicate identities",
                ));
            }
        }
    }

    Ok(())
}

// =============================================================================
// Protocol requirements
// =============================================================================

/// Declares requirements that a caller can impose on a consensus protocol.
///
/// This allows the resilience policy to reject a protocol that is incapable of
/// meeting the required safety properties before an operation begins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsensusRequirements {
    /// Epoch fencing must be supported.
    pub require_epoch_fencing: bool,

    /// Proposal integrity/digest binding must be supported.
    pub require_proposal_integrity: bool,

    /// Stale proposals must be rejected.
    pub require_stale_rejection: bool,

    /// Deterministic decisions are required.
    pub require_determinism: bool,

    /// Idempotent replay is required.
    pub require_idempotent_replay: bool,

    /// Participant fault tolerance is required.
    pub require_fault_tolerance: bool,

    /// Cancellation is required.
    pub require_cancellation: bool,

    /// Caller-supplied deadlines are required.
    pub require_deadlines: bool,
}

impl ConsensusRequirements {
    /// Validates protocol capabilities against these requirements.
    pub fn validate(
        self,
        capabilities: ConsensusCapabilities,
    ) -> ResilienceResult<()> {
        if self.require_epoch_fencing
            && !capabilities.epoch_fencing
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not support epoch fencing",
            ));
        }

        if self.require_proposal_integrity
            && !capabilities.proposal_integrity
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not support proposal integrity",
            ));
        }

        if self.require_stale_rejection
            && !capabilities.stale_proposal_rejection
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not reject stale proposals",
            ));
        }

        if self.require_determinism
            && !capabilities.deterministic_decision
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not provide deterministic decisions",
            ));
        }

        if self.require_idempotent_replay
            && !capabilities.idempotent_replay
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not support idempotent replay",
            ));
        }

        if self.require_fault_tolerance
            && !capabilities.fault_tolerant_participants
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not provide required participant fault tolerance",
            ));
        }

        if self.require_cancellation
            && !capabilities.cancellation
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not support cancellation",
            ));
        }

        if self.require_deadlines && !capabilities.deadlines {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "consensus protocol does not support caller-supplied deadlines",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Default requirements
// =============================================================================

impl Default for ConsensusRequirements {
    fn default() -> Self {
        Self {
            require_epoch_fencing: true,
            require_proposal_integrity: true,
            require_stale_rejection: true,
            require_determinism: false,
            require_idempotent_replay: true,
            require_fault_tolerance: false,
            require_cancellation: false,
            require_deadlines: false,
        }
    }
}

// =============================================================================
// Consensus lifecycle
// =============================================================================

/// Lifecycle state of a consensus operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsensusLifecycle {
    /// Proposal has not yet been submitted.
    Created,

    /// Proposal has been submitted.
    Proposed,

    /// Votes are being collected.
    Voting,

    /// A decision is available.
    Decided,

    /// Decision has crossed the commit boundary.
    Committed,

    /// Operation has been rejected.
    Rejected,

    /// Operation was cancelled before commitment.
    Cancelled,

    /// Operation became stale because a newer epoch superseded it.
    Stale,
}

impl ConsensusLifecycle {
    /// Whether the lifecycle is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Committed
                | Self::Rejected
                | Self::Cancelled
                | Self::Stale
        )
    }
}

// =============================================================================
// Lifecycle transition validation
// =============================================================================

/// Validates a consensus lifecycle transition.
///
/// This function does not mutate state.
pub fn validate_lifecycle_transition(
    from: ConsensusLifecycle,
    to: ConsensusLifecycle,
) -> ResilienceResult<()> {
    let valid = match (from, to) {
        (ConsensusLifecycle::Created, ConsensusLifecycle::Proposed) => true,

        (ConsensusLifecycle::Proposed, ConsensusLifecycle::Voting) => true,

        (ConsensusLifecycle::Voting, ConsensusLifecycle::Voting) => true,

        (ConsensusLifecycle::Voting, ConsensusLifecycle::Decided) => true,

        (ConsensusLifecycle::Decided, ConsensusLifecycle::Committed) => true,

        (ConsensusLifecycle::Decided, ConsensusLifecycle::Rejected) => true,

        (ConsensusLifecycle::Proposed, ConsensusLifecycle::Cancelled) => true,

        (ConsensusLifecycle::Voting, ConsensusLifecycle::Cancelled) => true,

        (ConsensusLifecycle::Proposed, ConsensusLifecycle::Stale) => true,

        (ConsensusLifecycle::Voting, ConsensusLifecycle::Stale) => true,

        (ConsensusLifecycle::Decided, ConsensusLifecycle::Stale) => true,

        (ConsensusLifecycle::Committed, ConsensusLifecycle::Committed) => true,

        (ConsensusLifecycle::Rejected, ConsensusLifecycle::Rejected) => true,

        (ConsensusLifecycle::Cancelled, ConsensusLifecycle::Cancelled) => true,

        (ConsensusLifecycle::Stale, ConsensusLifecycle::Stale) => true,

        _ => false,
    };

    if valid {
        Ok(())
    } else {
        Err(ResilienceError::new(
            ResilienceErrorCode::InvalidState,
            format!(
                "invalid consensus lifecycle transition: {from:?} -> {to:?}"
            ),
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn consensus_id() -> ConsensusId {
        ConsensusId::new("consensus-1").expect("valid consensus id")
    }

    fn proposal_id() -> ProposalId {
        ProposalId::new("proposal-1").expect("valid proposal id")
    }

    fn execution_id() -> ExecutionId {
        ExecutionId::new("execution-1").expect("valid execution id")
    }

    fn participant(value: &str) -> ParticipantId {
        ParticipantId::new(value).expect("valid participant")
    }

    fn epoch() -> ConsensusEpoch {
        ConsensusEpoch::try_from_u64(1).expect("valid epoch")
    }

    fn digest() -> ProposalDigest {
        ProposalDigest::new("digest-1").expect("valid digest")
    }

    fn proposal() -> ConsensusProposal {
        ConsensusProposal::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            epoch(),
            ConsensusMode::Cooperative,
            digest(),
            vec![
                participant("participant-a"),
                participant("participant-b"),
            ],
            None,
        )
        .expect("valid proposal")
    }

    #[test]
    fn rejects_empty_identifiers() {
        assert!(ConsensusId::new("").is_err());
        assert!(ProposalId::new("   ").is_err());
        assert!(ExecutionId::new("").is_err());
        assert!(ParticipantId::new("").is_err());
        assert!(ProposalDigest::new("").is_err());
    }

    #[test]
    fn rejects_control_characters() {
        assert!(ParticipantId::new("worker\n1").is_err());
    }

    #[test]
    fn proposal_rejects_duplicate_participants() {
        let result = ConsensusProposal::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            epoch(),
            ConsensusMode::Cooperative,
            digest(),
            vec![participant("a"), participant("a")],
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn vote_must_belong_to_proposal() {
        let proposal = proposal();

        let vote = ParticipantVote::new(
            consensus_id(),
            ProposalId::new("different-proposal").expect("valid"),
            execution_id(),
            participant("participant-a"),
            epoch(),
            ConsensusSequence::initial(),
            Vote::Accept,
            None,
        )
        .expect("valid vote");

        assert!(vote.validate_against(&proposal).is_err());
    }

    #[test]
    fn unknown_participant_cannot_vote() {
        let proposal = proposal();

        let vote = ParticipantVote::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            participant("unknown"),
            epoch(),
            ConsensusSequence::initial(),
            Vote::Accept,
            None,
        )
        .expect("valid vote");

        assert!(vote.validate_against(&proposal).is_err());
    }

    #[test]
    fn duplicate_votes_are_rejected() {
        let proposal = proposal();

        let first = ParticipantVote::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            participant("participant-a"),
            epoch(),
            ConsensusSequence::initial(),
            Vote::Accept,
            None,
        )
        .expect("valid vote");

        let second = ParticipantVote::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            participant("participant-a"),
            epoch(),
            ConsensusSequence::initial(),
            Vote::Reject,
            None,
        )
        .expect("valid vote");

        let request = ConsensusRequest::new(
            proposal,
            vec![first, second],
        );

        assert!(request.is_err());
    }

    #[test]
    fn vote_evidence_is_aggregated_without_defining_quorum() {
        let proposal = proposal();

        let first = ParticipantVote::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            participant("participant-a"),
            epoch(),
            ConsensusSequence::initial(),
            Vote::Accept,
            None,
        )
        .expect("valid vote");

        let second = ParticipantVote::new(
            consensus_id(),
            proposal_id(),
            execution_id(),
            participant("participant-b"),
            epoch(),
            ConsensusSequence::initial(),
            Vote::Reject,
            None,
        )
        .expect("valid vote");

        let request =
            ConsensusRequest::new(proposal, vec![first, second])
                .expect("valid request");

        let evidence =
            aggregate_vote_evidence(&request).expect("valid evidence");

        assert_eq!(evidence.participants_considered(), 2);
        assert_eq!(evidence.accepts(), 1);
        assert_eq!(evidence.rejects(), 1);
        assert_eq!(evidence.abstentions(), 0);
    }

    #[test]
    fn committed_decision_must_have_affirmative_evidence() {
        let proposal = proposal();

        let evidence =
            DecisionEvidence::new(
                2,
                0,
                2,
                0,
                ConsensusSequence::initial(),
            )
            .expect("valid evidence");

        let result =
            ConsensusDecision::new(
                &proposal,
                ConsensusStatus::Committed,
                evidence,
            );

        assert!(result.is_err());
    }

    #[test]
    fn decision_must_match_proposal() {
        let proposal = proposal();

        let evidence =
            DecisionEvidence::new(
                2,
                2,
                0,
                0,
                ConsensusSequence::initial(),
            )
            .expect("valid evidence");

        let decision =
            ConsensusDecision::new(
                &proposal,
                ConsensusStatus::Committed,
                evidence,
            )
            .expect("valid decision");

        assert!(
            validate_decision_for_proposal(&decision, &proposal).is_ok()
        );
    }

    #[test]
    fn stale_epoch_is_rejected() {
        let expected =
            ConsensusEpoch::try_from_u64(2).expect("valid epoch");

        let actual =
            ConsensusEpoch::try_from_u64(1).expect("valid epoch");

        assert!(validate_epoch(expected, actual).is_err());
    }

    #[test]
    fn matching_epoch_is_accepted() {
        assert!(validate_epoch(epoch(), epoch()).is_ok());
    }

    #[test]
    fn epoch_advances_without_wrapping() {
        let epoch = epoch();

        let next = epoch.next().expect("epoch should advance");

        assert_eq!(next.get(), 2);
    }

    #[test]
    fn lifecycle_rejects_commit_without_decision() {
        assert!(
            validate_lifecycle_transition(
                ConsensusLifecycle::Voting,
                ConsensusLifecycle::Committed
            )
            .is_err()
        );
    }

    #[test]
    fn lifecycle_allows_decision_to_commit() {
        assert!(
            validate_lifecycle_transition(
                ConsensusLifecycle::Decided,
                ConsensusLifecycle::Committed
            )
            .is_ok()
        );
    }

    #[test]
    fn participant_set_validation_is_order_independent() {
        let proposal = proposal();

        let participants = vec![
            participant("participant-b"),
            participant("participant-a"),
        ];

        assert!(
            validate_participant_set(
                &proposal,
                &participants
            )
            .is_ok()
        );
    }

    #[test]
    fn participant_set_rejects_missing_participant() {
        let proposal = proposal();

        let participants = vec![participant("participant-a")];

        assert!(
            validate_participant_set(
                &proposal,
                &participants
            )
            .is_err()
        );
    }

    #[test]
    fn vote_types_have_expected_semantics() {
        assert!(Vote::Accept.is_accept());
        assert!(Vote::Reject.is_reject());
        assert!(Vote::Abstain.is_abstain());
    }

    #[test]
    fn requirements_can_validate_capabilities() {
        let requirements = ConsensusRequirements {
            require_epoch_fencing: true,
            require_proposal_integrity: true,
            require_stale_rejection: true,
            require_determinism: true,
            require_idempotent_replay: true,
            require_fault_tolerance: false,
            require_cancellation: false,
            require_deadlines: false,
        };

        let capabilities = ConsensusCapabilities {
            idempotent_replay: true,
            epoch_fencing: true,
            proposal_integrity: true,
            stale_proposal_rejection: true,
            cancellation: false,
            deadlines: false,
            deterministic_decision: true,
            fault_tolerant_participants: false,
        };

        assert!(requirements.validate(capabilities).is_ok());
    }
}