
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Subsystem
//!
//! Path: `src/ai/agents/mod.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! This module is the canonical composition root for the Zamani AI
//! agent computational domain.
//!
//! It exposes the existing agent subsystems through one coherent,
//! target-independent public API.
//!
//! The agent subsystem describes computational intent, structure,
//! communication, coordination, planning, capabilities, memory,
//! tools and policy attachments.
//!
//! It does not implement a separate compiler, semantic analyzer,
//! universal intermediate representation, distributed runtime,
//! networking stack, authorization engine or hardware abstraction.
//!
//! ## Architectural ownership
//!
//! This module coordinates the following existing implementations:
//!
//! - definition.rs
//! - capabilities.rs
//! - communication.rs
//! - coordination.rs
//! - planning.rs
//! - memory.rs
//! - tools.rs
//! - policies.rs
//!
//! Each implementation retains ownership of its own domain.
//!
//! This file owns only module composition and public API exposure.
//!
//! ## Canonical repository authorities
//!
//! Source grammar:
//!
//! - grammar/ai/agents.g4
//! - grammar/ai/multi-agent.g4
//! - grammar/ai/communication.g4
//! - grammar/ai/planning.g4
//! - grammar/ai/capabilities.g4
//! - grammar/ai/ai-capabilities.g4
//!
//! Grammar composition:
//!
//! - grammar/ai/ai.g4
//!
//! Canonical identities:
//!
//! - crate::ai::ids
//!
//! AI capabilities:
//!
//! - crate::ai::capabilities
//!
//! AI requirements:
//!
//! - crate::ai::requirements
//!
//! Semantic authority:
//!
//! - crate::frontend::semantic
//!
//! Universal computational representation:
//!
//! - crate::zuir
//!
//! Distributed communication and execution:
//!
//! - crate::distributed
//!
//! Persistent memory integration:
//!
//! - crate::runtime
//!
//! Security enforcement:
//!
//! - crate::distributed::security
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic name resolution.
//! - Universal capability identities.
//! - Capability discovery.
//! - Capability evidence verification.
//! - Resource allocation.
//! - Hardware selection.
//! - Agent execution.
//! - Model execution.
//! - Agent scheduling.
//! - Distributed topology.
//! - Message transport.
//! - Persistent memory storage.
//! - Tool execution.
//! - Authentication.
//! - Authorization.
//! - Policy evaluation.
//! - Universal IR.
//! - Physical placement.
//!
//! These responsibilities remain with their authoritative
//! repository subsystems.
//!
//! ## POCO-REAF
//!
//! Agent descriptions and coordination contracts are independent
//! of:
//!
//! - CPU architecture.
//! - GPU architecture.
//! - Accelerator vendors.
//! - FPGA implementations.
//! - ASIC implementations.
//! - Quantum processors.
//! - Neuromorphic processors.
//! - Memory topology.
//! - Network topology.
//! - Operating systems.
//! - Distributed node counts.
//!
//! The same logical agent program must remain portable across
//! compatible execution environments.
//!
//! The compiler and runtime determine its realization.
//!
//! An execution target that cannot satisfy an agent's requirements
//! must report explicit infeasibility.
//!
//! ## Scalability
//!
//! This module introduces no artificial semantic ceilings on:
//!
//! - Agent definitions.
//! - Agent populations.
//! - Agent responsibilities.
//! - Agent interfaces.
//! - Agent capabilities.
//! - Communication endpoints.
//! - Communication channels.
//! - Coordination groups.
//! - Coordination relationships.
//! - Planning goals.
//! - Planning steps.
//! - Memory declarations.
//! - Tool definitions.
//! - Policy attachments.
//! - Distributed participants.
//!
//! Actual execution is bounded by available memory, representation
//! limits, configured operational budgets, security policies and
//! physical resource availability.
//!
//! This is not a promise of physically infinite computation.
//!
//! ## Module dependency direction
//!
//! ```text
//!                    AI IDENTITIES
//!                         |
//!                         v
//!                  AGENT DEFINITIONS
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!          v              v              v
//!     CAPABILITIES   COMMUNICATION   POLICIES
//!          |              |              |
//!          +--------------+--------------+
//!                         |
//!              +----------+----------+
//!              |                     |
//!              v                     v
//!          PLANNING              MEMORY
//!              |                     |
//!              +----------+----------+
//!                         |
//!                         v
//!                    COORDINATION
//!                         |
//!                         v
//!                       TOOLS
//!                         |
//!                         v
//!                  AGENT SUBSYSTEM
//!                         |
//!                         v
//!                  SEMANTIC / ZUIR
//!                         |
//!                         v
//!                 DISTRIBUTED / RUNTIME
//! ```
//!
//! This diagram describes conceptual integration, not mandatory
//! Rust module dependencies.
//!
//! ## Integration contract
//!
//! `definition`
//!     -> Canonical AiAgentId and agent definitions.
//!
//! `capabilities`
//!     -> Shared AI capability and requirement systems.
//!
//! `communication`
//!     -> Logical communication contracts.
//!
//! `coordination`
//!     -> Heterogeneous multi-agent coordination.
//!
//! `planning`
//!     -> Declarative agent planning.
//!
//! `memory`
//!     -> Runtime and persistent-memory provider interfaces.
//!
//! `tools`
//!     -> Authorized external and internal tool-provider interfaces.
//!
//! `policies`
//!     -> Canonical policy references and attachments.
//!
//! `frontend::semantic`
//!     -> AST-based semantic validation.
//!
//! `zuir`
//!     -> Target-independent computational representation.
//!
//! `distributed`
//!     -> Communication transport and distributed execution.
//!
//! `runtime`
//!     -> Actual agent lifecycle and execution.
//!
//! ## Error handling
//!
//! Each subsystem retains its own strongly typed error.
//!
//! This composition root does not erase subsystem errors,
//! silently convert failures into success, or introduce a
//! competing universal error representation.
//!
//! Higher-level execution orchestration must preserve the original
//! error and its provenance.
//!
//! ## Thread safety
//!
//! This module does not introduce global mutable state.
//!
//! Provider implementations are responsible for satisfying the
//! concurrency requirements of their execution environments.
//!
//! ## Serialization
//!
//! Serialization contracts remain owned by the individual
//! domain types.
//!
//! This module does not introduce another serialization format.
//!
//! ## Compatibility
//!
//! Existing module-qualified APIs remain available:
//!
//! - agents::definition::*
//! - agents::capabilities::*
//! - agents::communication::*
//! - agents::coordination::*
//! - agents::planning::*
//! - agents::memory::*
//! - agents::tools::*
//! - agents::policies::*
//!
//! Selected domain types are also exposed directly through
//! the agent module's public API.
//!
//! ## Freeze criteria
//!
//! - Every declared module resolves to an existing file.
//! - No duplicate domain implementation is introduced.
//! - Existing module-qualified APIs remain available.
//! - Public agent types are accessible.
//! - Canonical AI identifiers are reused.
//! - Canonical capability identities are reused.
//! - No duplicate semantic authority is introduced.
//! - No duplicate communication transport is introduced.
//! - No duplicate memory storage is introduced.
//! - No duplicate authorization system is introduced.
//! - No hardware-specific assumptions are introduced.
//! - No artificial universal capacity ceilings are introduced.
//! - No unsafe Rust is used.
//! - All subsystem tests pass.
//! - Parent AI module integration is complete.

// ============================================================================
// AGENT DOMAIN MODULES
// ============================================================================

/// Declarative agent identity, structure, interfaces and lifecycle
/// declarations.
///
/// Owns the definition of what an agent is.
pub mod definition;

/// Agent capability offers, requirements and capability profiles.
///
/// Reuses the canonical AI capability and requirement systems.
pub mod capabilities;

/// Declarative agent communication contracts.
///
/// Does not implement networking or message transport.
pub mod communication;

/// Multi-agent coordination groups, strategies and relationships.
///
/// Does not implement distributed scheduling or execution.
pub mod coordination;

/// Agent goals, objectives, actions, steps and dependencies.
///
/// Does not implement planning execution algorithms.
pub mod planning;

/// Agent memory declarations, requests and provider interfaces.
///
/// Does not implement persistent storage.
pub mod memory;

/// Agent tool definitions, invocation contracts and providers.
///
/// Does not independently authorize or execute tools.
pub mod tools;

/// Agent policy references, attachments and structural validation.
///
/// Does not evaluate policies or grant authorization.
pub mod policies;

// ============================================================================
// AGENT DEFINITION PUBLIC API
// ============================================================================

pub use definition::{
    AgentDefinition,
    AgentDefinitionError,
    AgentDefinitionResult,
    AgentInterface,
    AgentInterfacePort,
    AgentLifecycle,
    AgentLifecycleState,
    AgentLifecycleTransition,
    AgentName,
    AgentPortDirection,
    AgentResponsibility,
    QualifiedAgentName,
};

// ============================================================================
// AGENT CAPABILITY PUBLIC API
// ============================================================================

pub use capabilities::{
    AgentCapabilitiesError,
    AgentCapabilitiesResult,
    AgentCapabilityProfile,
};

// ============================================================================
// AGENT COMMUNICATION PUBLIC API
// ============================================================================

pub use communication::{
    AgentCommunicationChannel,
    AgentCommunicationEndpoint,
    AgentCommunicationError,
    AgentCommunicationProfile,
    AgentCommunicationResult,
    AgentCommunicationStream,
    AgentMessageContract,
    AgentMessageInvocation,
    CommunicationName,
    CommunicationReference,
    CommunicationTypeRef,
    CommunicationValueRef,
    DeliveryGuarantee,
    EndpointDirection,
    OrderingGuarantee,
};

// ============================================================================
// AGENT COORDINATION PUBLIC API
// ============================================================================

pub use coordination::{
    AgentCoordinationError,
    AgentCoordinationGroup,
    AgentCoordinationPlan,
    AgentCoordinationRelationship,
    AgentCoordinationResult,
    AgentParticipation,
    CoordinationName,
    CoordinationReference,
    CoordinationRelationshipKind,
    CoordinationRole,
    CoordinationStrategy,
    ParticipationKind,
    ParticipationRequirement,
};

// ============================================================================
// AGENT PLANNING PUBLIC API
// ============================================================================

pub use planning::{
    AgentPlan,
    AgentPlanningError,
    AgentPlanningResult,
    PlanningAction,
    PlanningAdaptation,
    PlanningAlternative,
    PlanningCapability,
    PlanningCondition,
    PlanningConditionKind,
    PlanningConstraint,
    PlanningDependency,
    PlanningDependencyKind,
    PlanningEvidence,
    PlanningGoal,
    PlanningName,
    PlanningObjective,
    PlanningPolicy,
    PlanningProvenance,
    PlanningReference,
    PlanningRequirement,
    PlanningStep,
    PlanningStepId,
};

// ============================================================================
// AGENT MEMORY PUBLIC API
// ============================================================================

pub use memory::{
    AgentMemoryAccess,
    AgentMemoryDeclaration,
    AgentMemoryError,
    AgentMemoryFuture,
    AgentMemoryOperation,
    AgentMemoryProfile,
    AgentMemoryRecord,
    AgentMemoryRequest,
    AgentMemoryResponse,
    AgentMemoryResult,
    AgentMemoryProvider,
};

// ============================================================================
// AGENT TOOL PUBLIC API
// ============================================================================

pub use tools::{
    AgentToolDefinition,
    AgentToolError,
    AgentToolFuture,
    AgentToolInvocation,
    AgentToolInvocationId,
    AgentToolName,
    AgentToolParameter,
    AgentToolProvider,
    AgentToolProviderReference,
    AgentToolProvenanceReference,
    AgentToolReference,
    AgentToolResponse,
    AgentToolResult,
    AgentToolSchema,
    AgentToolValueType,
    AgentToolVersion,
    AgentToolAuthorizationReference,
};

// ============================================================================
// AGENT POLICY PUBLIC API
// ============================================================================

pub use policies::{
    AgentPolicyArgument,
    AgentPolicyBinding,
    AgentPolicyBindingId,
    AgentPolicyError,
    AgentPolicyExpressionReference,
    AgentPolicyReference,
    AgentPolicyResult,
    AgentPolicySet,
    AgentPolicySubject,
    AgentPolicyTargetReference,
};

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Agent subsystem architectural invariants.
///
/// These constants describe architectural properties, not
/// operational resource limits.
pub mod invariants {
    /// Agent definitions are independent of physical hardware.
    pub const HARDWARE_INDEPENDENT: bool = true;

    /// Agent capability declarations do not prove target support.
    pub const CAPABILITY_DECLARATIONS_REQUIRE_EVIDENCE: bool = true;

    /// Agent communication contracts do not implement transport.
    pub const COMMUNICATION_TRANSPORT_IS_EXTERNAL: bool = true;

    /// Agent memory declarations do not implement persistent storage.
    pub const MEMORY_STORAGE_IS_EXTERNAL: bool = true;

    /// Agent policy references do not grant authorization.
    pub const POLICY_ATTACHMENTS_ARE_NOT_AUTHORIZATION: bool = true;

    /// Agent planning descriptions do not implement scheduling.
    pub const PLANNING_IS_DECLARATIVE: bool = true;

    /// Agent execution is delegated to canonical execution infrastructure.
    pub const EXECUTION_IS_EXTERNAL: bool = true;
}
