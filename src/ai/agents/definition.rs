#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Agent Definitions
//!
//! Path: `src/ai/agents/definition.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines the declarative computational identity and structure of an
//! AI agent in the Zamani programming language.
//!
//! An agent definition describes:
//!
//! - agent identity;
//! - agent name;
//! - agent responsibilities;
//! - agent input and output interfaces;
//! - declarative lifecycle states;
//! - permitted lifecycle transitions;
//! - definition metadata.
//!
//! This module describes what an agent is.
//!
//! It does not execute an agent.
//!
//! ## Canonical ownership
//!
//! The following repository components remain authoritative:
//!
//! - `grammar/ai/agents.g4`:
//!   Agent source syntax.
//!
//! - `grammar/ai/multi-agent.g4`:
//!   Multi-agent composition syntax.
//!
//! - `grammar/ai/communication.g4`:
//!   Agent communication syntax.
//!
//! - `grammar/ai/planning.g4`:
//!   Agent planning syntax.
//!
//! - `src/frontend/semantic/domains/ai.rs`:
//!   AI semantic analysis.
//!
//! - `src/ai/ids.rs`:
//!   AI-domain identifiers.
//!
//! - `src/zuir/dialects/ai/`:
//!   Universal AI computational representation.
//!
//! - `src/distributed/`:
//!   Distributed execution and communication.
//!
//! - `src/runtime/`:
//!   Actual execution and runtime lifecycle.
//!
//! ## Ownership boundaries
//!
//! This module owns declarative agent definitions.
//!
//! It does not own:
//!
//! - source parsing;
//! - AST construction;
//! - semantic name resolution;
//! - model execution;
//! - tool execution;
//! - communication transport;
//! - distributed scheduling;
//! - agent coordination algorithms;
//! - persistent agent memory;
//! - authorization;
//! - physical resource allocation;
//! - hardware selection;
//! - execution state;
//! - universal AI IR.
//!
//! ## Scalability
//!
//! This module imposes no artificial limits on:
//!
//! - the number of agent definitions;
//! - the number of responsibilities;
//! - the number of interface ports;
//! - the number of lifecycle states;
//! - the number of lifecycle transitions;
//! - the number of metadata entries;
//! - the size of an agent population.
//!
//! Actual memory availability, representation limits, configured
//! operational budgets, and execution feasibility remain explicit
//! implementation or runtime concerns.
//!
//! This is not a promise of physically infinite computation.
//!
//! ## POCO-REAF
//!
//! Agent definitions are independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - quantum processor;
//! - FPGA;
//! - ASIC;
//! - memory topology;
//! - network topology;
//! - distributed node count;
//! - operating system;
//! - execution backend.
//!
//! An agent definition describes computational intent.
//!
//! The compiler and execution infrastructure determine realization.
//!
//! ## Integration
//!
//! `src/ai/ids.rs`
//!     -> AiAgentId
//!
//! `src/ai/agents/capabilities.rs`
//!     -> Agent capability declarations
//!
//! `src/ai/agents/communication.rs`
//!     -> Agent communication contracts
//!
//! `src/ai/agents/coordination.rs`
//!     -> Agent coordination
//!
//! `src/ai/agents/planning.rs`
//!     -> Agent planning
//!
//! `src/ai/agents/memory.rs`
//!     -> Agent memory interfaces
//!
//! `src/ai/agents/tools.rs`
//!     -> Agent tool interfaces
//!
//! `src/ai/agents/policies.rs`
//!     -> Agent policy references
//!
//! `src/ai/agents/mod.rs`
//!     -> Agent subsystem composition root
//!
//! `src/frontend/semantic/domains/ai.rs`
//!     -> Canonical semantic validation
//!
//! `src/zuir/dialects/ai/`
//!     -> Universal AI computational representation
//!
//! `src/distributed/`
//!     -> Communication, scheduling, and execution
//!
//! `src/runtime/`
//!     -> Actual lifecycle execution
//!
//! ## Freeze criteria
//!
//! - Agent identity uses AiAgentId.
//! - No duplicate universal identifier system.
//! - Definitions are target-independent.
//! - Responsibilities are extensible.
//! - Interfaces are declarative.
//! - Lifecycle declarations are separate from runtime state.
//! - Invalid names and definitions are rejected.
//! - Duplicate lifecycle states are rejected.
//! - Invalid lifecycle transitions are rejected.
//! - No fixed agent population ceiling exists.
//! - No unsafe Rust is used.
//! - Serialization preserves definition structure.
//! - Structural validation is deterministic.

use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, BTreeSet};

use std::error::Error;

use std::fmt;

use crate::ai::ids::AiAgentId;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for agent-definition operations.
pub type AgentDefinitionResult<T> = Result<T, AgentDefinitionError>;

// ============================================================================
// AGENT NAME
// ============================================================================

/// A validated, target-independent agent name.
///
/// This is a domain-level identifier.
///
/// Canonical source-language name resolution remains owned by the
/// frontend semantic subsystem.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct AgentName(String);

impl AgentName {
    /// Construct a validated agent name.
    ///
    /// Names must not be empty or contain leading or trailing
    /// whitespace.
    ///
    /// The complete language-level identifier grammar is owned by
    /// the frontend semantic subsystem.
    pub fn new(value: impl Into<String>) -> AgentDefinitionResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(AgentDefinitionError::EmptyAgentName);
        }

        if value.trim() != value {
            return Err(AgentDefinitionError::InvalidAgentName(
                value,
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(AgentDefinitionError::InvalidAgentName(
                value,
            ));
        }

        Ok(Self(value))
    }

    /// Return the agent name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the name and return its underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for AgentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// QUALIFIED AGENT NAME
// ============================================================================

/// A qualified agent name.
///
/// This identifies an agent within a logical namespace.
///
/// Namespace resolution and collision checking against the complete
/// Zamani program remain the responsibility of semantic analysis.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct QualifiedAgentName {
    namespace: Vec<String>,
    name: AgentName,
}

impl QualifiedAgentName {
    /// Construct a qualified agent name.
    pub fn new(
        namespace: Vec<String>,
        name: AgentName,
    ) -> AgentDefinitionResult<Self> {
        for component in &namespace {
            if component.is_empty()
                || component.trim() != component
                || component.chars().any(char::is_control)
            {
                return Err(
                    AgentDefinitionError::InvalidNamespaceComponent(
                        component.clone(),
                    ),
                );
            }
        }

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the namespace components.
    pub fn namespace(&self) -> &[String] {
        &self.namespace
    }

    /// Return the unqualified agent name.
    pub fn name(&self) -> &AgentName {
        &self.name
    }

    /// Return a stable textual representation.
    ///
    /// The representation is intended for diagnostics and display.
    /// It is not a persistent global identity.
    pub fn display_name(&self) -> String {
        if self.namespace.is_empty() {
            return self.name.to_string();
        }

        let mut components = self.namespace.clone();

        components.push(self.name.as_str().to_owned());

        components.join("::")
    }
}

// ============================================================================
// RESPONSIBILITY
// ============================================================================

/// A declarative responsibility assigned to an agent.
///
/// Responsibilities describe computational intent.
///
/// They do not execute work or grant capabilities.
///
/// Examples include:
///
/// - `research`
/// - `planning`
/// - `inference`
/// - `verification`
/// - `coordination`
/// - `data_analysis`
///
/// The set of responsibility names is open-world.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct AgentResponsibility(String);

impl AgentResponsibility {
    /// Construct a responsibility identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentDefinitionResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(
                AgentDefinitionError::EmptyResponsibility,
            );
        }

        if value.trim() != value {
            return Err(
                AgentDefinitionError::InvalidResponsibility(
                    value,
                ),
            );
        }

        if value.chars().any(char::is_control) {
            return Err(
                AgentDefinitionError::InvalidResponsibility(
                    value,
                ),
            );
        }

        Ok(Self(value))
    }

    /// Return the responsibility identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// INTERFACE PORT
// ============================================================================

/// Direction of an agent interface port.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AgentPortDirection {
    /// Information accepted by the agent.
    Input,

    /// Information produced by the agent.
    Output,
}

/// A declarative agent interface port.
///
/// `type_reference` is an opaque reference to a type in the canonical
/// Zamani type system.
///
/// This module does not create a competing type system.
///
/// The frontend semantic subsystem resolves and validates the
/// reference.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentInterfacePort {
    name: String,

    type_reference: String,

    direction: AgentPortDirection,

    optional: bool,

    description: Option<String>,
}

impl AgentInterfacePort {
    /// Construct an agent interface port.
    pub fn new(
        name: impl Into<String>,
        type_reference: impl Into<String>,
        direction: AgentPortDirection,
    ) -> AgentDefinitionResult<Self> {
        let name = name.into();

        let type_reference = type_reference.into();

        validate_non_empty_identifier(
            &name,
            AgentDefinitionError::InvalidInterfacePortName,
        )?;

        validate_non_empty_identifier(
            &type_reference,
            AgentDefinitionError::InvalidTypeReference,
        )?;

        Ok(Self {
            name,
            type_reference,
            direction,
            optional: false,
            description: None,
        })
    }

    /// Mark this port as optional.
    pub fn optional(mut self) -> Self {
        self.optional = true;

        self
    }

    /// Attach descriptive metadata.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());

        self
    }

    /// Return the port name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical type reference.
    pub fn type_reference(&self) -> &str {
        &self.type_reference
    }

    /// Return the port direction.
    pub fn direction(&self) -> AgentPortDirection {
        self.direction
    }

    /// Return whether this port is optional.
    pub fn is_optional(&self) -> bool {
        self.optional
    }

    /// Return the optional description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// AGENT INTERFACE
// ============================================================================

/// Declarative input and output interfaces of an agent.
///
/// Interface ports are ordered because declaration order may be
/// meaningful for source mapping and diagnostics.
///
/// Interface compatibility is checked by semantic analysis.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentInterface {
    ports: Vec<AgentInterfacePort>,
}

impl AgentInterface {
    /// Construct an empty interface.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an interface from ports.
    pub fn from_ports(
        ports: Vec<AgentInterfacePort>,
    ) -> AgentDefinitionResult<Self> {
        let interface = Self {
            ports,
        };

        interface.validate()?;

        Ok(interface)
    }

    /// Add an interface port.
    ///
    /// Duplicate names are rejected across both input and output
    /// directions to preserve a single unambiguous port namespace.
    pub fn add_port(
        &mut self,
        port: AgentInterfacePort,
    ) -> AgentDefinitionResult<()> {
        if self.ports.iter().any(
            |existing| existing.name == port.name,
        ) {
            return Err(
                AgentDefinitionError::DuplicateInterfacePort(
                    port.name,
                ),
            );
        }

        self.ports.push(port);

        Ok(())
    }

    /// Return all interface ports.
    pub fn ports(&self) -> &[AgentInterfacePort] {
        &self.ports
    }

    /// Return input ports.
    pub fn inputs(
        &self,
    ) -> impl Iterator<Item = &AgentInterfacePort> {
        self.ports.iter().filter(
            |port| port.direction == AgentPortDirection::Input,
        )
    }

    /// Return output ports.
    pub fn outputs(
        &self,
    ) -> impl Iterator<Item = &AgentInterfacePort> {
        self.ports.iter().filter(
            |port| port.direction == AgentPortDirection::Output,
        )
    }

    /// Validate interface structure.
    pub fn validate(&self) -> AgentDefinitionResult<()> {
        let mut names = BTreeSet::new();

        for port in &self.ports {
            validate_non_empty_identifier(
                &port.name,
                AgentDefinitionError::InvalidInterfacePortName,
            )?;

            validate_non_empty_identifier(
                &port.type_reference,
                AgentDefinitionError::InvalidTypeReference,
            )?;

            if !names.insert(port.name.as_str()) {
                return Err(
                    AgentDefinitionError::DuplicateInterfacePort(
                        port.name.clone(),
                    ),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// LIFECYCLE STATE
// ============================================================================

/// A declarative lifecycle state.
///
/// This does not represent the actual state of a running agent.
///
/// Runtime lifecycle state belongs to the execution infrastructure.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct AgentLifecycleState(String);

impl AgentLifecycleState {
    /// Construct a lifecycle state name.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentDefinitionResult<Self> {
        let value = value.into();

        validate_non_empty_identifier(
            &value,
            AgentDefinitionError::InvalidLifecycleState,
        )?;

        Ok(Self(value))
    }

    /// Return the state name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// LIFECYCLE TRANSITION
// ============================================================================

/// A declarative lifecycle transition.
///
/// This describes a permitted state transition.
///
/// It does not execute the transition.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentLifecycleTransition {
    from: AgentLifecycleState,

    to: AgentLifecycleState,

    condition: Option<String>,
}

impl AgentLifecycleTransition {
    /// Construct a lifecycle transition.
    pub fn new(
        from: AgentLifecycleState,
        to: AgentLifecycleState,
    ) -> AgentDefinitionResult<Self> {
        if from == to {
            return Err(
                AgentDefinitionError::SelfTransition(
                    from.as_str().to_owned(),
                ),
            );
        }

        Ok(Self {
            from,
            to,
            condition: None,
        })
    }

    /// Attach an opaque condition reference.
    ///
    /// The condition is resolved by semantic analysis and evaluated
    /// by the appropriate execution infrastructure.
    pub fn with_condition(
        mut self,
        condition: impl Into<String>,
    ) -> AgentDefinitionResult<Self> {
        let condition = condition.into();

        validate_non_empty_identifier(
            &condition,
            AgentDefinitionError::InvalidTransitionCondition,
        )?;

        self.condition = Some(condition);

        Ok(self)
    }

    /// Return the source state.
    pub fn from(&self) -> &AgentLifecycleState {
        &self.from
    }

    /// Return the destination state.
    pub fn to(&self) -> &AgentLifecycleState {
        &self.to
    }

    /// Return the optional condition reference.
    pub fn condition(&self) -> Option<&str> {
        self.condition.as_deref()
    }
}

// ============================================================================
// AGENT LIFECYCLE
// ============================================================================

/// Declarative lifecycle structure for an agent.
///
/// The lifecycle is a graph of states and permitted transitions.
///
/// It is not a runtime state machine implementation.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentLifecycle {
    states: BTreeSet<AgentLifecycleState>,

    initial_state: Option<AgentLifecycleState>,

    transitions: Vec<AgentLifecycleTransition>,
}

impl AgentLifecycle {
    /// Construct an empty lifecycle declaration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a lifecycle state.
    pub fn add_state(
        &mut self,
        state: AgentLifecycleState,
    ) -> AgentDefinitionResult<()> {
        if !self.states.insert(state.clone()) {
            return Err(
                AgentDefinitionError::DuplicateLifecycleState(
                    state.as_str().to_owned(),
                ),
            );
        }

        Ok(())
    }

    /// Set the initial lifecycle state.
    pub fn set_initial_state(
        &mut self,
        state: AgentLifecycleState,
    ) -> AgentDefinitionResult<()> {
        if !self.states.contains(&state) {
            return Err(
                AgentDefinitionError::UnknownLifecycleState(
                    state.as_str().to_owned(),
                ),
            );
        }

        if self.initial_state.is_some() {
            return Err(
                AgentDefinitionError::InitialStateAlreadySet,
            );
        }

        self.initial_state = Some(state);

        Ok(())
    }

    /// Add a lifecycle transition.
    pub fn add_transition(
        &mut self,
        transition: AgentLifecycleTransition,
    ) -> AgentDefinitionResult<()> {
        if !self.states.contains(&transition.from) {
            return Err(
                AgentDefinitionError::UnknownLifecycleState(
                    transition.from.as_str().to_owned(),
                ),
            );
        }

        if !self.states.contains(&transition.to) {
            return Err(
                AgentDefinitionError::UnknownLifecycleState(
                    transition.to.as_str().to_owned(),
                ),
            );
        }

        if self.transitions.iter().any(
            |existing| {
                existing.from == transition.from
                    && existing.to == transition.to
                    && existing.condition == transition.condition
            },
        ) {
            return Err(
                AgentDefinitionError::DuplicateLifecycleTransition,
            );
        }

        self.transitions.push(transition);

        Ok(())
    }

    /// Return lifecycle states.
    pub fn states(
        &self,
    ) -> &BTreeSet<AgentLifecycleState> {
        &self.states
    }

    /// Return the initial state.
    pub fn initial_state(
        &self,
    ) -> Option<&AgentLifecycleState> {
        self.initial_state.as_ref()
    }

    /// Return lifecycle transitions.
    pub fn transitions(
        &self,
    ) -> &[AgentLifecycleTransition] {
        &self.transitions
    }

    /// Validate lifecycle structure.
    pub fn validate(&self) -> AgentDefinitionResult<()> {
        if let Some(initial_state) = &self.initial_state {
            if !self.states.contains(initial_state) {
                return Err(
                    AgentDefinitionError::UnknownLifecycleState(
                        initial_state.as_str().to_owned(),
                    ),
                );
            }
        }

        for transition in &self.transitions {
            if !self.states.contains(&transition.from) {
                return Err(
                    AgentDefinitionError::UnknownLifecycleState(
                        transition.from.as_str().to_owned(),
                    ),
                );
            }

            if !self.states.contains(&transition.to) {
                return Err(
                    AgentDefinitionError::UnknownLifecycleState(
                        transition.to.as_str().to_owned(),
                    ),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// AGENT DEFINITION
// ============================================================================

/// A complete declarative AI agent definition.
///
/// This type represents the structural identity of an agent.
///
/// It is not an executing agent instance.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentDefinition {
    id: AiAgentId,

    name: QualifiedAgentName,

    responsibilities: BTreeSet<AgentResponsibility>,

    interface: AgentInterface,

    lifecycle: AgentLifecycle,

    metadata: BTreeMap<String, String>,
}

impl AgentDefinition {
    /// Construct a new agent definition.
    pub fn new(
        id: AiAgentId,
        name: QualifiedAgentName,
    ) -> Self {
        Self {
            id,
            name,
            responsibilities: BTreeSet::new(),
            interface: AgentInterface::new(),
            lifecycle: AgentLifecycle::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Return the canonical AI agent identity.
    pub fn id(&self) -> AiAgentId {
        self.id
    }

    /// Return the qualified agent name.
    pub fn name(&self) -> &QualifiedAgentName {
        &self.name
    }

    /// Return the agent responsibilities.
    pub fn responsibilities(
        &self,
    ) -> &BTreeSet<AgentResponsibility> {
        &self.responsibilities
    }

    /// Return the agent interface.
    pub fn interface(&self) -> &AgentInterface {
        &self.interface
    }

    /// Return the declarative lifecycle.
    pub fn lifecycle(&self) -> &AgentLifecycle {
        &self.lifecycle
    }

    /// Return metadata.
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Add a responsibility.
    pub fn add_responsibility(
        &mut self,
        responsibility: AgentResponsibility,
    ) {
        self.responsibilities.insert(responsibility);
    }

    /// Replace the agent interface.
    pub fn set_interface(
        &mut self,
        interface: AgentInterface,
    ) -> AgentDefinitionResult<()> {
        interface.validate()?;

        self.interface = interface;

        Ok(())
    }

    /// Replace the declarative lifecycle.
    pub fn set_lifecycle(
        &mut self,
        lifecycle: AgentLifecycle,
    ) -> AgentDefinitionResult<()> {
        lifecycle.validate()?;

        self.lifecycle = lifecycle;

        Ok(())
    }

    /// Insert metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentDefinitionResult<Option<String>> {
        let key = key.into();

        validate_non_empty_identifier(
            &key,
            AgentDefinitionError::InvalidMetadataKey,
        )?;

        Ok(self.metadata.insert(key, value.into()))
    }

    /// Validate the complete agent definition.
    pub fn validate(&self) -> AgentDefinitionResult<()> {
        self.interface.validate()?;

        self.lifecycle.validate()?;

        for responsibility in &self.responsibilities {
            validate_non_empty_identifier(
                responsibility.as_str(),
                AgentDefinitionError::InvalidResponsibility,
            )?;
        }

        for key in self.metadata.keys() {
            validate_non_empty_identifier(
                key,
                AgentDefinitionError::InvalidMetadataKey,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_non_empty_identifier(
    value: &str,
    error: impl FnOnce(String) -> AgentDefinitionError,
) -> AgentDefinitionResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(error(value.to_owned()));
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured agent-definition error.
///
/// These errors describe invalid declarative agent structures.
///
/// They do not replace the canonical frontend diagnostic system.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum AgentDefinitionError {
    /// An agent name is empty.
    EmptyAgentName,

    /// An agent name is invalid.
    InvalidAgentName(String),

    /// A namespace component is invalid.
    InvalidNamespaceComponent(String),

    /// A responsibility is empty.
    EmptyResponsibility,

    /// A responsibility identifier is invalid.
    InvalidResponsibility(String),

    /// An interface port name is invalid.
    InvalidInterfacePortName(String),

    /// An interface port is duplicated.
    DuplicateInterfacePort(String),

    /// A type reference is invalid.
    InvalidTypeReference(String),

    /// A lifecycle state is invalid.
    InvalidLifecycleState(String),

    /// A lifecycle state is duplicated.
    DuplicateLifecycleState(String),

    /// A lifecycle state does not exist.
    UnknownLifecycleState(String),

    /// The initial lifecycle state has already been set.
    InitialStateAlreadySet,

    /// A lifecycle transition references the same state.
    SelfTransition(String),

    /// A lifecycle transition is duplicated.
    DuplicateLifecycleTransition,

    /// A lifecycle transition condition is invalid.
    InvalidTransitionCondition(String),

    /// A metadata key is invalid.
    InvalidMetadataKey(String),
}

impl fmt::Display for AgentDefinitionError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyAgentName => {
                f.write_str("agent name must not be empty")
            }

            Self::InvalidAgentName(name) => {
                write!(f, "invalid agent name: {name}")
            }

            Self::InvalidNamespaceComponent(component) => {
                write!(
                    f,
                    "invalid namespace component: {component}"
                )
            }

            Self::EmptyResponsibility => {
                f.write_str(
                    "agent responsibility must not be empty",
                )
            }

            Self::InvalidResponsibility(value) => {
                write!(
                    f,
                    "invalid agent responsibility: {value}"
                )
            }

            Self::InvalidInterfacePortName(name) => {
                write!(
                    f,
                    "invalid interface port name: {name}"
                )
            }

            Self::DuplicateInterfacePort(name) => {
                write!(
                    f,
                    "duplicate interface port: {name}"
                )
            }

            Self::InvalidTypeReference(reference) => {
                write!(
                    f,
                    "invalid interface type reference: {reference}"
                )
            }

            Self::InvalidLifecycleState(state) => {
                write!(
                    f,
                    "invalid lifecycle state: {state}"
                )
            }

            Self::DuplicateLifecycleState(state) => {
                write!(
                    f,
                    "duplicate lifecycle state: {state}"
                )
            }

            Self::UnknownLifecycleState(state) => {
                write!(
                    f,
                    "unknown lifecycle state: {state}"
                )
            }

            Self::InitialStateAlreadySet => {
                f.write_str(
                    "initial lifecycle state is already set",
                )
            }

            Self::SelfTransition(state) => {
                write!(
                    f,
                    "lifecycle self-transition is invalid: {state}"
                )
            }

            Self::DuplicateLifecycleTransition => {
                f.write_str(
                    "duplicate lifecycle transition",
                )
            }

            Self::InvalidTransitionCondition(condition) => {
                write!(
                    f,
                    "invalid lifecycle condition: {condition}"
                )
            }

            Self::InvalidMetadataKey(key) => {
                write!(
                    f,
                    "invalid metadata key: {key}"
                )
            }
        }
    }
}

impl Error for AgentDefinitionError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::ids::AiAgentId;

    fn agent_id(value: u128) -> AiAgentId {
        AiAgentId::new(value)
            .expect("test agent ID must be valid")
    }

    fn agent_name(
        value: &str,
    ) -> QualifiedAgentName {
        QualifiedAgentName::new(
            Vec::new(),
            AgentName::new(value)
                .expect("test agent name must be valid"),
        )
        .expect("test qualified name must be valid")
    }

    #[test]
    fn creates_agent_definition() {
        let definition = AgentDefinition::new(
            agent_id(1),
            agent_name("Researcher"),
        );

        assert_eq!(definition.id().get(), 1);

        assert_eq!(
            definition.name().display_name(),
            "Researcher"
        );
    }

    #[test]
    fn rejects_empty_agent_name() {
        assert!(matches!(
            AgentName::new(""),
            Err(AgentDefinitionError::EmptyAgentName)
        ));
    }

    #[test]
    fn rejects_whitespace_agent_name() {
        assert!(matches!(
            AgentName::new(" Researcher "),
            Err(
                AgentDefinitionError::InvalidAgentName(_)
            )
        ));
    }

    #[test]
    fn supports_qualified_agent_names() {
        let name = QualifiedAgentName::new(
            vec![
                "research".to_owned(),
                "planning".to_owned(),
            ],
            AgentName::new("Planner")
                .expect("valid agent name"),
        )
        .expect("valid qualified name");

        assert_eq!(
            name.display_name(),
            "research::planning::Planner"
        );
    }

    #[test]
    fn responsibilities_are_extensible() {
        let mut definition = AgentDefinition::new(
            agent_id(1),
            agent_name("Researcher"),
        );

        definition.add_responsibility(
            AgentResponsibility::new("research")
                .expect("valid responsibility"),
        );

        definition.add_responsibility(
            AgentResponsibility::new("future_custom_reasoning")
                .expect("valid responsibility"),
        );

        assert_eq!(
            definition.responsibilities().len(),
            2
        );
    }

    #[test]
    fn duplicate_responsibilities_are_idempotent() {
        let mut definition = AgentDefinition::new(
            agent_id(1),
            agent_name("Researcher"),
        );

        let responsibility =
            AgentResponsibility::new("research")
                .expect("valid responsibility");

        definition.add_responsibility(
            responsibility.clone(),
        );

        definition.add_responsibility(
            responsibility,
        );

        assert_eq!(
            definition.responsibilities().len(),
            1
        );
    }

    #[test]
    fn interface_ports_are_validated() {
        let input = AgentInterfacePort::new(
            "query",
            "String",
            AgentPortDirection::Input,
        )
        .expect("valid input");

        let output = AgentInterfacePort::new(
            "answer",
            "String",
            AgentPortDirection::Output,
        )
        .expect("valid output");

        let interface = AgentInterface::from_ports(
            vec![input, output],
        )
        .expect("valid interface");

        assert_eq!(
            interface.inputs().count(),
            1
        );

        assert_eq!(
            interface.outputs().count(),
            1
        );
    }

    #[test]
    fn duplicate_interface_ports_are_rejected() {
        let mut interface = AgentInterface::new();

        interface.add_port(
            AgentInterfacePort::new(
                "value",
                "Integer",
                AgentPortDirection::Input,
            )
            .expect("valid port"),
        )
        .expect("first port");

        let result = interface.add_port(
            AgentInterfacePort::new(
                "value",
                "Integer",
                AgentPortDirection::Output,
            )
            .expect("valid port"),
        );

        assert!(matches!(
            result,
            Err(
                AgentDefinitionError::DuplicateInterfacePort(_)
            )
        ));
    }

    #[test]
    fn optional_interface_ports_are_supported() {
        let port = AgentInterfacePort::new(
            "context",
            "Context",
            AgentPortDirection::Input,
        )
        .expect("valid port")
        .optional();

        assert!(port.is_optional());
    }

    #[test]
    fn lifecycle_states_are_validated() {
        let mut lifecycle = AgentLifecycle::new();

        let idle = AgentLifecycleState::new("idle")
            .expect("valid state");

        let active = AgentLifecycleState::new("active")
            .expect("valid state");

        lifecycle.add_state(idle.clone())
            .expect("add idle");

        lifecycle.add_state(active.clone())
            .expect("add active");

        lifecycle.set_initial_state(idle.clone())
            .expect("set initial state");

        lifecycle.add_transition(
            AgentLifecycleTransition::new(
                idle,
                active,
            )
            .expect("valid transition"),
        )
        .expect("add transition");

        lifecycle.validate()
            .expect("valid lifecycle");
    }

    #[test]
    fn duplicate_lifecycle_states_are_rejected() {
        let mut lifecycle = AgentLifecycle::new();

        let state = AgentLifecycleState::new("idle")
            .expect("valid state");

        lifecycle.add_state(state.clone())
            .expect("first state");

        assert!(matches!(
            lifecycle.add_state(state),
            Err(
                AgentDefinitionError::DuplicateLifecycleState(_)
            )
        ));
    }

    #[test]
    fn unknown_lifecycle_state_is_rejected() {
        let mut lifecycle = AgentLifecycle::new();

        let unknown = AgentLifecycleState::new("unknown")
            .expect("valid state");

        assert!(matches!(
            lifecycle.set_initial_state(unknown),
            Err(
                AgentDefinitionError::UnknownLifecycleState(_)
            )
        ));
    }

    #[test]
    fn self_transitions_are_rejected() {
        let state = AgentLifecycleState::new("idle")
            .expect("valid state");

        assert!(matches!(
            AgentLifecycleTransition::new(
                state.clone(),
                state,
            ),
            Err(
                AgentDefinitionError::SelfTransition(_)
            )
        ));
    }

    #[test]
    fn complete_definition_validates() {
        let mut definition = AgentDefinition::new(
            agent_id(42),
            agent_name("Researcher"),
        );

        definition.add_responsibility(
            AgentResponsibility::new("research")
                .expect("valid responsibility"),
        );

        let interface = AgentInterface::from_ports(
            vec![
                AgentInterfacePort::new(
                    "query",
                    "String",
                    AgentPortDirection::Input,
                )
                .expect("valid input"),

                AgentInterfacePort::new(
                    "answer",
                    "String",
                    AgentPortDirection::Output,
                )
                .expect("valid output"),
            ],
        )
        .expect("valid interface");

        definition.set_interface(interface)
            .expect("set interface");

        definition.validate()
            .expect("valid definition");
    }

    #[test]
    fn serialization_round_trip_preserves_definition() {
        let mut definition = AgentDefinition::new(
            agent_id(42),
            agent_name("Researcher"),
        );

        definition.add_responsibility(
            AgentResponsibility::new("research")
                .expect("valid responsibility"),
        );

        let encoded = serde_json::to_string(
            &definition,
        )
        .expect("serialize");

        let decoded: AgentDefinition =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(
            decoded,
            definition
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut definition = AgentDefinition::new(
            agent_id(1),
            agent_name("Researcher"),
        );

        definition.insert_metadata(
            "z",
            "last",
        )
        .expect("metadata");

        definition.insert_metadata(
            "a",
            "first",
        )
        .expect("metadata");

        let keys: Vec<_> = definition
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(
            keys,
            vec!["a", "z"]
        );
    }
}