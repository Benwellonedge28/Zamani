
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Planning
//!
//! Path: `src/ai/agents/planning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, declarative planning contracts for AI agents.
//!
//! Planning describes computational intent, including:
//!
//! - goals;
//! - objectives;
//! - actions;
//! - preconditions;
//! - postconditions;
//! - planning steps;
//! - dependencies;
//! - alternatives;
//! - constraints;
//! - requirements;
//! - capability references;
//! - policy references;
//! - evidence;
//! - provenance;
//! - adaptation intent.
//!
//! This module provides structural validation and dependency analysis.
//!
//! It does not implement planning algorithms or execute plans.
//!
//! ## Canonical ownership
//!
//! - `grammar/ai/planning.g4`:
//!   Canonical planning source grammar.
//!
//! - `grammar/ai/agents.g4`:
//!   Agent source grammar.
//!
//! - `grammar/ai/multi-agent.g4`:
//!   Multi-agent composition grammar.
//!
//! - `src/frontend/semantic/domains/ai.rs`:
//!   Canonical AI semantic validation.
//!
//! - `src/ai/ids.rs`:
//!   Canonical AI identifiers.
//!
//! - `src/ai/agents/definition.rs`:
//!   Agent definitions.
//!
//! - `src/ai/agents/capabilities.rs`:
//!   Agent capability contracts.
//!
//! - `src/ai/agents/coordination.rs`:
//!   Agent coordination.
//!
//! - `src/ai/agents/communication.rs`:
//!   Communication contracts.
//!
//! - `src/zuir/dialects/ai/`:
//!   Universal AI computational representation.
//!
//! - `src/distributed/`:
//!   Distributed placement and scheduling.
//!
//! - `src/runtime/`:
//!   Actual execution.
//!
//! ## Ownership boundaries
//!
//! This module owns declarative agent planning.
//!
//! It does not own:
//!
//! - parsing;
//! - AST construction;
//! - semantic name resolution;
//! - planning algorithms;
//! - search algorithms;
//! - optimization algorithms;
//! - scheduling;
//! - placement;
//! - resource allocation;
//! - communication;
//! - coordination algorithms;
//! - policy enforcement;
//! - agent execution;
//! - runtime state;
//! - hardware selection;
//! - universal IR.
//!
//! ## Scalability
//!
//! No artificial semantic maximum is imposed on:
//!
//! - goals;
//! - objectives;
//! - actions;
//! - steps;
//! - dependencies;
//! - alternatives;
//! - constraints;
//! - requirements;
//! - agents;
//! - planning references.
//!
//! Collection growth is governed by actual memory availability
//! and explicitly configured operational budgets.
//!
//! Rust collections and identifier representations remain finite.
//! This module does not promise physically infinite computation.
//!
//! ## POCO-REAF
//!
//! Planning descriptions are independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - FPGA;
//! - ASIC;
//! - QPU;
//! - neuromorphic architecture;
//! - operating system;
//! - memory topology;
//! - network topology;
//! - distributed node count.
//!
//! Planning expresses what should happen.
//!
//! Downstream compiler and execution infrastructure determine
//! how and where that intent can be realized.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical grammar
//!        |
//!        v
//! Canonical frontend AST
//!        |
//!        v
//! Semantic validation
//!        |
//!        v
//! Agent definitions
//!        |
//!        v
//! Agent planning
//!        |
//!        +---- capabilities
//!        +---- coordination
//!        +---- communication
//!        +---- policies
//!        +---- provenance
//!        |
//!        v
//! Canonical ZUIR AI dialect
//!        |
//!        v
//! Compiler optimization and lowering
//!        |
//!        v
//! Hybrid / distributed planning
//!        |
//!        v
//! Runtime execution
//! ```
//!
//! Planning dependencies represent logical precedence.
//! They do not establish physical scheduling or execution order
//! beyond the dependency constraints themselves.
//!
//! ## Important validation rule
//!
//! Successful structural validation does not establish:
//!
//! - semantic correctness of opaque expressions;
//! - capability availability;
//! - resource feasibility;
//! - policy authorization;
//! - execution success.
//!
//! Those checks remain with their canonical owners.

use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use crate::ai::ids::AiAgentId;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for agent planning operations.
pub type AgentPlanningResult<T> =
    Result<T, AgentPlanningError>;

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_reference(
    value: &str,
    field: &'static str,
) -> AgentPlanningResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(AgentPlanningError::InvalidReference {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// PLANNING NAME
// ============================================================================

/// A validated, open-world planning name.
///
/// Names may represent goals, actions, objectives, constraints,
/// steps, policies or other planning concepts.
///
/// The canonical frontend remains responsible for resolving
/// source-language identifiers.
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
pub struct PlanningName(String);

impl PlanningName {
    /// Construct a validated planning name.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentPlanningResult<Self> {
        let value = value.into();

        validate_reference(&value, "planning name")?;

        Ok(Self(value))
    }

    /// Return the name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlanningName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// PLANNING REFERENCE
// ============================================================================

/// An opaque reference to a canonical Zamani semantic entity.
///
/// References can identify:
///
/// - actions;
//! - expressions;
//! - models;
//! - capabilities;
//! - policies;
//! - agents;
//! - resources;
//! - requirements;
//! - evidence;
//! - provenance;
//! - other computational domains.
///
/// This type does not resolve or execute the referenced entity.
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
pub struct PlanningReference(String);

impl PlanningReference {
    /// Construct an opaque planning reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentPlanningResult<Self> {
        let value = value.into();

        validate_reference(&value, "planning reference")?;

        Ok(Self(value))
    }

    /// Return the unresolved reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlanningReference {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// PLANNING STEP IDENTITY
// ============================================================================

/// A logical identity for a step inside a planning declaration.
///
/// This is scoped to its containing plan.
///
/// It is not a replacement for the canonical ZUIR OperationId
/// or any globally authoritative identity.
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
pub struct PlanningStepId(String);

impl PlanningStepId {
    /// Construct a planning step identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentPlanningResult<Self> {
        let value = value.into();

        validate_reference(&value, "planning step identity")?;

        Ok(Self(value))
    }

    /// Return the logical step identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlanningStepId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// GOALS
// ============================================================================

/// A declarative planning goal.
///
/// A goal describes a desired semantic condition.
///
/// Its actual meaning is resolved by semantic analysis.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningGoal {
    name: PlanningName,
    target: PlanningReference,
    condition: Option<PlanningReference>,
    description: Option<String>,
}

impl PlanningGoal {
    /// Construct a planning goal.
    pub fn new(
        name: PlanningName,
        target: PlanningReference,
    ) -> Self {
        Self {
            name,
            target,
            condition: None,
            description: None,
        }
    }

    /// Associate an opaque goal condition.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: PlanningReference,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Associate descriptive metadata.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the goal name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the desired target.
    #[must_use]
    pub fn target(&self) -> &PlanningReference {
        &self.target
    }

    /// Return the optional condition.
    #[must_use]
    pub fn condition(&self) -> Option<&PlanningReference> {
        self.condition.as_ref()
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// OBJECTIVES
// ============================================================================

/// A declarative planning objective.
///
/// Objectives may describe optimization preferences, quality criteria,
/// utility functions or other open-world computational objectives.
///
/// Their mathematical interpretation belongs to semantic analysis.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningObjective {
    name: PlanningName,
    expression: Option<PlanningReference>,
    type_reference: Option<PlanningReference>,
}

impl PlanningObjective {
    /// Construct an objective.
    pub fn new(name: PlanningName) -> Self {
        Self {
            name,
            expression: None,
            type_reference: None,
        }
    }

    /// Associate an objective expression.
    #[must_use]
    pub fn with_expression(
        mut self,
        expression: PlanningReference,
    ) -> Self {
        self.expression = Some(expression);
        self
    }

    /// Associate a canonical semantic type reference.
    #[must_use]
    pub fn with_type(
        mut self,
        type_reference: PlanningReference,
    ) -> Self {
        self.type_reference = Some(type_reference);
        self
    }

    /// Return the objective name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the expression.
    #[must_use]
    pub fn expression(&self) -> Option<&PlanningReference> {
        self.expression.as_ref()
    }

    /// Return the type reference.
    #[must_use]
    pub fn type_reference(&self) -> Option<&PlanningReference> {
        self.type_reference.as_ref()
    }
}

// ============================================================================
// ACTIONS
// ============================================================================

/// A declarative action description.
///
/// An action represents a possible computational operation.
///
/// This type does not invoke an agent, tool, model or external service.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningAction {
    name: PlanningName,
    operation: PlanningReference,
    arguments: Vec<PlanningReference>,
    type_reference: Option<PlanningReference>,
    body: Option<PlanningReference>,
}

impl PlanningAction {
    /// Construct an action.
    pub fn new(
        name: PlanningName,
        operation: PlanningReference,
    ) -> Self {
        Self {
            name,
            operation,
            arguments: Vec::new(),
            type_reference: None,
            body: None,
        }
    }

    /// Add an opaque argument reference.
    #[must_use]
    pub fn with_argument(
        mut self,
        argument: PlanningReference,
    ) -> Self {
        self.arguments.push(argument);
        self
    }

    /// Associate a canonical type reference.
    #[must_use]
    pub fn with_type(
        mut self,
        type_reference: PlanningReference,
    ) -> Self {
        self.type_reference = Some(type_reference);
        self
    }

    /// Associate an opaque action body reference.
    #[must_use]
    pub fn with_body(
        mut self,
        body: PlanningReference,
    ) -> Self {
        self.body = Some(body);
        self
    }

    /// Return the action name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the referenced operation.
    #[must_use]
    pub fn operation(&self) -> &PlanningReference {
        &self.operation
    }

    /// Return action arguments.
    #[must_use]
    pub fn arguments(&self) -> &[PlanningReference] {
        &self.arguments
    }

    /// Return the optional type.
    #[must_use]
    pub fn type_reference(&self) -> Option<&PlanningReference> {
        self.type_reference.as_ref()
    }

    /// Return the optional body.
    #[must_use]
    pub fn body(&self) -> Option<&PlanningReference> {
        self.body.as_ref()
    }
}

// ============================================================================
// CONDITIONS
// ============================================================================

/// The semantic role of a planning condition.
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
pub enum PlanningConditionKind {
    /// A condition that must hold before an action or plan applies.
    Precondition,

    /// An expected condition after an action or plan.
    Postcondition,
}

/// A declarative planning condition.
///
/// The referenced expression is interpreted by the canonical
/// semantic system.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningCondition {
    kind: PlanningConditionKind,
    expression: PlanningReference,
    subject: Option<PlanningReference>,
}

impl PlanningCondition {
    /// Construct a planning condition.
    pub fn new(
        kind: PlanningConditionKind,
        expression: PlanningReference,
    ) -> Self {
        Self {
            kind,
            expression,
            subject: None,
        }
    }

    /// Associate the action or planning subject.
    #[must_use]
    pub fn with_subject(
        mut self,
        subject: PlanningReference,
    ) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Return the condition kind.
    #[must_use]
    pub const fn kind(&self) -> PlanningConditionKind {
        self.kind
    }

    /// Return the expression reference.
    #[must_use]
    pub fn expression(&self) -> &PlanningReference {
        &self.expression
    }

    /// Return the optional subject.
    #[must_use]
    pub fn subject(&self) -> Option<&PlanningReference> {
        self.subject.as_ref()
    }
}

// ============================================================================
// PLANNING STEP
// ============================================================================

/// A declarative step in a planning description.
///
/// A step identifies an operation that may be considered for
/// subsequent realization.
///
/// It is not a runtime task or scheduled job.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningStep {
    id: PlanningStepId,
    operation: PlanningReference,
    arguments: Vec<PlanningReference>,
    type_reference: Option<PlanningReference>,
    description: Option<String>,
}

impl PlanningStep {
    /// Construct a planning step.
    pub fn new(
        id: PlanningStepId,
        operation: PlanningReference,
    ) -> Self {
        Self {
            id,
            operation,
            arguments: Vec::new(),
            type_reference: None,
            description: None,
        }
    }

    /// Add an argument reference.
    #[must_use]
    pub fn with_argument(
        mut self,
        argument: PlanningReference,
    ) -> Self {
        self.arguments.push(argument);
        self
    }

    /// Associate a type reference.
    #[must_use]
    pub fn with_type(
        mut self,
        type_reference: PlanningReference,
    ) -> Self {
        self.type_reference = Some(type_reference);
        self
    }

    /// Associate descriptive metadata.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the step identity.
    #[must_use]
    pub fn id(&self) -> &PlanningStepId {
        &self.id
    }

    /// Return the operation reference.
    #[must_use]
    pub fn operation(&self) -> &PlanningReference {
        &self.operation
    }

    /// Return arguments.
    #[must_use]
    pub fn arguments(&self) -> &[PlanningReference] {
        &self.arguments
    }

    /// Return the optional type reference.
    #[must_use]
    pub fn type_reference(&self) -> Option<&PlanningReference> {
        self.type_reference.as_ref()
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// DEPENDENCIES
// ============================================================================

/// The logical relationship represented by a step dependency.
///
/// These relationships do not define physical scheduling.
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
#[serde(rename_all = "snake_case")]
pub enum PlanningDependencyKind {
    /// The dependent step requires the prerequisite step.
    Requires,

    /// The dependent step consumes data produced by the prerequisite.
    Data,

    /// The dependent step requires a capability established by the
    /// prerequisite.
    Capability,

    /// The dependent step has a causal dependency on the prerequisite.
    Causal,

    /// The dependent step has another declared relationship.
    Custom(PlanningName),
}

/// A directed planning dependency.
///
/// Direction:
///
/// prerequisite -> dependent
///
/// The prerequisite must precede the dependent in a valid
/// topological ordering.
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
pub struct PlanningDependency {
    prerequisite: PlanningStepId,
    dependent: PlanningStepId,
    kind: PlanningDependencyKind,
    condition: Option<PlanningReference>,
}

impl PlanningDependency {
    /// Construct a dependency.
    pub fn new(
        prerequisite: PlanningStepId,
        dependent: PlanningStepId,
        kind: PlanningDependencyKind,
    ) -> AgentPlanningResult<Self> {
        if prerequisite == dependent {
            return Err(
                AgentPlanningError::SelfDependency(
                    prerequisite.to_string(),
                ),
            );
        }

        Ok(Self {
            prerequisite,
            dependent,
            kind,
            condition: None,
        })
    }

    /// Associate an optional dependency condition.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: PlanningReference,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Return the prerequisite step.
    #[must_use]
    pub fn prerequisite(&self) -> &PlanningStepId {
        &self.prerequisite
    }

    /// Return the dependent step.
    #[must_use]
    pub fn dependent(&self) -> &PlanningStepId {
        &self.dependent
    }

    /// Return the dependency kind.
    #[must_use]
    pub fn kind(&self) -> &PlanningDependencyKind {
        &self.kind
    }

    /// Return the optional dependency condition.
    #[must_use]
    pub fn condition(&self) -> Option<&PlanningReference> {
        self.condition.as_ref()
    }
}

// ============================================================================
// ALTERNATIVES
// ============================================================================

/// A declarative planning alternative.
///
/// The planner or downstream optimizer decides whether an alternative
/// is appropriate.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningAlternative {
    name: PlanningName,
    expression: PlanningReference,
    condition: Option<PlanningReference>,
}

impl PlanningAlternative {
    /// Construct an alternative.
    pub fn new(
        name: PlanningName,
        expression: PlanningReference,
    ) -> Self {
        Self {
            name,
            expression,
            condition: None,
        }
    }

    /// Associate an applicability condition.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: PlanningReference,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Return the alternative name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the expression reference.
    #[must_use]
    pub fn expression(&self) -> &PlanningReference {
        &self.expression
    }

    /// Return the optional condition.
    #[must_use]
    pub fn condition(&self) -> Option<&PlanningReference> {
        self.condition.as_ref()
    }
}

// ============================================================================
// CONSTRAINTS
// ============================================================================

/// A declarative planning constraint.
///
/// Constraint interpretation and enforcement belong to the
/// canonical semantic and policy systems.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningConstraint {
    name: PlanningName,
    expression: PlanningReference,
}

impl PlanningConstraint {
    /// Construct a planning constraint.
    pub fn new(
        name: PlanningName,
        expression: PlanningReference,
    ) -> Self {
        Self { name, expression }
    }

    /// Return the constraint name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the constraint expression.
    #[must_use]
    pub fn expression(&self) -> &PlanningReference {
        &self.expression
    }
}

// ============================================================================
// REQUIREMENTS
// ============================================================================

/// A declarative planning requirement.
///
/// The referenced requirement is resolved through the canonical
/// semantic resource and requirement systems.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningRequirement {
    name: PlanningName,
    reference: PlanningReference,
}

impl PlanningRequirement {
    /// Construct a planning requirement.
    pub fn new(
        name: PlanningName,
        reference: PlanningReference,
    ) -> Self {
        Self { name, reference }
    }

    /// Return the requirement name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the canonical requirement reference.
    #[must_use]
    pub fn reference(&self) -> &PlanningReference {
        &self.reference
    }
}

// ============================================================================
// CAPABILITY REFERENCES
// ============================================================================

/// A declarative capability reference.
///
/// This does not prove capability availability.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningCapability {
    name: PlanningName,
    reference: PlanningReference,
}

impl PlanningCapability {
    /// Construct a capability reference.
    pub fn new(
        name: PlanningName,
        reference: PlanningReference,
    ) -> Self {
        Self { name, reference }
    }

    /// Return the capability name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the capability reference.
    #[must_use]
    pub fn reference(&self) -> &PlanningReference {
        &self.reference
    }
}

// ============================================================================
// POLICY REFERENCES
// ============================================================================

/// A declarative policy reference.
///
/// Actual policy interpretation and enforcement remain external.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningPolicy {
    name: PlanningName,
    reference: PlanningReference,
}

impl PlanningPolicy {
    /// Construct a policy reference.
    pub fn new(
        name: PlanningName,
        reference: PlanningReference,
    ) -> Self {
        Self { name, reference }
    }

    /// Return the policy name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the policy reference.
    #[must_use]
    pub fn reference(&self) -> &PlanningReference {
        &self.reference
    }
}

// ============================================================================
// EVIDENCE
// ============================================================================

/// A declarative evidence reference.
///
/// Evidence verification and trust assessment belong to the
/// canonical evidence and provenance systems.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningEvidence {
    name: PlanningName,
    reference: PlanningReference,
}

impl PlanningEvidence {
    /// Construct an evidence reference.
    pub fn new(
        name: PlanningName,
        reference: PlanningReference,
    ) -> Self {
        Self { name, reference }
    }

    /// Return the evidence name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &PlanningReference {
        &self.reference
    }
}

// ============================================================================
// PROVENANCE
// ============================================================================

/// A declarative provenance reference.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningProvenance {
    name: PlanningName,
    reference: PlanningReference,
}

impl PlanningProvenance {
    /// Construct a provenance reference.
    pub fn new(
        name: PlanningName,
        reference: PlanningReference,
    ) -> Self {
        Self { name, reference }
    }

    /// Return the provenance name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the provenance reference.
    #[must_use]
    pub fn reference(&self) -> &PlanningReference {
        &self.reference
    }
}

// ============================================================================
// ADAPTATION
// ============================================================================

/// A declarative planning adaptation request.
///
/// Adaptation describes intended semantic change.
///
/// It does not mutate an executing plan or runtime state.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct PlanningAdaptation {
    target: PlanningReference,
    expression: PlanningReference,
}

impl PlanningAdaptation {
    /// Construct an adaptation request.
    pub fn new(
        target: PlanningReference,
        expression: PlanningReference,
    ) -> Self {
        Self {
            target,
            expression,
        }
    }

    /// Return the adaptation target.
    #[must_use]
    pub fn target(&self) -> &PlanningReference {
        &self.target
    }

    /// Return the adaptation expression.
    #[must_use]
    pub fn expression(&self) -> &PlanningReference {
        &self.expression
    }
}

// ============================================================================
// AGENT PLAN
// ============================================================================

/// A portable, declarative planning description for an agent.
///
/// The plan owns planning declarations and logical dependencies.
///
/// It does not represent an executing plan.
///
/// It does not contain runtime step statuses, physical placement,
/// scheduler decisions or execution results.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AgentPlan {
    name: PlanningName,

    agent: Option<AiAgentId>,

    goals: Vec<PlanningGoal>,

    objectives: Vec<PlanningObjective>,

    actions: Vec<PlanningAction>,

    conditions: Vec<PlanningCondition>,

    steps: BTreeMap<PlanningStepId, PlanningStep>,

    dependencies: BTreeSet<PlanningDependency>,

    alternatives: Vec<PlanningAlternative>,

    constraints: Vec<PlanningConstraint>,

    requirements: Vec<PlanningRequirement>,

    capabilities: Vec<PlanningCapability>,

    policies: Vec<PlanningPolicy>,

    evidence: Vec<PlanningEvidence>,

    provenance: Vec<PlanningProvenance>,

    adaptations: Vec<PlanningAdaptation>,

    metadata: BTreeMap<String, String>,
}

impl AgentPlan {
    /// Construct an empty agent plan.
    pub fn new(name: PlanningName) -> Self {
        Self {
            name,
            agent: None,
            goals: Vec::new(),
            objectives: Vec::new(),
            actions: Vec::new(),
            conditions: Vec::new(),
            steps: BTreeMap::new(),
            dependencies: BTreeSet::new(),
            alternatives: Vec::new(),
            constraints: Vec::new(),
            requirements: Vec::new(),
            capabilities: Vec::new(),
            policies: Vec::new(),
            evidence: Vec::new(),
            provenance: Vec::new(),
            adaptations: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Associate the agent owning this planning description.
    #[must_use]
    pub fn for_agent(
        mut self,
        agent: AiAgentId,
    ) -> Self {
        self.agent = Some(agent);
        self
    }

    /// Return the planning name.
    #[must_use]
    pub fn name(&self) -> &PlanningName {
        &self.name
    }

    /// Return the optional owning agent.
    #[must_use]
    pub const fn agent(&self) -> Option<AiAgentId> {
        self.agent
    }

    /// Return declared goals.
    #[must_use]
    pub fn goals(&self) -> &[PlanningGoal] {
        &self.goals
    }

    /// Return declared objectives.
    #[must_use]
    pub fn objectives(&self) -> &[PlanningObjective] {
        &self.objectives
    }

    /// Return declared actions.
    #[must_use]
    pub fn actions(&self) -> &[PlanningAction] {
        &self.actions
    }

    /// Return declared conditions.
    #[must_use]
    pub fn conditions(&self) -> &[PlanningCondition] {
        &self.conditions
    }

    /// Return planning steps in deterministic identity order.
    #[must_use]
    pub fn steps(&self) -> &BTreeMap<PlanningStepId, PlanningStep> {
        &self.steps
    }

    /// Return logical dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<PlanningDependency> {
        &self.dependencies
    }

    /// Return alternatives.
    #[must_use]
    pub fn alternatives(&self) -> &[PlanningAlternative] {
        &self.alternatives
    }

    /// Return constraints.
    #[must_use]
    pub fn constraints(&self) -> &[PlanningConstraint] {
        &self.constraints
    }

    /// Return requirements.
    #[must_use]
    pub fn requirements(&self) -> &[PlanningRequirement] {
        &self.requirements
    }

    /// Return capability references.
    #[must_use]
    pub fn capabilities(&self) -> &[PlanningCapability] {
        &self.capabilities
    }

    /// Return policy references.
    #[must_use]
    pub fn policies(&self) -> &[PlanningPolicy] {
        &self.policies
    }

    /// Return evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[PlanningEvidence] {
        &self.evidence
    }

    /// Return provenance references.
    #[must_use]
    pub fn provenance(&self) -> &[PlanningProvenance] {
        &self.provenance
    }

    /// Return adaptation declarations.
    #[must_use]
    pub fn adaptations(&self) -> &[PlanningAdaptation] {
        &self.adaptations
    }

    /// Return metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Add a goal.
    pub fn add_goal(
        &mut self,
        goal: PlanningGoal,
    ) {
        self.goals.push(goal);
    }

    /// Add an objective.
    pub fn add_objective(
        &mut self,
        objective: PlanningObjective,
    ) {
        self.objectives.push(objective);
    }

    /// Add an action.
    pub fn add_action(
        &mut self,
        action: PlanningAction,
    ) {
        self.actions.push(action);
    }

    /// Add a condition.
    pub fn add_condition(
        &mut self,
        condition: PlanningCondition,
    ) {
        self.conditions.push(condition);
    }

    /// Add a planning step.
    ///
    /// Duplicate identities are rejected.
    pub fn add_step(
        &mut self,
        step: PlanningStep,
    ) -> AgentPlanningResult<()> {
        let id = step.id.clone();

        if self.steps.contains_key(&id) {
            return Err(
                AgentPlanningError::DuplicateStep(
                    id.to_string(),
                ),
            );
        }

        self.steps.insert(id, step);

        Ok(())
    }

    /// Add a dependency.
    ///
    /// Both endpoint steps must already exist.
    ///
    /// This makes the operation atomic and prevents dangling
    /// dependencies from entering the plan through this API.
    pub fn add_dependency(
        &mut self,
        dependency: PlanningDependency,
    ) -> AgentPlanningResult<()> {
        if !self.steps.contains_key(&dependency.prerequisite) {
            return Err(
                AgentPlanningError::UnknownStep(
                    dependency.prerequisite.to_string(),
                ),
            );
        }

        if !self.steps.contains_key(&dependency.dependent) {
            return Err(
                AgentPlanningError::UnknownStep(
                    dependency.dependent.to_string(),
                ),
            );
        }

        if self.dependencies.contains(&dependency) {
            return Err(
                AgentPlanningError::DuplicateDependency,
            );
        }

        self.dependencies.insert(dependency);

        Ok(())
    }

    /// Add an alternative.
    pub fn add_alternative(
        &mut self,
        alternative: PlanningAlternative,
    ) {
        self.alternatives.push(alternative);
    }

    /// Add a constraint.
    pub fn add_constraint(
        &mut self,
        constraint: PlanningConstraint,
    ) {
        self.constraints.push(constraint);
    }

    /// Add a requirement.
    pub fn add_requirement(
        &mut self,
        requirement: PlanningRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Add a capability reference.
    pub fn add_capability(
        &mut self,
        capability: PlanningCapability,
    ) {
        self.capabilities.push(capability);
    }

    /// Add a policy reference.
    pub fn add_policy(
        &mut self,
        policy: PlanningPolicy,
    ) {
        self.policies.push(policy);
    }

    /// Add an evidence reference.
    pub fn add_evidence(
        &mut self,
        evidence: PlanningEvidence,
    ) {
        self.evidence.push(evidence);
    }

    /// Add a provenance reference.
    pub fn add_provenance(
        &mut self,
        provenance: PlanningProvenance,
    ) {
        self.provenance.push(provenance);
    }

    /// Add an adaptation declaration.
    pub fn add_adaptation(
        &mut self,
        adaptation: PlanningAdaptation,
    ) {
        self.adaptations.push(adaptation);
    }

    /// Insert or replace descriptive metadata.
    ///
    /// Metadata ordering remains deterministic.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AgentPlanningResult<Option<String>> {
        let key = key.into();

        validate_reference(&key, "metadata key")?;

        Ok(self.metadata.insert(key, value.into()))
    }

    /// Return the number of declared steps.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Return whether the plan has no declared steps.
    #[must_use]
    pub fn has_no_steps(&self) -> bool {
        self.steps.is_empty()
    }

    /// Validate the structural integrity of the complete plan.
    ///
    /// This validates:
    ///
    /// - unique goal names;
    /// - unique objective names;
    /// - unique action names;
    /// - unique alternative names;
    /// - unique constraint names;
    /// - dependency endpoint existence;
    /// - absence of self-dependencies;
    /// - absence of dependency cycles.
    ///
    /// It does not validate the semantics of opaque references.
    pub fn validate(&self) -> AgentPlanningResult<()> {
        validate_unique_names(
            self.goals.iter().map(|item| &item.name),
            "goal",
        )?;

        validate_unique_names(
            self.objectives.iter().map(|item| &item.name),
            "objective",
        )?;

        validate_unique_names(
            self.actions.iter().map(|item| &item.name),
            "action",
        )?;

        validate_unique_names(
            self.alternatives.iter().map(|item| &item.name),
            "alternative",
        )?;

        validate_unique_names(
            self.constraints.iter().map(|item| &item.name),
            "constraint",
        )?;

        for dependency in &self.dependencies {
            if dependency.prerequisite == dependency.dependent {
                return Err(
                    AgentPlanningError::SelfDependency(
                        dependency.prerequisite.to_string(),
                    ),
                );
            }

            if !self.steps.contains_key(&dependency.prerequisite) {
                return Err(
                    AgentPlanningError::UnknownStep(
                        dependency.prerequisite.to_string(),
                    ),
                );
            }

            if !self.steps.contains_key(&dependency.dependent) {
                return Err(
                    AgentPlanningError::UnknownStep(
                        dependency.dependent.to_string(),
                    ),
                );
            }
        }

        self.topological_order()?;

        Ok(())
    }

    /// Return a deterministic topological ordering of the steps.
    ///
    /// Every prerequisite appears before its dependent.
    ///
    /// Among simultaneously eligible steps, lexical identity order
    /// is used to provide deterministic output.
    ///
    /// This is dependency analysis, not physical scheduling.
    pub fn topological_order(
        &self,
    ) -> AgentPlanningResult<Vec<PlanningStepId>> {
        let mut indegree: BTreeMap<PlanningStepId, usize> =
            self.steps
                .keys()
                .cloned()
                .map(|id| (id, 0))
                .collect();

        let mut outgoing: BTreeMap<
            PlanningStepId,
            BTreeSet<PlanningStepId>,
        > = self.steps
            .keys()
            .cloned()
            .map(|id| (id, BTreeSet::new()))
            .collect();

        for dependency in &self.dependencies {
            if !self.steps.contains_key(&dependency.prerequisite) {
                return Err(
                    AgentPlanningError::UnknownStep(
                        dependency.prerequisite.to_string(),
                    ),
                );
            }

            if !self.steps.contains_key(&dependency.dependent) {
                return Err(
                    AgentPlanningError::UnknownStep(
                        dependency.dependent.to_string(),
                    ),
                );
            }

            let neighbors = outgoing
                .get_mut(&dependency.prerequisite)
                .ok_or_else(|| {
                    AgentPlanningError::UnknownStep(
                        dependency.prerequisite.to_string(),
                    )
                })?;

            if neighbors.insert(dependency.dependent.clone()) {
                let count = indegree
                    .get_mut(&dependency.dependent)
                    .ok_or_else(|| {
                        AgentPlanningError::UnknownStep(
                            dependency.dependent.to_string(),
                        )
                    })?;

                *count += 1;
            }
        }

        let mut ready = BTreeSet::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                ready.insert(id.clone());
            }
        }

        let mut result = Vec::with_capacity(self.steps.len());

        while let Some(current) = ready.pop_first() {
            result.push(current.clone());

            let neighbors = outgoing
                .get(&current)
                .ok_or_else(|| {
                    AgentPlanningError::UnknownStep(
                        current.to_string(),
                    )
                })?;

            for neighbor in neighbors {
                let degree = indegree
                    .get_mut(neighbor)
                    .ok_or_else(|| {
                        AgentPlanningError::UnknownStep(
                            neighbor.to_string(),
                        )
                    })?;

                *degree -= 1;

                if *degree == 0 {
                    ready.insert(neighbor.clone());
                }
            }
        }

        if result.len() != self.steps.len() {
            let blocked = indegree
                .iter()
                .filter(|(_, degree)| **degree > 0)
                .map(|(id, _)| id.to_string())
                .collect();

            return Err(
                AgentPlanningError::DependencyCycle {
                    blocked,
                },
            );
        }

        Ok(result)
    }

    /// Return the steps whose declared dependencies are satisfied
    /// by the supplied completed-step identities.
    ///
    /// This method does not inspect or modify runtime state.
    ///
    /// The caller supplies the completion evidence.
    pub fn ready_steps<'a>(
        &'a self,
        completed: &BTreeSet<PlanningStepId>,
    ) -> AgentPlanningResult<Vec<&'a PlanningStep>> {
        self.validate()?;

        for id in completed {
            if !self.steps.contains_key(id) {
                return Err(
                    AgentPlanningError::UnknownStep(
                        id.to_string(),
                    ),
                );
            }
        }

        let mut ready = Vec::new();

        for (id, step) in &self.steps {
            if completed.contains(id) {
                continue;
            }

            let satisfied = self.dependencies
                .iter()
                .filter(|dependency| dependency.dependent == *id)
                .all(|dependency| {
                    completed.contains(&dependency.prerequisite)
                });

            if satisfied {
                ready.push(step);
            }
        }

        Ok(ready)
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_unique_names<'a>(
    names: impl Iterator<Item = &'a PlanningName>,
    category: &'static str,
) -> AgentPlanningResult<()> {
    let mut seen = BTreeSet::new();

    for name in names {
        if !seen.insert(name) {
            return Err(
                AgentPlanningError::DuplicateDeclaration {
                    category,
                    name: name.to_string(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Structured errors produced by agent planning.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum AgentPlanningError {
    /// A reference has invalid representation.
    InvalidReference {
        /// The affected field.
        field: &'static str,

        /// The invalid value.
        value: String,
    },

    /// A declaration name is duplicated.
    DuplicateDeclaration {
        /// The declaration category.
        category: &'static str,

        /// The duplicated name.
        name: String,
    },

    /// A planning step identity is duplicated.
    DuplicateStep(String),

    /// A referenced step does not exist.
    UnknownStep(String),

    /// A step depends on itself.
    SelfDependency(String),

    /// A dependency has already been declared.
    DuplicateDependency,

    /// The dependency graph contains a cycle.
    DependencyCycle {
        /// Steps remaining blocked by the cycle.
        blocked: Vec<String>,
    },
}

impl fmt::Display for AgentPlanningError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidReference { field, value } => {
                write!(
                    f,
                    "invalid {field}: {value:?}"
                )
            }

            Self::DuplicateDeclaration { category, name } => {
                write!(
                    f,
                    "duplicate {category} declaration: {name}"
                )
            }

            Self::DuplicateStep(id) => {
                write!(
                    f,
                    "duplicate planning step: {id}"
                )
            }

            Self::UnknownStep(id) => {
                write!(
                    f,
                    "unknown planning step: {id}"
                )
            }

            Self::SelfDependency(id) => {
                write!(
                    f,
                    "planning step cannot depend on itself: {id}"
                )
            }

            Self::DuplicateDependency => {
                f.write_str("duplicate planning dependency")
            }

            Self::DependencyCycle { blocked } => {
                write!(
                    f,
                    "planning dependency cycle detected; blocked steps: {blocked:?}"
                )
            }
        }
    }
}

impl Error for AgentPlanningError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> PlanningName {
        PlanningName::new(value).expect("valid name")
    }

    fn reference(value: &str) -> PlanningReference {
        PlanningReference::new(value).expect("valid reference")
    }

    fn step(value: &str) -> PlanningStep {
        PlanningStep::new(
            PlanningStepId::new(value).expect("valid step"),
            reference(format!("operation::{value}")),
        )
    }

    fn dependency(
        from: &str,
        to: &str,
    ) -> PlanningDependency {
        PlanningDependency::new(
            PlanningStepId::new(from).expect("valid source"),
            PlanningStepId::new(to).expect("valid target"),
            PlanningDependencyKind::Requires,
        )
        .expect("valid dependency")
    }

    #[test]
    fn planning_names_reject_invalid_values() {
        assert!(PlanningName::new("").is_err());
        assert!(PlanningName::new(" name").is_err());
        assert!(PlanningName::new("name ").is_err());
        assert!(PlanningName::new("na\nme").is_err());
    }

    #[test]
    fn planning_references_are_open_world() {
        let reference = reference(
            "future::quantum::agent::operation",
        );

        assert_eq!(
            reference.as_str(),
            "future::quantum::agent::operation"
        );
    }

    #[test]
    fn empty_plan_is_valid() {
        let plan = AgentPlan::new(name("research"));

        assert!(plan.validate().is_ok());
        assert!(plan.has_no_steps());
    }

    #[test]
    fn steps_can_be_added() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));
        plan.add_step(step("analyze"));

        assert_eq!(plan.step_count(), 2);
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn duplicate_steps_are_rejected() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));

        assert!(matches!(
            plan.add_step(step("collect")),
            Err(AgentPlanningError::DuplicateStep(_))
        ));
    }

    #[test]
    fn unknown_dependency_endpoints_are_rejected() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));

        assert!(matches!(
            plan.add_dependency(dependency("missing", "collect")),
            Err(AgentPlanningError::UnknownStep(_))
        ));
    }

    #[test]
    fn self_dependencies_are_rejected() {
        let id = PlanningStepId::new("collect")
            .expect("valid ID");

        assert!(matches!(
            PlanningDependency::new(
                id.clone(),
                id,
                PlanningDependencyKind::Requires,
            ),
            Err(AgentPlanningError::SelfDependency(_))
        ));
    }

    #[test]
    fn duplicate_dependencies_are_rejected() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("a"));
        plan.add_step(step("b"));

        plan.add_dependency(dependency("a", "b"))
            .expect("first dependency");

        assert_eq!(
            plan.add_dependency(dependency("a", "b")),
            Err(AgentPlanningError::DuplicateDependency)
        );
    }

    #[test]
    fn dependency_order_is_correct() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));
        plan.add_step(step("analyze"));
        plan.add_step(step("report"));

        plan.add_dependency(dependency("collect", "analyze"))
            .expect("dependency");

        plan.add_dependency(dependency("analyze", "report"))
            .expect("dependency");

        let order = plan.topological_order()
            .expect("valid ordering");

        let positions: BTreeMap<_, _> = order
            .iter()
            .enumerate()
            .map(|(index, id)| (id.as_str(), index))
            .collect();

        assert!(
            positions["collect"] < positions["analyze"]
        );

        assert!(
            positions["analyze"] < positions["report"]
        );
    }

    #[test]
    fn independent_steps_have_deterministic_order() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("zeta"));
        plan.add_step(step("alpha"));
        plan.add_step(step("middle"));

        let order = plan.topological_order()
            .expect("valid ordering");

        let names: Vec<_> = order
            .iter()
            .map(|id| id.as_str())
            .collect();

        assert_eq!(
            names,
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn dependency_cycles_are_detected() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("a"));
        plan.add_step(step("b"));
        plan.add_step(step("c"));

        plan.add_dependency(dependency("a", "b"))
            .expect("dependency");

        plan.add_dependency(dependency("b", "c"))
            .expect("dependency");

        plan.add_dependency(dependency("c", "a"))
            .expect("dependency");

        assert!(matches!(
            plan.validate(),
            Err(AgentPlanningError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn ready_steps_respect_dependencies() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));
        plan.add_step(step("analyze"));
        plan.add_step(step("report"));

        plan.add_dependency(dependency("collect", "analyze"))
            .expect("dependency");

        plan.add_dependency(dependency("analyze", "report"))
            .expect("dependency");

        let completed = BTreeSet::new();

        let ready = plan.ready_steps(&completed)
            .expect("ready steps");

        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id().as_str(), "collect");

        let completed = BTreeSet::from([
            PlanningStepId::new("collect").expect("valid ID"),
        ]);

        let ready = plan.ready_steps(&completed)
            .expect("ready steps");

        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id().as_str(), "analyze");
    }

    #[test]
    fn completed_steps_are_not_returned_as_ready() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));

        let completed = BTreeSet::from([
            PlanningStepId::new("collect").expect("valid ID"),
        ]);

        assert!(
            plan.ready_steps(&completed)
                .expect("ready steps")
                .is_empty()
        );
    }

    #[test]
    fn unknown_completed_steps_are_rejected() {
        let plan = AgentPlan::new(name("research"));

        let completed = BTreeSet::from([
            PlanningStepId::new("unknown").expect("valid ID"),
        ]);

        assert!(matches!(
            plan.ready_steps(&completed),
            Err(AgentPlanningError::UnknownStep(_))
        ));
    }

    #[test]
    fn duplicate_goal_names_are_rejected() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_goal(PlanningGoal::new(
            name("solve"),
            reference("problem"),
        ));

        plan.add_goal(PlanningGoal::new(
            name("solve"),
            reference("another_problem"),
        ));

        assert!(matches!(
            plan.validate(),
            Err(AgentPlanningError::DuplicateDeclaration {
                category: "goal",
                ..
            })
        ));
    }

    #[test]
    fn planning_metadata_is_deterministic() {
        let mut plan = AgentPlan::new(name("research"));

        plan.insert_metadata("zeta", "last")
            .expect("metadata");

        plan.insert_metadata("alpha", "first")
            .expect("metadata");

        let keys: Vec<_> = plan.metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn serialization_round_trip_preserves_plan() {
        let mut plan = AgentPlan::new(name("research"));

        plan.add_step(step("collect"));
        plan.add_step(step("analyze"));

        plan.add_dependency(dependency("collect", "analyze"))
            .expect("dependency");

        plan.add_goal(PlanningGoal::new(
            name("complete_research"),
            reference("research_complete"),
        ));

        let encoded = serde_json::to_string(&plan)
            .expect("serialization");

        let decoded: AgentPlan = serde_json::from_str(&encoded)
            .expect("deserialization");

        assert_eq!(decoded, plan);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn dependency_analysis_does_not_depend_on_hardware() {
        let mut plan = AgentPlan::new(name("portable"));

        plan.add_step(step("input"));
        plan.add_step(step("process"));

        plan.add_dependency(dependency("input", "process"))
            .expect("dependency");

        assert_eq!(
            plan.topological_order()
                .expect("ordering")
                .len(),
            2
        );
    }
}
