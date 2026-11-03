
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Coordination
//!
//! Path: `src/ai/agents/coordination.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, declarative coordination semantics for heterogeneous
//! AI agents and multi-agent computational systems.
//!
//! This module describes:
//!
//! - coordination strategies;
//! - coordination groups;
//! - heterogeneous agent participation;
//! - participation requirements;
//! - coordination relationships;
//! - group composition;
//! - delegation and supervision relationships;
//! - semantic requirement references;
//! - capability references;
//! - policy references;
//! - provenance references;
//! - structural validation.
//!
//! It describes computational coordination intent.
//!
//! It does not execute agents.
//!
//! ## Canonical ownership
//!
//! - `src/ai/ids.rs`
//!   Owns `AiAgentId`.
//!
//! - `src/ai/agents/definition.rs`
//!   Owns agent definitions.
//!
//! - `src/ai/agents/capabilities.rs`
//!   Owns agent capability profiles.
//!
//! - `src/ai/agents/communication.rs`
//!   Owns communication contracts.
//!
//! - `src/ai/agents/planning.rs`
//!   Owns agent planning.
//!
//! - `src/ai/agents/policies.rs`
//!   Owns agent policy interfaces.
//!
//! - `src/frontend/semantic/`
//!   Owns canonical semantic validation.
//!
//! - `src/zuir/dialects/ai/`
//!   Owns universal AI computational representation.
//!
//! - `src/hybrid/`
//!   Owns cross-domain composition.
//!
//! - `src/distributed/`
//!   Owns distributed placement, scheduling and execution.
//!
//! - `src/runtime/`
//!   Owns actual execution.
//!
//! ## Ownership boundaries
//!
//! This module owns declarative coordination contracts.
//!
//! It does not own:
//!
//! - source parsing;
//! - AST construction;
//! - canonical name resolution;
//! - capability discovery;
//! - capability verification;
//! - communication transport;
//! - network routing;
//! - agent memory;
//! - planning algorithms;
//! - scheduling;
//! - placement;
//! - resource allocation;
//! - authorization enforcement;
//! - agent execution;
//! - runtime state;
//! - physical hardware selection;
//! - universal IR.
//!
//! ## POCO-REAF
//!
//! Coordination descriptions are independent of:
//!
//! - processor architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - FPGA;
//! - ASIC;
//! - quantum processor;
//! - neuromorphic hardware;
//! - memory topology;
//! - network topology;
//! - operating system;
//! - distributed node count.
//!
//! No artificial universal maximum is imposed on:
//!
//! - agents;
//! - coordination groups;
//! - participants;
//! - relationships;
//! - coordination strategies;
//! - semantic references;
//! - policy references;
//! - provenance references.
//!
//! Physical resource limitations and operational budgets are
//! evaluated by downstream infrastructure.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical frontend AST
//!          |
//!          v
//!    Semantic analysis
//!          |
//!          v
//! Agent definitions
//!          |
//!          v
//! Agent capabilities
//!          |
//!          v
//! Agent communication
//!          |
//!          v
//! Agent coordination
//!          |
//!          v
//! Canonical ZUIR AI dialect
//!          |
//!          v
//! Hybrid/distributed planning
//!          |
//!          v
//! Runtime execution
//! ```
//!
//! ## Freeze criteria
//!
//! - Uses canonical AiAgentId.
//! - Supports heterogeneous participants.
//! - Coordination strategies are open-world.
//! - No fixed population ceiling exists.
//! - No scheduler is implemented.
//! - No transport implementation is duplicated.
//! - Relationships are structurally validated.
//! - Group composition is deterministic.
//! - Failed composition is atomic.
//! - Requirements and policies remain externally authoritative.
//! - No unsafe Rust is used.

// ============================================================================
// IMPORTS
// ============================================================================

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::ai::ids::AiAgentId;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for agent coordination operations.
pub type AgentCoordinationResult<T> =
    Result<T, AgentCoordinationError>;

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_name(
    value: &str,
    field: &'static str,
) -> AgentCoordinationResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(
            AgentCoordinationError::InvalidName {
                field,
                value: value.to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// COORDINATION NAME
// ============================================================================

/// A validated, open-world coordination identifier.
///
/// This can identify:
///
/// - groups;
//! - strategies;
//! - roles;
//! - relationship kinds;
//! - coordination extensions.
///
/// It is not a source-language name resolver.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CoordinationName(String);

impl CoordinationName {
    /// Construct a validated coordination name.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentCoordinationResult<Self> {
        let value = value.into();

        validate_name(&value, "coordination name")?;

        Ok(Self(value))
    }

    /// Return the underlying name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CoordinationName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// SEMANTIC REFERENCE
// ============================================================================

/// An opaque reference to an existing Zamani semantic entity.
///
/// The referenced entity may belong to:
///
/// - the semantic type system;
//! - capabilities;
//! - requirements;
//! - contracts;
//! - policies;
//! - provenance;
//! - planning;
//! - communication;
//! - hybrid computation;
//! - distributed computation.
///
/// This type does not resolve or implement the referenced entity.
///
/// Resolution remains the responsibility of the canonical frontend
/// and relevant subsystem.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CoordinationReference(String);

impl CoordinationReference {
    /// Construct a semantic reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentCoordinationResult<Self> {
        let value = value.into();

        validate_name(&value, "semantic reference")?;

        Ok(Self(value))
    }

    /// Return the unresolved reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CoordinationReference {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// COORDINATION STRATEGY
// ============================================================================

/// An open-world coordination strategy descriptor.
///
/// Strategies are referenced by logical identity.
///
/// Examples:
///
/// - `coordination::collaborative`
/// - `coordination::hierarchical`
/// - `coordination::consensus`
/// - `coordination::swarm`
/// - `coordination::federated`
/// - `coordination::negotiated`
/// - `coordination::custom`
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct CoordinationStrategy {
    name: CoordinationName,

    version: Option<String>,

    parameters: BTreeMap<String, String>,
}

impl CoordinationStrategy {
    /// Construct a coordination strategy descriptor.
    pub fn new(
        name: CoordinationName,
    ) -> Self {
        Self {
            name,
            version: None,
            parameters: BTreeMap::new(),
        }
    }

    /// Set the strategy version.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> AgentCoordinationResult<Self> {
        let version = version.into();

        validate_name(&version, "strategy version")?;

        self.version = Some(version);

        Ok(self)
    }

    /// Add an opaque strategy parameter.
    ///
    /// Parameter interpretation belongs to the registered strategy
    /// implementation.
    pub fn add_parameter(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentCoordinationResult<()> {
        let name = name.into();
        let value = value.into();

        validate_name(&name, "strategy parameter")?;

        if self.parameters.contains_key(&name) {
            return Err(
                AgentCoordinationError::DuplicateStrategyParameter {
                    name,
                },
            );
        }

        self.parameters.insert(name, value);

        Ok(())
    }

    /// Return the strategy identity.
    #[must_use]
    pub fn name(&self) -> &CoordinationName {
        &self.name
    }

    /// Return the optional strategy version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Return strategy parameters.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }

    /// Validate strategy structure.
    pub fn validate(
        &self,
    ) -> AgentCoordinationResult<()> {
        for name in self.parameters.keys() {
            validate_name(name, "strategy parameter")?;
        }

        Ok(())
    }
}

// ============================================================================
// PARTICIPATION REQUIREMENT
// ============================================================================

/// Required participation semantics for a coordination group.
///
/// This declares a logical requirement.
///
/// It does not implement consensus or quorum algorithms.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum ParticipationRequirement {
    /// Every declared participant is required.
    All,

    /// At least one participant must participate.
    Any,

    /// At least the specified number of participants must participate.
    AtLeast(usize),

    /// An externally registered participation rule.
    Custom(CoordinationName),
}

impl Default for ParticipationRequirement {
    fn default() -> Self {
        Self::All
    }
}

// ============================================================================
// PARTICIPANT ROLE
// ============================================================================

/// A declarative role assigned to a participating agent.
///
/// Roles are open-world and are not execution privileges.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CoordinationRole(CoordinationName);

impl CoordinationRole {
    /// Construct a coordination role.
    pub fn new(
        name: CoordinationName,
    ) -> Self {
        Self(name)
    }

    /// Return the role name.
    #[must_use]
    pub fn name(&self) -> &CoordinationName {
        &self.0
    }
}

// ============================================================================
// PARTICIPATION
// ============================================================================

/// Whether participation is mandatory or optional.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum ParticipationKind {
    /// The participant is required by the declaration.
    Required,

    /// The participant may participate when appropriate.
    Optional,
}

/// A declarative agent participation record.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct AgentParticipation {
    agent_id: AiAgentId,

    role: Option<CoordinationRole>,

    kind: ParticipationKind,

    attributes: BTreeMap<String, String>,
}

impl AgentParticipation {
    /// Construct a required participant.
    #[must_use]
    pub fn required(
        agent_id: AiAgentId,
    ) -> Self {
        Self {
            agent_id,
            role: None,
            kind: ParticipationKind::Required,
            attributes: BTreeMap::new(),
        }
    }

    /// Construct an optional participant.
    #[must_use]
    pub fn optional(
        agent_id: AiAgentId,
    ) -> Self {
        Self {
            agent_id,
            role: None,
            kind: ParticipationKind::Optional,
            attributes: BTreeMap::new(),
        }
    }

    /// Assign a logical coordination role.
    #[must_use]
    pub fn with_role(
        mut self,
        role: CoordinationRole,
    ) -> Self {
        self.role = Some(role);
        self
    }

    /// Add an opaque participant attribute.
    pub fn add_attribute(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentCoordinationResult<()> {
        let name = name.into();
        let value = value.into();

        validate_name(&name, "participant attribute")?;

        if self.attributes.contains_key(&name) {
            return Err(
                AgentCoordinationError::DuplicateParticipantAttribute {
                    agent_id: self.agent_id,
                    name,
                },
            );
        }

        self.attributes.insert(name, value);

        Ok(())
    }

    /// Return the participant identity.
    #[must_use]
    pub const fn agent_id(&self) -> AiAgentId {
        self.agent_id
    }

    /// Return the optional coordination role.
    #[must_use]
    pub fn role(&self) -> Option<&CoordinationRole> {
        self.role.as_ref()
    }

    /// Return the participation kind.
    #[must_use]
    pub const fn kind(&self) -> ParticipationKind {
        self.kind
    }

    /// Return participant attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

// ============================================================================
// RELATIONSHIP KIND
// ============================================================================

/// An open-world relationship classification.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CoordinationRelationshipKind(CoordinationName);

impl CoordinationRelationshipKind {
    /// Construct a relationship classification.
    pub fn new(
        name: CoordinationName,
    ) -> Self {
        Self(name)
    }

    /// Return the relationship classification.
    #[must_use]
    pub fn name(&self) -> &CoordinationName {
        &self.0
    }
}

// ============================================================================
// RELATIONSHIP
// ============================================================================

/// A directed semantic relationship between two agents.
///
/// Examples include:
///
/// - delegation;
//! - supervision;
//! - collaboration;
//! - dependency;
//! - coordination;
//! - synchronization.
///
/// This record does not execute the relationship.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct AgentCoordinationRelationship {
    source: AiAgentId,

    target: AiAgentId,

    kind: CoordinationRelationshipKind,

    condition: Option<CoordinationReference>,
}

impl AgentCoordinationRelationship {
    /// Construct a directed relationship.
    pub fn new(
        source: AiAgentId,
        target: AiAgentId,
        kind: CoordinationRelationshipKind,
    ) -> AgentCoordinationResult<Self> {
        if source == target {
            return Err(
                AgentCoordinationError::SelfRelationship {
                    agent_id: source,
                },
            );
        }

        Ok(Self {
            source,
            target,
            kind,
            condition: None,
        })
    }

    /// Attach a canonical semantic condition reference.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: CoordinationReference,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Return the relationship source.
    #[must_use]
    pub const fn source(&self) -> AiAgentId {
        self.source
    }

    /// Return the relationship target.
    #[must_use]
    pub const fn target(&self) -> AiAgentId {
        self.target
    }

    /// Return the relationship kind.
    #[must_use]
    pub fn kind(&self) -> &CoordinationRelationshipKind {
        &self.kind
    }

    /// Return the optional condition reference.
    #[must_use]
    pub fn condition(&self) -> Option<&CoordinationReference> {
        self.condition.as_ref()
    }
}

// ============================================================================
// COORDINATION GROUP
// ============================================================================

/// A declarative coordination group.
///
/// Groups can contain heterogeneous agents.
///
/// An agent may participate in multiple groups.
///
/// Group membership does not imply physical co-location.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct AgentCoordinationGroup {
    name: CoordinationName,

    strategy: CoordinationStrategy,

    participation: ParticipationRequirement,

    participants: BTreeMap<AiAgentId, AgentParticipation>,

    relationships: Vec<AgentCoordinationRelationship>,

    requirements: BTreeSet<CoordinationReference>,

    capabilities: BTreeSet<CoordinationReference>,

    policies: BTreeSet<CoordinationReference>,

    provenance: BTreeSet<CoordinationReference>,
}

impl AgentCoordinationGroup {
    /// Construct a coordination group.
    pub fn new(
        name: CoordinationName,
        strategy: CoordinationStrategy,
    ) -> Self {
        Self {
            name,
            strategy,
            participation: ParticipationRequirement::default(),
            participants: BTreeMap::new(),
            relationships: Vec::new(),
            requirements: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            policies: BTreeSet::new(),
            provenance: BTreeSet::new(),
        }
    }

    /// Set the participation requirement.
    #[must_use]
    pub fn with_participation_requirement(
        mut self,
        requirement: ParticipationRequirement,
    ) -> Self {
        self.participation = requirement;
        self
    }

    /// Return the group name.
    #[must_use]
    pub fn name(&self) -> &CoordinationName {
        &self.name
    }

    /// Return the coordination strategy.
    #[must_use]
    pub fn strategy(&self) -> &CoordinationStrategy {
        &self.strategy
    }

    /// Return the participation requirement.
    #[must_use]
    pub fn participation_requirement(
        &self,
    ) -> &ParticipationRequirement {
        &self.participation
    }

    /// Return the participants in deterministic identity order.
    #[must_use]
    pub fn participants(
        &self,
    ) -> &BTreeMap<AiAgentId, AgentParticipation> {
        &self.participants
    }

    /// Return the group relationships.
    #[must_use]
    pub fn relationships(
        &self,
    ) -> &[AgentCoordinationRelationship] {
        &self.relationships
    }

    /// Return shared semantic requirement references.
    #[must_use]
    pub fn requirements(
        &self,
    ) -> &BTreeSet<CoordinationReference> {
        &self.requirements
    }

    /// Return canonical capability references.
    #[must_use]
    pub fn capabilities(
        &self,
    ) -> &BTreeSet<CoordinationReference> {
        &self.capabilities
    }

    /// Return policy references.
    #[must_use]
    pub fn policies(
        &self,
    ) -> &BTreeSet<CoordinationReference> {
        &self.policies
    }

    /// Return provenance references.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> &BTreeSet<CoordinationReference> {
        &self.provenance
    }

    /// Add a participant.
    ///
    /// Duplicate agent identities are rejected.
    pub fn add_participant(
        &mut self,
        participant: AgentParticipation,
    ) -> AgentCoordinationResult<()> {
        let id = participant.agent_id();

        if self.participants.contains_key(&id) {
            return Err(
                AgentCoordinationError::DuplicateParticipant {
                    agent_id: id,
                },
            );
        }

        self.participants.insert(id, participant);

        Ok(())
    }

    /// Remove a participant.
    ///
    /// Relationships involving the participant are also removed.
    ///
    /// Returns the removed participant when present.
    pub fn remove_participant(
        &mut self,
        agent_id: AiAgentId,
    ) -> Option<AgentParticipation> {
        let removed = self.participants.remove(&agent_id)?;

        self.relationships.retain(
            |relationship| {
                relationship.source() != agent_id
                    && relationship.target() != agent_id
            },
        );

        Some(removed)
    }

    /// Find a participant by canonical identity.
    #[must_use]
    pub fn find_participant(
        &self,
        agent_id: AiAgentId,
    ) -> Option<&AgentParticipation> {
        self.participants.get(&agent_id)
    }

    /// Add a relationship.
    ///
    /// Both participants must already belong to the group.
    pub fn add_relationship(
        &mut self,
        relationship: AgentCoordinationRelationship,
    ) -> AgentCoordinationResult<()> {
        let source = relationship.source();
        let target = relationship.target();

        if !self.participants.contains_key(&source) {
            return Err(
                AgentCoordinationError::UnknownParticipant {
                    agent_id: source,
                },
            );
        }

        if !self.participants.contains_key(&target) {
            return Err(
                AgentCoordinationError::UnknownParticipant {
                    agent_id: target,
                },
            );
        }

        if self.relationships.contains(&relationship) {
            return Err(
                AgentCoordinationError::DuplicateRelationship,
            );
        }

        self.relationships.push(relationship);

        Ok(())
    }

    /// Add a shared semantic requirement reference.
    pub fn add_requirement(
        &mut self,
        reference: CoordinationReference,
    ) {
        self.requirements.insert(reference);
    }

    /// Add a canonical capability reference.
    ///
    /// The capability must be resolved and verified by the
    /// canonical capability infrastructure.
    pub fn add_capability(
        &mut self,
        reference: CoordinationReference,
    ) {
        self.capabilities.insert(reference);
    }

    /// Add a policy reference.
    ///
    /// Policy interpretation and enforcement remain external.
    pub fn add_policy(
        &mut self,
        reference: CoordinationReference,
    ) {
        self.policies.insert(reference);
    }

    /// Add a provenance reference.
    pub fn add_provenance(
        &mut self,
        reference: CoordinationReference,
    ) {
        self.provenance.insert(reference);
    }

    /// Validate the complete coordination group.
    pub fn validate(
        &self,
    ) -> AgentCoordinationResult<()> {
        self.strategy.validate()?;

        if self.participants.is_empty() {
            return Err(
                AgentCoordinationError::EmptyGroup {
                    name: self.name.clone(),
                },
            );
        }

        match self.participation {
            ParticipationRequirement::All
            | ParticipationRequirement::Any => {}

            ParticipationRequirement::AtLeast(required) => {
                if required == 0 {
                    return Err(
                        AgentCoordinationError::InvalidParticipationRequirement,
                    );
                }

                if required > self.participants.len() {
                    return Err(
                        AgentCoordinationError::InsufficientParticipants {
                            required,
                            available: self.participants.len(),
                        },
                    );
                }
            }

            ParticipationRequirement::Custom(_) => {}
        }

        let mut relationships = BTreeSet::new();

        for relationship in &self.relationships {
            if !self.participants.contains_key(
                &relationship.source(),
            ) {
                return Err(
                    AgentCoordinationError::UnknownParticipant {
                        agent_id: relationship.source(),
                    },
                );
            }

            if !self.participants.contains_key(
                &relationship.target(),
            ) {
                return Err(
                    AgentCoordinationError::UnknownParticipant {
                        agent_id: relationship.target(),
                    },
                );
            }

            if !relationships.insert(relationship) {
                return Err(
                    AgentCoordinationError::DuplicateRelationship,
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// COORDINATION PLAN
// ============================================================================

/// A complete declarative agent coordination description.
///
/// This is an AI-domain semantic structure.
///
/// It is not a scheduler or execution plan.
///
/// Actual placement, partitioning, scheduling and execution
/// are performed by their respective infrastructure.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
)]
pub struct AgentCoordinationPlan {
    groups: BTreeMap<CoordinationName, AgentCoordinationGroup>,
}

impl AgentCoordinationPlan {
    /// Construct an empty coordination plan.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all coordination groups.
    #[must_use]
    pub fn groups(
        &self,
    ) -> &BTreeMap<CoordinationName, AgentCoordinationGroup> {
        &self.groups
    }

    /// Return whether the plan has no groups.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Return the number of declared groups.
    #[must_use]
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Add a coordination group.
    ///
    /// The group is validated before insertion.
    pub fn add_group(
        &mut self,
        group: AgentCoordinationGroup,
    ) -> AgentCoordinationResult<()> {
        group.validate()?;

        let name = group.name().clone();

        if self.groups.contains_key(&name) {
            return Err(
                AgentCoordinationError::DuplicateGroup {
                    name,
                },
            );
        }

        self.groups.insert(name, group);

        Ok(())
    }

    /// Find a coordination group by name.
    #[must_use]
    pub fn find_group(
        &self,
        name: &CoordinationName,
    ) -> Option<&AgentCoordinationGroup> {
        self.groups.get(name)
    }

    /// Remove a coordination group.
    ///
    /// Returns the removed group when present.
    pub fn remove_group(
        &mut self,
        name: &CoordinationName,
    ) -> Option<AgentCoordinationGroup> {
        self.groups.remove(name)
    }

    /// Merge another coordination plan atomically.
    ///
    /// If any group conflicts or fails validation, the original
    /// plan remains unchanged.
    pub fn merge(
        &mut self,
        other: Self,
    ) -> AgentCoordinationResult<()> {
        let mut candidate = self.clone();

        for (_, group) in other.groups {
            candidate.add_group(group)?;
        }

        *self = candidate;

        Ok(())
    }

    /// Validate all coordination groups.
    ///
    /// Groups are evaluated in deterministic name order.
    pub fn validate(
        &self,
    ) -> AgentCoordinationResult<()> {
        for group in self.groups.values() {
            group.validate()?;
        }

        Ok(())
    }

    /// Return all distinct participating agent identities.
    ///
    /// An agent belonging to multiple groups appears only once.
    #[must_use]
    pub fn participant_ids(&self) -> BTreeSet<AiAgentId> {
        self.groups
            .values()
            .flat_map(|group| group.participants.keys().copied())
            .collect()
    }

    /// Return the number of distinct participating agents.
    #[must_use]
    pub fn participant_count(&self) -> usize {
        self.participant_ids().len()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by declarative agent coordination.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum AgentCoordinationError {
    /// An invalid name or reference was supplied.
    InvalidName {
        /// The affected field.
        field: &'static str,

        /// The invalid value.
        value: String,
    },

    /// A strategy parameter was declared more than once.
    DuplicateStrategyParameter {
        /// The parameter name.
        name: String,
    },

    /// A participant was declared more than once.
    DuplicateParticipant {
        /// The participant identity.
        agent_id: AiAgentId,
    },

    /// A participant attribute was declared more than once.
    DuplicateParticipantAttribute {
        /// The participant identity.
        agent_id: AiAgentId,

        /// The attribute name.
        name: String,
    },

    /// A relationship references an unknown participant.
    UnknownParticipant {
        /// The unresolved participant identity.
        agent_id: AiAgentId,
    },

    /// An agent cannot form a relationship with itself.
    SelfRelationship {
        /// The participant identity.
        agent_id: AiAgentId,
    },

    /// A relationship was declared more than once.
    DuplicateRelationship,

    /// A coordination group contains no participants.
    EmptyGroup {
        /// The group name.
        name: CoordinationName,
    },

    /// A participation threshold is invalid.
    InvalidParticipationRequirement,

    /// The group does not contain enough participants.
    InsufficientParticipants {
        /// Required participant count.
        required: usize,

        /// Available participant count.
        available: usize,
    },

    /// A coordination group was declared more than once.
    DuplicateGroup {
        /// The group name.
        name: CoordinationName,
    },
}

impl fmt::Display for AgentCoordinationError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidName { field, value } => {
                write!(
                    f,
                    "Invalid {field}: {value:?}"
                )
            }

            Self::DuplicateStrategyParameter { name } => {
                write!(
                    f,
                    "Duplicate coordination strategy parameter: {name}"
                )
            }

            Self::DuplicateParticipant { agent_id } => {
                write!(
                    f,
                    "Duplicate coordination participant: {agent_id}"
                )
            }

            Self::DuplicateParticipantAttribute {
                agent_id,
                name,
            } => {
                write!(
                    f,
                    "Duplicate attribute {name} for agent {agent_id}"
                )
            }

            Self::UnknownParticipant { agent_id } => {
                write!(
                    f,
                    "Unknown coordination participant: {agent_id}"
                )
            }

            Self::SelfRelationship { agent_id } => {
                write!(
                    f,
                    "Self-relationship is invalid for agent {agent_id}"
                )
            }

            Self::DuplicateRelationship => {
                f.write_str(
                    "Duplicate agent coordination relationship"
                )
            }

            Self::EmptyGroup { name } => {
                write!(
                    f,
                    "Coordination group {name} has no participants"
                )
            }

            Self::InvalidParticipationRequirement => {
                f.write_str(
                    "Participation requirement must be greater than zero"
                )
            }

            Self::InsufficientParticipants {
                required,
                available,
            } => {
                write!(
                    f,
                    "Coordination requires {required} participants, \
                     but only {available} are declared"
                )
            }

            Self::DuplicateGroup { name } => {
                write!(
                    f,
                    "Duplicate coordination group: {name}"
                )
            }
        }
    }
}

impl Error for AgentCoordinationError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(value: u128) -> AiAgentId {
        AiAgentId::new(value).expect("valid agent identity")
    }

    fn name(value: &str) -> CoordinationName {
        CoordinationName::new(value).expect("valid name")
    }

    fn strategy() -> CoordinationStrategy {
        CoordinationStrategy::new(
            name("coordination::collaborative"),
        )
    }

    fn participant(
        id: u128,
    ) -> AgentParticipation {
        AgentParticipation::required(agent(id))
    }

    fn group(
        group_name: &str,
        participants: &[u128],
    ) -> AgentCoordinationGroup {
        let mut group = AgentCoordinationGroup::new(
            name(group_name),
            strategy(),
        );

        for id in participants {
            group.add_participant(
                participant(*id),
            ).expect("participant insertion");
        }

        group
    }

    #[test]
    fn accepts_valid_coordination_name() {
        let value = name("coordination::swarm");

        assert_eq!(
            value.as_str(),
            "coordination::swarm"
        );
    }

    #[test]
    fn rejects_empty_coordination_name() {
        assert!(CoordinationName::new("").is_err());
    }

    #[test]
    fn rejects_whitespace_name() {
        assert!(CoordinationName::new(" invalid").is_err());
        assert!(CoordinationName::new("invalid ").is_err());
    }

    #[test]
    fn accepts_open_world_strategies() {
        let custom = CoordinationStrategy::new(
            name("future::quantum_neuro_symbolic"),
        );

        assert_eq!(
            custom.name().as_str(),
            "future::quantum_neuro_symbolic"
        );
    }

    #[test]
    fn strategy_parameters_are_unique() {
        let mut value = strategy();

        value.add_parameter("mode", "adaptive")
            .expect("first parameter");

        assert!(
            value.add_parameter("mode", "static").is_err()
        );

        assert_eq!(
            value.parameters().get("mode").map(String::as_str),
            Some("adaptive")
        );
    }

    #[test]
    fn supports_required_and_optional_participants() {
        let mut value = group("research", &[]);

        value.add_participant(
            AgentParticipation::required(agent(1)),
        ).expect("required participant");

        value.add_participant(
            AgentParticipation::optional(agent(2)),
        ).expect("optional participant");

        assert_eq!(
            value.participants().len(),
            2
        );

        assert_eq!(
            value.find_participant(agent(1)).unwrap().kind(),
            ParticipationKind::Required
        );

        assert_eq!(
            value.find_participant(agent(2)).unwrap().kind(),
            ParticipationKind::Optional
        );
    }

    #[test]
    fn rejects_duplicate_participants() {
        let mut value = group("research", &[1]);

        assert!(
            value.add_participant(participant(1)).is_err()
        );
    }

    #[test]
    fn supports_agent_roles() {
        let role = CoordinationRole::new(
            name("researcher"),
        );

        let value = participant(1).with_role(role);

        assert_eq!(
            value.role().unwrap().name().as_str(),
            "researcher"
        );
    }

    #[test]
    fn supports_participant_attributes() {
        let mut value = participant(1);

        value.add_attribute(
            "domain",
            "quantum",
        ).expect("attribute");

        assert_eq!(
            value.attributes().get("domain"),
            Some(&"quantum".to_owned())
        );
    }

    #[test]
    fn rejects_duplicate_participant_attributes() {
        let mut value = participant(1);

        value.add_attribute("domain", "classical")
            .expect("first attribute");

        assert!(
            value.add_attribute("domain", "quantum").is_err()
        );
    }

    #[test]
    fn rejects_self_relationships() {
        let result = AgentCoordinationRelationship::new(
            agent(1),
            agent(1),
            CoordinationRelationshipKind::new(
                name("delegation"),
            ),
        );

        assert!(matches!(
            result,
            Err(AgentCoordinationError::SelfRelationship { .. })
        ));
    }

    #[test]
    fn accepts_valid_relationships() {
        let relationship = AgentCoordinationRelationship::new(
            agent(1),
            agent(2),
            CoordinationRelationshipKind::new(
                name("delegation"),
            ),
        );

        assert!(relationship.is_ok());
    }

    #[test]
    fn rejects_relationship_with_unknown_source() {
        let mut value = group("research", &[2]);

        let relationship = AgentCoordinationRelationship::new(
            agent(1),
            agent(2),
            CoordinationRelationshipKind::new(
                name("delegation"),
            ),
        ).expect("relationship");

        assert!(matches!(
            value.add_relationship(relationship),
            Err(AgentCoordinationError::UnknownParticipant { .. })
        ));
    }

    #[test]
    fn rejects_relationship_with_unknown_target() {
        let mut value = group("research", &[1]);

        let relationship = AgentCoordinationRelationship::new(
            agent(1),
            agent(2),
            CoordinationRelationshipKind::new(
                name("supervision"),
            ),
        ).expect("relationship");

        assert!(value.add_relationship(relationship).is_err());
    }

    #[test]
    fn accepts_valid_group() {
        let value = group("research", &[1, 2, 3]);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_empty_group() {
        let value = group("empty", &[]);

        assert!(matches!(
            value.validate(),
            Err(AgentCoordinationError::EmptyGroup { .. })
        ));
    }

    #[test]
    fn validates_participation_threshold() {
        let value = group("research", &[1, 2])
            .with_participation_requirement(
                ParticipationRequirement::AtLeast(3),
            );

        assert!(matches!(
            value.validate(),
            Err(AgentCoordinationError::InsufficientParticipants { .. })
        ));
    }

    #[test]
    fn rejects_zero_participation_threshold() {
        let value = group("research", &[1])
            .with_participation_requirement(
                ParticipationRequirement::AtLeast(0),
            );

        assert!(matches!(
            value.validate(),
            Err(
                AgentCoordinationError::InvalidParticipationRequirement
            )
        ));
    }

    #[test]
    fn accepts_custom_participation_requirement() {
        let value = group("research", &[1, 2])
            .with_participation_requirement(
                ParticipationRequirement::Custom(
                    name("future::weighted_consensus"),
                ),
            );

        assert!(value.validate().is_ok());
    }

    #[test]
    fn accepts_semantic_references() {
        let mut value = group("hybrid", &[1, 2]);

        value.add_requirement(
            CoordinationReference::new(
                "semantic::resource_requirement",
            ).expect("reference"),
        );

        value.add_capability(
            CoordinationReference::new(
                "quantum::computation",
            ).expect("reference"),
        );

        value.add_policy(
            CoordinationReference::new(
                "security::agent_policy",
            ).expect("reference"),
        );

        value.add_provenance(
            CoordinationReference::new(
                "provenance::source_declaration",
            ).expect("reference"),
        );

        assert_eq!(value.requirements().len(), 1);
        assert_eq!(value.capabilities().len(), 1);
        assert_eq!(value.policies().len(), 1);
        assert_eq!(value.provenance().len(), 1);
    }

    #[test]
    fn supports_multiple_coordination_groups() {
        let mut plan = AgentCoordinationPlan::new();

        plan.add_group(group("research", &[1, 2]))
            .expect("research group");

        plan.add_group(group("verification", &[2, 3]))
            .expect("verification group");

        assert_eq!(plan.len(), 2);
        assert_eq!(plan.participant_count(), 3);
    }

    #[test]
    fn agent_can_belong_to_multiple_groups() {
        let mut plan = AgentCoordinationPlan::new();

        plan.add_group(group("research", &[1, 2]))
            .expect("first group");

        plan.add_group(group("analysis", &[1, 3]))
            .expect("second group");

        assert_eq!(plan.participant_count(), 3);
    }

    #[test]
    fn rejects_duplicate_group_names() {
        let mut plan = AgentCoordinationPlan::new();

        plan.add_group(group("research", &[1]))
            .expect("first group");

        assert!(plan.add_group(
            group("research", &[2])
        ).is_err());
    }

    #[test]
    fn removes_participant_and_relationships() {
        let mut value = group("research", &[1, 2]);

        value.add_relationship(
            AgentCoordinationRelationship::new(
                agent(1),
                agent(2),
                CoordinationRelationshipKind::new(
                    name("delegation"),
                ),
            ).expect("relationship"),
        ).expect("relationship insertion");

        assert_eq!(value.relationships().len(), 1);

        value.remove_participant(agent(2));

        assert_eq!(value.participants().len(), 1);
        assert!(value.relationships().is_empty());
    }

    #[test]
    fn plan_validation_is_deterministic() {
        let mut plan = AgentCoordinationPlan::new();

        plan.add_group(group("zeta", &[1, 2]))
            .expect("group");

        plan.add_group(group("alpha", &[3, 4]))
            .expect("group");

        assert!(plan.validate().is_ok());

        let first = plan.participant_ids();
        let second = plan.participant_ids();

        assert_eq!(first, second);
    }

    #[test]
    fn merge_is_atomic_on_conflict() {
        let mut original = AgentCoordinationPlan::new();

        original.add_group(group("research", &[1]))
            .expect("original group");

        let mut incoming = AgentCoordinationPlan::new();

        incoming.add_group(group("verification", &[2]))
            .expect("incoming group");

        incoming.add_group(group("research", &[3]))
            .expect("conflicting group");

        assert!(original.merge(incoming).is_err());

        assert_eq!(original.len(), 1);

        assert!(
            original.find_group(&name("verification")).is_none()
        );
    }

    #[test]
    fn merge_succeeds_without_conflicts() {
        let mut first = AgentCoordinationPlan::new();

        first.add_group(group("research", &[1]))
            .expect("group");

        let mut second = AgentCoordinationPlan::new();

        second.add_group(group("analysis", &[2]))
            .expect("group");

        first.merge(second).expect("merge");

        assert_eq!(first.len(), 2);
        assert_eq!(first.participant_count(), 2);
    }

    #[test]
    fn supports_large_identity_values() {
        let id = agent(u128::MAX);

        let mut value = group("large", &[]);

        value.add_participant(
            AgentParticipation::required(id),
        ).expect("participant");

        assert!(value.find_participant(id).is_some());
    }

    #[test]
    fn supports_many_dynamic_participants() {
        let mut value = group("scalable", &[]);

        for id in 1..=10_000_u128 {
            value.add_participant(participant(id))
                .expect("participant insertion");
        }

        assert_eq!(value.participants().len(), 10_000);

        assert!(value.validate().is_ok());
    }
}
