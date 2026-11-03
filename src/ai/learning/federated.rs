
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Federated Learning Contracts
//!
//! Path: `src/ai/learning/federated.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent contracts for federated learning.
//!
//! Federated learning describes coordinated learning across logically
//! independent participants. It does not require the source program to
//! identify physical machines, devices, network addresses, or execution
//! providers.
//!
//! This module describes:
//!
//! - Federated-learning problem definitions.
//! - Logical federation identities.
//! - Logical participant identities and roles.
//! - Local training references.
//! - Model and dataset references.
//! - Aggregation contracts.
//! - Communication requirements.
//! - Privacy and data-governance requirements.
//! - Recovery and failure-handling requirements.
//! - Federated phases and rounds.
//! - Reproducibility and provenance references.
//! - Structural validation.
//!
//! It does not execute federated learning.
//!
//! ## Ownership
//!
//! This module owns federated-learning-specific declarations and their
//! local structural invariants.
//!
//! ## Does not own
//!
//! - Source syntax or AST construction.
//! - Semantic name resolution.
//! - Canonical type checking.
//! - Generic training workflow definitions.
//! - Learning algorithm implementations.
//! - Tensor storage.
//! - Model parameter storage.
//! - Numerical aggregation kernels.
//! - Differential privacy implementations.
//! - Cryptographic implementations.
//! - Participant discovery or authentication.
//! - Distributed participant identity allocation.
//! - Network protocols or transport.
//! - Distributed scheduling or placement.
//! - Physical resource allocation.
//! - Checkpoint storage.
//! - Runtime execution.
//! - Backend implementations.
//!
//! ## Canonical integration
//!
//! AI errors:
//!     crate::ai::error
//!
//! AI identities:
//!     crate::ai::ids
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! Generic training workflows:
//!     crate::ai::learning::training
//!
//! Learning algorithms:
//!     crate::ai::learning::algorithms
//!
//! Distributed identities:
//!     crate::distributed::ids
//!
//! Distributed participant descriptors:
//!     crate::distributed::model::participant
//!
//! Federated-learning source syntax:
//!     grammar/ai/federated-learning.g4
//!
//! AI grammar composition:
//!     grammar/ai/ai.g4
//!
//! Generic training syntax:
//!     grammar/ai/training.g4
//!
//! Actual execution:
//!     compiler / distributed / runtime / backend
//!
//! ## POCO-REAF
//!
//! Federated declarations describe computational intent rather than
//! physical execution topology.
//!
//! No universal maximum is imposed on:
//!
//! - Federations.
//! - Participants.
//! - Participant groups.
//! - Rounds.
//! - Phases.
//! - Models.
//! - Datasets.
//! - Aggregation inputs.
//! - Local training operations.
//! - Policies.
//! - Requirements.
//! - Capabilities.
//! - Distributed workers.
//! - Execution nodes.
//! - Devices.
//! - Accelerators.
//!
//! Actual capacity is determined by available resources and explicit
//! operational policies.
//!
//! Literal infinite computation is not promised.
//!
//! ## Important semantic distinctions
//!
//! A participant reference is not proof that a participant exists,
//! is currently reachable, is authenticated, or is authorized.
//!
//! A privacy requirement is not proof that a privacy mechanism has
//! been implemented or correctly enforced.
//!
//! An aggregation reference is not an aggregation implementation.
//!
//! Structural validity is not execution feasibility.
//!
//! Registration of a federation is not evidence that its participants
//! have successfully completed a training round.
//!
//! ## Freeze criteria
//!
//! - Generic training contracts are reused.
//! - AI model and dataset identities are reused.
//! - Participant identities remain logical and target-independent.
//! - Aggregation algorithms are open-world references.
//! - Privacy mechanisms are not hard-coded.
//! - Communication and recovery are declarative.
//! - Round and phase references are validated.
//! - Cross-reference consistency is checked.
//! - Structural validity is distinguished from feasibility.
//! - No arbitrary federation-size ceilings exist.
//! - No unsafe Rust is used.

use core::fmt;
use std::collections::BTreeSet;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::{
    AiDatasetId,
    AiModelId,
};

use crate::ai::requirements::AiRequirements;

use crate::ai::learning::training::{
    TrainingReproducibilityContract,
    TrainingValueRef,
};

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_text(
    description: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(
            AiError::new(
                AiErrorKind::InvalidInput,
                format!("{description} cannot be empty"),
            )
            .with_phase(AiErrorPhase::Validation),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            AiError::new(
                AiErrorKind::InvalidInput,
                format!("{description} contains control characters"),
            )
            .with_phase(AiErrorPhase::Validation),
        );
    }

    Ok(())
}

fn invalid_federated(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::InvalidInput,
        message,
    )
    .with_phase(AiErrorPhase::Validation)
}

fn validate_reference(
    description: &str,
    reference: &TrainingValueRef,
) -> AiResult<()> {
    validate_text(
        description,
        reference.as_str(),
    )
}

fn validate_unique<T>(
    values: &[T],
    description: &str,
) -> AiResult<()>
where
    T: Ord + fmt::Display,
{
    let mut seen = BTreeSet::new();

    for value in values {
        if !seen.insert(value) {
            return Err(invalid_federated(format!(
                "duplicate {description}: {value}"
            )));
        }
    }

    Ok(())
}

// ============================================================================
// FEDERATION IDENTITY
// ============================================================================

/// Identity of a logical federated-learning declaration.
///
/// This is a semantic identity, not a physical federation address,
/// network endpoint, distributed namespace, or execution identifier.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct FederatedLearningId(String);

impl FederatedLearningId {
    /// Creates a validated federation identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "federated-learning identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FederatedLearningId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PARTICIPANT REFERENCES
// ============================================================================

/// Logical reference to a federated-learning participant.
///
/// This is deliberately separate from the distributed runtime's
/// `ParticipantId`.
///
/// A source-level participant reference identifies a logical role in a
/// federation. A distributed participant ID identifies a participant
/// in a particular distributed execution or membership context.
///
/// Conversion between them belongs to an explicit integration adapter.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct FederatedParticipantRef(String);

impl FederatedParticipantRef {
    /// Creates a validated logical participant reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "federated participant reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the logical participant reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FederatedParticipantRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PARTICIPANT ROLES
// ============================================================================

/// Open-world logical role of a federated participant.
///
/// Examples include coordinator, contributor, evaluator, aggregator,
/// validator, or application-defined roles.
///
/// These are descriptive roles, not a closed list of supported
/// participant types.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct FederatedParticipantRole(String);

impl FederatedParticipantRole {
    /// Creates a validated participant role.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "federated participant role",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the role name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FederatedParticipantRole {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PARTICIPANT CONTRACT
// ============================================================================

/// Declarative contract for one logical federation participant.
///
/// The participant may represent a user, organization, service, process,
/// execution context, agent, or another application-defined entity.
///
/// Physical realization and membership verification are downstream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedParticipant {
    identity: FederatedParticipantRef,
    role: FederatedParticipantRole,
    model: Option<AiModelId>,
    dataset: Option<AiDatasetId>,
    local_training: TrainingValueRef,
    consent_policy: Option<TrainingValueRef>,
    requirements: AiRequirements,
}

impl FederatedParticipant {
    /// Creates a participant declaration.
    pub fn new(
        identity: FederatedParticipantRef,
        role: FederatedParticipantRole,
        local_training: TrainingValueRef,
    ) -> AiResult<Self> {
        let participant = Self {
            identity,
            role,
            model: None,
            dataset: None,
            local_training,
            consent_policy: None,
            requirements: AiRequirements::new(),
        };

        participant.validate()?;

        Ok(participant)
    }

    /// Associates the canonical AI model identity.
    #[must_use]
    pub fn with_model(
        mut self,
        model: AiModelId,
    ) -> Self {
        self.model = Some(model);
        self
    }

    /// Associates the canonical AI dataset identity.
    #[must_use]
    pub fn with_dataset(
        mut self,
        dataset: AiDatasetId,
    ) -> Self {
        self.dataset = Some(dataset);
        self
    }

    /// Associates a consent or participation-policy reference.
    pub fn with_consent_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "participant consent policy",
            &policy,
        )?;

        self.consent_policy = Some(policy);
        Ok(self)
    }

    /// Replaces the participant's AI requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Returns the logical participant identity.
    #[must_use]
    pub fn identity(&self) -> &FederatedParticipantRef {
        &self.identity
    }

    /// Returns the logical participant role.
    #[must_use]
    pub fn role(&self) -> &FederatedParticipantRole {
        &self.role
    }

    /// Returns the optional model identity.
    #[must_use]
    pub const fn model(&self) -> Option<AiModelId> {
        self.model
    }

    /// Returns the optional dataset identity.
    #[must_use]
    pub const fn dataset(&self) -> Option<AiDatasetId> {
        self.dataset
    }

    /// Returns the local training workflow reference.
    #[must_use]
    pub fn local_training(&self) -> &TrainingValueRef {
        &self.local_training
    }

    /// Returns the optional consent policy.
    #[must_use]
    pub fn consent_policy(&self) -> Option<&TrainingValueRef> {
        self.consent_policy.as_ref()
    }

    /// Returns participant requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Validates the participant's local declaration.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "participant identity",
            self.identity.as_str(),
        )?;

        validate_text(
            "participant role",
            self.role.as_str(),
        )?;

        validate_reference(
            "participant local training",
            &self.local_training,
        )?;

        if let Some(policy) = &self.consent_policy {
            validate_reference(
                "participant consent policy",
                policy,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// AGGREGATION CONTRACT
// ============================================================================

/// Participation semantics for an aggregation operation.
///
/// This describes which logical participant contributions are eligible.
/// It does not implement participant selection or aggregation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
#[non_exhaustive]
pub enum AggregationParticipation {
    /// Aggregation requires every participant declared for the round.
    AllDeclaredParticipants,

    /// Aggregation may use participants whose contributions are available
    /// and satisfy the declared policies.
    AvailableParticipants,

    /// Eligibility is determined by an externally defined quorum policy.
    QuorumDefined,

    /// Participation semantics are defined by an extension.
    ExtensionDefined,
}

/// Declarative aggregation contract.
///
/// The algorithm reference is open-world. It may identify a library
/// implementation, registered learning algorithm, custom operation,
/// or future aggregation mechanism.
///
/// This structure does not execute the algorithm.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedAggregationContract {
    algorithm: TrainingValueRef,
    input: TrainingValueRef,
    output: TrainingValueRef,
    participation: AggregationParticipation,
    policy: Option<TrainingValueRef>,
}

impl FederatedAggregationContract {
    /// Creates an aggregation contract.
    pub fn new(
        algorithm: TrainingValueRef,
        input: TrainingValueRef,
        output: TrainingValueRef,
        participation: AggregationParticipation,
    ) -> AiResult<Self> {
        let contract = Self {
            algorithm,
            input,
            output,
            participation,
            policy: None,
        };

        contract.validate()?;

        Ok(contract)
    }

    /// Associates an aggregation policy reference.
    pub fn with_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "aggregation policy",
            &policy,
        )?;

        self.policy = Some(policy);
        Ok(self)
    }

    /// Returns the aggregation algorithm reference.
    #[must_use]
    pub fn algorithm(&self) -> &TrainingValueRef {
        &self.algorithm
    }

    /// Returns the aggregation input reference.
    #[must_use]
    pub fn input(&self) -> &TrainingValueRef {
        &self.input
    }

    /// Returns the aggregation output reference.
    #[must_use]
    pub fn output(&self) -> &TrainingValueRef {
        &self.output
    }

    /// Returns the participation semantics.
    #[must_use]
    pub const fn participation(&self) -> AggregationParticipation {
        self.participation
    }

    /// Returns the optional aggregation policy.
    #[must_use]
    pub fn policy(&self) -> Option<&TrainingValueRef> {
        self.policy.as_ref()
    }

    /// Validates the aggregation contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(
            "aggregation algorithm",
            &self.algorithm,
        )?;

        validate_reference(
            "aggregation input",
            &self.input,
        )?;

        validate_reference(
            "aggregation output",
            &self.output,
        )?;

        if let Some(policy) = &self.policy {
            validate_reference(
                "aggregation policy",
                policy,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// COMMUNICATION CONTRACT
// ============================================================================

/// Declarative federated communication contract.
///
/// Communication protocol identity, message delivery, authentication,
/// confidentiality and transport are resolved by their owning systems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedCommunicationContract {
    channel: TrainingValueRef,
    protocol: TrainingValueRef,
    confidentiality_policy: Option<TrainingValueRef>,
    integrity_policy: Option<TrainingValueRef>,
    data_locality_policy: Option<TrainingValueRef>,
}

impl FederatedCommunicationContract {
    /// Creates a communication contract.
    pub fn new(
        channel: TrainingValueRef,
        protocol: TrainingValueRef,
    ) -> AiResult<Self> {
        let contract = Self {
            channel,
            protocol,
            confidentiality_policy: None,
            integrity_policy: None,
            data_locality_policy: None,
        };

        contract.validate()?;

        Ok(contract)
    }

    /// Associates a confidentiality policy.
    pub fn with_confidentiality_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "communication confidentiality policy",
            &policy,
        )?;

        self.confidentiality_policy = Some(policy);
        Ok(self)
    }

    /// Associates an integrity policy.
    pub fn with_integrity_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "communication integrity policy",
            &policy,
        )?;

        self.integrity_policy = Some(policy);
        Ok(self)
    }

    /// Associates a data-locality policy.
    pub fn with_data_locality_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "data-locality policy",
            &policy,
        )?;

        self.data_locality_policy = Some(policy);
        Ok(self)
    }

    /// Returns the communication channel reference.
    #[must_use]
    pub fn channel(&self) -> &TrainingValueRef {
        &self.channel
    }

    /// Returns the protocol reference.
    #[must_use]
    pub fn protocol(&self) -> &TrainingValueRef {
        &self.protocol
    }

    /// Returns the confidentiality policy.
    #[must_use]
    pub fn confidentiality_policy(&self) -> Option<&TrainingValueRef> {
        self.confidentiality_policy.as_ref()
    }

    /// Returns the integrity policy.
    #[must_use]
    pub fn integrity_policy(&self) -> Option<&TrainingValueRef> {
        self.integrity_policy.as_ref()
    }

    /// Returns the data-locality policy.
    #[must_use]
    pub fn data_locality_policy(&self) -> Option<&TrainingValueRef> {
        self.data_locality_policy.as_ref()
    }

    /// Validates the communication contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(
            "communication channel",
            &self.channel,
        )?;

        validate_reference(
            "communication protocol",
            &self.protocol,
        )?;

        for policy in [
            self.confidentiality_policy.as_ref(),
            self.integrity_policy.as_ref(),
            self.data_locality_policy.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            validate_reference(
                "communication policy",
                policy,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// PRIVACY CONTRACT
// ============================================================================

/// Declarative privacy and data-governance contract.
///
/// Privacy mechanisms are identified by references rather than hard-coded
/// into this module. This allows future privacy technologies and policies
/// to be introduced without changing the core federated API.
///
/// A declaration does not prove that a privacy guarantee is achieved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedPrivacyContract {
    policy: TrainingValueRef,
    mechanism: TrainingValueRef,
    privacy_budget: Option<TrainingValueRef>,
    disclosure_policy: Option<TrainingValueRef>,
    data_retention_policy: Option<TrainingValueRef>,
}

impl FederatedPrivacyContract {
    /// Creates a privacy contract.
    pub fn new(
        policy: TrainingValueRef,
        mechanism: TrainingValueRef,
    ) -> AiResult<Self> {
        let contract = Self {
            policy,
            mechanism,
            privacy_budget: None,
            disclosure_policy: None,
            data_retention_policy: None,
        };

        contract.validate()?;

        Ok(contract)
    }

    /// Associates a symbolic privacy-budget reference.
    ///
    /// The referenced value may represent a scalar, vector, structured
    /// budget, or an extension-defined privacy accounting model.
    pub fn with_privacy_budget(
        mut self,
        budget: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "privacy budget",
            &budget,
        )?;

        self.privacy_budget = Some(budget);
        Ok(self)
    }

    /// Associates a disclosure policy.
    pub fn with_disclosure_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "disclosure policy",
            &policy,
        )?;

        self.disclosure_policy = Some(policy);
        Ok(self)
    }

    /// Associates a data-retention policy.
    pub fn with_data_retention_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "data-retention policy",
            &policy,
        )?;

        self.data_retention_policy = Some(policy);
        Ok(self)
    }

    /// Returns the privacy policy.
    #[must_use]
    pub fn policy(&self) -> &TrainingValueRef {
        &self.policy
    }

    /// Returns the privacy mechanism reference.
    #[must_use]
    pub fn mechanism(&self) -> &TrainingValueRef {
        &self.mechanism
    }

    /// Returns the optional privacy budget.
    #[must_use]
    pub fn privacy_budget(&self) -> Option<&TrainingValueRef> {
        self.privacy_budget.as_ref()
    }

    /// Returns the optional disclosure policy.
    #[must_use]
    pub fn disclosure_policy(&self) -> Option<&TrainingValueRef> {
        self.disclosure_policy.as_ref()
    }

    /// Returns the optional data-retention policy.
    #[must_use]
    pub fn data_retention_policy(&self) -> Option<&TrainingValueRef> {
        self.data_retention_policy.as_ref()
    }

    /// Validates the privacy declaration's references.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(
            "privacy policy",
            &self.policy,
        )?;

        validate_reference(
            "privacy mechanism",
            &self.mechanism,
        )?;

        for reference in [
            self.privacy_budget.as_ref(),
            self.disclosure_policy.as_ref(),
            self.data_retention_policy.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            validate_reference(
                "privacy policy reference",
                reference,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// RECOVERY CONTRACT
// ============================================================================

/// Declarative semantics for handling participant failures.
///
/// This does not implement retries, recovery, or distributed consensus.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
#[non_exhaustive]
pub enum ParticipantFailureSemantics {
    /// The round requires all declared participants to complete.
    RequireAll,

    /// The round may proceed when eligible contributions remain available.
    PermitPartialParticipation,

    /// A separately defined recovery or quorum policy determines whether
    /// the round may proceed.
    PolicyDefined,

    /// Failure semantics are defined by an extension.
    ExtensionDefined,
}

/// Declarative recovery contract for federated learning.
///
/// References connect this declaration to the distributed resilience
/// and checkpoint subsystems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedRecoveryContract {
    strategy: TrainingValueRef,
    checkpoint_policy: Option<TrainingValueRef>,
    failure_semantics: ParticipantFailureSemantics,
}

impl FederatedRecoveryContract {
    /// Creates a recovery contract.
    pub fn new(
        strategy: TrainingValueRef,
        failure_semantics: ParticipantFailureSemantics,
    ) -> AiResult<Self> {
        validate_reference(
            "federated recovery strategy",
            &strategy,
        )?;

        Ok(Self {
            strategy,
            checkpoint_policy: None,
            failure_semantics,
        })
    }

    /// Associates a checkpoint policy.
    pub fn with_checkpoint_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "federated checkpoint policy",
            &policy,
        )?;

        self.checkpoint_policy = Some(policy);
        Ok(self)
    }

    /// Returns the recovery strategy.
    #[must_use]
    pub fn strategy(&self) -> &TrainingValueRef {
        &self.strategy
    }

    /// Returns the optional checkpoint policy.
    #[must_use]
    pub fn checkpoint_policy(&self) -> Option<&TrainingValueRef> {
        self.checkpoint_policy.as_ref()
    }

    /// Returns participant-failure semantics.
    #[must_use]
    pub const fn failure_semantics(
        &self,
    ) -> ParticipantFailureSemantics {
        self.failure_semantics
    }

    /// Validates the recovery contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(
            "federated recovery strategy",
            &self.strategy,
        )?;

        if let Some(policy) = &self.checkpoint_policy {
            validate_reference(
                "federated checkpoint policy",
                policy,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// FEDERATED PHASE
// ============================================================================

/// Identity of a phase within a federated-learning round.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct FederatedPhaseId(String);

impl FederatedPhaseId {
    /// Creates a validated phase identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "federated phase identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the phase identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FederatedPhaseId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A declarative phase in a federated-learning round.
///
/// The operation reference may describe local training, evaluation,
/// aggregation preparation, synchronization, validation, or an
/// extension-defined operation.
///
/// Execution ordering and synchronization are resolved downstream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedPhase {
    identity: FederatedPhaseId,
    operation: TrainingValueRef,
    participants: Vec<FederatedParticipantRef>,
}

impl FederatedPhase {
    /// Creates a phase declaration.
    pub fn new(
        identity: FederatedPhaseId,
        operation: TrainingValueRef,
    ) -> AiResult<Self> {
        let phase = Self {
            identity,
            operation,
            participants: Vec::new(),
        };

        phase.validate()?;

        Ok(phase)
    }

    /// Adds a participant that is eligible for this phase.
    #[must_use]
    pub fn with_participant(
        mut self,
        participant: FederatedParticipantRef,
    ) -> Self {
        self.participants.push(participant);
        self
    }

    /// Returns the phase identity.
    #[must_use]
    pub fn identity(&self) -> &FederatedPhaseId {
        &self.identity
    }

    /// Returns the operation reference.
    #[must_use]
    pub fn operation(&self) -> &TrainingValueRef {
        &self.operation
    }

    /// Returns the declared participant references.
    #[must_use]
    pub fn participants(&self) -> &[FederatedParticipantRef] {
        &self.participants
    }

    /// Validates local phase invariants.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "federated phase identity",
            self.identity.as_str(),
        )?;

        validate_reference(
            "federated phase operation",
            &self.operation,
        )?;

        validate_unique(
            &self.participants,
            "phase participant",
        )?;

        Ok(())
    }
}

// ============================================================================
// FEDERATED ROUND
// ============================================================================

/// Identity of a federated-learning round.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct FederatedRoundId(String);

impl FederatedRoundId {
    /// Creates a validated round identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "federated round identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the round identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FederatedRoundId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Declarative federated-learning round.
///
/// Round ordering is preserved by the containing problem's vector.
/// No fixed round count or round-number width is imposed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedRound {
    identity: FederatedRoundId,
    participants: Vec<FederatedParticipantRef>,
    local_training: TrainingValueRef,
    aggregation: Option<TrainingValueRef>,
    evaluation: Option<TrainingValueRef>,
    phases: Vec<FederatedPhase>,
}

impl FederatedRound {
    /// Creates a round declaration.
    pub fn new(
        identity: FederatedRoundId,
        local_training: TrainingValueRef,
    ) -> AiResult<Self> {
        let round = Self {
            identity,
            participants: Vec::new(),
            local_training,
            aggregation: None,
            evaluation: None,
            phases: Vec::new(),
        };

        round.validate()?;

        Ok(round)
    }

    /// Adds an eligible participant to the round.
    #[must_use]
    pub fn with_participant(
        mut self,
        participant: FederatedParticipantRef,
    ) -> Self {
        self.participants.push(participant);
        self
    }

    /// Associates a round-specific aggregation operation.
    pub fn with_aggregation(
        mut self,
        aggregation: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "round aggregation",
            &aggregation,
        )?;

        self.aggregation = Some(aggregation);
        Ok(self)
    }

    /// Associates an evaluation operation.
    pub fn with_evaluation(
        mut self,
        evaluation: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "round evaluation",
            &evaluation,
        )?;

        self.evaluation = Some(evaluation);
        Ok(self)
    }

    /// Adds a phase while preserving declaration order.
    pub fn with_phase(
        mut self,
        phase: FederatedPhase,
    ) -> AiResult<Self> {
        phase.validate()?;

        if self.phases.iter().any(
            |existing| existing.identity() == phase.identity()
        ) {
            return Err(invalid_federated(format!(
                "duplicate phase identity: {}",
                phase.identity()
            )));
        }

        self.phases.push(phase);
        Ok(self)
    }

    /// Returns the round identity.
    #[must_use]
    pub fn identity(&self) -> &FederatedRoundId {
        &self.identity
    }

    /// Returns round participants.
    #[must_use]
    pub fn participants(&self) -> &[FederatedParticipantRef] {
        &self.participants
    }

    /// Returns the local training workflow reference.
    #[must_use]
    pub fn local_training(&self) -> &TrainingValueRef {
        &self.local_training
    }

    /// Returns the optional round-specific aggregation reference.
    #[must_use]
    pub fn aggregation(&self) -> Option<&TrainingValueRef> {
        self.aggregation.as_ref()
    }

    /// Returns the optional evaluation reference.
    #[must_use]
    pub fn evaluation(&self) -> Option<&TrainingValueRef> {
        self.evaluation.as_ref()
    }

    /// Returns phases in declaration order.
    #[must_use]
    pub fn phases(&self) -> &[FederatedPhase] {
        &self.phases
    }

    /// Validates local round invariants.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "federated round identity",
            self.identity.as_str(),
        )?;

        validate_reference(
            "round local training",
            &self.local_training,
        )?;

        validate_unique(
            &self.participants,
            "round participant",
        )?;

        if let Some(aggregation) = &self.aggregation {
            validate_reference(
                "round aggregation",
                aggregation,
            )?;
        }

        if let Some(evaluation) = &self.evaluation {
            validate_reference(
                "round evaluation",
                evaluation,
            )?;
        }

        let mut phase_ids = BTreeSet::new();

        for phase in &self.phases {
            phase.validate()?;

            if !phase_ids.insert(phase.identity()) {
                return Err(invalid_federated(format!(
                    "duplicate phase identity: {}",
                    phase.identity()
                )));
            }
        }

        Ok(())
    }
}

// ============================================================================
// FEDERATED LEARNING PROBLEM
// ============================================================================

/// Complete target-independent federated-learning declaration.
///
/// The declaration composes existing model, dataset, training,
/// requirements, communication, aggregation, privacy and recovery
/// contracts.
///
/// It is not an execution plan or a distributed runtime object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedLearningProblem {
    identity: FederatedLearningId,
    model: Option<AiModelId>,
    participants: Vec<FederatedParticipant>,
    aggregation: FederatedAggregationContract,
    communication: FederatedCommunicationContract,
    privacy: Option<FederatedPrivacyContract>,
    recovery: Option<FederatedRecoveryContract>,
    rounds: Vec<FederatedRound>,
    requirements: AiRequirements,
    reproducibility: Option<TrainingReproducibilityContract>,
    provenance: Option<TrainingValueRef>,
}

impl FederatedLearningProblem {
    /// Creates a federated-learning problem.
    pub fn new(
        identity: FederatedLearningId,
        aggregation: FederatedAggregationContract,
        communication: FederatedCommunicationContract,
    ) -> AiResult<Self> {
        let problem = Self {
            identity,
            model: None,
            participants: Vec::new(),
            aggregation,
            communication,
            privacy: None,
            recovery: None,
            rounds: Vec::new(),
            requirements: AiRequirements::new(),
            reproducibility: None,
            provenance: None,
        };

        problem.validate()?;

        Ok(problem)
    }

    /// Associates the canonical AI model identity.
    #[must_use]
    pub fn with_model(
        mut self,
        model: AiModelId,
    ) -> Self {
        self.model = Some(model);
        self
    }

    /// Adds a logical federation participant.
    pub fn with_participant(
        mut self,
        participant: FederatedParticipant,
    ) -> AiResult<Self> {
        participant.validate()?;

        if self.participants.iter().any(
            |existing| existing.identity() == participant.identity()
        ) {
            return Err(invalid_federated(format!(
                "duplicate federation participant: {}",
                participant.identity()
            )));
        }

        self.participants.push(participant);
        Ok(self)
    }

    /// Associates a privacy contract.
    pub fn with_privacy(
        mut self,
        privacy: FederatedPrivacyContract,
    ) -> AiResult<Self> {
        privacy.validate()?;

        self.privacy = Some(privacy);
        Ok(self)
    }

    /// Associates a recovery contract.
    pub fn with_recovery(
        mut self,
        recovery: FederatedRecoveryContract,
    ) -> AiResult<Self> {
        recovery.validate()?;

        self.recovery = Some(recovery);
        Ok(self)
    }

    /// Adds a federated round while preserving declaration order.
    pub fn with_round(
        mut self,
        round: FederatedRound,
    ) -> AiResult<Self> {
        round.validate()?;

        if self.rounds.iter().any(
            |existing| existing.identity() == round.identity()
        ) {
            return Err(invalid_federated(format!(
                "duplicate federated round: {}",
                round.identity()
            )));
        }

        self.rounds.push(round);
        Ok(self)
    }

    /// Replaces the federation's AI requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Associates the existing generic training reproducibility contract.
    #[must_use]
    pub fn with_reproducibility(
        mut self,
        reproducibility: TrainingReproducibilityContract,
    ) -> Self {
        self.reproducibility = Some(reproducibility);
        self
    }

    /// Associates a provenance reference.
    pub fn with_provenance(
        mut self,
        provenance: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "federated-learning provenance",
            &provenance,
        )?;

        self.provenance = Some(provenance);
        Ok(self)
    }

    /// Returns the federation identity.
    #[must_use]
    pub fn identity(&self) -> &FederatedLearningId {
        &self.identity
    }

    /// Returns the optional canonical model identity.
    #[must_use]
    pub const fn model(&self) -> Option<AiModelId> {
        self.model
    }

    /// Returns federation participants in declaration order.
    #[must_use]
    pub fn participants(&self) -> &[FederatedParticipant] {
        &self.participants
    }

    /// Returns the federation aggregation contract.
    #[must_use]
    pub fn aggregation(&self) -> &FederatedAggregationContract {
        &self.aggregation
    }

    /// Returns the communication contract.
    #[must_use]
    pub fn communication(&self) -> &FederatedCommunicationContract {
        &self.communication
    }

    /// Returns the optional privacy contract.
    #[must_use]
    pub fn privacy(&self) -> Option<&FederatedPrivacyContract> {
        self.privacy.as_ref()
    }

    /// Returns the optional recovery contract.
    #[must_use]
    pub fn recovery(&self) -> Option<&FederatedRecoveryContract> {
        self.recovery.as_ref()
    }

    /// Returns rounds in declaration order.
    #[must_use]
    pub fn rounds(&self) -> &[FederatedRound] {
        &self.rounds
    }

    /// Returns federation requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Returns the optional reproducibility contract.
    #[must_use]
    pub fn reproducibility(
        &self,
    ) -> Option<&TrainingReproducibilityContract> {
        self.reproducibility.as_ref()
    }

    /// Returns the optional provenance reference.
    #[must_use]
    pub fn provenance(&self) -> Option<&TrainingValueRef> {
        self.provenance.as_ref()
    }

    /// Validates all local and cross-reference invariants.
    ///
    /// This is structural validation only. It does not resolve external
    /// references, prove privacy guarantees, authenticate participants,
    /// or establish execution feasibility.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "federated-learning identity",
            self.identity.as_str(),
        )?;

        self.aggregation.validate()?;
        self.communication.validate()?;

        if let Some(privacy) = &self.privacy {
            privacy.validate()?;
        }

        if let Some(recovery) = &self.recovery {
            recovery.validate()?;
        }

        if let Some(provenance) = &self.provenance {
            validate_reference(
                "federated-learning provenance",
                provenance,
            )?;
        }

        // Participant identities must be unique.
        let participant_ids: Vec<&FederatedParticipantRef> =
            self.participants
                .iter()
                .map(FederatedParticipant::identity)
                .collect();

        validate_unique(
            &participant_ids,
            "federation participant",
        )?;

        let declared_participants: BTreeSet<&FederatedParticipantRef> =
            self.participants
                .iter()
                .map(FederatedParticipant::identity)
                .collect();

        for participant in &self.participants {
            participant.validate()?;
        }

        // Round identities must be unique.
        let round_ids: Vec<&FederatedRoundId> =
            self.rounds
                .iter()
                .map(FederatedRound::identity)
                .collect();

        validate_unique(
            &round_ids,
            "federated round",
        )?;

        for round in &self.rounds {
            round.validate()?;

            // A round may only reference participants declared by
            // this federation.
            for participant in round.participants() {
                if !declared_participants.contains(participant) {
                    return Err(invalid_federated(format!(
                        "round {} references undeclared participant {}",
                        round.identity(),
                        participant,
                    )));
                }
            }

            // A phase may only reference participants included in
            // its containing round.
            let round_participants: BTreeSet<&FederatedParticipantRef> =
                round.participants()
                    .iter()
                    .collect();

            for phase in round.phases() {
                for participant in phase.participants() {
                    if !round_participants.contains(participant) {
                        return Err(invalid_federated(format!(
                            "phase {} references participant {} \
                             that is not included in round {}",
                            phase.identity(),
                            participant,
                            round.identity(),
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Returns whether the declaration is structurally valid.
    ///
    /// This convenience method performs the same structural validation
    /// as [`Self::validate`].
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

// ============================================================================
// VALIDATION LEVEL
// ============================================================================

/// Scope of a federated-learning validation result.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
#[non_exhaustive]
pub enum FederatedValidationLevel {
    /// Only local structure and internal references were checked.
    Structural,

    /// Canonical semantic validation was also completed.
    Semantic,

    /// Capability and resource feasibility were evaluated downstream.
    Feasibility,

    /// A downstream execution authority verified the applicable
    /// execution contracts.
    ExecutionReady,
}

/// A federated-learning validation report.
///
/// This report does not independently certify semantic validity,
/// feasibility, authorization, privacy, or execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederatedValidationReport {
    level: FederatedValidationLevel,
    valid: bool,
}

impl FederatedValidationReport {
    /// Creates a structural validation report from a problem.
    #[must_use]
    pub fn structural(
        problem: &FederatedLearningProblem,
    ) -> Self {
        Self {
            level: FederatedValidationLevel::Structural,
            valid: problem.validate().is_ok(),
        }
    }

    /// Returns the validation level.
    #[must_use]
    pub const fn level(&self) -> FederatedValidationLevel {
        self.level
    }

    /// Returns whether the checks represented by this report succeeded.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn value(
        identity: &str,
    ) -> TrainingValueRef {
        TrainingValueRef::new(identity)
            .expect("test reference must be valid")
    }

    fn participant(
        identity: &str,
    ) -> FederatedParticipant {
        FederatedParticipant::new(
            FederatedParticipantRef::new(identity)
                .expect("participant identity"),
            FederatedParticipantRole::new("contributor")
                .expect("participant role"),
            value("training.local"),
        )
        .expect("participant")
    }

    fn aggregation() -> FederatedAggregationContract {
        FederatedAggregationContract::new(
            value("aggregation.algorithm"),
            value("aggregation.inputs"),
            value("aggregation.output"),
            AggregationParticipation::AvailableParticipants,
        )
        .expect("aggregation contract")
    }

    fn communication() -> FederatedCommunicationContract {
        FederatedCommunicationContract::new(
            value("communication.channel"),
            value("communication.protocol"),
        )
        .expect("communication contract")
    }

    fn problem() -> FederatedLearningProblem {
        FederatedLearningProblem::new(
            FederatedLearningId::new("federation.example")
                .expect("federation identity"),
            aggregation(),
            communication(),
        )
        .expect("federated problem")
    }

    fn round() -> FederatedRound {
        FederatedRound::new(
            FederatedRoundId::new("round.initial")
                .expect("round identity"),
            value("training.local"),
        )
        .expect("round")
    }

    #[test]
    fn federation_identity_rejects_empty_values() {
        assert!(
            FederatedLearningId::new(" ").is_err()
        );
    }

    #[test]
    fn participant_reference_rejects_empty_values() {
        assert!(
            FederatedParticipantRef::new("").is_err()
        );
    }

    #[test]
    fn participant_role_is_open_world() {
        let role = FederatedParticipantRole::new(
            "future.application.role",
        )
        .expect("open-world role");

        assert_eq!(
            role.as_str(),
            "future.application.role",
        );
    }

    #[test]
    fn participant_requires_local_training_reference() {
        let result = FederatedParticipant::new(
            FederatedParticipantRef::new("participant.one")
                .expect("identity"),
            FederatedParticipantRole::new("contributor")
                .expect("role"),
            value("training.local"),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn participant_can_reuse_canonical_model_and_dataset_ids() {
        let model = AiModelId::new(17)
            .expect("model identity");

        let dataset = AiDatasetId::new(23)
            .expect("dataset identity");

        let participant = participant("participant.one")
            .with_model(model)
            .with_dataset(dataset);

        assert_eq!(participant.model(), Some(model));
        assert_eq!(participant.dataset(), Some(dataset));
    }

    #[test]
    fn aggregation_uses_open_world_algorithm_references() {
        let aggregation = FederatedAggregationContract::new(
            value("custom.future.aggregation"),
            value("aggregation.inputs"),
            value("aggregation.output"),
            AggregationParticipation::ExtensionDefined,
        )
        .expect("aggregation");

        assert_eq!(
            aggregation.algorithm().as_str(),
            "custom.future.aggregation",
        );
    }

    #[test]
    fn communication_policies_are_optional_and_extensible() {
        let communication = communication()
            .with_confidentiality_policy(
                value("security.confidentiality"),
            )
            .expect("confidentiality")
            .with_integrity_policy(
                value("security.integrity"),
            )
            .expect("integrity")
            .with_data_locality_policy(
                value("data.locality"),
            )
            .expect("locality");

        assert!(
            communication.confidentiality_policy().is_some()
        );

        assert!(
            communication.integrity_policy().is_some()
        );

        assert!(
            communication.data_locality_policy().is_some()
        );
    }

    #[test]
    fn privacy_contract_preserves_symbolic_budget() {
        let privacy = FederatedPrivacyContract::new(
            value("privacy.policy"),
            value("privacy.mechanism"),
        )
        .expect("privacy")
        .with_privacy_budget(
            value("privacy.dynamic_budget"),
        )
        .expect("budget");

        assert_eq!(
            privacy.privacy_budget()
                .map(TrainingValueRef::as_str),
            Some("privacy.dynamic_budget"),
        );
    }

    #[test]
    fn recovery_contract_is_declarative() {
        let recovery = FederatedRecoveryContract::new(
            value("recovery.strategy"),
            ParticipantFailureSemantics::PolicyDefined,
        )
        .expect("recovery")
        .with_checkpoint_policy(
            value("checkpoint.policy"),
        )
        .expect("checkpoint");

        assert_eq!(
            recovery.failure_semantics(),
            ParticipantFailureSemantics::PolicyDefined,
        );

        assert!(recovery.checkpoint_policy().is_some());
    }

    #[test]
    fn phase_rejects_duplicate_participants() {
        let participant_ref = FederatedParticipantRef::new(
            "participant.one",
        )
        .expect("participant");

        let phase = FederatedPhase::new(
            FederatedPhaseId::new("phase.local")
                .expect("phase"),
            value("operation.local"),
        )
        .expect("phase")
        .with_participant(participant_ref.clone())
        .with_participant(participant_ref);

        assert!(phase.validate().is_err());
    }

    #[test]
    fn round_rejects_duplicate_participants() {
        let participant_ref = FederatedParticipantRef::new(
            "participant.one",
        )
        .expect("participant");

        let round = round()
            .with_participant(participant_ref.clone())
            .with_participant(participant_ref);

        assert!(round.validate().is_err());
    }

    #[test]
    fn round_preserves_phase_order() {
        let first = FederatedPhase::new(
            FederatedPhaseId::new("phase.local")
                .expect("phase"),
            value("operation.local"),
        )
        .expect("phase");

        let second = FederatedPhase::new(
            FederatedPhaseId::new("phase.aggregate")
                .expect("phase"),
            value("operation.aggregate"),
        )
        .expect("phase");

        let round = round()
            .with_phase(first)
            .expect("first phase")
            .with_phase(second)
            .expect("second phase");

        assert_eq!(
            round.phases()[0].identity().as_str(),
            "phase.local",
        );

        assert_eq!(
            round.phases()[1].identity().as_str(),
            "phase.aggregate",
        );
    }

    #[test]
    fn federation_accepts_declared_participants_and_rounds() {
        let first = FederatedParticipantRef::new(
            "participant.one",
        )
        .expect("participant");

        let second = FederatedParticipantRef::new(
            "participant.two",
        )
        .expect("participant");

        let federation = problem()
            .with_participant(participant("participant.one"))
            .expect("first participant")
            .with_participant(participant("participant.two"))
            .expect("second participant")
            .with_round(
                round()
                    .with_participant(first)
                    .with_participant(second),
            )
            .expect("round");

        assert!(federation.validate().is_ok());
    }

    #[test]
    fn federation_rejects_duplicate_participant_identity() {
        let federation = problem()
            .with_participant(participant("participant.one"))
            .expect("first participant");

        let result = federation.with_participant(
            participant("participant.one"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn federation_rejects_round_referencing_unknown_participant() {
        let federation = problem()
            .with_participant(participant("participant.one"))
            .expect("participant")
            .with_round(
                round().with_participant(
                    FederatedParticipantRef::new(
                        "participant.unknown",
                    )
                    .expect("reference"),
                ),
            )
            .expect("round");

        assert!(federation.validate().is_err());
    }

    #[test]
    fn phase_cannot_reference_participant_outside_its_round() {
        let declared = FederatedParticipantRef::new(
            "participant.one",
        )
        .expect("participant");

        let unknown = FederatedParticipantRef::new(
            "participant.two",
        )
        .expect("participant");

        let phase = FederatedPhase::new(
            FederatedPhaseId::new("phase.local")
                .expect("phase"),
            value("operation.local"),
        )
        .expect("phase")
        .with_participant(unknown);

        let round = round()
            .with_participant(declared)
            .with_phase(phase)
            .expect("phase");

        let federation = problem()
            .with_participant(participant("participant.one"))
            .expect("participant")
            .with_round(round)
            .expect("round");

        assert!(federation.validate().is_err());
    }

    #[test]
    fn duplicate_round_identity_is_rejected() {
        let federation = problem()
            .with_round(round())
            .expect("first round");

        let result = federation.with_round(round());

        assert!(result.is_err());
    }

    #[test]
    fn requirements_are_preserved() {
        let requirements = AiRequirements::new();

        let federation = problem()
            .with_requirements(requirements.clone());

        assert_eq!(
            federation.requirements(),
            &requirements,
        );
    }

    #[test]
    fn reproducibility_reuses_generic_training_contract() {
        use crate::ai::learning::training::{
            TrainingReproducibility,
            TrainingReproducibilityContract,
        };

        let reproducibility =
            TrainingReproducibilityContract::new(
                TrainingReproducibility::Reproducible,
            );

        let federation = problem()
            .with_reproducibility(reproducibility);

        assert!(
            federation.reproducibility().is_some()
        );
    }

    #[test]
    fn structural_validation_does_not_claim_feasibility() {
        let report =
            FederatedValidationReport::structural(&problem());

        assert_eq!(
            report.level(),
            FederatedValidationLevel::Structural,
        );

        assert!(report.is_valid());
    }

    #[test]
    fn large_identity_values_are_not_artificially_limited() {
        let identity = FederatedParticipantRef::new(
            format!("participant.{}", u128::MAX),
        )
        .expect("large symbolic identity");

        assert!(
            identity.as_str().ends_with(&u128::MAX.to_string())
        );
    }

    #[test]
    fn federation_has_no_fixed_participant_ceiling() {
        let mut federation = problem();

        for index in 0..512 {
            federation = federation
                .with_participant(
                    participant(&format!("participant.{index}")),
                )
                .expect("participant identity must be unique");
        }

        assert_eq!(
            federation.participants().len(),
            512,
        );

        assert!(federation.validate().is_ok());
    }
}
