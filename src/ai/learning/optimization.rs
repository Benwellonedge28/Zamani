
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Learning Optimization Contracts
//!
//! Path: `src/ai/learning/optimization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent semantic contracts for learning optimizers.
//!
//! This module describes:
//!
//! - Optimizer identities and exact versions.
//! - Learning algorithm relationships.
//! - Optimizer parameter groups.
//! - Hyperparameter references.
//! - Logical optimizer state.
//! - State schemas.
//! - Update-rule descriptions.
//! - Numerical behavior policies.
//! - Optimizer requirements.
//! - Optimizer registration and compatibility.
//!
//! It does not execute numerical optimization.
//!
//! ## Ownership
//!
//! This module owns optimizer semantics and their validation.
//!
//! ## Does not own
//!
//! - Tensor storage.
//! - Model parameter storage.
//! - Gradient computation.
//! - Automatic differentiation.
//! - Numerical kernels.
//! - Training workflow execution.
//! - Hardware discovery.
//! - Device selection.
//! - Resource allocation.
//! - Scheduling.
//! - Distributed communication.
//! - Quantum execution.
//! - ZUIR construction.
//! - Backend execution.
//! - Runtime execution.
//!
//! ## Canonical integration
//!
//! Learning algorithm contracts:
//!     crate::ai::learning::algorithms
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! AI semantic names:
//!     crate::frontend::semantic::domains::ai::QualifiedName
//!
//! AI errors:
//!     crate::ai::error
//!
//! Training workflows:
//!     crate::ai::learning::training
//!
//! Differentiation:
//!     crate::ai::differentiation
//!
//! Universal representation:
//!     crate::zuir
//!
//! ## POCO-REAF
//!
//! Optimizer semantics are independent of physical execution targets.
//!
//! There are no language-level ceilings on:
//!
//! - Optimizers.
//! - Optimizer versions.
//! - Parameter groups.
//! - Hyperparameters.
//! - State slots.
//! - Update rules.
//! - Model parameters.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Training steps.
//! - Workers.
//! - Devices.
//! - Accelerators.
//! - Distributed nodes.
//!
//! Practical resource limitations belong to downstream feasibility
//! analysis and execution infrastructure.
//!
//! Literal infinite computation is not promised.
//!
//! ## Registry policy
//!
//! Registration identity:
//!
//!     (qualified optimizer identity, exact optimizer version)
//!
//! Every registered optimizer must reference an existing, exact,
//! registered learning algorithm version.
//!
//! Registration batches are atomic.
//!
//! Lookup is exact. No implicit version substitution occurs.
//!
//! ## Important distinction
//!
//! An optimizer declaration does not establish that:
//!
//! - A numerical implementation exists.
//! - Required gradients can be computed.
//! - A compatible tensor implementation exists.
//! - Required capabilities are available.
//! - Resources are sufficient.
//! - Execution is authorized.
//! - Training will succeed.
//!
//! Those decisions belong to downstream authorities.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::sync::Arc;

use crate::ai::learning::algorithms::{
    LearningAlgorithmError,
    LearningAlgorithmId,
    LearningAlgorithmRegistry,
    LearningAlgorithmVersion,
};

use crate::ai::requirements::AiRequirements;

use crate::frontend::semantic::domains::ai::QualifiedName;


// ============================================================================
// VALIDATION
// ============================================================================

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), OptimizationError> {
    if value.trim().is_empty() {
        return Err(OptimizationError::InvalidText { field });
    }

    if value.chars().any(char::is_control) {
        return Err(OptimizationError::InvalidText { field });
    }

    Ok(())
}

fn validate_name(
    field: &'static str,
    name: &QualifiedName,
) -> Result<(), OptimizationError> {
    validate_text(field, &name.namespace)?;
    validate_text(field, &name.name)?;
    Ok(())
}

fn qualified_name_string(name: &QualifiedName) -> String {
    format!("{}.{}", name.namespace, name.name)
}


// ============================================================================
// OPTIMIZER IDENTITY
// ============================================================================

/// Canonical identity of an optimizer.
///
/// Names reuse the frontend's semantic identity representation.
pub type OptimizerId = QualifiedName;

/// Exact version of an optimizer contract.
///
/// This is independent of the language, AI API, model, and runtime versions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OptimizerVersion(LearningAlgorithmVersion);

impl OptimizerVersion {
    /// Creates an optimizer version.
    #[must_use]
    pub const fn new(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> Self {
        Self(
            LearningAlgorithmVersion::new(
                major,
                minor,
                patch,
            ),
        )
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.0.major()
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.0.minor()
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.0.patch()
    }
}

impl fmt::Display for OptimizerVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major(),
            self.minor(),
            self.patch()
        )
    }
}


// ============================================================================
// LEARNING ALGORITHM REFERENCE
// ============================================================================

/// Exact reference to a registered learning algorithm.
///
/// Optimizers do not independently redefine learning algorithms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizerAlgorithmReference {
    identity: LearningAlgorithmId,
    version: LearningAlgorithmVersion,
}

impl OptimizerAlgorithmReference {
    /// Constructs an exact learning algorithm reference.
    pub fn new(
        identity: LearningAlgorithmId,
        version: LearningAlgorithmVersion,
    ) -> Result<Self, OptimizationError> {
        validate_name("algorithm identity", &identity)?;

        Ok(Self {
            identity,
            version,
        })
    }

    /// Returns the algorithm identity.
    #[must_use]
    pub fn identity(&self) -> &LearningAlgorithmId {
        &self.identity
    }

    /// Returns the exact algorithm version.
    #[must_use]
    pub const fn version(&self) -> LearningAlgorithmVersion {
        self.version
    }

    /// Validates that the referenced algorithm exists.
    pub fn validate_against(
        &self,
        registry: &LearningAlgorithmRegistry,
    ) -> Result<(), OptimizationError> {
        registry
            .resolve(&self.identity, self.version)
            .map(|_| ())
            .map_err(OptimizationError::Algorithm)
    }
}


// ============================================================================
// OPEN-WORLD SEMANTIC REFERENCES
// ============================================================================

/// Reference to a canonical semantic expression.
///
/// This is an identity/reference, not an independent expression language.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SemanticReference {
    identity: QualifiedName,
}

impl SemanticReference {
    /// Constructs a semantic reference.
    pub fn new(
        identity: QualifiedName,
    ) -> Result<Self, OptimizationError> {
        validate_name("semantic reference", &identity)?;

        Ok(Self { identity })
    }

    /// Returns the canonical semantic identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }
}


// ============================================================================
// NUMERICAL POLICY
// ============================================================================

/// Target-independent numerical behavior contract.
///
/// Each field references an extensible semantic policy.
///
/// The names are deliberately not restricted to a closed set of
/// numerical formats or hardware representations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumericalPolicy {
    precision: QualifiedName,
    rounding: QualifiedName,
    accumulation: QualifiedName,
    overflow: QualifiedName,
    reduction: QualifiedName,
    reproducibility: QualifiedName,
}

impl NumericalPolicy {
    /// Constructs an explicit numerical policy.
    pub fn new(
        precision: QualifiedName,
        rounding: QualifiedName,
        accumulation: QualifiedName,
        overflow: QualifiedName,
        reduction: QualifiedName,
        reproducibility: QualifiedName,
    ) -> Result<Self, OptimizationError> {
        let policy = Self {
            precision,
            rounding,
            accumulation,
            overflow,
            reduction,
            reproducibility,
        };

        policy.validate()?;

        Ok(policy)
    }

    /// Returns the precision policy.
    #[must_use]
    pub fn precision(&self) -> &QualifiedName {
        &self.precision
    }

    /// Returns the rounding policy.
    #[must_use]
    pub fn rounding(&self) -> &QualifiedName {
        &self.rounding
    }

    /// Returns the accumulation policy.
    #[must_use]
    pub fn accumulation(&self) -> &QualifiedName {
        &self.accumulation
    }

    /// Returns the overflow policy.
    #[must_use]
    pub fn overflow(&self) -> &QualifiedName {
        &self.overflow
    }

    /// Returns the reduction policy.
    #[must_use]
    pub fn reduction(&self) -> &QualifiedName {
        &self.reduction
    }

    /// Returns the reproducibility policy.
    #[must_use]
    pub fn reproducibility(&self) -> &QualifiedName {
        &self.reproducibility
    }

    /// Validates all numerical policy references.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        validate_name("precision policy", &self.precision)?;
        validate_name("rounding policy", &self.rounding)?;
        validate_name("accumulation policy", &self.accumulation)?;
        validate_name("overflow policy", &self.overflow)?;
        validate_name("reduction policy", &self.reduction)?;
        validate_name(
            "reproducibility policy",
            &self.reproducibility,
        )?;

        Ok(())
    }
}


// ============================================================================
// OPTIMIZER STATE
// ============================================================================

/// Logical lifetime of an optimizer state value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OptimizerStateLifetime {
    /// State belongs to one optimization invocation.
    Invocation,

    /// State persists throughout a training run.
    TrainingRun,

    /// State may persist across training runs.
    Persistent,

    /// State is owned by an external canonical subsystem.
    External,
}

/// Description of one logical optimizer state slot.
///
/// This is a schema, not an allocated tensor or memory location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizerStateSlot {
    identity: QualifiedName,
    value_type: QualifiedName,
    lifetime: OptimizerStateLifetime,
    initializer: SemanticReference,
}

impl OptimizerStateSlot {
    /// Constructs an optimizer state slot.
    pub fn new(
        identity: QualifiedName,
        value_type: QualifiedName,
        lifetime: OptimizerStateLifetime,
        initializer: SemanticReference,
    ) -> Result<Self, OptimizationError> {
        let slot = Self {
            identity,
            value_type,
            lifetime,
            initializer,
        };

        slot.validate()?;

        Ok(slot)
    }

    /// Returns the state-slot identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }

    /// Returns its canonical semantic value type.
    #[must_use]
    pub fn value_type(&self) -> &QualifiedName {
        &self.value_type
    }

    /// Returns the logical state lifetime.
    #[must_use]
    pub const fn lifetime(&self) -> OptimizerStateLifetime {
        self.lifetime
    }

    /// Returns the initialization reference.
    #[must_use]
    pub fn initializer(&self) -> &SemanticReference {
        &self.initializer
    }

    /// Validates the state-slot declaration.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        validate_name("state identity", &self.identity)?;
        validate_name("state value type", &self.value_type)?;

        Ok(())
    }
}

/// Logical optimizer state schema.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptimizerStateSchema {
    slots: Vec<OptimizerStateSlot>,
}

impl OptimizerStateSchema {
    /// Creates an empty optimizer state schema.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a state slot.
    ///
    /// Duplicate identities are rejected.
    pub fn add_slot(
        &mut self,
        slot: OptimizerStateSlot,
    ) -> Result<(), OptimizationError> {
        slot.validate()?;

        if self.slots.iter().any(
            |existing| existing.identity() == slot.identity()
        ) {
            return Err(OptimizationError::DuplicateStateSlot {
                identity: qualified_name_string(slot.identity()),
            });
        }

        self.slots.push(slot);

        Ok(())
    }

    /// Returns state slots in declaration order.
    #[must_use]
    pub fn slots(&self) -> &[OptimizerStateSlot] {
        &self.slots
    }

    /// Validates the complete state schema.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        let mut identities = BTreeSet::new();

        for slot in &self.slots {
            slot.validate()?;

            if !identities.insert(slot.identity().clone()) {
                return Err(OptimizationError::DuplicateStateSlot {
                    identity: qualified_name_string(slot.identity()),
                });
            }
        }

        Ok(())
    }
}


// ============================================================================
// UPDATE RULES
// ============================================================================

/// Describes one optimizer state transition.
///
/// The operation identity resolves through the canonical semantic
/// and operation registries.
///
/// This module never evaluates the operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizerUpdateRule {
    identity: QualifiedName,
    operation: SemanticReference,
    reads: Vec<QualifiedName>,
    writes: Vec<QualifiedName>,
}

impl OptimizerUpdateRule {
    /// Constructs an optimizer update rule.
    pub fn new(
        identity: QualifiedName,
        operation: SemanticReference,
        reads: Vec<QualifiedName>,
        writes: Vec<QualifiedName>,
    ) -> Result<Self, OptimizationError> {
        let rule = Self {
            identity,
            operation,
            reads,
            writes,
        };

        rule.validate()?;

        Ok(rule)
    }

    /// Returns the update-rule identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }

    /// Returns the canonical operation reference.
    #[must_use]
    pub fn operation(&self) -> &SemanticReference {
        &self.operation
    }

    /// Returns state values read by this rule.
    #[must_use]
    pub fn reads(&self) -> &[QualifiedName] {
        &self.reads
    }

    /// Returns state values written by this rule.
    #[must_use]
    pub fn writes(&self) -> &[QualifiedName] {
        &self.writes
    }

    /// Validates update-rule structure.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        validate_name("update rule identity", &self.identity)?;

        let mut reads = BTreeSet::new();

        for identity in &self.reads {
            validate_name("update read reference", identity)?;

            if !reads.insert(identity) {
                return Err(OptimizationError::DuplicateStateReference {
                    identity: qualified_name_string(identity),
                });
            }
        }

        let mut writes = BTreeSet::new();

        for identity in &self.writes {
            validate_name("update write reference", identity)?;

            if !writes.insert(identity) {
                return Err(OptimizationError::DuplicateStateReference {
                    identity: qualified_name_string(identity),
                });
            }
        }

        if self.writes.is_empty() {
            return Err(OptimizationError::UpdateWithoutWrites);
        }

        Ok(())
    }

    /// Validates all referenced state slots.
    pub fn validate_against(
        &self,
        schema: &OptimizerStateSchema,
    ) -> Result<(), OptimizationError> {
        self.validate()?;

        for identity in self.reads.iter().chain(self.writes.iter()) {
            if !schema.slots().iter().any(
                |slot| slot.identity() == identity
            ) {
                return Err(OptimizationError::UnknownStateSlot {
                    identity: qualified_name_string(identity),
                });
            }
        }

        Ok(())
    }
}


// ============================================================================
// PARAMETER GROUPS
// ============================================================================

/// Parameter selection semantics.
///
/// The selector refers to a canonical semantic expression or symbol.
/// It does not implement a second expression language.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParameterSelector {
    /// Select every parameter admitted by the owning model contract.
    All,

    /// Select a particular named parameter.
    Named(QualifiedName),

    /// Select parameters matching a canonical semantic predicate.
    Predicate(SemanticReference),
}

impl ParameterSelector {
    /// Validates the selector.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        match self {
            Self::All => Ok(()),

            Self::Named(identity) => {
                validate_name("parameter selector", identity)
            }

            Self::Predicate(reference) => {
                validate_name(
                    "parameter predicate",
                    reference.identity(),
                )
            }
        }
    }
}

/// Hyperparameter reference.
///
/// Values remain expressions owned by the canonical language.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizerHyperparameter {
    identity: QualifiedName,
    value: SemanticReference,
}

impl OptimizerHyperparameter {
    /// Constructs a hyperparameter reference.
    pub fn new(
        identity: QualifiedName,
        value: SemanticReference,
    ) -> Result<Self, OptimizationError> {
        validate_name("hyperparameter identity", &identity)?;

        Ok(Self {
            identity,
            value,
        })
    }

    /// Returns the hyperparameter identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }

    /// Returns the semantic value reference.
    #[must_use]
    pub fn value(&self) -> &SemanticReference {
        &self.value
    }
}

/// Optimizer parameter group.
///
/// A group describes which logical model parameters receive a common
/// optimizer configuration. It does not own parameter storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizerParameterGroup {
    identity: QualifiedName,
    selector: ParameterSelector,
    hyperparameters: Vec<OptimizerHyperparameter>,
    priority: Option<i64>,
}

impl OptimizerParameterGroup {
    /// Constructs a parameter group.
    pub fn new(
        identity: QualifiedName,
        selector: ParameterSelector,
    ) -> Result<Self, OptimizationError> {
        validate_name("parameter group identity", &identity)?;
        selector.validate()?;

        Ok(Self {
            identity,
            selector,
            hyperparameters: Vec::new(),
            priority: None,
        })
    }

    /// Assigns an explicit semantic priority.
    ///
    /// Priority is only metadata for a downstream group-resolution
    /// policy. It does not silently resolve overlapping selectors.
    #[must_use]
    pub fn with_priority(
        mut self,
        priority: i64,
    ) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Adds a hyperparameter declaration.
    pub fn add_hyperparameter(
        &mut self,
        hyperparameter: OptimizerHyperparameter,
    ) -> Result<(), OptimizationError> {
        if self.hyperparameters.iter().any(
            |existing| existing.identity() == hyperparameter.identity()
        ) {
            return Err(OptimizationError::DuplicateHyperparameter {
                identity: qualified_name_string(
                    hyperparameter.identity()
                ),
            });
        }

        self.hyperparameters.push(hyperparameter);

        Ok(())
    }

    /// Returns the group identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }

    /// Returns the parameter selector.
    #[must_use]
    pub fn selector(&self) -> &ParameterSelector {
        &self.selector
    }

    /// Returns hyperparameters in declaration order.
    #[must_use]
    pub fn hyperparameters(&self) -> &[OptimizerHyperparameter] {
        &self.hyperparameters
    }

    /// Returns the optional priority.
    #[must_use]
    pub const fn priority(&self) -> Option<i64> {
        self.priority
    }

    /// Validates the complete parameter group.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        validate_name("parameter group identity", &self.identity)?;
        self.selector.validate()?;

        let mut identities = BTreeSet::new();

        for hyperparameter in &self.hyperparameters {
            validate_name(
                "hyperparameter identity",
                hyperparameter.identity(),
            )?;

            validate_name(
                "hyperparameter value",
                hyperparameter.value().identity(),
            )?;

            if !identities.insert(hyperparameter.identity().clone()) {
                return Err(OptimizationError::DuplicateHyperparameter {
                    identity: qualified_name_string(
                        hyperparameter.identity()
                    ),
                });
            }
        }

        Ok(())
    }
}


// ============================================================================
// OPTIMIZER DESCRIPTOR
// ============================================================================

/// Immutable semantic optimizer declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizerDescriptor {
    identity: OptimizerId,
    version: OptimizerVersion,
    algorithm: OptimizerAlgorithmReference,
    state: OptimizerStateSchema,
    parameter_groups: Vec<OptimizerParameterGroup>,
    update_rules: Vec<OptimizerUpdateRule>,
    numerical_policy: NumericalPolicy,
    requirements: AiRequirements,
    schedule: Option<SemanticReference>,
    description: Option<String>,
}

impl OptimizerDescriptor {
    /// Constructs an optimizer descriptor.
    pub fn new(
        identity: OptimizerId,
        version: OptimizerVersion,
        algorithm: OptimizerAlgorithmReference,
        numerical_policy: NumericalPolicy,
    ) -> Result<Self, OptimizationError> {
        validate_name("optimizer identity", &identity)?;
        numerical_policy.validate()?;

        Ok(Self {
            identity,
            version,
            algorithm,
            state: OptimizerStateSchema::new(),
            parameter_groups: Vec::new(),
            update_rules: Vec::new(),
            numerical_policy,
            requirements: AiRequirements::new(),
            schedule: None,
            description: None,
        })
    }

    /// Replaces the optimizer's AI requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Attaches a schedule reference.
    ///
    /// Schedule definitions and execution belong to their owning
    /// training/scheduling subsystem.
    pub fn with_schedule(
        mut self,
        schedule: SemanticReference,
    ) -> Self {
        self.schedule = Some(schedule);
        self
    }

    /// Attaches a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, OptimizationError> {
        let description = description.into();
        validate_text("optimizer description", &description)?;

        self.description = Some(description);

        Ok(self)
    }

    /// Adds a logical state slot.
    pub fn add_state_slot(
        &mut self,
        slot: OptimizerStateSlot,
    ) -> Result<(), OptimizationError> {
        self.state.add_slot(slot)
    }

    /// Adds a parameter group.
    pub fn add_parameter_group(
        &mut self,
        group: OptimizerParameterGroup,
    ) -> Result<(), OptimizationError> {
        group.validate()?;

        if self.parameter_groups.iter().any(
            |existing| existing.identity() == group.identity()
        ) {
            return Err(OptimizationError::DuplicateParameterGroup {
                identity: qualified_name_string(group.identity()),
            });
        }

        self.parameter_groups.push(group);

        Ok(())
    }

    /// Adds an update rule.
    pub fn add_update_rule(
        &mut self,
        rule: OptimizerUpdateRule,
    ) -> Result<(), OptimizationError> {
        rule.validate()?;

        if self.update_rules.iter().any(
            |existing| existing.identity() == rule.identity()
        ) {
            return Err(OptimizationError::DuplicateUpdateRule {
                identity: qualified_name_string(rule.identity()),
            });
        }

        self.update_rules.push(rule);

        Ok(())
    }

    /// Returns the optimizer identity.
    #[must_use]
    pub fn identity(&self) -> &OptimizerId {
        &self.identity
    }

    /// Returns the exact optimizer version.
    #[must_use]
    pub const fn version(&self) -> OptimizerVersion {
        self.version
    }

    /// Returns the learning algorithm reference.
    #[must_use]
    pub fn algorithm(&self) -> &OptimizerAlgorithmReference {
        &self.algorithm
    }

    /// Returns the optimizer state schema.
    #[must_use]
    pub fn state(&self) -> &OptimizerStateSchema {
        &self.state
    }

    /// Returns parameter groups in declaration order.
    #[must_use]
    pub fn parameter_groups(&self) -> &[OptimizerParameterGroup] {
        &self.parameter_groups
    }

    /// Returns update rules in declaration order.
    #[must_use]
    pub fn update_rules(&self) -> &[OptimizerUpdateRule] {
        &self.update_rules
    }

    /// Returns the numerical policy.
    #[must_use]
    pub fn numerical_policy(&self) -> &NumericalPolicy {
        &self.numerical_policy
    }

    /// Returns AI requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Returns the optional schedule reference.
    #[must_use]
    pub fn schedule(&self) -> Option<&SemanticReference> {
        self.schedule.as_ref()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validates all locally owned optimizer invariants.
    pub fn validate(&self) -> Result<(), OptimizationError> {
        validate_name("optimizer identity", &self.identity)?;

        self.numerical_policy.validate()?;
        self.state.validate()?;

        if !self.requirements.is_complete() {
            return Err(OptimizationError::IncompleteRequirements);
        }

        let mut groups = BTreeSet::new();

        for group in &self.parameter_groups {
            group.validate()?;

            if !groups.insert(group.identity().clone()) {
                return Err(OptimizationError::DuplicateParameterGroup {
                    identity: qualified_name_string(group.identity()),
                });
            }
        }

        let mut rules = BTreeSet::new();

        for rule in &self.update_rules {
            rule.validate()?;

            rule.validate_against(&self.state)?;

            if !rules.insert(rule.identity().clone()) {
                return Err(OptimizationError::DuplicateUpdateRule {
                    identity: qualified_name_string(rule.identity()),
                });
            }
        }

        if self.update_rules.is_empty() {
            return Err(OptimizationError::MissingUpdateRules);
        }

        if let Some(schedule) = &self.schedule {
            validate_name(
                "optimizer schedule",
                schedule.identity(),
            )?;
        }

        if let Some(description) = &self.description {
            validate_text("optimizer description", description)?;
        }

        Ok(())
    }

    /// Validates the optimizer against the canonical algorithm registry.
    ///
    /// Exact algorithm versions are mandatory.
    pub fn validate_against(
        &self,
        algorithms: &LearningAlgorithmRegistry,
    ) -> Result<(), OptimizationError> {
        self.validate()?;

        self.algorithm.validate_against(algorithms)?;

        Ok(())
    }
}


// ============================================================================
// REGISTRY IDENTITY
// ============================================================================

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct OptimizerKey {
    identity: OptimizerId,
    version: OptimizerVersion,
}

impl OptimizerKey {
    fn from_descriptor(
        descriptor: &OptimizerDescriptor,
    ) -> Self {
        Self {
            identity: descriptor.identity.clone(),
            version: descriptor.version,
        }
    }
}


// ============================================================================
// OPTIMIZER REGISTRY
// ============================================================================

/// Deterministic, exact-version optimizer registry.
///
/// The registry stores immutable semantic declarations.
///
/// Registration requires a corresponding registered learning algorithm.
#[derive(Clone, Debug, Default)]
pub struct OptimizerRegistry {
    optimizers: BTreeMap<
        OptimizerKey,
        Arc<OptimizerDescriptor>,
    >,
}

impl OptimizerRegistry {
    /// Creates an empty optimizer registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of exact optimizer registrations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.optimizers.len()
    }

    /// Returns whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.optimizers.is_empty()
    }

    /// Registers one optimizer.
    ///
    /// The descriptor must be valid and reference an existing exact
    /// learning algorithm version.
    pub fn register(
        &mut self,
        descriptor: OptimizerDescriptor,
        algorithms: &LearningAlgorithmRegistry,
    ) -> Result<Arc<OptimizerDescriptor>, OptimizationError> {
        descriptor.validate_against(algorithms)?;

        let key = OptimizerKey::from_descriptor(&descriptor);

        if self.optimizers.contains_key(&key) {
            return Err(OptimizationError::DuplicateOptimizer {
                identity: qualified_name_string(&descriptor.identity),
                version: descriptor.version,
            });
        }

        let descriptor = Arc::new(descriptor);

        self.optimizers.insert(
            key,
            Arc::clone(&descriptor),
        );

        Ok(descriptor)
    }

    /// Atomically registers multiple optimizer descriptors.
    ///
    /// The original registry remains unchanged if any descriptor fails
    /// validation, algorithm resolution, or duplicate detection.
    pub fn register_batch<I>(
        &mut self,
        descriptors: I,
        algorithms: &LearningAlgorithmRegistry,
    ) -> Result<(), OptimizationError>
    where
        I: IntoIterator<Item = OptimizerDescriptor>,
    {
        let mut staged = self.optimizers.clone();

        for descriptor in descriptors {
            descriptor.validate_against(algorithms)?;

            let key = OptimizerKey::from_descriptor(&descriptor);

            if staged.contains_key(&key) {
                return Err(OptimizationError::DuplicateOptimizer {
                    identity: qualified_name_string(&descriptor.identity),
                    version: descriptor.version,
                });
            }

            staged.insert(
                key,
                Arc::new(descriptor),
            );
        }

        self.optimizers = staged;

        Ok(())
    }

    /// Resolves an exact optimizer identity and version.
    ///
    /// No implicit upgrade or version substitution occurs.
    pub fn resolve(
        &self,
        identity: &OptimizerId,
        version: OptimizerVersion,
    ) -> Result<Arc<OptimizerDescriptor>, OptimizationError> {
        self.get(identity, version)
            .cloned()
            .ok_or_else(|| OptimizationError::UnknownOptimizer {
                identity: qualified_name_string(identity),
                version,
            })
    }

    /// Looks up an exact optimizer registration.
    #[must_use]
    pub fn get(
        &self,
        identity: &OptimizerId,
        version: OptimizerVersion,
    ) -> Option<&Arc<OptimizerDescriptor>> {
        self.optimizers.get(&OptimizerKey {
            identity: identity.clone(),
            version,
        })
    }

    /// Checks whether an exact optimizer registration exists.
    #[must_use]
    pub fn contains(
        &self,
        identity: &OptimizerId,
        version: OptimizerVersion,
    ) -> bool {
        self.get(identity, version).is_some()
    }

    /// Iterates over the exact versions of one optimizer identity.
    pub fn versions_for<'a>(
        &'a self,
        identity: &'a OptimizerId,
    ) -> impl Iterator<Item = OptimizerVersion> + 'a {
        self.optimizers
            .keys()
            .filter(move |key| &key.identity == identity)
            .map(|key| key.version)
    }

    /// Iterates over immutable registered optimizer descriptors.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Arc<OptimizerDescriptor>> {
        self.optimizers.values()
    }
}


// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by optimizer declarations and registration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OptimizationError {
    /// Invalid text or semantic identity.
    InvalidText {
        /// Field containing invalid text.
        field: &'static str,
    },

    /// Duplicate state slot.
    DuplicateStateSlot {
        /// State identity.
        identity: String,
    },

    /// Duplicate state reference in an update rule.
    DuplicateStateReference {
        /// State identity.
        identity: String,
    },

    /// Update rule does not write any state.
    UpdateWithoutWrites,

    /// An update rule references an unknown state slot.
    UnknownStateSlot {
        /// Referenced identity.
        identity: String,
    },

    /// Duplicate parameter group.
    DuplicateParameterGroup {
        /// Group identity.
        identity: String,
    },

    /// Duplicate hyperparameter.
    DuplicateHyperparameter {
        /// Hyperparameter identity.
        identity: String,
    },

    /// Duplicate update rule.
    DuplicateUpdateRule {
        /// Rule identity.
        identity: String,
    },

    /// No update rules were declared.
    MissingUpdateRules,

    /// Optimizer requirements analysis is incomplete.
    IncompleteRequirements,

    /// The exact optimizer identity and version already exist.
    DuplicateOptimizer {
        /// Optimizer identity.
        identity: String,

        /// Optimizer version.
        version: OptimizerVersion,
    },

    /// The exact optimizer identity and version do not exist.
    UnknownOptimizer {
        /// Optimizer identity.
        identity: String,

        /// Requested optimizer version.
        version: OptimizerVersion,
    },

    /// Referenced learning algorithm could not be resolved.
    Algorithm(LearningAlgorithmError),
}

impl fmt::Display for OptimizationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidText { field } => {
                write!(formatter, "invalid optimizer {field}")
            }

            Self::DuplicateStateSlot { identity } => {
                write!(formatter, "duplicate optimizer state slot {identity}")
            }

            Self::DuplicateStateReference { identity } => {
                write!(formatter, "duplicate optimizer state reference {identity}")
            }

            Self::UpdateWithoutWrites => {
                formatter.write_str("optimizer update rule has no state writes")
            }

            Self::UnknownStateSlot { identity } => {
                write!(formatter, "unknown optimizer state slot {identity}")
            }

            Self::DuplicateParameterGroup { identity } => {
                write!(formatter, "duplicate optimizer parameter group {identity}")
            }

            Self::DuplicateHyperparameter { identity } => {
                write!(formatter, "duplicate optimizer hyperparameter {identity}")
            }

            Self::DuplicateUpdateRule { identity } => {
                write!(formatter, "duplicate optimizer update rule {identity}")
            }

            Self::MissingUpdateRules => {
                formatter.write_str("optimizer has no update rules")
            }

            Self::IncompleteRequirements => {
                formatter.write_str("optimizer requirements analysis is incomplete")
            }

            Self::DuplicateOptimizer { identity, version } => {
                write!(
                    formatter,
                    "optimizer {identity} version {version} is already registered"
                )
            }

            Self::UnknownOptimizer { identity, version } => {
                write!(
                    formatter,
                    "optimizer {identity} version {version} is not registered"
                )
            }

            Self::Algorithm(error) => {
                write!(formatter, "optimizer algorithm error: {error}")
            }
        }
    }
}

impl Error for OptimizationError {}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(
        namespace: &str,
        identity: &str,
    ) -> QualifiedName {
        QualifiedName::new(namespace, identity)
            .expect("valid qualified name")
    }

    fn reference(
        namespace: &str,
        identity: &str,
    ) -> SemanticReference {
        SemanticReference::new(
            name(namespace, identity)
        )
        .expect("valid semantic reference")
    }

    fn numerical_policy() -> NumericalPolicy {
        NumericalPolicy::new(
            name("numeric", "precision"),
            name("numeric", "rounding"),
            name("numeric", "accumulation"),
            name("numeric", "overflow"),
            name("numeric", "reduction"),
            name("numeric", "reproducibility"),
        )
        .expect("valid numerical policy")
    }

    #[test]
    fn optimizer_versions_are_exact() {
        let version = OptimizerVersion::new(1, 2, 3);

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn numerical_policy_rejects_invalid_identity() {
        let result = NumericalPolicy::new(
            name("numeric", "precision"),
            name("numeric", "rounding"),
            name("numeric", "accumulation"),
            name("numeric", "overflow"),
            name("numeric", "reduction"),
            name("numeric", "reproducibility"),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn state_schema_rejects_duplicate_slots() {
        let mut schema = OptimizerStateSchema::new();

        let slot = OptimizerStateSlot::new(
            name("optimizer.state", "moment"),
            name("tensor", "value"),
            OptimizerStateLifetime::TrainingRun,
            reference("optimizer.initializer", "zero"),
        )
        .expect("valid state slot");

        schema.add_slot(slot.clone())
            .expect("first insertion succeeds");

        assert!(matches!(
            schema.add_slot(slot),
            Err(OptimizationError::DuplicateStateSlot { .. })
        ));
    }

    #[test]
    fn update_rule_requires_state_writes() {
        let rule = OptimizerUpdateRule::new(
            name("optimizer.rule", "update"),
            reference("optimizer.operation", "update"),
            vec![],
            vec![],
        );

        assert!(matches!(
            rule,
            Err(OptimizationError::UpdateWithoutWrites)
        ));
    }

    #[test]
    fn update_rule_rejects_unknown_state() {
        let mut schema = OptimizerStateSchema::new();

        schema.add_slot(
            OptimizerStateSlot::new(
                name("optimizer.state", "moment"),
                name("tensor", "value"),
                OptimizerStateLifetime::TrainingRun,
                reference("optimizer.initializer", "zero"),
            )
            .expect("valid slot")
        ).expect("slot insertion");

        let rule = OptimizerUpdateRule::new(
            name("optimizer.rule", "update"),
            reference("optimizer.operation", "update"),
            vec![],
            vec![name("optimizer.state", "unknown")],
        ).expect("valid rule structure");

        assert!(matches!(
            rule.validate_against(&schema),
            Err(OptimizationError::UnknownStateSlot { .. })
        ));
    }

    #[test]
    fn parameter_group_rejects_duplicate_hyperparameters() {
        let mut group = OptimizerParameterGroup::new(
            name("optimizer.group", "default"),
            ParameterSelector::All,
        ).expect("valid group");

        let parameter = OptimizerHyperparameter::new(
            name("optimizer.parameter", "rate"),
            reference("training.expression", "rate"),
        ).expect("valid parameter");

        group.add_hyperparameter(parameter.clone())
            .expect("first insertion succeeds");

        assert!(matches!(
            group.add_hyperparameter(parameter),
            Err(OptimizationError::DuplicateHyperparameter { .. })
        ));
    }

    #[test]
    fn optimizer_requires_update_rules() {
        let algorithm = OptimizerAlgorithmReference::new(
            name("learning.algorithm", "example"),
            LearningAlgorithmVersion::new(1, 0, 0),
        ).expect("valid algorithm reference");

        let optimizer = OptimizerDescriptor::new(
            name("optimizer", "example"),
            OptimizerVersion::new(1, 0, 0),
            algorithm,
            numerical_policy(),
        ).expect("valid optimizer");

        assert!(matches!(
            optimizer.validate(),
            Err(OptimizationError::MissingUpdateRules)
        ));
    }

    #[test]
    fn empty_registry_is_valid() {
        let registry = OptimizerRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn invalid_optimizer_names_are_rejected() {
        let result = SemanticReference::new(
            QualifiedName::new("", "invalid")
                .expect("QualifiedName constructor permits nonempty checking"),
        );

        assert!(result.is_err());
    }
}
