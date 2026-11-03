
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Capabilities
//!
//! Path: `src/ai/agents/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines the computational capabilities offered and required by an
//! AI agent.
//!
//! An agent capability profile describes:
//!
//! - capabilities an agent declares that it can provide;
//! - capabilities required for the agent's valid operation;
//! - preferred capabilities;
//! - permitted capabilities;
//! - forbidden capabilities;
//! - additional AI computational requirements;
//! - requirement-analysis completeness.
//!
//! This module describes agent computational intent.
//!
//! It does not execute agents or establish that a physical execution
//! environment supports their declared capabilities.
//!
//! ## Canonical ownership
//!
//! The following repository components remain authoritative:
//!
//! - `src/ai/ids.rs`
//!   Owns `AiAgentId`.
//!
//! - `src/ai/capabilities.rs`
//!   Owns AI capability descriptors, canonical capability aliases,
//!   capability namespace helpers and AI evidence adapters.
//!
//! - `src/ai/requirements.rs`
//!   Owns `AiRequirements`, `AiCapabilityRequirement`,
//!   `CapabilityStrength` and requirement provenance.
//!
//! - `src/frontend/semantic/capabilities/model.rs`
//!   Owns universal capability identities and requirements.
//!
//! - `src/frontend/semantic/capabilities/evidence.rs`
//!   Owns capability evidence.
//!
//! - `src/distributed/capabilities.rs`
//!   Owns distributed capability discovery.
//!
//! - `grammar/ai/agents.g4`
//!   Owns agent source syntax.
//!
//! - `grammar/ai/capabilities.g4`
//!   Owns AI capability syntax adapters.
//!
//! - `grammar/ai/ai-capabilities.g4`
//!   Owns AI capability composition contracts.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Agent capability profiles.
//! - Agent capability offers.
//! - Agent capability requirement aggregation.
//! - Agent-specific capability validation.
//! - Capability profile composition.
//! - Explicit incomplete-analysis propagation.
//!
//! ## Does not own
//!
//! - Universal capability identities.
//! - Hardware capability inventories.
//! - Capability provider registration.
//! - Hardware discovery.
//! - Capability evidence verification.
//! - Authentication or authorization.
//! - Physical resource allocation.
//! - Target selection.
//! - Distributed placement.
//! - Scheduling.
//! - Networking.
//! - Agent execution.
//! - Runtime agent state.
//! - Model execution.
//! - Universal IR.
//!
//! ## POCO-REAF
//!
//! Agent capability descriptions are independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - FPGA implementation;
//! - ASIC implementation;
//! - QPU implementation;
//! - neuromorphic architecture;
//! - operating system;
//! - memory topology;
//! - network topology;
//! - distributed node count.
//!
//! Capability names are open-world.
//!
//! No universal maximum number of:
//!
//! - agent capabilities;
//! - capability requirements;
//! - agents;
//! - workers;
//! - accelerators;
//! - devices;
//! - execution targets;
//! - computational domains.
//!
//! is imposed.
//!
//! Collections grow subject to actual implementation resources and
//! explicitly configured operational budgets.
//!
//! Literal infinite physical capacity is not promised.
//!
//! ## Important distinction
//!
//! An agent declaring a capability does not prove that its current
//! execution target supports that capability.
//!
//! A declaration is computational intent.
//!
//! Verified support requires appropriately scoped capability evidence.
//!
//! ## Integration
//!
//! `agents/definition.rs`
//!     -> AiAgentId and AgentDefinition
//!
//! `ai/capabilities.rs`
//!     -> AiCapabilityDescriptor and canonical capability adapters
//!
//! `ai/requirements.rs`
//!     -> AiRequirements and capability strengths
//!
//! `frontend/semantic/capabilities/`
//!     -> canonical capability validation and evidence
//!
//! `distributed/capabilities.rs`
//!     -> distributed capability discovery
//!
//! `grammar/ai/agents.g4`
//!     -> agent source declarations
//!
//! `grammar/ai/capabilities.g4`
//!     -> capability intent
//!
//! `agents/mod.rs`
//!     -> public agent subsystem composition
//!
//! ## Freeze criteria
//!
//! - Uses canonical AiAgentId.
//! - Uses canonical capability identities.
//! - Supports external-domain capabilities.
//! - Preserves required/preferred/allowed/forbidden distinctions.
//! - Does not claim that declared capabilities are verified.
//! - Does not implement another capability registry.
//! - Preserves incomplete requirement analysis.
//! - Rejects invalid capability descriptors.
//! - Rejects duplicate capability offers.
//! - Preserves requirement provenance.
//! - Supports explicit profile composition.
//! - Supports dynamically growing collections.
//! - Does not contain artificial universal ceilings.
//! - Uses safe Rust only.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use crate::ai::capabilities::{
    AiCapabilityDescriptor,
    AiCapabilityId,
    AiCapabilityCategory,
    ai_capability_id,
};

use crate::ai::ids::AiAgentId;

use crate::ai::requirements::{
    AiCapabilityRequirement,
    AiRequirements,
    CapabilityStrength,
    RequirementProvenance,
    RequirementError,
};

use crate::frontend::semantic::capabilities::model::{
    CapabilityId,
    CapabilityModelError,
    CapabilityRequirement,
};

// ============================================================================
// RESULT
// ============================================================================

/// Result type for agent capability operations.
pub type AgentCapabilitiesResult<T> =
    Result<T, AgentCapabilitiesError>;

// ============================================================================
// AGENT CAPABILITY PROFILE
// ============================================================================

/// Declarative capability profile belonging to an AI agent.
///
/// The profile describes:
///
/// - capability offers declared by the agent;
/// - capability requirements needed by the agent;
/// - additional AI requirements associated with the agent.
///
/// Capability offers are declarations, not verified execution-target
/// support.
///
/// Capability requirements retain their canonical semantic meaning.
///
/// The profile is independent of physical execution environments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCapabilityProfile {
    agent_id: AiAgentId,

    offers: Vec<AiCapabilityDescriptor>,

    requirements: AiRequirements,
}

impl AgentCapabilityProfile {
    /// Construct an empty capability profile for an agent.
    ///
    /// The agent identity must already have been validated by the
    /// canonical AI identifier implementation.
    #[must_use]
    pub fn new(agent_id: AiAgentId) -> Self {
        Self {
            agent_id,
            offers: Vec::new(),
            requirements: AiRequirements::new(),
        }
    }

    /// Return the identity of the agent owning this profile.
    #[must_use]
    pub const fn agent_id(&self) -> AiAgentId {
        self.agent_id
    }

    /// Return all declared capability offers.
    ///
    /// These declarations do not independently prove that a target
    /// can execute the corresponding capabilities.
    #[must_use]
    pub fn offers(&self) -> &[AiCapabilityDescriptor] {
        &self.offers
    }

    /// Return the complete AI requirement envelope.
    ///
    /// The envelope includes:
    ///
    /// - AI-specific requirements;
    /// - capability requirements;
    /// - resource requirements;
    /// - resource constraints;
    /// - requirement-analysis status.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Return whether requirement analysis is complete.
    ///
    /// This does not mean that the requirements have been proven
    /// feasible on an execution target.
    #[must_use]
    pub fn is_analysis_complete(&self) -> bool {
        self.requirements.is_complete()
    }

    /// Return whether the profile contains no declarations.
    ///
    /// Requirement-analysis completeness must be checked separately.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.offers.is_empty() && self.requirements.is_empty()
    }

    // ========================================================================
    // CAPABILITY OFFERS
    // ========================================================================

    /// Declare a capability offered by this agent.
    ///
    /// The descriptor may identify an AI capability or an external
    /// computational capability.
    ///
    /// For example:
    ///
    /// - ai::inference
    /// - ai::reasoning
    /// - tensor::compute
    /// - distributed::communication
    /// - quantum::computation
    ///
    /// The descriptor is validated using the canonical AI capability
    /// descriptor implementation.
    ///
    /// Duplicate capability identities are rejected.
    ///
    /// No claim of hardware support or runtime availability is made.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - descriptor validation fails;
    /// - the capability has already been declared by this agent.
    pub fn add_offer(
        &mut self,
        descriptor: AiCapabilityDescriptor,
    ) -> AgentCapabilitiesResult<()> {
        descriptor
            .validate()
            .map_err(|source| {
                AgentCapabilitiesError::InvalidOffer {
                    capability: descriptor
                        .capability()
                        .qualified_name(),
                    source,
                }
            })?;

        let identity = descriptor.capability().qualified_name();

        if self.offers.iter().any(|existing| {
            existing.capability().qualified_name() == identity
        }) {
            return Err(
                AgentCapabilitiesError::DuplicateOffer {
                    capability: identity,
                },
            );
        }

        self.offers.push(descriptor);

        Ok(())
    }

    /// Declare an AI-owned capability offered by this agent.
    ///
    /// Capability categories remain open-world.
    ///
    /// This convenience method delegates identity construction and
    /// descriptor validation to the canonical AI capability subsystem.
    ///
    /// # Errors
    ///
    /// Returns an error if the capability name or category is invalid,
    /// or if the capability has already been declared.
    pub fn add_ai_offer(
        &mut self,
        name: impl Into<String>,
        category: AiCapabilityCategory,
        description: impl Into<String>,
    ) -> AgentCapabilitiesResult<()> {
        let descriptor = AiCapabilityDescriptor::for_ai(
            name,
            category,
            description,
        )
        .map_err(|source| {
            AgentCapabilitiesError::InvalidCapabilityIdentity {
                source,
            }
        })?;

        self.add_offer(descriptor)
    }

    /// Find a declared capability offer by canonical identity.
    #[must_use]
    pub fn find_offer(
        &self,
        capability: &CapabilityId,
    ) -> Option<&AiCapabilityDescriptor> {
        self.offers
            .iter()
            .find(|offer| offer.capability() == capability)
    }

    /// Remove a previously declared capability offer.
    ///
    /// Returns the removed descriptor when one existed.
    ///
    /// Removing an offer does not modify capability requirements.
    pub fn remove_offer(
        &mut self,
        capability: &CapabilityId,
    ) -> Option<AiCapabilityDescriptor> {
        let position = self
            .offers
            .iter()
            .position(|offer| offer.capability() == capability)?;

        Some(self.offers.remove(position))
    }

    // ========================================================================
    // CAPABILITY REQUIREMENTS
    // ========================================================================

    /// Add an existing canonical AI capability requirement.
    ///
    /// The requirement retains its original:
    ///
    /// - canonical capability identity;
    /// - version constraint;
    /// - required/preferred/allowed/forbidden strength;
    /// - provenance.
    ///
    /// No conversion into a different capability representation occurs.
    pub fn add_capability_requirement(
        &mut self,
        requirement: AiCapabilityRequirement,
    ) {
        self.requirements.add_capability(requirement);
    }

    /// Add a canonical capability predicate with an explicit strength.
    ///
    /// This accepts both AI and external-domain capability identities.
    ///
    /// Examples:
    ///
    /// - ai::inference
    /// - accelerator::tensor_compute
    /// - distributed::collective
    /// - quantum::measurement
    ///
    /// # Errors
    ///
    /// The canonical capability identity must already be valid.
    pub fn require_capability(
        &mut self,
        predicate: CapabilityRequirement,
        strength: CapabilityStrength,
    ) {
        self.add_capability_requirement(
            AiCapabilityRequirement::new(
                predicate,
                strength,
                RequirementProvenance::derived(),
            ),
        );
    }

    /// Add a requirement for an AI-owned capability.
    ///
    /// The capability name is open-world.
    ///
    /// No predefined list of AI capabilities is required.
    ///
    /// # Errors
    ///
    /// Returns an error if the capability name is invalid.
    pub fn require_ai_capability(
        &mut self,
        name: impl Into<String>,
        strength: CapabilityStrength,
    ) -> AgentCapabilitiesResult<()> {
        let identity = ai_capability_id(name)
            .map_err(|source| {
                AgentCapabilitiesError::InvalidCapabilityIdentity {
                    source,
                }
            })?;

        self.require_capability(
            CapabilityRequirement::new(identity),
            strength,
        );

        Ok(())
    }

    /// Add a requirement with explicit source AST provenance.
    ///
    /// This method is intended for semantic adapters that have already
    /// resolved the originating canonical AST node.
    pub fn require_capability_from_node(
        &mut self,
        predicate: CapabilityRequirement,
        strength: CapabilityStrength,
        provenance: RequirementProvenance,
    ) {
        self.add_capability_requirement(
            AiCapabilityRequirement::new(
                predicate,
                strength,
                provenance,
            ),
        );
    }

    // ========================================================================
    // OTHER AI REQUIREMENTS
    // ========================================================================

    /// Add another AI-specific computational requirement.
    ///
    /// This delegates to the canonical AI requirement envelope.
    pub fn add_ai_requirement(
        &mut self,
        requirement: crate::ai::requirements::AiRequirement,
    ) {
        self.requirements.add_requirement(requirement);
    }

    /// Mark the profile's requirement analysis incomplete.
    ///
    /// Incomplete analysis must never be interpreted as proof that
    /// the agent has no additional requirements.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied reason is invalid.
    pub fn mark_analysis_incomplete(
        &mut self,
        reason: impl Into<String>,
    ) -> AgentCapabilitiesResult<()> {
        self.requirements
            .mark_incomplete(reason)
            .map_err(AgentCapabilitiesError::Requirement)
    }

    /// Merge another requirement envelope into this profile.
    ///
    /// All existing requirements and provenance are preserved.
    ///
    /// The completeness status is propagated.
    pub fn merge_requirements(
        &mut self,
        requirements: AiRequirements,
    ) {
        self.requirements.merge(requirements);
    }

    // ========================================================================
    // VALIDATION
    // ========================================================================

    /// Validate the structural integrity of this capability profile.
    ///
    /// This checks:
    ///
    /// - every declared offer;
    /// - capability identity uniqueness among offers;
    /// - descriptor metadata validity.
    ///
    /// This does not establish target feasibility or capability support.
    ///
    /// Incomplete requirement analysis is reported separately through
    /// `ensure_complete`.
    pub fn validate(&self) -> AgentCapabilitiesResult<()> {
        let mut identities = BTreeSet::new();

        for descriptor in &self.offers {
            descriptor
                .validate()
                .map_err(|source| {
                    AgentCapabilitiesError::InvalidOffer {
                        capability: descriptor
                            .capability()
                            .qualified_name(),
                        source,
                    }
                })?;

            let identity = descriptor.capability().qualified_name();

            if !identities.insert(identity.clone()) {
                return Err(
                    AgentCapabilitiesError::DuplicateOffer {
                        capability: identity,
                    },
                );
            }
        }

        Ok(())
    }

    /// Require complete requirement analysis.
    ///
    /// An incomplete profile cannot be certified as complete.
    ///
    /// This does not evaluate execution feasibility.
    ///
    /// # Errors
    ///
    /// Returns an error if one or more requirement sources remain
    /// unresolved.
    pub fn ensure_complete(
        &self,
    ) -> AgentCapabilitiesResult<()> {
        self.requirements
            .clone()
            .into_complete()
            .map(|_| ())
            .map_err(AgentCapabilitiesError::IncompleteRequirements)
    }

    /// Validate the profile and require complete requirement analysis.
    ///
    /// Use this when a downstream consumer requires a fully analyzed
    /// agent capability contract.
    pub fn validate_complete(
        &self,
    ) -> AgentCapabilitiesResult<()> {
        self.validate()?;
        self.ensure_complete()
    }

    // ========================================================================
    // COMPOSITION
    // ========================================================================

    /// Merge another agent capability profile.
    ///
    /// Both profiles must belong to the same agent.
    ///
    /// Offers are combined without silently overwriting existing offers.
    /// Requirement declarations are preserved.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the profiles belong to different agents;
    /// - a duplicate capability offer exists;
    /// - a descriptor is invalid.
    pub fn merge(
        &mut self,
        other: Self,
    ) -> AgentCapabilitiesResult<()> {
        if self.agent_id != other.agent_id {
            return Err(
                AgentCapabilitiesError::AgentIdentityMismatch {
                    expected: self.agent_id,
                    actual: other.agent_id,
                },
            );
        }

        // Validate before mutating the current profile.
        other.validate()?;

        for offer in &other.offers {
            if self.find_offer(offer.capability()).is_some() {
                return Err(
                    AgentCapabilitiesError::DuplicateOffer {
                        capability: offer
                            .capability()
                            .qualified_name(),
                    },
                );
            }
        }

        // All validation has succeeded. Mutation is now safe.
        self.offers.extend(other.offers);
        self.requirements.merge(other.requirements);

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured errors produced by agent capability operations.
///
/// These errors preserve canonical validation failures where applicable.
///
/// They do not replace the frontend diagnostic system.
#[derive(Debug)]
#[non_exhaustive]
pub enum AgentCapabilitiesError {
    /// An offered capability descriptor is invalid.
    InvalidOffer {
        /// Canonical capability identity.
        capability: String,

        /// Original descriptor validation error.
        source: CapabilityModelError,
    },

    /// A capability offer is already declared.
    DuplicateOffer {
        /// Conflicting canonical capability identity.
        capability: String,
    },

    /// A capability identity could not be constructed.
    InvalidCapabilityIdentity {
        /// Original canonical validation error.
        source: CapabilityModelError,
    },

    /// Two profiles belong to different agents.
    AgentIdentityMismatch {
        /// Identity expected by the receiving profile.
        expected: AiAgentId,

        /// Identity found in the incoming profile.
        actual: AiAgentId,
    },

    /// An AI requirement operation failed.
    Requirement(RequirementError),

    /// Requirement analysis is incomplete.
    IncompleteRequirements(RequirementError),
}

impl fmt::Display for AgentCapabilitiesError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidOffer {
                capability,
                source,
            } => write!(
                formatter,
                "invalid capability offer `{capability}`: {source}"
            ),

            Self::DuplicateOffer {
                capability,
            } => write!(
                formatter,
                "agent capability `{capability}` has already been declared"
            ),

            Self::InvalidCapabilityIdentity {
                source,
            } => write!(
                formatter,
                "invalid agent capability identity: {source}"
            ),

            Self::AgentIdentityMismatch {
                expected,
                actual,
            } => write!(
                formatter,
                "agent capability profile identity mismatch: \
                 expected {expected}, received {actual}"
            ),

            Self::Requirement(source) => write!(
                formatter,
                "agent requirement error: {source}"
            ),

            Self::IncompleteRequirements(source) => write!(
                formatter,
                "agent capability analysis is incomplete: {source}"
            ),
        }
    }
}

impl Error for AgentCapabilitiesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidOffer { source, .. }
            | Self::InvalidCapabilityIdentity { source } => {
                Some(source)
            }

            Self::Requirement(source)
            | Self::IncompleteRequirements(source) => {
                Some(source)
            }

            Self::DuplicateOffer { .. }
            | Self::AgentIdentityMismatch { .. } => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_id(value: u128) -> AiAgentId {
        AiAgentId::new(value)
            .expect("valid agent identity")
    }

    fn category(value: &str) -> AiCapabilityCategory {
        AiCapabilityCategory::new(value)
            .expect("valid capability category")
    }

    fn ai_offer(
        name: &str,
    ) -> AiCapabilityDescriptor {
        AiCapabilityDescriptor::for_ai(
            name,
            category("agent_computation"),
            "Agent computational capability",
        )
        .expect("valid AI capability")
    }

    #[test]
    fn creates_agent_capability_profile() {
        let profile = AgentCapabilityProfile::new(agent_id(1));

        assert_eq!(profile.agent_id(), agent_id(1));
        assert!(profile.offers().is_empty());
        assert!(profile.requirements().is_empty());
        assert!(profile.is_analysis_complete());
        assert!(profile.is_empty());
    }

    #[test]
    fn declares_ai_capability_offer() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .add_offer(ai_offer("inference"))
            .expect("valid offer");

        let identity = ai_capability_id("inference")
            .expect("valid identity");

        assert!(profile.find_offer(&identity).is_some());
        assert_eq!(profile.offers().len(), 1);
    }

    #[test]
    fn supports_open_world_capabilities() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .add_offer(ai_offer("future_neural_computation_v9"))
            .expect("future capability");

        assert_eq!(profile.offers().len(), 1);
    }

    #[test]
    fn supports_external_domain_capabilities() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        let identity = CapabilityId::new(
            "quantum",
            "computation",
        )
        .expect("valid capability");

        let descriptor = AiCapabilityDescriptor::new(
            identity.clone(),
            category("quantum_assisted_computation"),
            "Quantum computation used by an agent",
        );

        profile
            .add_offer(descriptor)
            .expect("external capability offer");

        assert!(profile.find_offer(&identity).is_some());
    }

    #[test]
    fn duplicate_offers_are_rejected() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .add_offer(ai_offer("reasoning"))
            .expect("first offer");

        let result = profile.add_offer(ai_offer("reasoning"));

        assert!(matches!(
            result,
            Err(AgentCapabilitiesError::DuplicateOffer { .. })
        ));
    }

    #[test]
    fn capability_offers_can_be_removed() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .add_offer(ai_offer("planning"))
            .expect("valid offer");

        let identity = ai_capability_id("planning")
            .expect("valid identity");

        assert!(profile.remove_offer(&identity).is_some());
        assert!(profile.find_offer(&identity).is_none());
    }

    #[test]
    fn required_capabilities_preserve_their_strength() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .require_ai_capability(
                "inference",
                CapabilityStrength::Required,
            )
            .expect("valid requirement");

        let requirements = profile.requirements();

        assert_eq!(requirements.capabilities().len(), 1);

        assert_eq!(
            requirements.capabilities()[0].strength(),
            CapabilityStrength::Required
        );
    }

    #[test]
    fn preferred_capabilities_remain_advisory() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .require_ai_capability(
                "accelerated_inference",
                CapabilityStrength::Preferred,
            )
            .expect("valid preference");

        assert_eq!(
            profile.requirements().capabilities()[0].strength(),
            CapabilityStrength::Preferred
        );
    }

    #[test]
    fn allowed_and_forbidden_capabilities_remain_distinct() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .require_ai_capability(
                "distributed_execution",
                CapabilityStrength::Allowed,
            )
            .expect("allowed capability");

        profile
            .require_ai_capability(
                "untrusted_execution",
                CapabilityStrength::Forbidden,
            )
            .expect("forbidden capability");

        let requirements = profile.requirements().capabilities();

        assert_eq!(requirements.len(), 2);

        assert_eq!(
            requirements[0].strength(),
            CapabilityStrength::Allowed
        );

        assert_eq!(
            requirements[1].strength(),
            CapabilityStrength::Forbidden
        );
    }

    #[test]
    fn invalid_capability_names_are_rejected() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        assert!(
            profile
                .require_ai_capability(
                    "",
                    CapabilityStrength::Required,
                )
                .is_err()
        );
    }

    #[test]
    fn incomplete_analysis_is_not_certified() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .mark_analysis_incomplete(
                "An agent capability annotation remains unresolved",
            )
            .expect("valid incomplete reason");

        assert!(!profile.is_analysis_complete());

        assert!(profile.ensure_complete().is_err());
        assert!(profile.validate_complete().is_err());
    }

    #[test]
    fn complete_analysis_passes_completeness_check() {
        let profile = AgentCapabilityProfile::new(agent_id(1));

        profile
            .validate_complete()
            .expect("empty but completely analyzed profile");
    }

    #[test]
    fn profiles_cannot_be_merged_across_agents() {
        let mut first = AgentCapabilityProfile::new(agent_id(1));
        let second = AgentCapabilityProfile::new(agent_id(2));

        assert!(matches!(
            first.merge(second),
            Err(AgentCapabilitiesError::AgentIdentityMismatch { .. })
        ));
    }

    #[test]
    fn merging_preserves_requirements_and_offers() {
        let mut first = AgentCapabilityProfile::new(agent_id(1));
        let mut second = AgentCapabilityProfile::new(agent_id(1));

        first
            .add_offer(ai_offer("reasoning"))
            .expect("first offer");

        second
            .add_offer(ai_offer("planning"))
            .expect("second offer");

        second
            .require_ai_capability(
                "inference",
                CapabilityStrength::Required,
            )
            .expect("requirement");

        first.merge(second).expect("valid merge");

        assert_eq!(first.offers().len(), 2);
        assert_eq!(first.requirements().capabilities().len(), 1);
    }

    #[test]
    fn failed_merge_does_not_partially_modify_profile() {
        let mut first = AgentCapabilityProfile::new(agent_id(1));
        let mut second = AgentCapabilityProfile::new(agent_id(1));

        first
            .add_offer(ai_offer("reasoning"))
            .expect("first offer");

        second
            .add_offer(ai_offer("reasoning"))
            .expect("second profile offer");

        assert!(first.merge(second).is_err());

        assert_eq!(first.offers().len(), 1);
    }

    #[test]
    fn profile_validation_detects_deserialized_duplicates() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        profile.offers.push(ai_offer("reasoning"));
        profile.offers.push(ai_offer("reasoning"));

        assert!(profile.validate().is_err());
    }

    #[test]
    fn profile_supports_dynamically_growing_capability_sets() {
        let mut profile = AgentCapabilityProfile::new(agent_id(1));

        for index in 0..10_000_u64 {
            let name = format!("extension_capability_{index}");

            profile
                .add_offer(ai_offer(&name))
                .expect("unique capability");
        }

        assert_eq!(profile.offers().len(), 10_000);
        profile.validate().expect("valid capability profile");
    }

    #[test]
    fn capability_profiles_use_distinct_agent_identities() {
        let first = AgentCapabilityProfile::new(agent_id(1));
        let second = AgentCapabilityProfile::new(agent_id(2));

        assert_ne!(first.agent_id(), second.agent_id());
    }
}
