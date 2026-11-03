
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Learning Objectives and Loss Contracts
//!
//! Path: `src/ai/learning/objectives.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent contracts for objectives, objective terms,
//! objective composition, optimization constraints, and loss functions.
//!
//! This module describes what a learning or optimization process is intended
//! to achieve. It does not evaluate objectives or execute loss functions.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Learning objective definitions.
//! - Objective terms and their directions.
//! - Multi-objective composition declarations.
//! - Objective constraint declarations.
//! - Loss function declarations.
//! - Symbolic loss input references.
//! - Objective and loss contract validation.
//! - Conversion to the existing training binding interface.
//!
//! ## Does not own
//!
//! - Source syntax or AST construction.
//! - Name resolution.
//! - Canonical semantic type checking.
//! - Mathematical expression evaluation.
//! - Tensor storage.
//! - Tensor operations.
//! - Gradient computation.
//! - Automatic differentiation.
//! - Optimizer algorithms or optimizer state.
//! - Numerical kernels.
//! - Training workflow execution.
//! - Model execution.
//! - Resource allocation or feasibility.
//! - Hardware discovery or selection.
//! - Scheduling, placement, or distributed communication.
//! - ZUIR operation definitions or lowering.
//! - Runtime or backend execution.
//!
//! ## Canonical dependencies
//!
//! - `crate::ai::error::{AiError, AiErrorKind, AiResult}`
//! - `crate::ai::requirements::AiRequirements`
//! - `crate::ai::learning::training::{TrainingBinding, TrainingBindingKind}`
//! - `crate::ai::learning::training::TrainingValueRef`
//! - `crate::zuir::version::VersionTriple`
//!
//! ## Grammar authority
//!
//! Source-level training objective and loss syntax belongs to:
//!
//! - `grammar/ai/training.g4`
//! - `grammar/spec/ai.md`
//! - `grammar/ai/README.md`
//!
//! The grammar uses annotations and canonical expressions. This module
//! receives references produced by the canonical frontend and semantic
//! adapter. It does not parse source text or create an AI-specific AST.
//!
//! ## Integration
//!
//! - `training.rs` consumes objective and loss definitions through the
//!   existing `TrainingBinding` contract.
//! - `optimization.rs` consumes objective direction, composition and
//!   constraint declarations when describing optimizer semantics.
//! - `gradients.rs` and `differentiation/` provide gradient contracts.
//! - `model/` provides model and parameter references.
//! - `tensor/` provides canonical tensor and shape semantics.
//! - `requirements.rs` provides AI capability and resource requirements.
//! - Frontend semantic analysis resolves source expressions and validates
//!   their types before constructing these contracts.
//! - ZUIR receives canonical operations through a separate lowering adapter.
//! - Runtime and backend layers perform actual evaluation.
//!
//! ## POCO-REAF and scalability
//!
//! Objective and loss declarations are independent of execution targets.
//!
//! This module imposes no language-level maximum on:
//!
//! - objective definitions;
//! - objective terms;
//! - constraints;
//! - loss definitions;
//! - metadata entries;
//! - model parameters;
//! - tensor dimensions or rank;
//! - training steps;
//! - workers or devices.
//!
//! Collections grow according to available implementation resources.
//! Operational budgets and physical feasibility remain downstream concerns.
//!
//! Literal infinite computation is not promised.
//!
//! ## Semantic invariants
//!
//! - An objective is not an executable optimizer.
//! - A loss declaration is not a computed loss value.
//! - A symbolic reference is not proof that the referenced expression exists.
//! - A valid local contract is not proof of semantic type compatibility.
//! - A valid objective is not proof that a feasible solution exists.
//! - A target-specific limitation must not change source-level objective
//!   meaning.
//! - An incomplete requirement analysis must never be silently promoted
//!   to complete.
//! - Objective ordering is preserved where it is semantically significant.
//! - Weighted composition requires an explicit weight for every term.
//! - Custom composition semantics remain open-world.
//!
//! ## Freeze criteria
//!
//! - Existing training references and binding types are reused.
//! - Objective and loss identities are versioned.
//! - Objective terms and constraints are validated deterministically.
//! - Multi-objective composition is extensible.
//! - Loss semantics are expressed through symbolic references.
//! - AI requirements are preserved without inventing completeness.
//! - No numerical execution or tensor storage is implemented here.
//! - No arbitrary workload ceiling or unsafe Rust is introduced.

use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{AiError, AiErrorKind, AiResult};
use crate::ai::learning::training::{
    TrainingBinding,
    TrainingBindingKind,
    TrainingValueRef,
};
use crate::ai::requirements::AiRequirements;
use crate::zuir::version::VersionTriple;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
    subject: &'static str,
) -> AiError {
    AiError::new(AiErrorKind::InvalidInput, message)
        .with_detail("subsystem", "ai.learning.objectives")
        .with_detail("subject", subject)
}

fn validate_text(
    value: &str,
    subject: &'static str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{subject} must not be empty"),
            subject,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{subject} contains a control character"),
            subject,
        ));
    }

    Ok(())
}

fn validate_reference(
    reference: &TrainingValueRef,
    subject: &'static str,
) -> AiResult<()> {
    validate_text(reference.as_str(), subject)
}

fn validate_metadata(
    metadata: &BTreeMap<String, TrainingValueRef>,
) -> AiResult<()> {
    for (key, value) in metadata {
        validate_text(key, "metadata key")?;
        validate_reference(value, "metadata reference")?;
    }

    Ok(())
}

// ============================================================================
// OBJECTIVE DIRECTION
// ============================================================================

/// Desired direction or satisfaction semantics for an objective term.
///
/// Direction is semantic intent. It does not prescribe a particular
/// optimization algorithm or numerical implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ObjectiveDirection {
    /// Minimize the referenced objective value.
    Minimize,

    /// Maximize the referenced objective value.
    Maximize,

    /// Seek a declared symbolic target value.
    ///
    /// The target remains a reference to a canonical Zamani expression.
    Target(TrainingValueRef),

    /// Satisfy the objective's declared predicate or semantic condition.
    ///
    /// The meaning of satisfaction is established by the referenced
    /// expression and canonical semantic analysis.
    Satisfy,
}

impl ObjectiveDirection {
    /// Validate any symbolic values carried by this direction.
    pub fn validate(&self) -> AiResult<()> {
        if let Self::Target(target) = self {
            validate_reference(target, "objective target")?;
        }

        Ok(())
    }
}

// ============================================================================
// OBJECTIVE TERM
// ============================================================================

/// One independently identifiable term in an objective definition.
///
/// The expression is a symbolic reference to a canonical Zamani value.
/// This module does not evaluate that expression.
///
/// A term may represent a scalar, structured value, probabilistic value,
/// tensor-derived value, quantum-derived value, or an extension-defined
/// objective quantity, subject to semantic validation elsewhere.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveTerm {
    identity: String,
    expression: TrainingValueRef,
    direction: ObjectiveDirection,
    weight: Option<TrainingValueRef>,
    description: Option<String>,
}

impl ObjectiveTerm {
    /// Construct an objective term.
    ///
    /// `identity` is local to its containing objective definition.
    /// `expression` refers to the canonical expression representing
    /// the objective value.
    pub fn new(
        identity: impl Into<String>,
        expression: TrainingValueRef,
        direction: ObjectiveDirection,
    ) -> AiResult<Self> {
        let identity = identity.into();
        validate_text(&identity, "objective term identity")?;
        validate_reference(&expression, "objective expression")?;
        direction.validate()?;

        Ok(Self {
            identity,
            expression,
            direction,
            weight: None,
            description: None,
        })
    }

    /// Attach a symbolic weight expression.
    ///
    /// The referenced expression is not evaluated here. Its type,
    /// numerical meaning, and compatibility are resolved by semantic
    /// analysis and the appropriate numerical subsystem.
    pub fn with_weight(
        mut self,
        weight: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(&weight, "objective term weight")?;
        self.weight = Some(weight);
        Ok(self)
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AiResult<Self> {
        let description = description.into();
        validate_text(&description, "objective term description")?;
        self.description = Some(description);
        Ok(self)
    }

    /// Return the term's local identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Return the canonical expression reference.
    #[must_use]
    pub fn expression(&self) -> &TrainingValueRef {
        &self.expression
    }

    /// Return the desired direction.
    #[must_use]
    pub fn direction(&self) -> &ObjectiveDirection {
        &self.direction
    }

    /// Return the optional symbolic weight.
    #[must_use]
    pub fn weight(&self) -> Option<&TrainingValueRef> {
        self.weight.as_ref()
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate the local term contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(&self.identity, "objective term identity")?;
        validate_reference(&self.expression, "objective expression")?;
        self.direction.validate()?;

        if let Some(weight) = &self.weight {
            validate_reference(weight, "objective term weight")?;
        }

        if let Some(description) = &self.description {
            validate_text(description, "objective term description")?;
        }

        Ok(())
    }
}

// ============================================================================
// OBJECTIVE COMPOSITION
// ============================================================================

/// Composition semantics for multiple objective terms.
///
/// The variants describe common composition relationships, not a closed
/// catalogue of optimization algorithms.
///
/// An extension can supply its own composition semantics through `Custom`.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ObjectiveComposition {
    /// Exactly one objective term.
    Single,

    /// Combine terms using their explicitly declared symbolic weights.
    Weighted,

    /// Treat terms as distinct objectives in a Pareto-style relationship.
    ///
    /// The actual dominance and frontier semantics are resolved by the
    /// selected compatible optimization implementation.
    Pareto,

    /// Preserve the declared term order as lexicographic priority.
    Lexicographic,

    /// Delegate composition semantics to an extension-defined reference.
    Custom(TrainingValueRef),
}

impl ObjectiveComposition {
    /// Validate extension-defined composition references.
    pub fn validate(&self) -> AiResult<()> {
        if let Self::Custom(reference) = self {
            validate_reference(reference, "objective composition")?;
        }

        Ok(())
    }
}

// ============================================================================
// OBJECTIVE CONSTRAINTS
// ============================================================================

/// Classification of an objective-related constraint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ObjectiveConstraintKind {
    /// A mandatory condition that an admissible solution must satisfy.
    Hard,

    /// A condition that may be traded against other objective terms
    /// according to the declared optimization semantics.
    Soft,
}

/// A constraint associated with an objective definition.
///
/// `predicate` references a canonical Zamani expression. A hard constraint
/// is not merely a preference. A soft constraint does not silently become
/// mandatory.
///
/// The expression's meaning and type are checked by canonical semantic
/// analysis. This structure does not evaluate the predicate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveConstraint {
    identity: String,
    predicate: TrainingValueRef,
    kind: ObjectiveConstraintKind,
    penalty: Option<TrainingValueRef>,
    description: Option<String>,
}

impl ObjectiveConstraint {
    /// Construct an objective constraint.
    pub fn new(
        identity: impl Into<String>,
        predicate: TrainingValueRef,
        kind: ObjectiveConstraintKind,
    ) -> AiResult<Self> {
        let identity = identity.into();
        validate_text(&identity, "objective constraint identity")?;
        validate_reference(&predicate, "objective constraint predicate")?;

        Ok(Self {
            identity,
            predicate,
            kind,
            penalty: None,
            description: None,
        })
    }

    /// Attach an optional symbolic penalty expression.
    ///
    /// A penalty is meaningful only according to the selected constraint
    /// and optimization semantics. This method does not calculate it.
    pub fn with_penalty(
        mut self,
        penalty: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(&penalty, "objective constraint penalty")?;
        self.penalty = Some(penalty);
        Ok(self)
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AiResult<Self> {
        let description = description.into();
        validate_text(&description, "objective constraint description")?;
        self.description = Some(description);
        Ok(self)
    }

    /// Return the local constraint identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Return the predicate expression reference.
    #[must_use]
    pub fn predicate(&self) -> &TrainingValueRef {
        &self.predicate
    }

    /// Return the constraint classification.
    #[must_use]
    pub const fn kind(&self) -> ObjectiveConstraintKind {
        self.kind
    }

    /// Return the optional penalty expression.
    #[must_use]
    pub fn penalty(&self) -> Option<&TrainingValueRef> {
        self.penalty.as_ref()
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate the local constraint contract.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            &self.identity,
            "objective constraint identity",
        )?;

        validate_reference(
            &self.predicate,
            "objective constraint predicate",
        )?;

        if let Some(penalty) = &self.penalty {
            validate_reference(
                penalty,
                "objective constraint penalty",
            )?;
        }

        if let Some(description) = &self.description {
            validate_text(
                description,
                "objective constraint description",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// OBJECTIVE DEFINITION
// ============================================================================

/// A versioned, target-independent learning objective definition.
///
/// The definition may contain one or multiple objective terms and optional
/// constraints. It preserves source-declared ordering.
///
/// The objective's referenced expressions are resolved and type-checked
/// by the canonical frontend semantic subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveDefinition {
    identity: TrainingValueRef,
    version: VersionTriple,
    composition: ObjectiveComposition,
    terms: Vec<ObjectiveTerm>,
    constraints: Vec<ObjectiveConstraint>,
    requirements: AiRequirements,
    metadata: BTreeMap<String, TrainingValueRef>,
    description: Option<String>,
}

impl ObjectiveDefinition {
    /// Construct an objective definition.
    ///
    /// The definition starts with no terms and must receive at least one
    /// term before it is considered valid.
    pub fn new(
        identity: TrainingValueRef,
        version: VersionTriple,
        composition: ObjectiveComposition,
    ) -> AiResult<Self> {
        validate_reference(&identity, "objective identity")?;
        composition.validate()?;

        Ok(Self {
            identity,
            version,
            composition,
            terms: Vec::new(),
            constraints: Vec::new(),
            requirements: AiRequirements::new(),
            metadata: BTreeMap::new(),
            description: None,
        })
    }

    /// Append an objective term.
    ///
    /// Term ordering is preserved. For lexicographic composition this
    /// ordering is semantically significant.
    #[must_use]
    pub fn with_term(
        mut self,
        term: ObjectiveTerm,
    ) -> Self {
        self.terms.push(term);
        self
    }

    /// Append an objective constraint.
    #[must_use]
    pub fn with_constraint(
        mut self,
        constraint: ObjectiveConstraint,
    ) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Replace the AI requirement envelope.
    ///
    /// The supplied requirement analysis status is preserved. Incomplete
    /// analysis is not silently promoted to complete.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Add or replace an extension metadata reference.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        let key = key.into();
        validate_text(&key, "objective metadata key")?;
        validate_reference(&value, "objective metadata reference")?;

        self.metadata.insert(key, value);
        Ok(self)
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AiResult<Self> {
        let description = description.into();
        validate_text(&description, "objective description")?;
        self.description = Some(description);
        Ok(self)
    }

    /// Return the objective identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }

    /// Return the objective contract version.
    #[must_use]
    pub const fn version(&self) -> VersionTriple {
        self.version
    }

    /// Return the composition semantics.
    #[must_use]
    pub fn composition(&self) -> &ObjectiveComposition {
        &self.composition
    }

    /// Return objective terms in declaration order.
    #[must_use]
    pub fn terms(&self) -> &[ObjectiveTerm] {
        &self.terms
    }

    /// Return objective constraints in declaration order.
    #[must_use]
    pub fn constraints(&self) -> &[ObjectiveConstraint] {
        &self.constraints
    }

    /// Return the AI requirement envelope.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Return extension metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, TrainingValueRef> {
        &self.metadata
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate the complete local objective contract.
    ///
    /// This validates structure and local invariants only. It does not
    /// resolve symbols, evaluate expressions, establish numerical
    /// compatibility, or prove optimization feasibility.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(&self.identity, "objective identity")?;
        self.composition.validate()?;

        if self.terms.is_empty() {
            return Err(invalid_input(
                "an objective must declare at least one term",
                "objective terms",
            ));
        }

        let mut term_identities = BTreeSet::new();

        for term in &self.terms {
            term.validate()?;

            if !term_identities.insert(term.identity()) {
                return Err(invalid_input(
                    format!(
                        "duplicate objective term identity: {}",
                        term.identity()
                    ),
                    "objective terms",
                ));
            }
        }

        match &self.composition {
            ObjectiveComposition::Single => {
                if self.terms.len() != 1 {
                    return Err(invalid_input(
                        "single composition requires exactly one objective term",
                        "objective composition",
                    ));
                }
            }

            ObjectiveComposition::Weighted => {
                for term in &self.terms {
                    if term.weight().is_none() {
                        return Err(invalid_input(
                            format!(
                                "weighted composition requires a weight for term {}",
                                term.identity()
                            ),
                            "objective weights",
                        ));
                    }
                }
            }

            ObjectiveComposition::Pareto
            | ObjectiveComposition::Lexicographic
            | ObjectiveComposition::Custom(_) => {}
        }

        let mut constraint_identities = BTreeSet::new();

        for constraint in &self.constraints {
            constraint.validate()?;

            if !constraint_identities.insert(constraint.identity()) {
                return Err(invalid_input(
                    format!(
                        "duplicate objective constraint identity: {}",
                        constraint.identity()
                    ),
                    "objective constraints",
                ));
            }
        }

        validate_metadata(&self.metadata)?;

        if let Some(description) = &self.description {
            validate_text(description, "objective description")?;
        }

        // Requirement completeness is deliberately not asserted here.
        // The requirement envelope retains its own analysis status and
        // is inspected by the semantic/execution boundary.
        Ok(())
    }

    /// Convert this definition into the existing training objective binding.
    ///
    /// This does not register or resolve the objective. It creates the
    /// symbolic binding consumed by `training.rs`.
    pub fn to_training_binding(&self) -> AiResult<TrainingBinding> {
        self.validate()?;

        TrainingBinding::new(
            TrainingBindingKind::Objective,
            self.identity.as_str(),
            self.identity.clone(),
        )
    }
}

// ============================================================================
// LOSS REDUCTION
// ============================================================================

/// A symbolic loss-reduction contract.
///
/// Reduction semantics are deliberately open-world. The reference may
/// identify a canonical expression or an extension-defined reduction
/// operation. This module does not assume that every loss is reduced by
/// a fixed `sum`, `mean`, or other closed list of operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LossReduction {
    operation: TrainingValueRef,
}

impl LossReduction {
    /// Construct a symbolic loss-reduction declaration.
    pub fn new(operation: TrainingValueRef) -> AiResult<Self> {
        validate_reference(&operation, "loss reduction operation")?;
        Ok(Self { operation })
    }

    /// Return the reduction operation reference.
    #[must_use]
    pub fn operation(&self) -> &TrainingValueRef {
        &self.operation
    }

    /// Validate the reduction declaration.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(
            &self.operation,
            "loss reduction operation",
        )
    }
}

// ============================================================================
// LOSS DEFINITION
// ============================================================================

/// A versioned symbolic loss-function definition.
///
/// A loss definition references the operation and the values it consumes.
/// It does not execute the operation or assume a particular numerical
/// representation.
///
/// Predictions, targets, masks, sample weights, and reduction semantics
/// remain symbolic references to canonical Zamani values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LossDefinition {
    identity: TrainingValueRef,
    version: VersionTriple,
    operation: TrainingValueRef,
    predictions: TrainingValueRef,
    targets: TrainingValueRef,
    sample_weights: Option<TrainingValueRef>,
    mask: Option<TrainingValueRef>,
    reduction: Option<LossReduction>,
    requirements: AiRequirements,
    metadata: BTreeMap<String, TrainingValueRef>,
    description: Option<String>,
}

impl LossDefinition {
    /// Construct a loss definition.
    ///
    /// `operation` identifies the canonical loss computation.
    /// `predictions` and `targets` identify the values supplied to it.
    pub fn new(
        identity: TrainingValueRef,
        version: VersionTriple,
        operation: TrainingValueRef,
        predictions: TrainingValueRef,
        targets: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(&identity, "loss identity")?;
        validate_reference(&operation, "loss operation")?;
        validate_reference(&predictions, "loss predictions")?;
        validate_reference(&targets, "loss targets")?;

        Ok(Self {
            identity,
            version,
            operation,
            predictions,
            targets,
            sample_weights: None,
            mask: None,
            reduction: None,
            requirements: AiRequirements::new(),
            metadata: BTreeMap::new(),
            description: None,
        })
    }

    /// Attach a symbolic sample-weight reference.
    pub fn with_sample_weights(
        mut self,
        sample_weights: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            &sample_weights,
            "loss sample weights",
        )?;

        self.sample_weights = Some(sample_weights);
        Ok(self)
    }

    /// Attach a symbolic mask reference.
    pub fn with_mask(
        mut self,
        mask: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(&mask, "loss mask")?;
        self.mask = Some(mask);
        Ok(self)
    }

    /// Attach an explicit reduction contract.
    pub fn with_reduction(
        mut self,
        reduction: LossReduction,
    ) -> AiResult<Self> {
        reduction.validate()?;
        self.reduction = Some(reduction);
        Ok(self)
    }

    /// Replace the AI requirement envelope.
    ///
    /// The supplied analysis status is preserved.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Add or replace extension metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        let key = key.into();
        validate_text(&key, "loss metadata key")?;
        validate_reference(&value, "loss metadata reference")?;

        self.metadata.insert(key, value);
        Ok(self)
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AiResult<Self> {
        let description = description.into();
        validate_text(&description, "loss description")?;
        self.description = Some(description);
        Ok(self)
    }

    /// Return the loss identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }

    /// Return the loss contract version.
    #[must_use]
    pub const fn version(&self) -> VersionTriple {
        self.version
    }

    /// Return the loss operation reference.
    #[must_use]
    pub fn operation(&self) -> &TrainingValueRef {
        &self.operation
    }

    /// Return the prediction reference.
    #[must_use]
    pub fn predictions(&self) -> &TrainingValueRef {
        &self.predictions
    }

    /// Return the target reference.
    #[must_use]
    pub fn targets(&self) -> &TrainingValueRef {
        &self.targets
    }

    /// Return the optional sample-weight reference.
    #[must_use]
    pub fn sample_weights(&self) -> Option<&TrainingValueRef> {
        self.sample_weights.as_ref()
    }

    /// Return the optional mask reference.
    #[must_use]
    pub fn mask(&self) -> Option<&TrainingValueRef> {
        self.mask.as_ref()
    }

    /// Return the optional reduction contract.
    #[must_use]
    pub fn reduction(&self) -> Option<&LossReduction> {
        self.reduction.as_ref()
    }

    /// Return the AI requirement envelope.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Return extension metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, TrainingValueRef> {
        &self.metadata
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate the complete local loss contract.
    ///
    /// This does not resolve the operation, compare tensor shapes,
    /// evaluate the loss, or prove differentiability. Those checks
    /// belong to canonical semantic analysis, tensor validation,
    /// and differentiation.
    pub fn validate(&self) -> AiResult<()> {
        validate_reference(&self.identity, "loss identity")?;
        validate_reference(&self.operation, "loss operation")?;
        validate_reference(&self.predictions, "loss predictions")?;
        validate_reference(&self.targets, "loss targets")?;

        if let Some(weights) = &self.sample_weights {
            validate_reference(weights, "loss sample weights")?;
        }

        if let Some(mask) = &self.mask {
            validate_reference(mask, "loss mask")?;
        }

        if let Some(reduction) = &self.reduction {
            reduction.validate()?;
        }

        validate_metadata(&self.metadata)?;

        if let Some(description) = &self.description {
            validate_text(description, "loss description")?;
        }

        Ok(())
    }

    /// Convert this definition into the existing training loss binding.
    ///
    /// This creates a symbolic training binding. It does not register,
    /// evaluate, or execute the loss.
    pub fn to_training_binding(&self) -> AiResult<TrainingBinding> {
        self.validate()?;

        TrainingBinding::new(
            TrainingBindingKind::Loss,
            self.identity.as_str(),
            self.identity.clone(),
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(value: &str) -> TrainingValueRef {
        TrainingValueRef::new(value).expect("valid test reference")
    }

    fn version() -> VersionTriple {
        VersionTriple::new(1, 0, 0)
    }

    fn term(
        identity: &str,
        expression: &str,
        direction: ObjectiveDirection,
    ) -> ObjectiveTerm {
        ObjectiveTerm::new(
            identity,
            reference(expression),
            direction,
        )
        .expect("valid test objective term")
    }

    #[test]
    fn validates_single_objective() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "accuracy",
            "evaluation.accuracy",
            ObjectiveDirection::Maximize,
        ));

        assert!(objective.validate().is_ok());
    }

    #[test]
    fn rejects_objective_without_terms() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective");

        assert!(objective.validate().is_err());
    }

    #[test]
    fn rejects_multiple_terms_in_single_composition() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "accuracy",
            "evaluation.accuracy",
            ObjectiveDirection::Maximize,
        ))
        .with_term(term(
            "latency",
            "evaluation.latency",
            ObjectiveDirection::Minimize,
        ));

        assert!(objective.validate().is_err());
    }

    #[test]
    fn supports_weighted_multi_objective_contracts() {
        let accuracy = term(
            "accuracy",
            "evaluation.accuracy",
            ObjectiveDirection::Maximize,
        )
        .with_weight(reference("weights.accuracy"))
        .expect("valid weight");

        let latency = term(
            "latency",
            "evaluation.latency",
            ObjectiveDirection::Minimize,
        )
        .with_weight(reference("weights.latency"))
        .expect("valid weight");

        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Weighted,
        )
        .expect("valid objective")
        .with_term(accuracy)
        .with_term(latency);

        assert!(objective.validate().is_ok());
        assert_eq!(objective.terms().len(), 2);
    }

    #[test]
    fn weighted_composition_requires_every_term_weight() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Weighted,
        )
        .expect("valid objective")
        .with_term(term(
            "accuracy",
            "evaluation.accuracy",
            ObjectiveDirection::Maximize,
        ));

        assert!(objective.validate().is_err());
    }

    #[test]
    fn preserves_lexicographic_term_order() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Lexicographic,
        )
        .expect("valid objective")
        .with_term(term(
            "primary",
            "evaluation.safety",
            ObjectiveDirection::Maximize,
        ))
        .with_term(term(
            "secondary",
            "evaluation.performance",
            ObjectiveDirection::Maximize,
        ));

        assert!(objective.validate().is_ok());
        assert_eq!(objective.terms()[0].identity(), "primary");
        assert_eq!(objective.terms()[1].identity(), "secondary");
    }

    #[test]
    fn rejects_duplicate_term_identities() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Pareto,
        )
        .expect("valid objective")
        .with_term(term(
            "quality",
            "evaluation.accuracy",
            ObjectiveDirection::Maximize,
        ))
        .with_term(term(
            "quality",
            "evaluation.latency",
            ObjectiveDirection::Minimize,
        ));

        assert!(objective.validate().is_err());
    }

    #[test]
    fn supports_symbolic_target_direction() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "target",
            "evaluation.error",
            ObjectiveDirection::Target(reference("target.error")),
        ));

        assert!(objective.validate().is_ok());
    }

    #[test]
    fn validates_hard_and_soft_constraints() {
        let hard = ObjectiveConstraint::new(
            "safety",
            reference("constraints.safe"),
            ObjectiveConstraintKind::Hard,
        )
        .expect("valid hard constraint");

        let soft = ObjectiveConstraint::new(
            "latency",
            reference("constraints.latency"),
            ObjectiveConstraintKind::Soft,
        )
        .expect("valid soft constraint")
        .with_penalty(reference("penalties.latency"))
        .expect("valid penalty");

        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "quality",
            "evaluation.quality",
            ObjectiveDirection::Maximize,
        ))
        .with_constraint(hard)
        .with_constraint(soft);

        assert!(objective.validate().is_ok());
        assert_eq!(objective.constraints().len(), 2);
    }

    #[test]
    fn rejects_duplicate_constraint_identities() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "quality",
            "evaluation.quality",
            ObjectiveDirection::Maximize,
        ))
        .with_constraint(
            ObjectiveConstraint::new(
                "constraint",
                reference("constraints.first"),
                ObjectiveConstraintKind::Hard,
            )
            .expect("valid constraint"),
        )
        .with_constraint(
            ObjectiveConstraint::new(
                "constraint",
                reference("constraints.second"),
                ObjectiveConstraintKind::Soft,
            )
            .expect("valid constraint"),
        );

        assert!(objective.validate().is_err());
    }

    #[test]
    fn converts_objective_to_existing_training_binding() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "quality",
            "evaluation.quality",
            ObjectiveDirection::Maximize,
        ));

        let binding = objective
            .to_training_binding()
            .expect("valid training binding");

        assert_eq!(binding.kind(), TrainingBindingKind::Objective);
        assert_eq!(binding.name(), "training.objective");
        assert_eq!(binding.value().as_str(), "training.objective");
    }

    #[test]
    fn validates_symbolic_loss_definition() {
        let loss = LossDefinition::new(
            reference("training.loss"),
            version(),
            reference("operations.loss"),
            reference("model.predictions"),
            reference("dataset.targets"),
        )
        .expect("valid loss")
        .with_sample_weights(reference("dataset.weights"))
        .expect("valid weights")
        .with_mask(reference("dataset.mask"))
        .expect("valid mask")
        .with_reduction(
            LossReduction::new(reference("operations.reduction"))
                .expect("valid reduction"),
        )
        .expect("valid reduction attachment");

        assert!(loss.validate().is_ok());
    }

    #[test]
    fn converts_loss_to_existing_training_binding() {
        let loss = LossDefinition::new(
            reference("training.loss"),
            version(),
            reference("operations.loss"),
            reference("model.predictions"),
            reference("dataset.targets"),
        )
        .expect("valid loss");

        let binding = loss
            .to_training_binding()
            .expect("valid training binding");

        assert_eq!(binding.kind(), TrainingBindingKind::Loss);
        assert_eq!(binding.name(), "training.loss");
        assert_eq!(binding.value().as_str(), "training.loss");
    }

    #[test]
    fn supports_extension_defined_composition() {
        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Custom(reference(
                "extensions.custom_composition",
            )),
        )
        .expect("valid objective")
        .with_term(term(
            "quality",
            "evaluation.quality",
            ObjectiveDirection::Maximize,
        ));

        assert!(objective.validate().is_ok());
    }

    #[test]
    fn preserves_incomplete_requirement_analysis() {
        let mut requirements = AiRequirements::new();
        requirements
            .mark_incomplete("capability analysis pending")
            .expect("valid incomplete reason");

        let objective = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_term(term(
            "quality",
            "evaluation.quality",
            ObjectiveDirection::Maximize,
        ))
        .with_requirements(requirements);

        assert!(objective.validate().is_ok());
        assert!(!objective.requirements().is_complete());
    }

    #[test]
    fn rejects_empty_symbolic_references() {
        assert!(TrainingValueRef::new("  ").is_err());
    }

    #[test]
    fn rejects_control_characters_in_descriptions() {
        let result = ObjectiveDefinition::new(
            reference("training.objective"),
            version(),
            ObjectiveComposition::Single,
        )
        .expect("valid objective")
        .with_description("invalid\nobjective");

        assert!(result.is_err());
    }
}
