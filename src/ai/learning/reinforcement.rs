
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Reinforcement Learning
//!
//! Path: `src/ai/learning/reinforcement.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent reinforcement-learning contracts.
//!
//! Reinforcement learning is represented as a computational domain,
//! not as a hardware-specific execution implementation.
//!
//! This module describes:
//!
//! - Reinforcement-learning problems.
//! - Environment contracts.
//! - State and action references.
//! - Reward specifications.
//! - Policies.
//! - Value functions.
//! - Exploration strategies.
//! - State transitions.
//! - Episodes and trajectories.
//! - Return semantics.
//! - Discounting.
//! - Learning updates.
//! - Reproducibility declarations.
//! - Structural validation.
//!
//! It does not execute reinforcement learning.
//!
//! ## Ownership
//!
//! This module owns reinforcement-learning-specific declarations
//! and their local structural validation.
//!
//! ## Does not own
//!
//! - Source grammar or AST construction.
//! - Semantic name resolution.
//! - Universal type checking.
//! - Tensor storage.
//! - Numerical kernels.
//! - Model parameter storage.
//! - Automatic differentiation.
//! - Optimizer implementations.
//! - Environment simulation.
//! - Random-number generation.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling or placement.
//! - Distributed communication.
//! - Runtime execution.
//! - Backend implementations.
//!
//! ## Canonical integration
//!
//! AI identifiers:
//!     crate::ai::ids
//!
//! AI errors:
//!     crate::ai::error
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! Learning algorithm declarations:
//!     crate::ai::learning::algorithms
//!
//! Training workflows:
//!     crate::ai::learning::training
//!
//! Gradient computation:
//!     crate::ai::learning::gradients
//!
//! Learning optimization:
//!     crate::ai::learning::optimization
//!
//! Semantic validation:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal computation:
//!     crate::zuir
//!
//! Actual execution:
//!     compiler / runtime / backend
//!
//! ## Grammar authority
//!
//! grammar/ai/learning.g4
//! grammar/ai/training.g4
//!
//! The universal learning statement remains owned by:
//!
//! grammar/statements/learn.g4
//!
//! ## POCO-REAF
//!
//! No universal maximum is imposed on:
//!
//! - States.
//! - Actions.
//! - Environments.
//! - Policies.
//! - Episodes.
//! - Trajectory transitions.
//! - Reward components.
//! - Objectives.
//! - Learning updates.
//! - Agents.
//! - Workers.
//! - Training iterations.
//! - Distributed participants.
//! - Model parameters.
//!
//! Actual capacity depends on implementation resources and
//! explicitly supplied operational budgets.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Important semantic distinction
//!
//! A valid reinforcement-learning declaration does not establish:
//!
//! - Environment implementation availability.
//! - Mathematical convergence.
//! - Policy optimality.
//! - Numerical correctness.
//! - Capability satisfaction.
//! - Resource feasibility.
//! - Execution authorization.
//! - Successful training.
//!
//! These require separate downstream validation and execution.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::{
    AiModelId,
    AiTrainingRunId,
};

use crate::ai::requirements::AiRequirements;

use crate::ai::learning::training::TrainingValueRef;

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
                format!(
                    "{description} contains control characters"
                ),
            )
            .with_phase(AiErrorPhase::Validation),
        );
    }

    Ok(())
}

fn invalid_rl(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::InvalidInput,
        message,
    )
    .with_phase(AiErrorPhase::Validation)
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
            return Err(invalid_rl(format!(
                "duplicate {description}: {value}"
            )));
        }
    }

    Ok(())
}

fn validate_reference(
    description: &str,
    reference: &TrainingValueRef,
) -> AiResult<()> {
    validate_text(description, reference.as_str())
}

// ============================================================================
// REINFORCEMENT LEARNING IDENTITIES
// ============================================================================

/// Identity of a reinforcement-learning environment declaration.
///
/// This is a symbolic identity, not a physical simulator identity.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct EnvironmentRef(String);

impl EnvironmentRef {
    /// Creates a validated environment reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text("environment reference", &value)?;

        Ok(Self(value))
    }

    /// Returns the environment reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EnvironmentRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Identity of an individual reinforcement-learning episode.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct EpisodeId(String);

impl EpisodeId {
    /// Creates an episode identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text("episode identity", &value)?;

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EpisodeId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ENVIRONMENT SEMANTICS
// ============================================================================

/// Classification of reinforcement-learning environment semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EnvironmentDynamics {
    /// The environment does not change independently of actions.
    Static,

    /// Environment transitions depend on time or external processes.
    Dynamic,

    /// Transitions involve probabilistic behavior.
    Stochastic,

    /// Dynamics are determined by an extension.
    ExtensionDefined,
}

/// Observability contract of an environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Observability {
    /// The complete environment state is observable.
    FullyObservable,

    /// Only observations of the environment state are available.
    PartiallyObservable,

    /// Observation semantics are extension-defined.
    ExtensionDefined,
}

/// Episode behavior of an environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EpisodeSemantics {
    /// Episodes have an explicit terminal condition.
    Episodic,

    /// Execution may continue indefinitely.
    Continuing,

    /// Episode semantics are determined by an extension.
    ExtensionDefined,
}

/// Contract describing a reinforcement-learning environment.
///
/// It declares logical behavior and does not implement simulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentContract {
    identity: EnvironmentRef,
    dynamics: EnvironmentDynamics,
    observability: Observability,
    episodes: EpisodeSemantics,
    state_type: TrainingValueRef,
    action_type: TrainingValueRef,
}

impl EnvironmentContract {
    /// Creates an environment contract.
    pub fn new(
        identity: EnvironmentRef,
        dynamics: EnvironmentDynamics,
        observability: Observability,
        episodes: EpisodeSemantics,
        state_type: TrainingValueRef,
        action_type: TrainingValueRef,
    ) -> AiResult<Self> {
        let contract = Self {
            identity,
            dynamics,
            observability,
            episodes,
            state_type,
            action_type,
        };

        contract.validate()?;

        Ok(contract)
    }

    /// Returns the environment identity.
    #[must_use]
    pub fn identity(&self) -> &EnvironmentRef {
        &self.identity
    }

    /// Returns environment dynamics.
    #[must_use]
    pub const fn dynamics(&self) -> EnvironmentDynamics {
        self.dynamics
    }

    /// Returns the observability contract.
    #[must_use]
    pub const fn observability(&self) -> Observability {
        self.observability
    }

    /// Returns episode semantics.
    #[must_use]
    pub const fn episodes(&self) -> EpisodeSemantics {
        self.episodes
    }

    /// Returns the declared state type reference.
    #[must_use]
    pub fn state_type(&self) -> &TrainingValueRef {
        &self.state_type
    }

    /// Returns the declared action type reference.
    #[must_use]
    pub fn action_type(&self) -> &TrainingValueRef {
        &self.action_type
    }

    /// Validates the environment declaration.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "environment identity",
            self.identity.as_str(),
        )?;

        validate_reference(
            "environment state type",
            &self.state_type,
        )?;

        validate_reference(
            "environment action type",
            &self.action_type,
        )
    }
}

// ============================================================================
// STATE AND ACTION REFERENCES
// ============================================================================

/// Symbolic reference to a reinforcement-learning state.
///
/// The actual state representation may be a tensor, structured value,
/// observation, quantum-derived value, or extension-defined object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateRef {
    value: TrainingValueRef,
    observed: bool,
}

impl StateRef {
    /// Creates a state reference.
    pub fn new(
        value: TrainingValueRef,
        observed: bool,
    ) -> AiResult<Self> {
        validate_reference("state reference", &value)?;

        Ok(Self {
            value,
            observed,
        })
    }

    /// Returns the underlying symbolic value.
    #[must_use]
    pub fn value(&self) -> &TrainingValueRef {
        &self.value
    }

    /// Returns whether this state is an observation.
    #[must_use]
    pub const fn is_observed(&self) -> bool {
        self.observed
    }
}

/// Symbolic reference to a reinforcement-learning action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionRef {
    value: TrainingValueRef,
}

impl ActionRef {
    /// Creates an action reference.
    pub fn new(
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference("action reference", &value)?;

        Ok(Self { value })
    }

    /// Returns the action reference.
    #[must_use]
    pub fn value(&self) -> &TrainingValueRef {
        &self.value
    }
}

// ============================================================================
// REWARD SEMANTICS
// ============================================================================

/// Aggregation semantics for reward components.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RewardAggregation {
    /// A scalar reward.
    Scalar,

    /// Multiple independently meaningful reward components.
    MultiObjective,

    /// Reward represented by a structured value.
    Structured,

    /// Reward aggregation is extension-defined.
    ExtensionDefined,
}

/// A symbolic reward component.
///
/// The value may be a scalar, tensor, probability, structured value,
/// or expression. Numerical evaluation is delegated downstream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewardComponent {
    identity: String,
    value: TrainingValueRef,
    weight: Option<TrainingValueRef>,
}

impl RewardComponent {
    /// Creates a reward component.
    pub fn new(
        identity: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        let identity = identity.into();

        validate_text("reward component identity", &identity)?;
        validate_reference("reward value", &value)?;

        Ok(Self {
            identity,
            value,
            weight: None,
        })
    }

    /// Associates a symbolic objective weight.
    pub fn with_weight(
        mut self,
        weight: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference("reward weight", &weight)?;

        self.weight = Some(weight);

        Ok(self)
    }

    /// Returns the reward component identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the reward value reference.
    #[must_use]
    pub fn value(&self) -> &TrainingValueRef {
        &self.value
    }

    /// Returns the optional symbolic weight.
    #[must_use]
    pub fn weight(&self) -> Option<&TrainingValueRef> {
        self.weight.as_ref()
    }
}

/// Complete reward specification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewardSpecification {
    aggregation: RewardAggregation,
    components: Vec<RewardComponent>,
}

impl RewardSpecification {
    /// Creates a reward specification.
    #[must_use]
    pub fn new(
        aggregation: RewardAggregation,
        components: Vec<RewardComponent>,
    ) -> Self {
        Self {
            aggregation,
            components,
        }
    }

    /// Returns the aggregation semantics.
    #[must_use]
    pub const fn aggregation(&self) -> RewardAggregation {
        self.aggregation
    }

    /// Returns reward components.
    #[must_use]
    pub fn components(&self) -> &[RewardComponent] {
        &self.components
    }

    /// Validates reward structure.
    pub fn validate(&self) -> AiResult<()> {
        if self.components.is_empty() {
            return Err(invalid_rl(
                "a reward specification requires at least one component",
            ));
        }

        let identities: Vec<&str> = self.components
            .iter()
            .map(|component| component.identity())
            .collect();

        validate_unique(
            &identities,
            "reward component identity",
        )?;

        for component in &self.components {
            validate_text(
                "reward component identity",
                component.identity(),
            )?;

            validate_reference(
                "reward component value",
                component.value(),
            )?;

            if let Some(weight) = component.weight() {
                validate_reference("reward weight", weight)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// POLICY CONTRACTS
// ============================================================================

/// Policy behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PolicyBehavior {
    /// The policy selects actions without stochastic sampling.
    Deterministic,

    /// The policy defines a probability distribution over actions.
    Stochastic,

    /// The policy combines deterministic and stochastic behavior.
    Hybrid,

    /// The policy behavior is extension-defined.
    ExtensionDefined,
}

/// Policy update semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PolicyUpdateSemantics {
    /// Policy parameters may change during learning.
    Trainable,

    /// The policy remains fixed.
    Fixed,

    /// Policy changes are controlled by an external process.
    ExternallyManaged,

    /// Update behavior is extension-defined.
    ExtensionDefined,
}

/// Reinforcement-learning policy contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyContract {
    identity: TrainingValueRef,
    behavior: PolicyBehavior,
    update: PolicyUpdateSemantics,
    observation_input: TrainingValueRef,
    action_output: TrainingValueRef,
}

impl PolicyContract {
    /// Creates a policy contract.
    pub fn new(
        identity: TrainingValueRef,
        behavior: PolicyBehavior,
        update: PolicyUpdateSemantics,
        observation_input: TrainingValueRef,
        action_output: TrainingValueRef,
    ) -> AiResult<Self> {
        let policy = Self {
            identity,
            behavior,
            update,
            observation_input,
            action_output,
        };

        policy.validate()?;

        Ok(policy)
    }

    /// Returns the policy identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }

    /// Returns policy behavior.
    #[must_use]
    pub const fn behavior(&self) -> PolicyBehavior {
        self.behavior
    }

    /// Returns policy update semantics.
    #[must_use]
    pub const fn update_semantics(&self) -> PolicyUpdateSemantics {
        self.update
    }

    /// Returns the observation input.
    #[must_use]
    pub fn observation_input(&self) -> &TrainingValueRef {
        &self.observation_input
    }

    /// Returns the action output.
    #[must_use]
    pub fn action_output(&self) -> &TrainingValueRef {
        &self.action_output
    }

    /// Validates the policy.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference("policy identity", &self.identity)?;

        validate_reference(
            "policy observation input",
            &self.observation_input,
        )?;

        validate_reference(
            "policy action output",
            &self.action_output,
        )
    }
}

// ============================================================================
// VALUE FUNCTIONS
// ============================================================================

/// Value-function interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ValueFunctionKind {
    /// Estimates the value of a state.
    StateValue,

    /// Estimates the value of a state-action pair.
    ActionValue,

    /// Estimates a state-action value using an extension-defined model.
    GeneralizedActionValue,

    /// Value semantics are extension-defined.
    ExtensionDefined,
}

/// Value-function contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueFunctionContract {
    identity: TrainingValueRef,
    kind: ValueFunctionKind,
    input: TrainingValueRef,
    output: TrainingValueRef,
}

impl ValueFunctionContract {
    /// Creates a value-function contract.
    pub fn new(
        identity: TrainingValueRef,
        kind: ValueFunctionKind,
        input: TrainingValueRef,
        output: TrainingValueRef,
    ) -> AiResult<Self> {
        let function = Self {
            identity,
            kind,
            input,
            output,
        };

        function.validate()?;

        Ok(function)
    }

    /// Returns the identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }

    /// Returns the function kind.
    #[must_use]
    pub const fn kind(&self) -> ValueFunctionKind {
        self.kind
    }

    /// Returns the input reference.
    #[must_use]
    pub fn input(&self) -> &TrainingValueRef {
        &self.input
    }

    /// Returns the output reference.
    #[must_use]
    pub fn output(&self) -> &TrainingValueRef {
        &self.output
    }

    /// Validates the value-function contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference("value function identity", &self.identity)?;

        validate_reference("value function input", &self.input)?;

        validate_reference("value function output", &self.output)
    }
}

// ============================================================================
// EXPLORATION
// ============================================================================

/// Exploration strategy declaration.
///
/// New exploration strategies can be introduced without changing
/// the core Rust enumeration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExplorationStrategy {
    identity: TrainingValueRef,
    parameters: BTreeMap<String, TrainingValueRef>,
}

impl ExplorationStrategy {
    /// Creates an exploration strategy.
    pub fn new(
        identity: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference("exploration strategy", &identity)?;

        Ok(Self {
            identity,
            parameters: BTreeMap::new(),
        })
    }

    /// Adds an extension-defined strategy parameter.
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_text("exploration parameter name", &name)?;
        validate_reference("exploration parameter value", &value)?;

        if self.parameters.contains_key(&name) {
            return Err(invalid_rl(format!(
                "duplicate exploration parameter: {name}"
            )));
        }

        self.parameters.insert(name, value);

        Ok(self)
    }

    /// Returns the strategy identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }

    /// Returns deterministic strategy parameters.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, TrainingValueRef> {
        &self.parameters
    }
}

// ============================================================================
// DISCOUNTING AND RETURN SEMANTICS
// ============================================================================

/// Discounting contract.
///
/// A concrete discount factor must be finite and within its mathematical
/// domain. Symbolic factors remain references for semantic evaluation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Discounting {
    /// Undiscounted return.
    Undiscounted,

    /// Explicit numerical discount factor.
    ///
    /// The factor must be finite and lie within [0, 1].
    Constant(f64),

    /// Symbolically defined discount factor.
    Symbolic(TrainingValueRef),

    /// Discounting semantics are extension-defined.
    ExtensionDefined,
}

impl Discounting {
    /// Validates the discounting declaration.
    pub fn validate(&self) -> AiResult<()> {
        match self {
            Self::Constant(value) => {
                if !value.is_finite() || !(0.0..=1.0).contains(value) {
                    return Err(invalid_rl(
                        "discount factor must be finite and between zero and one",
                    ));
                }
            }

            Self::Symbolic(reference) => {
                validate_reference(
                    "symbolic discount factor",
                    reference,
                )?;
            }

            Self::Undiscounted | Self::ExtensionDefined => {}
        }

        Ok(())
    }
}

/// Return calculation semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReturnSemantics {
    /// Sum of rewards.
    Cumulative,

    /// Discounted cumulative reward.
    Discounted,

    /// Average reward objective.
    AverageReward,

    /// Generalized return semantics.
    Generalized,
}

// ============================================================================
// TERMINATION AND TRUNCATION
// ============================================================================

/// Logical termination status of a transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransitionBoundary {
    /// The environment continues.
    Continuing,

    /// A genuine terminal state has been reached.
    Terminated,

    /// The trajectory was interrupted without establishing
    /// that the underlying environment reached a terminal state.
    Truncated,
}

// ============================================================================
// STATE TRANSITIONS
// ============================================================================

/// One symbolic reinforcement-learning transition.
///
/// Termination and truncation are deliberately distinct.
/// This distinction is essential for correct return estimation,
/// bootstrapping and temporal-difference learning.
#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    sequence: u128,
    state: StateRef,
    action: ActionRef,
    reward: Vec<RewardComponent>,
    next_state: StateRef,
    boundary: TransitionBoundary,
}

impl Transition {
    /// Creates a transition.
    pub fn new(
        sequence: u128,
        state: StateRef,
        action: ActionRef,
        reward: Vec<RewardComponent>,
        next_state: StateRef,
        boundary: TransitionBoundary,
    ) -> AiResult<Self> {
        if reward.is_empty() {
            return Err(invalid_rl(
                "transition requires at least one reward component",
            ));
        }

        for component in &reward {
            validate_reference(
                "transition reward",
                component.value(),
            )?;
        }

        Ok(Self {
            sequence,
            state,
            action,
            reward,
            next_state,
            boundary,
        })
    }

    /// Returns the transition sequence number.
    #[must_use]
    pub const fn sequence(&self) -> u128 {
        self.sequence
    }

    /// Returns the initial state.
    #[must_use]
    pub fn state(&self) -> &StateRef {
        &self.state
    }

    /// Returns the action.
    #[must_use]
    pub fn action(&self) -> &ActionRef {
        &self.action
    }

    /// Returns reward components.
    #[must_use]
    pub fn reward(&self) -> &[RewardComponent] {
        &self.reward
    }

    /// Returns the resulting state.
    #[must_use]
    pub fn next_state(&self) -> &StateRef {
        &self.next_state
    }

    /// Returns the transition boundary.
    #[must_use]
    pub const fn boundary(&self) -> TransitionBoundary {
        self.boundary
    }

    /// Validates local transition invariants.
    pub fn validate(&self) -> AiResult<()> {
        if self.reward.is_empty() {
            return Err(invalid_rl(
                "transition has no reward components",
            ));
        }

        for component in &self.reward {
            validate_reference(
                "transition reward",
                component.value(),
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// TRAJECTORIES
// ============================================================================

/// A reinforcement-learning trajectory.
///
/// A trajectory contains ordered symbolic transitions.
/// It does not own numerical replay-buffer storage.
#[derive(Clone, Debug, PartialEq)]
pub struct Trajectory {
    episode: EpisodeId,
    environment: EnvironmentRef,
    transitions: Vec<Transition>,
}

impl Trajectory {
    /// Creates an empty trajectory.
    #[must_use]
    pub fn new(
        episode: EpisodeId,
        environment: EnvironmentRef,
    ) -> Self {
        Self {
            episode,
            environment,
            transitions: Vec::new(),
        }
    }

    /// Adds a transition.
    ///
    /// Sequence numbers must be strictly increasing.
    /// A terminal or truncated transition must be the last transition.
    pub fn with_transition(
        mut self,
        transition: Transition,
    ) -> AiResult<Self> {
        transition.validate()?;

        if let Some(previous) = self.transitions.last() {
            if transition.sequence() <= previous.sequence() {
                return Err(invalid_rl(
                    "trajectory transition sequence must increase",
                ));
            }

            if previous.boundary() != TransitionBoundary::Continuing {
                return Err(invalid_rl(
                    "a transition cannot follow a terminated or truncated transition",
                ));
            }
        }

        self.transitions.push(transition);

        Ok(self)
    }

    /// Returns the episode identity.
    #[must_use]
    pub fn episode(&self) -> &EpisodeId {
        &self.episode
    }

    /// Returns the environment identity.
    #[must_use]
    pub fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns ordered transitions.
    #[must_use]
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    /// Returns whether the trajectory has transitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.transitions.is_empty()
    }

    /// Returns whether the final transition terminated the environment.
    #[must_use]
    pub fn is_terminated(&self) -> bool {
        self.transitions.last().is_some_and(
            |transition| {
                transition.boundary()
                    == TransitionBoundary::Terminated
            },
        )
    }

    /// Returns whether the trajectory was truncated.
    #[must_use]
    pub fn is_truncated(&self) -> bool {
        self.transitions.last().is_some_and(
            |transition| {
                transition.boundary()
                    == TransitionBoundary::Truncated
            },
        )
    }

    /// Validates trajectory structure.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "trajectory environment",
            self.environment.as_str(),
        )?;

        for (index, transition) in self.transitions.iter().enumerate() {
            transition.validate()?;

            if let Some(next) = self.transitions.get(index + 1) {
                if next.sequence() <= transition.sequence() {
                    return Err(invalid_rl(
                        "trajectory transitions are not ordered",
                    ));
                }

                if transition.boundary() != TransitionBoundary::Continuing {
                    return Err(invalid_rl(
                        "trajectory contains transitions after its boundary",
                    ));
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// LEARNING UPDATE
// ============================================================================

/// Classification of a reinforcement-learning update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LearningUpdateKind {
    /// Policy improvement.
    PolicyImprovement,

    /// State-value estimation update.
    StateValueUpdate,

    /// Action-value estimation update.
    ActionValueUpdate,

    /// Temporal-difference update.
    TemporalDifference,

    /// Advantage estimation.
    AdvantageEstimation,

    /// Reward-model update.
    RewardModelUpdate,

    /// Model-based environment update.
    EnvironmentModelUpdate,

    /// Generalized reinforcement-learning update.
    Generalized,

    /// Extension-defined update.
    ExtensionDefined,
}

/// Symbolic reinforcement-learning update contract.
///
/// Actual parameter updates are performed by learning optimization
/// and runtime infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningUpdate {
    identity: TrainingValueRef,
    kind: LearningUpdateKind,
    inputs: Vec<TrainingValueRef>,
    outputs: Vec<TrainingValueRef>,
    dependencies: Vec<TrainingValueRef>,
}

impl LearningUpdate {
    /// Creates an update contract.
    pub fn new(
        identity: TrainingValueRef,
        kind: LearningUpdateKind,
    ) -> AiResult<Self> {
        validate_reference("learning update identity", &identity)?;

        Ok(Self {
            identity,
            kind,
            inputs: Vec::new(),
            outputs: Vec::new(),
            dependencies: Vec::new(),
        })
    }

    /// Adds an update input.
    pub fn with_input(
        mut self,
        input: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference("learning update input", &input)?;

        self.inputs.push(input);

        Ok(self)
    }

    /// Adds an update output.
    pub fn with_output(
        mut self,
        output: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference("learning update output", &output)?;

        self.outputs.push(output);

        Ok(self)
    }

    /// Adds an update dependency.
    pub fn with_dependency(
        mut self,
        dependency: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "learning update dependency",
            &dependency,
        )?;

        self.dependencies.push(dependency);

        Ok(self)
    }

    /// Returns the update identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }

    /// Returns update classification.
    #[must_use]
    pub const fn kind(&self) -> LearningUpdateKind {
        self.kind
    }

    /// Returns update inputs.
    #[must_use]
    pub fn inputs(&self) -> &[TrainingValueRef] {
        &self.inputs
    }

    /// Returns update outputs.
    #[must_use]
    pub fn outputs(&self) -> &[TrainingValueRef] {
        &self.outputs
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[TrainingValueRef] {
        &self.dependencies
    }

    /// Validates the update contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference("learning update", &self.identity)?;

        if self.outputs.is_empty() {
            return Err(invalid_rl(
                "learning update requires at least one declared output",
            ));
        }

        for input in &self.inputs {
            validate_reference("learning update input", input)?;
        }

        for output in &self.outputs {
            validate_reference("learning update output", output)?;
        }

        for dependency in &self.dependencies {
            validate_reference(
                "learning update dependency",
                dependency,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// REINFORCEMENT-LEARNING PROBLEM
// ============================================================================

/// Complete reinforcement-learning problem declaration.
///
/// This is the principal domain object consumed by downstream
/// semantic, IR and execution integration.
#[derive(Clone, Debug, PartialEq)]
pub struct ReinforcementLearningProblem {
    model: Option<AiModelId>,
    training_run: Option<AiTrainingRunId>,
    environment: EnvironmentContract,
    policy: PolicyContract,
    reward: RewardSpecification,
    value_function: Option<ValueFunctionContract>,
    exploration: Option<ExplorationStrategy>,
    discounting: Discounting,
    return_semantics: ReturnSemantics,
    requirements: AiRequirements,
    updates: Vec<LearningUpdate>,
    metadata: BTreeMap<String, TrainingValueRef>,
}

impl ReinforcementLearningProblem {
    /// Creates a reinforcement-learning problem.
    pub fn new(
        environment: EnvironmentContract,
        policy: PolicyContract,
        reward: RewardSpecification,
    ) -> AiResult<Self> {
        let problem = Self {
            model: None,
            training_run: None,
            environment,
            policy,
            reward,
            value_function: None,
            exploration: None,
            discounting: Discounting::Undiscounted,
            return_semantics: ReturnSemantics::Cumulative,
            requirements: AiRequirements::new(),
            updates: Vec::new(),
            metadata: BTreeMap::new(),
        };

        problem.validate()?;

        Ok(problem)
    }

    /// Associates a canonical AI model identity.
    #[must_use]
    pub fn with_model(
        mut self,
        model: AiModelId,
    ) -> Self {
        self.model = Some(model);
        self
    }

    /// Associates a canonical training-run identity.
    #[must_use]
    pub fn with_training_run(
        mut self,
        run: AiTrainingRunId,
    ) -> Self {
        self.training_run = Some(run);
        self
    }

    /// Associates a value function.
    pub fn with_value_function(
        mut self,
        value_function: ValueFunctionContract,
    ) -> AiResult<Self> {
        value_function.validate()?;

        self.value_function = Some(value_function);

        Ok(self)
    }

    /// Associates an exploration strategy.
    #[must_use]
    pub fn with_exploration(
        mut self,
        exploration: ExplorationStrategy,
    ) -> Self {
        self.exploration = Some(exploration);
        self
    }

    /// Sets discounting semantics.
    pub fn with_discounting(
        mut self,
        discounting: Discounting,
    ) -> AiResult<Self> {
        discounting.validate()?;

        self.discounting = discounting;

        Ok(self)
    }

    /// Sets return semantics.
    #[must_use]
    pub fn with_return_semantics(
        mut self,
        semantics: ReturnSemantics,
    ) -> Self {
        self.return_semantics = semantics;
        self
    }

    /// Replaces AI computational requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Adds a learning update.
    pub fn with_update(
        mut self,
        update: LearningUpdate,
    ) -> AiResult<Self> {
        update.validate()?;

        self.updates.push(update);

        Ok(self)
    }

    /// Adds symbolic metadata.
    pub fn with_metadata(
        mut self,
        name: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_text("reinforcement metadata key", &name)?;
        validate_reference("reinforcement metadata value", &value)?;

        if self.metadata.contains_key(&name) {
            return Err(invalid_rl(format!(
                "duplicate reinforcement metadata key: {name}"
            )));
        }

        self.metadata.insert(name, value);

        Ok(self)
    }

    /// Returns the optional model identity.
    #[must_use]
    pub const fn model(&self) -> Option<AiModelId> {
        self.model
    }

    /// Returns the optional training-run identity.
    #[must_use]
    pub const fn training_run(&self) -> Option<AiTrainingRunId> {
        self.training_run
    }

    /// Returns the environment contract.
    #[must_use]
    pub fn environment(&self) -> &EnvironmentContract {
        &self.environment
    }

    /// Returns the policy.
    #[must_use]
    pub fn policy(&self) -> &PolicyContract {
        &self.policy
    }

    /// Returns the reward specification.
    #[must_use]
    pub fn reward(&self) -> &RewardSpecification {
        &self.reward
    }

    /// Returns the optional value function.
    #[must_use]
    pub fn value_function(&self) -> Option<&ValueFunctionContract> {
        self.value_function.as_ref()
    }

    /// Returns the optional exploration strategy.
    #[must_use]
    pub fn exploration(&self) -> Option<&ExplorationStrategy> {
        self.exploration.as_ref()
    }

    /// Returns discounting semantics.
    #[must_use]
    pub fn discounting(&self) -> &Discounting {
        &self.discounting
    }

    /// Returns return semantics.
    #[must_use]
    pub const fn return_semantics(&self) -> ReturnSemantics {
        self.return_semantics
    }

    /// Returns AI computational requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Returns declared learning updates.
    #[must_use]
    pub fn updates(&self) -> &[LearningUpdate] {
        &self.updates
    }

    /// Returns symbolic metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, TrainingValueRef> {
        &self.metadata
    }

    /// Validates all locally owned problem invariants.
    ///
    /// This does not perform semantic symbol resolution,
    /// mathematical convergence analysis or execution feasibility.
    pub fn validate(&self) -> AiResult<()> {
        self.environment.validate()?;
        self.policy.validate()?;
        self.reward.validate()?;
        self.discounting.validate()?;

        if let Some(value_function) = &self.value_function {
            value_function.validate()?;
        }

        for update in &self.updates {
            update.validate()?;
        }

        let mut update_ids = BTreeSet::new();

        for update in &self.updates {
            if !update_ids.insert(update.identity()) {
                return Err(invalid_rl(
                    "duplicate reinforcement learning update identity",
                ));
            }
        }

        for (name, value) in &self.metadata {
            validate_text("reinforcement metadata key", name)?;
            validate_reference("reinforcement metadata value", value)?;
        }

        Ok(())
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Scope of reinforcement-learning validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReinforcementValidationLevel {
    /// Local structural validation only.
    Structural,

    /// Canonical semantic analysis completed.
    Semantic,

    /// Mathematical legality was established.
    Mathematical,

    /// Target realization feasibility was evaluated.
    Feasibility,
}

/// Reinforcement-learning validation report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReinforcementValidationReport {
    level: ReinforcementValidationLevel,
    valid: bool,
    diagnostics: Vec<String>,
}

impl ReinforcementValidationReport {
    /// Creates a structural validation report.
    ///
    /// This constructor cannot claim that semantic, mathematical
    /// or execution validation has occurred.
    pub fn structural(
        problem: &ReinforcementLearningProblem,
    ) -> Self {
        match problem.validate() {
            Ok(()) => Self {
                level: ReinforcementValidationLevel::Structural,
                valid: true,
                diagnostics: Vec::new(),
            },

            Err(error) => Self {
                level: ReinforcementValidationLevel::Structural,
                valid: false,
                diagnostics: vec![error.to_string()],
            },
        }
    }

    /// Returns the validation level.
    #[must_use]
    pub const fn level(&self) -> ReinforcementValidationLevel {
        self.level
    }

    /// Returns whether this validation stage succeeded.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(value: &str) -> TrainingValueRef {
        TrainingValueRef::new(value)
            .expect("valid symbolic reference")
    }

    fn environment() -> EnvironmentContract {
        EnvironmentContract::new(
            EnvironmentRef::new("environment.simulation")
                .expect("environment identity"),
            EnvironmentDynamics::Stochastic,
            Observability::FullyObservable,
            EpisodeSemantics::Episodic,
            reference("state.type"),
            reference("action.type"),
        )
        .expect("valid environment")
    }

    fn policy() -> PolicyContract {
        PolicyContract::new(
            reference("policy.main"),
            PolicyBehavior::Stochastic,
            PolicyUpdateSemantics::Trainable,
            reference("observation.input"),
            reference("action.output"),
        )
        .expect("valid policy")
    }

    fn reward() -> RewardSpecification {
        RewardSpecification::new(
            RewardAggregation::Scalar,
            vec![
                RewardComponent::new(
                    "primary",
                    reference("reward.value"),
                )
                .expect("valid reward"),
            ],
        )
    }

    fn problem() -> ReinforcementLearningProblem {
        ReinforcementLearningProblem::new(
            environment(),
            policy(),
            reward(),
        )
        .expect("valid problem")
    }

    fn transition(
        sequence: u128,
        boundary: TransitionBoundary,
    ) -> Transition {
        Transition::new(
            sequence,
            StateRef::new(
                reference("state.current"),
                true,
            )
            .expect("state"),
            ActionRef::new(
                reference("action.selected"),
            )
            .expect("action"),
            vec![
                RewardComponent::new(
                    "reward",
                    reference("reward.value"),
                )
                .expect("reward"),
            ],
            StateRef::new(
                reference("state.next"),
                true,
            )
            .expect("next state"),
            boundary,
        )
        .expect("transition")
    }

    #[test]
    fn valid_environment_is_accepted() {
        assert!(environment().validate().is_ok());
    }

    #[test]
    fn valid_policy_is_accepted() {
        assert!(policy().validate().is_ok());
    }

    #[test]
    fn empty_reward_specification_is_rejected() {
        let result = RewardSpecification::new(
            RewardAggregation::Scalar,
            Vec::new(),
        );

        assert!(result.validate().is_err());
    }

    #[test]
    fn duplicate_reward_components_are_rejected() {
        let result = RewardSpecification::new(
            RewardAggregation::MultiObjective,
            vec![
                RewardComponent::new(
                    "objective",
                    reference("reward.first"),
                )
                .expect("first"),
                RewardComponent::new(
                    "objective",
                    reference("reward.second"),
                )
                .expect("second"),
            ],
        );

        assert!(result.validate().is_err());
    }

    #[test]
    fn valid_discount_factors_are_accepted() {
        assert!(Discounting::Constant(0.0).validate().is_ok());
        assert!(Discounting::Constant(0.5).validate().is_ok());
        assert!(Discounting::Constant(1.0).validate().is_ok());
    }

    #[test]
    fn invalid_discount_factors_are_rejected() {
        assert!(Discounting::Constant(-0.1).validate().is_err());
        assert!(Discounting::Constant(1.1).validate().is_err());
        assert!(Discounting::Constant(f64::NAN).validate().is_err());
        assert!(Discounting::Constant(f64::INFINITY).validate().is_err());
    }

    #[test]
    fn symbolic_discounting_is_supported() {
        assert!(
            Discounting::Symbolic(
                reference("discount.factor")
            )
            .validate()
            .is_ok()
        );
    }

    #[test]
    fn valid_problem_is_accepted() {
        assert!(problem().validate().is_ok());
    }

    #[test]
    fn learning_update_requires_output() {
        let update = LearningUpdate::new(
            reference("update.policy"),
            LearningUpdateKind::PolicyImprovement,
        )
        .expect("update");

        assert!(update.validate().is_err());
    }

    #[test]
    fn learning_update_with_output_is_valid() {
        let update = LearningUpdate::new(
            reference("update.policy"),
            LearningUpdateKind::PolicyImprovement,
        )
        .expect("update")
        .with_output(reference("policy.updated"))
        .expect("output");

        assert!(update.validate().is_ok());
    }

    #[test]
    fn trajectory_accepts_ordered_transitions() {
        let trajectory = Trajectory::new(
            EpisodeId::new("episode.one").expect("episode"),
            EnvironmentRef::new("environment.simulation")
                .expect("environment"),
        )
        .with_transition(
            transition(1, TransitionBoundary::Continuing),
        )
        .expect("first transition")
        .with_transition(
            transition(2, TransitionBoundary::Terminated),
        )
        .expect("terminal transition");

        assert!(trajectory.validate().is_ok());
        assert!(trajectory.is_terminated());
        assert!(!trajectory.is_truncated());
    }

    #[test]
    fn trajectory_rejects_non_increasing_sequence() {
        let trajectory = Trajectory::new(
            EpisodeId::new("episode.one").expect("episode"),
            EnvironmentRef::new("environment.simulation")
                .expect("environment"),
        )
        .with_transition(
            transition(2, TransitionBoundary::Continuing),
        )
        .expect("first transition");

        let result = trajectory.with_transition(
            transition(1, TransitionBoundary::Continuing),
        );

        assert!(result.is_err());
    }

    #[test]
    fn trajectory_rejects_transition_after_termination() {
        let trajectory = Trajectory::new(
            EpisodeId::new("episode.one").expect("episode"),
            EnvironmentRef::new("environment.simulation")
                .expect("environment"),
        )
        .with_transition(
            transition(1, TransitionBoundary::Terminated),
        )
        .expect("terminal transition");

        let result = trajectory.with_transition(
            transition(2, TransitionBoundary::Continuing),
        );

        assert!(result.is_err());
    }

    #[test]
    fn truncation_is_not_mistaken_for_termination() {
        let trajectory = Trajectory::new(
            EpisodeId::new("episode.truncated").expect("episode"),
            EnvironmentRef::new("environment.simulation")
                .expect("environment"),
        )
        .with_transition(
            transition(1, TransitionBoundary::Truncated),
        )
        .expect("truncated transition");

        assert!(trajectory.is_truncated());
        assert!(!trajectory.is_terminated());
    }

    #[test]
    fn model_and_training_ids_use_canonical_types() {
        let model = AiModelId::new(1).expect("model");
        let training = AiTrainingRunId::new(2).expect("training");

        let problem = problem()
            .with_model(model)
            .with_training_run(training);

        assert_eq!(problem.model(), Some(model));
        assert_eq!(problem.training_run(), Some(training));
    }

    #[test]
    fn structural_validation_does_not_claim_execution_feasibility() {
        let report =
            ReinforcementValidationReport::structural(&problem());

        assert!(report.is_valid());

        assert_eq!(
            report.level(),
            ReinforcementValidationLevel::Structural,
        );
    }

    #[test]
    fn requirements_are_preserved() {
        let requirements = AiRequirements::new();

        let problem = problem()
            .with_requirements(requirements.clone());

        assert_eq!(problem.requirements(), &requirements);
    }

    #[test]
    fn extension_defined_environment_semantics_are_supported() {
        let environment = EnvironmentContract::new(
            EnvironmentRef::new("future.environment")
                .expect("identity"),
            EnvironmentDynamics::ExtensionDefined,
            Observability::ExtensionDefined,
            EpisodeSemantics::ExtensionDefined,
            reference("future.state"),
            reference("future.action"),
        );

        assert!(environment.is_ok());
    }

    #[test]
    fn transition_sequence_supports_large_identifiers() {
        let transition = transition(
            u128::MAX,
            TransitionBoundary::Continuing,
        );

        assert_eq!(transition.sequence(), u128::MAX);
    }
}
