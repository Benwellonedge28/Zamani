
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Adaptive Learning Contracts
//!
//! Path: `src/ai/learning/adaptation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent adaptation contracts for AI learning.
//!
//! This module describes how an AI learning system may adapt its
//! models, learning strategies, policies, and learning behaviour
//! in response to feedback, evidence, observations, objectives,
//! and changing computational conditions.
//!
//! It describes computational intent, not execution.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Adaptive learning declarations.
//! - Adaptation targets.
//! - Adaptation sources.
//! - Adaptation strategies.
//! - Adaptation triggers.
//! - Adaptation context.
//! - Adaptation constraints.
//! - Adaptation policies as references.
//! - Adaptation lineage.
//! - Adaptation validation.
//! - Adaptation lifecycle descriptions.
//!
//! ## Does not own
//!
//! - Universal adaptation syntax.
//! - Canonical AST.
//! - Semantic name resolution.
//! - Model parameter storage.
//! - Tensor storage.
//! - Numerical training.
//! - Optimizer execution.
//! - Gradient computation.
//! - Hardware selection.
//! - Resource allocation.
//! - Distributed communication.
//! - Scheduling.
//! - Runtime execution.
//! - Authorization enforcement.
//! - Persistent provenance storage.
//!
//! ## Canonical integration
//!
//! Universal adaptation syntax:
//!     grammar/statements/adapt.g4
//!
//! AI adaptation grammar:
//!     grammar/ai/adaptation.g4
//!
//! AI learning grammar:
//!     grammar/ai/learning.g4
//!
//! Model identity:
//!     crate::ai::ids::AiModelId
//!
//! Training identity:
//!     crate::ai::ids::AiTrainingRunId
//!
//! Training workflows:
//!     crate::ai::learning::training
//!
//! Learning algorithms:
//!     crate::ai::learning::algorithms
//!
//! Learning optimization:
//!     crate::ai::learning::optimization
//!
//! Differentiation:
//!     crate::ai::differentiation
//!
//! Semantic adaptation:
//!     crate::frontend::semantic
//!
//! Execution:
//!     crate::runtime
//!
//! Distributed execution:
//!     crate::distributed
//!
//! ## POCO-REAF
//!
//! Adaptation declarations are independent of physical hardware.
//!
//! No universal limits are imposed on:
//!
//! - Models.
//! - Adaptation operations.
//! - Adaptation strategies.
//! - Feedback sources.
//! - Adaptation context.
//! - Adaptation history.
//! - Learning iterations.
//! - Agents.
//! - Workers.
//! - Devices.
//! - Accelerators.
//! - Tensor dimensions.
//! - Tensor rank.
//!
//! Actual feasibility depends on implementation resources,
//! declared requirements, capabilities, policies and physical reality.
//!
//! Literal infinite computation is not promised.
//!
//! ## Important invariant
//!
//! A valid adaptation declaration does not establish that:
//!
//! - Adaptation is authorized.
//! - The target is mutable.
//! - The selected algorithm exists.
//! - Required feedback is available.
//! - Required resources exist.
//! - A model update has succeeded.
//! - A runtime operation has occurred.
//!
//! These are separate downstream decisions.

use std::collections::BTreeSet;
use std::fmt;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiResult,
};

use crate::ai::ids::{
    AiModelId,
    AiTrainingRunId,
};

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_text(
    description: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(AiError::new(
            AiErrorKind::InvalidInput,
            format!("{description} cannot be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(AiError::new(
            AiErrorKind::InvalidInput,
            format!("{description} contains control characters"),
        ));
    }

    Ok(())
}

fn validate_qualified_name(
    description: &str,
    value: &str,
) -> AiResult<()> {
    validate_text(description, value)?;

    let mut components = value.split('.');

    if components.any(|component| component.trim().is_empty()) {
        return Err(AiError::new(
            AiErrorKind::InvalidInput,
            format!("{description} contains an empty namespace component"),
        ));
    }

    Ok(())
}

fn invalid_adaptation(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::InvalidInput,
        message,
    )
}

// ============================================================================
// OPEN-WORLD IDENTITIES
// ============================================================================

/// A qualified identity for an adaptation-related algorithm,
/// strategy, extension, policy, or semantic operation.
///
/// Names are open-world. New adaptation algorithms do not require
/// modifying a closed Rust enumeration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdaptationName(String);

impl AdaptationName {
    /// Creates a validated qualified identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_qualified_name(
            "adaptation identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the qualified identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AdaptationName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A symbolic reference to an adaptation input.
///
/// This is an opaque reference, not an expression AST.
/// The canonical semantic subsystem resolves its meaning.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdaptationValueRef(String);

impl AdaptationValueRef {
    /// Creates a validated symbolic reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "adaptation value reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Returns the symbolic reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AdaptationValueRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ADAPTATION TARGET
// ============================================================================

/// The semantic category of an adaptive learning target.
///
/// Named targets remain extensible.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AdaptationTarget {
    /// Adapt an existing AI model.
    Model(AiModelId),

    /// Adapt a learning run's future learning behaviour.
    TrainingRun(AiTrainingRunId),

    /// Adapt a named learning strategy.
    Strategy(AdaptationName),

    /// Adapt a named learning policy.
    Policy(AdaptationName),

    /// Adapt a named learning algorithm configuration.
    Algorithm(AdaptationName),

    /// Adapt a named learning state.
    State(AdaptationName),

    /// Extension-defined adaptation target.
    Extension {
        /// Extension-defined target type.
        kind: AdaptationName,

        /// Symbolic target identity.
        identity: AdaptationValueRef,
    },
}

/// Describes the scope of an adaptation target.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AdaptationScope {
    /// Adaptation concerns one logical model.
    Model,

    /// Adaptation concerns a learning run.
    Training,

    /// Adaptation concerns an algorithm.
    Algorithm,

    /// Adaptation concerns a strategy.
    Strategy,

    /// Adaptation concerns a policy.
    Policy,

    /// Adaptation concerns an extension-defined target.
    Extension,
}

impl AdaptationTarget {
    /// Returns the target's semantic scope.
    #[must_use]
    pub const fn scope(&self) -> AdaptationScope {
        match self {
            Self::Model(_) => AdaptationScope::Model,
            Self::TrainingRun(_) => AdaptationScope::Training,
            Self::Strategy(_) => AdaptationScope::Strategy,
            Self::Policy(_) => AdaptationScope::Policy,
            Self::Algorithm(_) => AdaptationScope::Algorithm,
            Self::State(_) => AdaptationScope::Training,
            Self::Extension { .. } => AdaptationScope::Extension,
        }
    }
}

// ============================================================================
// ADAPTATION SOURCE
// ============================================================================

/// The semantic origin of information influencing adaptation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AdaptationSource {
    /// Feedback supplied by an external or internal source.
    Feedback(AdaptationValueRef),

    /// Observation of an environment.
    Observation(AdaptationValueRef),

    /// Result of an inference operation.
    Inference(AdaptationValueRef),

    /// Result of a reasoning operation.
    Reasoning(AdaptationValueRef),

    /// Result of a training operation.
    Training(AiTrainingRunId),

    /// Evidence supplied by the canonical evidence subsystem.
    Evidence(AdaptationValueRef),

    /// Measurement supplied by another computational domain.
    Measurement(AdaptationValueRef),

    /// Result of evaluation.
    Evaluation(AdaptationValueRef),

    /// Extension-defined source.
    Extension {
        /// Source type.
        kind: AdaptationName,

        /// Source identity.
        identity: AdaptationValueRef,
    },
}

// ============================================================================
// ADAPTATION STRATEGY
// ============================================================================

/// A declared family of adaptation behaviour.
///
/// The strategy is an open-world semantic identity rather than
/// a closed enumeration of algorithm implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptationStrategy {
    identity: AdaptationName,
    version: Option<String>,
}

impl AdaptationStrategy {
    /// Creates a strategy reference.
    pub fn new(
        identity: AdaptationName,
    ) -> Self {
        Self {
            identity,
            version: None,
        }
    }

    /// Associates an explicit strategy version.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> AiResult<Self> {
        let version = version.into();

        validate_text(
            "adaptation strategy version",
            &version,
        )?;

        self.version = Some(version);

        Ok(self)
    }

    /// Returns the strategy identity.
    #[must_use]
    pub fn identity(&self) -> &AdaptationName {
        &self.identity
    }

    /// Returns the optional explicit version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

// ============================================================================
// ADAPTATION MODES
// ============================================================================

/// Describes when an adaptation is intended to occur.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AdaptationMode {
    /// Adaptation during an ongoing learning process.
    Online,

    /// Adaptation between learning or execution sessions.
    Offline,

    /// Adaptation after a completed learning stage.
    PostTraining,

    /// Adaptation as new knowledge becomes available.
    Continual,

    /// Adaptation based on transferred knowledge.
    Transfer,

    /// Adaptation under an extension-defined mode.
    Extension,
}

/// Describes the semantic trigger for an adaptation request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AdaptationTrigger {
    /// Explicitly requested adaptation.
    Explicit,

    /// New feedback becomes available.
    FeedbackAvailable,

    /// A new observation becomes available.
    ObservationAvailable,

    /// A learning stage completes.
    TrainingCompleted,

    /// Evaluation identifies a condition requiring adaptation.
    EvaluationResult,

    /// A policy or constraint changes.
    PolicyChange,

    /// An environment changes.
    EnvironmentChange,

    /// An extension-defined trigger.
    Extension,
}

// ============================================================================
// ADAPTATION CONSTRAINTS
// ============================================================================

/// Classification of an adaptation constraint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AdaptationConstraintKind {
    /// A mandatory condition for adaptation.
    Required,

    /// A condition that must never be violated.
    Invariant,

    /// A preferred adaptation characteristic.
    Preference,

    /// An optional implementation hint.
    Hint,

    /// Extension-defined constraint.
    Extension,
}

/// A symbolic adaptation constraint.
///
/// Actual expression interpretation and evaluation belong to
/// canonical semantic analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptationConstraint {
    name: AdaptationName,
    kind: AdaptationConstraintKind,
    condition: AdaptationValueRef,
}

impl AdaptationConstraint {
    /// Creates an adaptation constraint.
    pub fn new(
        name: AdaptationName,
        kind: AdaptationConstraintKind,
        condition: AdaptationValueRef,
    ) -> Self {
        Self {
            name,
            kind,
            condition,
        }
    }

    /// Returns the constraint identity.
    #[must_use]
    pub fn name(&self) -> &AdaptationName {
        &self.name
    }

    /// Returns the constraint category.
    #[must_use]
    pub const fn kind(&self) -> AdaptationConstraintKind {
        self.kind
    }

    /// Returns the symbolic condition.
    #[must_use]
    pub fn condition(&self) -> &AdaptationValueRef {
        &self.condition
    }
}

// ============================================================================
// ADAPTATION CONTEXT
// ============================================================================

/// A named value supplied as adaptation context.
///
/// Examples include objectives, constraints, feedback, evidence,
/// provenance references, and policy references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptationContextEntry {
    name: AdaptationName,
    value: AdaptationValueRef,
}

impl AdaptationContextEntry {
    /// Creates a context entry.
    pub fn new(
        name: AdaptationName,
        value: AdaptationValueRef,
    ) -> Self {
        Self {
            name,
            value,
        }
    }

    /// Returns the context name.
    #[must_use]
    pub fn name(&self) -> &AdaptationName {
        &self.name
    }

    /// Returns the symbolic context value.
    #[must_use]
    pub fn value(&self) -> &AdaptationValueRef {
        &self.value
    }
}

// ============================================================================
// POLICY REFERENCES
// ============================================================================

/// An opaque reference to a policy governing adaptation.
///
/// Policy interpretation and enforcement belong to the shared
/// policy and security infrastructure.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdaptationPolicyRef(AdaptationName);

impl AdaptationPolicyRef {
    /// Creates a policy reference.
    #[must_use]
    pub fn new(identity: AdaptationName) -> Self {
        Self(identity)
    }

    /// Returns the policy identity.
    #[must_use]
    pub fn identity(&self) -> &AdaptationName {
        &self.0
    }
}

// ============================================================================
// PROVENANCE REFERENCES
// ============================================================================

/// References the provenance of an adaptation declaration.
///
/// This is not a competing provenance storage system.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptationProvenance {
    source: Option<AdaptationValueRef>,
    parent_adaptation: Option<AdaptationValueRef>,
}

impl AdaptationProvenance {
    /// Creates a derived adaptation provenance record.
    #[must_use]
    pub fn derived() -> Self {
        Self {
            source: None,
            parent_adaptation: None,
        }
    }

    /// Associates a source provenance reference.
    #[must_use]
    pub fn with_source(
        mut self,
        source: AdaptationValueRef,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Associates a parent adaptation reference.
    #[must_use]
    pub fn with_parent(
        mut self,
        parent: AdaptationValueRef,
    ) -> Self {
        self.parent_adaptation = Some(parent);
        self
    }

    /// Returns the originating provenance reference.
    #[must_use]
    pub fn source(&self) -> Option<&AdaptationValueRef> {
        self.source.as_ref()
    }

    /// Returns the parent adaptation reference.
    #[must_use]
    pub fn parent(&self) -> Option<&AdaptationValueRef> {
        self.parent_adaptation.as_ref()
    }
}

impl Default for AdaptationProvenance {
    fn default() -> Self {
        Self::derived()
    }
}

// ============================================================================
// ADAPTATION LIFECYCLE
// ============================================================================

/// Descriptive lifecycle state of an adaptation request.
///
/// This is a domain contract, not a runtime execution state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AdaptationLifecycle {
    /// Adaptation has been declared.
    Declared,

    /// Structural validation has completed.
    Validated,

    /// Adaptation is ready for downstream planning.
    ReadyForPlanning,

    /// Adaptation has been rejected.
    Rejected,

    /// Adaptation has been superseded.
    Superseded,
}

impl AdaptationLifecycle {
    /// Returns whether this lifecycle state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Rejected | Self::Superseded
        )
    }
}

// ============================================================================
// ADAPTATION DEFINITION
// ============================================================================

/// Complete target-independent adaptive learning declaration.
///
/// The definition describes intent and references.
/// It does not perform model mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptationDefinition {
    identity: AdaptationName,
    target: AdaptationTarget,
    source: Option<AdaptationSource>,
    strategy: AdaptationStrategy,
    mode: AdaptationMode,
    trigger: AdaptationTrigger,
    context: Vec<AdaptationContextEntry>,
    constraints: Vec<AdaptationConstraint>,
    policies: Vec<AdaptationPolicyRef>,
    provenance: AdaptationProvenance,
}

impl AdaptationDefinition {
    /// Creates an adaptation definition.
    pub fn new(
        identity: AdaptationName,
        target: AdaptationTarget,
        strategy: AdaptationStrategy,
    ) -> Self {
        Self {
            identity,
            target,
            source: None,
            strategy,
            mode: AdaptationMode::Offline,
            trigger: AdaptationTrigger::Explicit,
            context: Vec::new(),
            constraints: Vec::new(),
            policies: Vec::new(),
            provenance: AdaptationProvenance::default(),
        }
    }

    /// Sets the adaptation source.
    #[must_use]
    pub fn with_source(
        mut self,
        source: AdaptationSource,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the adaptation mode.
    #[must_use]
    pub fn with_mode(
        mut self,
        mode: AdaptationMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the adaptation trigger.
    #[must_use]
    pub fn with_trigger(
        mut self,
        trigger: AdaptationTrigger,
    ) -> Self {
        self.trigger = trigger;
        self
    }

    /// Adds an adaptation context entry.
    #[must_use]
    pub fn with_context(
        mut self,
        entry: AdaptationContextEntry,
    ) -> Self {
        self.context.push(entry);
        self
    }

    /// Adds an adaptation constraint.
    #[must_use]
    pub fn with_constraint(
        mut self,
        constraint: AdaptationConstraint,
    ) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Adds a governing policy reference.
    #[must_use]
    pub fn with_policy(
        mut self,
        policy: AdaptationPolicyRef,
    ) -> Self {
        self.policies.push(policy);
        self
    }

    /// Sets adaptation provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: AdaptationProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Returns the adaptation identity.
    #[must_use]
    pub fn identity(&self) -> &AdaptationName {
        &self.identity
    }

    /// Returns the adaptation target.
    #[must_use]
    pub fn target(&self) -> &AdaptationTarget {
        &self.target
    }

    /// Returns the optional source.
    #[must_use]
    pub fn source(&self) -> Option<&AdaptationSource> {
        self.source.as_ref()
    }

    /// Returns the strategy.
    #[must_use]
    pub fn strategy(&self) -> &AdaptationStrategy {
        &self.strategy
    }

    /// Returns the adaptation mode.
    #[must_use]
    pub const fn mode(&self) -> AdaptationMode {
        self.mode
    }

    /// Returns the adaptation trigger.
    #[must_use]
    pub const fn trigger(&self) -> AdaptationTrigger {
        self.trigger
    }

    /// Returns adaptation context.
    #[must_use]
    pub fn context(&self) -> &[AdaptationContextEntry] {
        &self.context
    }

    /// Returns adaptation constraints.
    #[must_use]
    pub fn constraints(&self) -> &[AdaptationConstraint] {
        &self.constraints
    }

    /// Returns governing policies.
    #[must_use]
    pub fn policies(&self) -> &[AdaptationPolicyRef] {
        &self.policies
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> &AdaptationProvenance {
        &self.provenance
    }

    /// Validates structural invariants.
    ///
    /// This does not resolve semantic references or authorize
    /// adaptation.
    pub fn validate(&self) -> AiResult<AdaptationValidation> {
        validate_text(
            "adaptation identity",
            self.identity.as_str(),
        )?;

        validate_text(
            "adaptation strategy",
            self.strategy.identity().as_str(),
        )?;

        let mut context_names = BTreeSet::new();

        for entry in &self.context {
            if !context_names.insert(entry.name()) {
                return Err(invalid_adaptation(
                    format!(
                        "duplicate adaptation context name: {}",
                        entry.name()
                    ),
                ));
            }
        }

        let mut constraint_names = BTreeSet::new();

        for constraint in &self.constraints {
            if !constraint_names.insert(constraint.name()) {
                return Err(invalid_adaptation(
                    format!(
                        "duplicate adaptation constraint: {}",
                        constraint.name()
                    ),
                ));
            }
        }

        let mut policy_names = BTreeSet::new();

        for policy in &self.policies {
            if !policy_names.insert(policy.identity()) {
                return Err(invalid_adaptation(
                    format!(
                        "duplicate adaptation policy: {}",
                        policy.identity()
                    ),
                ));
            }
        }

        Ok(AdaptationValidation {
            complete: true,
            context_count: self.context.len(),
            constraint_count: self.constraints.len(),
            policy_count: self.policies.len(),
        })
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of structural adaptation validation.
///
/// Structural completeness must not be confused with semantic
/// validity or execution feasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptationValidation {
    complete: bool,
    context_count: usize,
    constraint_count: usize,
    policy_count: usize,
}

impl AdaptationValidation {
    /// Returns whether structural validation completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns the number of context entries.
    #[must_use]
    pub const fn context_count(&self) -> usize {
        self.context_count
    }

    /// Returns the number of constraints.
    #[must_use]
    pub const fn constraint_count(&self) -> usize {
        self.constraint_count
    }

    /// Returns the number of policies.
    #[must_use]
    pub const fn policy_count(&self) -> usize {
        self.policy_count
    }
}

// ============================================================================
// ADAPTATION COLLECTION
// ============================================================================

/// A collection of adaptation definitions.
///
/// Definitions are indexed by their unique semantic identity.
/// Enumeration is deterministic.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AdaptationSet {
    definitions: std::collections::BTreeMap<
        AdaptationName,
        AdaptationDefinition,
    >,
}

impl AdaptationSet {
    /// Creates an empty adaptation set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an adaptation definition.
    ///
    /// Duplicate identities are rejected without replacing
    /// the existing definition.
    pub fn register(
        &mut self,
        definition: AdaptationDefinition,
    ) -> AiResult<()> {
        definition.validate()?;

        let identity = definition.identity().clone();

        if self.definitions.contains_key(&identity) {
            return Err(AiError::new(
                AiErrorKind::Conflict,
                format!(
                    "adaptation already registered: {identity}"
                ),
            ));
        }

        self.definitions.insert(
            identity,
            definition,
        );

        Ok(())
    }

    /// Looks up an adaptation definition.
    #[must_use]
    pub fn get(
        &self,
        identity: &AdaptationName,
    ) -> Option<&AdaptationDefinition> {
        self.definitions.get(identity)
    }

    /// Returns definitions in deterministic identity order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AdaptationName, &AdaptationDefinition)> {
        self.definitions.iter()
    }

    /// Returns the number of registered definitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> AdaptationName {
        AdaptationName::new(value).unwrap()
    }

    fn value(value: &str) -> AdaptationValueRef {
        AdaptationValueRef::new(value).unwrap()
    }

    fn strategy() -> AdaptationStrategy {
        AdaptationStrategy::new(
            name("zamani.learning.adaptive"),
        )
    }

    fn model() -> AdaptationTarget {
        AdaptationTarget::Model(
            AiModelId::new(1).unwrap(),
        )
    }

    fn definition() -> AdaptationDefinition {
        AdaptationDefinition::new(
            name("experiment.adaptation"),
            model(),
            strategy(),
        )
    }

    #[test]
    fn accepts_open_world_adaptation_names() {
        assert!(name(
            "future.photonic.continual_learning"
        ).as_str().contains("photonic"));
    }

    #[test]
    fn rejects_empty_adaptation_identity() {
        assert!(
            AdaptationName::new("").is_err()
        );
    }

    #[test]
    fn rejects_empty_namespace_components() {
        assert!(
            AdaptationName::new("learning..adaptive").is_err()
        );
    }

    #[test]
    fn accepts_all_adaptation_modes() {
        let modes = [
            AdaptationMode::Online,
            AdaptationMode::Offline,
            AdaptationMode::PostTraining,
            AdaptationMode::Continual,
            AdaptationMode::Transfer,
            AdaptationMode::Extension,
        ];

        assert_eq!(modes.len(), 6);
    }

    #[test]
    fn accepts_feedback_driven_adaptation() {
        let adaptation = definition()
            .with_mode(AdaptationMode::Online)
            .with_trigger(AdaptationTrigger::FeedbackAvailable)
            .with_source(
                AdaptationSource::Feedback(
                    value("learning.feedback"),
                ),
            );

        assert!(adaptation.validate().is_ok());
    }

    #[test]
    fn accepts_training_driven_adaptation() {
        let adaptation = definition()
            .with_source(
                AdaptationSource::Training(
                    AiTrainingRunId::new(2).unwrap(),
                ),
            )
            .with_mode(AdaptationMode::Continual);

        assert!(adaptation.validate().is_ok());
    }

    #[test]
    fn accepts_symbolic_adaptation_context() {
        let adaptation = definition()
            .with_context(
                AdaptationContextEntry::new(
                    name("objective"),
                    value("training.objective"),
                ),
            );

        assert_eq!(
            adaptation.validate().unwrap().context_count(),
            1
        );
    }

    #[test]
    fn rejects_duplicate_context_names() {
        let adaptation = definition()
            .with_context(
                AdaptationContextEntry::new(
                    name("objective"),
                    value("objective.first"),
                ),
            )
            .with_context(
                AdaptationContextEntry::new(
                    name("objective"),
                    value("objective.second"),
                ),
            );

        assert!(adaptation.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_constraints() {
        let constraint = || {
            AdaptationConstraint::new(
                name("preserve_invariant"),
                AdaptationConstraintKind::Invariant,
                value("model.invariant"),
            )
        };

        let adaptation = definition()
            .with_constraint(constraint())
            .with_constraint(constraint());

        assert!(adaptation.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_policies() {
        let policy = || {
            AdaptationPolicyRef::new(
                name("security.adaptation_policy"),
            )
        };

        let adaptation = definition()
            .with_policy(policy())
            .with_policy(policy());

        assert!(adaptation.validate().is_err());
    }

    #[test]
    fn accepts_versioned_adaptation_strategy() {
        let strategy = strategy()
            .with_version("1.0.0")
            .unwrap();

        assert_eq!(
            strategy.version(),
            Some("1.0.0")
        );
    }

    #[test]
    fn preserves_adaptation_provenance() {
        let provenance = AdaptationProvenance::derived()
            .with_source(value("training.result"))
            .with_parent(value("adaptation.previous"));

        assert_eq!(
            provenance.source().unwrap().as_str(),
            "training.result"
        );

        assert_eq!(
            provenance.parent().unwrap().as_str(),
            "adaptation.previous"
        );
    }

    #[test]
    fn collection_rejects_duplicate_definitions() {
        let mut set = AdaptationSet::new();

        set.register(definition()).unwrap();

        assert!(
            set.register(definition()).is_err()
        );

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn collection_enumeration_is_deterministic() {
        let mut set = AdaptationSet::new();

        let second = AdaptationDefinition::new(
            name("z.adaptation"),
            model(),
            strategy(),
        );

        let first = AdaptationDefinition::new(
            name("a.adaptation"),
            model(),
            strategy(),
        );

        set.register(second).unwrap();
        set.register(first).unwrap();

        let names: Vec<_> = set
            .iter()
            .map(|(identity, _)| identity.as_str())
            .collect();

        assert_eq!(
            names,
            vec!["a.adaptation", "z.adaptation"]
        );
    }

    #[test]
    fn lifecycle_terminal_states_are_explicit() {
        assert!(
            AdaptationLifecycle::Rejected.is_terminal()
        );

        assert!(
            AdaptationLifecycle::Superseded.is_terminal()
        );

        assert!(
            !AdaptationLifecycle::Declared.is_terminal()
        );
    }

    #[test]
    fn supports_large_context_collections() {
        let mut adaptation = definition();

        for index in 0..10_000 {
            adaptation = adaptation.with_context(
                AdaptationContextEntry::new(
                    name(&format!("context.{index}")),
                    value(&format!("feedback.{index}")),
                ),
            );
        }

        let report = adaptation.validate().unwrap();

        assert_eq!(report.context_count(), 10_000);
    }

    #[test]
    fn supports_extension_defined_targets() {
        let target = AdaptationTarget::Extension {
            kind: name("future.neuromorphic.target"),
            identity: value("neuromorphic.state"),
        };

        assert_eq!(
            target.scope(),
            AdaptationScope::Extension
        );
    }
}
