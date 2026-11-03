
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI — Gradient Computation Contracts
//!
//! Path: src/ai/learning/gradients.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent gradient computation contracts for Zamani.
//!
//! This module describes WHAT gradient computation is requested,
//! its mathematical inputs and outputs, and its semantic requirements.
//!
//! It does not calculate gradients.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Gradient computation requests.
//! - Gradient contract identities.
//! - Differentiation targets.
//! - Differentiation variable references.
//! - Symbolic derivative orders.
//! - Gradient result classifications.
//! - Differentiation strategy requirements and preferences.
//! - Gradient output bindings.
//! - Gradient contract validation.
//! - Training interoperability.
//! - Gradient-specific metadata.
//!
//! ## Does not own
//!
//! - Source syntax.
//! - AST construction.
//! - Semantic name resolution.
//! - Mathematical expression evaluation.
//! - Differentiability analysis.
//! - Automatic differentiation.
//! - Forward-mode differentiation implementation.
//! - Reverse-mode differentiation implementation.
//! - Symbolic differentiation implementation.
//! - Numerical differentiation.
//! - Parameter-shift implementation.
//! - Tensor storage.
//! - Tensor numerical kernels.
//! - Optimizer algorithms.
//! - Physical memory allocation.
//! - Hardware discovery.
//! - Device selection.
//! - Scheduling.
//! - Distributed execution.
//! - ZUIR construction.
//! - Backend execution.
//!
//! ## Canonical dependencies
//!
//! AI errors:
//!     crate::ai::error
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! Canonical semantic names:
//!     crate::frontend::semantic::domains::ai::QualifiedName
//!
//! Canonical version:
//!     crate::zuir::version::VersionTriple
//!
//! Training references:
//!     crate::ai::learning::training::TrainingValueRef
//!
//! Training interoperability:
//!     crate::ai::learning::training::TrainingOperation
//!
//! Differentiability syntax:
//!     grammar/ai/differentiable.g4
//!
//! Differentiation syntax:
//!     grammar/ai/differentiation.g4
//!
//! Training syntax:
//!     grammar/ai/training.g4
//!
//! ## Integration
//!
//! Objectives:
//!     Supplies objective and loss expression references.
//!
//! Training:
//!     Supplies gradient computation operation references.
//!
//! Differentiation:
//!     Consumes gradient requests and establishes differentiability,
//!     mathematical legality and differentiation representations.
//!
//! Tensor:
//!     Supplies canonical symbolic tensor shape and type information.
//!
//! Model:
//!     Supplies model and parameter references.
//!
//! Requirements:
//!     Supplies AI capability and resource requirements.
//!
//! ZUIR:
//!     Receives canonical operations through the appropriate lowering
//!     and interoperability infrastructure.
//!
//! Runtime:
//!     Executes the lowered computation.
//!
//! ## POCO-REAF
//!
//! No universal maximum is imposed on:
//!
//! - Gradient contracts.
//! - Differentiation variables.
//! - Gradient outputs.
//! - Derivative order.
//! - Model parameters.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Training operations.
//! - Strategy declarations.
//! - Metadata entries.
//!
//! Symbolic references are preserved rather than prematurely evaluated.
//!
//! Practical allocation and execution limitations remain explicit
//! downstream concerns.
//!
//! Literal infinite computation is not promised.
//!
//! ## Important semantic distinction
//!
//! A valid gradient contract does not prove that:
//!
//! - The target is mathematically differentiable.
//! - The differentiated variables exist.
//! - The requested derivative is mathematically defined.
//! - A differentiation implementation exists.
//! - The requested strategy is available.
//! - Required capabilities exist.
//! - Sufficient resources exist.
//! - Execution will succeed.
//!
//! These decisions belong to their authoritative subsystems.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::requirements::AiRequirements;

use crate::ai::learning::training::{
    TrainingOperation,
    TrainingValueRef,
};

use crate::frontend::semantic::domains::ai::QualifiedName;

use crate::zuir::version::VersionTriple;


// ============================================================================
// ERROR HELPERS
// ============================================================================

fn gradient_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Differentiation)
}

fn invalid_gradient_input(
    message: impl Into<String>,
) -> AiError {
    gradient_error(
        AiErrorKind::InvalidInput,
        message,
    )
}

fn validate_text(
    field: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(
            invalid_gradient_input(
                format!("{field} cannot be empty"),
            ),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            invalid_gradient_input(
                format!("{field} contains control characters"),
            ),
        );
    }

    Ok(())
}

fn validate_reference(
    field: &str,
    reference: &TrainingValueRef,
) -> AiResult<()> {
    validate_text(field, reference.as_str())
}

fn qualified_name_key(
    name: &QualifiedName,
) -> String {
    format!(
        "{}::{}",
        name.namespace,
        name.name,
    )
}


// ============================================================================
// GRADIENT CONTRACT IDENTITY
// ============================================================================

/// Canonical semantic identity of a gradient contract.
///
/// This reuses the frontend's qualified-name representation.
///
/// It does not introduce another global identity allocator.
pub type GradientContractId = QualifiedName;


// ============================================================================
// GRADIENT RESULT CLASSIFICATION
// ============================================================================

/// Mathematical structure requested from a differentiation operation.
///
/// These classifications describe the intended mathematical result.
/// They do not select a numerical implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GradientResultKind {

    /// Gradient of a scalar-valued function with respect to variables.
    Gradient,

    /// Jacobian of a potentially vector-valued function.
    Jacobian,

    /// Hessian of a scalar-valued function.
    Hessian,

    /// A higher-order derivative.
    HigherOrder,

    /// Vector-Jacobian product.
    VectorJacobianProduct,

    /// Jacobian-vector product.
    JacobianVectorProduct,

    /// Hessian-vector product.
    HessianVectorProduct,

    /// Extension-defined derivative result.
    ///
    /// The reference is resolved through canonical semantic
    /// and extension registries.
    Custom(TrainingValueRef),
}

impl GradientResultKind {

    /// Validates any extension-defined references.
    pub fn validate(&self) -> AiResult<()> {
        if let Self::Custom(reference) = self {
            validate_reference(
                "custom gradient result kind",
                reference,
            )?;
        }

        Ok(())
    }
}


// ============================================================================
// DIFFERENTIATION STRATEGY INTENT
// ============================================================================

/// Semantic strength of a differentiation strategy request.
///
/// This distinction prevents preferences from accidentally becoming
/// mandatory execution requirements.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum StrategyIntent {

    /// The strategy is required for semantic correctness.
    Required,

    /// The strategy is preferred but alternatives are permitted.
    Preferred,

    /// The strategy is optional implementation guidance.
    Hint,
}

/// A symbolic differentiation strategy request.
///
/// Strategy names remain open-world.
///
/// Examples include:
///
/// - forward;
/// - reverse;
/// - symbolic;
/// - numerical;
/// - automatic;
/// - parameter_shift;
/// - adjoint;
/// - extension-defined strategies.
///
/// These names are not implemented or selected here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GradientStrategyRequest {

    strategy: TrainingValueRef,

    intent: StrategyIntent,
}

impl GradientStrategyRequest {

    /// Creates a strategy request.
    pub fn new(
        strategy: TrainingValueRef,
        intent: StrategyIntent,
    ) -> AiResult<Self> {
        validate_reference(
            "differentiation strategy",
            &strategy,
        )?;

        Ok(Self {
            strategy,
            intent,
        })
    }

    /// Returns the strategy reference.
    #[must_use]
    pub fn strategy(&self) -> &TrainingValueRef {
        &self.strategy
    }

    /// Returns the requested strategy intent.
    #[must_use]
    pub const fn intent(&self) -> StrategyIntent {
        self.intent
    }
}


// ============================================================================
// GRADIENT REQUEST
// ============================================================================

/// A target-independent derivative computation request.
///
/// This describes the mathematical inputs and intended derivative.
///
/// All expressions remain references to canonical Zamani values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GradientRequest {

    /// Function, objective, loss or other differentiable target.
    target: TrainingValueRef,

    /// Values with respect to which differentiation is requested.
    variables: Vec<TrainingValueRef>,

    /// Optional symbolic derivative order.
    ///
    /// The order is represented by a canonical expression reference.
    /// This permits symbolic and compile-time-resolved orders without
    /// imposing an arbitrary integer-width or grammar-level limit.
    order: Option<TrainingValueRef>,

    /// Requested mathematical result structure.
    result_kind: GradientResultKind,

    /// Differentiation strategy requirements and preferences.
    strategies: Vec<GradientStrategyRequest>,

    /// Extensible named differentiation options.
    ///
    /// Values remain canonical expression references.
    options: BTreeMap<String, TrainingValueRef>,
}

impl GradientRequest {

    /// Creates a gradient request.
    ///
    /// At least one differentiation variable is required.
    ///
    /// Semantic existence and mathematical legality are checked
    /// by the frontend and differentiation subsystems.
    pub fn new(
        target: TrainingValueRef,
        variables: Vec<TrainingValueRef>,
        result_kind: GradientResultKind,
    ) -> AiResult<Self> {

        validate_reference(
            "gradient target",
            &target,
        )?;

        if variables.is_empty() {
            return Err(
                invalid_gradient_input(
                    "a gradient request requires at least one differentiation variable",
                ),
            );
        }

        let mut seen = BTreeSet::new();

        for variable in &variables {
            validate_reference(
                "differentiation variable",
                variable,
            )?;

            if !seen.insert(variable.as_str()) {
                return Err(
                    invalid_gradient_input(
                        format!(
                            "duplicate differentiation variable: {}",
                            variable.as_str(),
                        ),
                    ),
                );
            }
        }

        result_kind.validate()?;

        Ok(Self {
            target,
            variables,
            order: None,
            result_kind,
            strategies: Vec::new(),
            options: BTreeMap::new(),
        })
    }

    /// Sets the symbolic derivative order.
    ///
    /// The expression is not evaluated here.
    pub fn with_order(
        mut self,
        order: TrainingValueRef,
    ) -> AiResult<Self> {
        validate_reference(
            "derivative order",
            &order,
        )?;

        self.order = Some(order);

        Ok(self)
    }

    /// Adds a differentiation strategy request.
    pub fn with_strategy(
        mut self,
        strategy: GradientStrategyRequest,
    ) -> AiResult<Self> {

        if self.strategies.contains(&strategy) {
            return Err(
                invalid_gradient_input(
                    "duplicate differentiation strategy request",
                ),
            );
        }

        self.strategies.push(strategy);

        Ok(self)
    }

    /// Adds an extensible differentiation option.
    ///
    /// Duplicate option names are rejected.
    pub fn with_option(
        mut self,
        name: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {

        let name = name.into();

        validate_text(
            "differentiation option name",
            &name,
        )?;

        validate_reference(
            "differentiation option value",
            &value,
        )?;

        if self.options.contains_key(&name) {
            return Err(
                invalid_gradient_input(
                    format!(
                        "duplicate differentiation option: {name}",
                    ),
                ),
            );
        }

        self.options.insert(name, value);

        Ok(self)
    }

    /// Returns the differentiation target.
    #[must_use]
    pub fn target(&self) -> &TrainingValueRef {
        &self.target
    }

    /// Returns differentiation variables in declaration order.
    #[must_use]
    pub fn variables(&self) -> &[TrainingValueRef] {
        &self.variables
    }

    /// Returns the optional symbolic derivative order.
    #[must_use]
    pub fn order(&self) -> Option<&TrainingValueRef> {
        self.order.as_ref()
    }

    /// Returns the requested mathematical result structure.
    #[must_use]
    pub fn result_kind(&self) -> &GradientResultKind {
        &self.result_kind
    }

    /// Returns strategy requests in declaration order.
    #[must_use]
    pub fn strategies(&self) -> &[GradientStrategyRequest] {
        &self.strategies
    }

    /// Returns deterministic named options.
    #[must_use]
    pub fn options(&self) -> &BTreeMap<String, TrainingValueRef> {
        &self.options
    }

    /// Validates all locally owned request invariants.
    pub fn validate(&self) -> AiResult<()> {

        validate_reference(
            "gradient target",
            &self.target,
        )?;

        if self.variables.is_empty() {
            return Err(
                invalid_gradient_input(
                    "gradient request has no differentiation variables",
                ),
            );
        }

        let mut variables = BTreeSet::new();

        for variable in &self.variables {
            validate_reference(
                "differentiation variable",
                variable,
            )?;

            if !variables.insert(variable.as_str()) {
                return Err(
                    invalid_gradient_input(
                        "gradient request contains duplicate variables",
                    ),
                );
            }
        }

        if let Some(order) = &self.order {
            validate_reference(
                "derivative order",
                order,
            )?;
        }

        self.result_kind.validate()?;

        let mut strategies = BTreeSet::new();

        for strategy in &self.strategies {
            validate_reference(
                "differentiation strategy",
                strategy.strategy(),
            )?;

            let key = format!(
                "{}::{:?}",
                strategy.strategy().as_str(),
                strategy.intent(),
            );

            if !strategies.insert(key) {
                return Err(
                    invalid_gradient_input(
                        "duplicate differentiation strategy",
                    ),
                );
            }
        }

        for (name, value) in &self.options {
            validate_text(
                "differentiation option name",
                name,
            )?;

            validate_reference(
                "differentiation option value",
                value,
            )?;
        }

        Ok(())
    }
}


// ============================================================================
// GRADIENT OUTPUT BINDINGS
// ============================================================================

/// Associates one differentiated variable with its symbolic result.
///
/// This is a semantic binding, not an allocated tensor or numerical value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GradientOutputBinding {

    variable: TrainingValueRef,

    output: TrainingValueRef,
}

impl GradientOutputBinding {

    /// Creates a gradient output binding.
    pub fn new(
        variable: TrainingValueRef,
        output: TrainingValueRef,
    ) -> AiResult<Self> {

        validate_reference(
            "gradient output variable",
            &variable,
        )?;

        validate_reference(
            "gradient output reference",
            &output,
        )?;

        Ok(Self {
            variable,
            output,
        })
    }

    /// Returns the differentiated variable.
    #[must_use]
    pub fn variable(&self) -> &TrainingValueRef {
        &self.variable
    }

    /// Returns the symbolic gradient output.
    #[must_use]
    pub fn output(&self) -> &TrainingValueRef {
        &self.output
    }
}


// ============================================================================
// GRADIENT CONTRACT
// ============================================================================

/// Complete target-independent gradient computation contract.
///
/// A contract describes a derivative request, its requirements,
/// result bindings and provenance references.
///
/// It does not execute differentiation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GradientContract {

    identity: GradientContractId,

    version: VersionTriple,

    request: GradientRequest,

    requirements: AiRequirements,

    outputs: Vec<GradientOutputBinding>,

    numerical_policy: Option<TrainingValueRef>,

    provenance: Option<TrainingValueRef>,

    metadata: BTreeMap<String, TrainingValueRef>,
}

impl GradientContract {

    /// Creates an unvalidated gradient contract.
    ///
    /// The default requirement collection is empty and complete.
    pub fn new(
        identity: GradientContractId,
        version: VersionTriple,
        request: GradientRequest,
    ) -> Self {

        Self {
            identity,
            version,
            request,
            requirements: AiRequirements::new(),
            outputs: Vec::new(),
            numerical_policy: None,
            provenance: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Replaces the AI computational requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Adds a gradient output binding.
    pub fn with_output(
        mut self,
        output: GradientOutputBinding,
    ) -> AiResult<Self> {

        if self.outputs.iter().any(|existing| {
            existing.variable == output.variable
        }) {
            return Err(
                invalid_gradient_input(
                    "duplicate gradient output binding for a variable",
                ),
            );
        }

        self.outputs.push(output);

        Ok(self)
    }

    /// Associates a numerical policy reference.
    ///
    /// The referenced policy is resolved by its owning subsystem.
    pub fn with_numerical_policy(
        mut self,
        policy: TrainingValueRef,
    ) -> AiResult<Self> {

        validate_reference(
            "gradient numerical policy",
            &policy,
        )?;

        self.numerical_policy = Some(policy);

        Ok(self)
    }

    /// Associates a provenance reference.
    ///
    /// Canonical provenance construction remains owned by
    /// the semantic provenance infrastructure.
    pub fn with_provenance(
        mut self,
        provenance: TrainingValueRef,
    ) -> AiResult<Self> {

        validate_reference(
            "gradient provenance",
            &provenance,
        )?;

        self.provenance = Some(provenance);

        Ok(self)
    }

    /// Adds extensible metadata.
    ///
    /// Duplicate keys are rejected.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {

        let key = key.into();

        validate_text(
            "gradient metadata key",
            &key,
        )?;

        validate_reference(
            "gradient metadata value",
            &value,
        )?;

        if self.metadata.contains_key(&key) {
            return Err(
                invalid_gradient_input(
                    format!(
                        "duplicate gradient metadata key: {key}",
                    ),
                ),
            );
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Returns the contract identity.
    #[must_use]
    pub fn identity(&self) -> &GradientContractId {
        &self.identity
    }

    /// Returns the exact contract version.
    #[must_use]
    pub const fn version(&self) -> VersionTriple {
        self.version
    }

    /// Returns the gradient request.
    #[must_use]
    pub fn request(&self) -> &GradientRequest {
        &self.request
    }

    /// Returns the AI computational requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Returns output bindings in declaration order.
    #[must_use]
    pub fn outputs(&self) -> &[GradientOutputBinding] {
        &self.outputs
    }

    /// Returns the optional numerical policy reference.
    #[must_use]
    pub fn numerical_policy(&self) -> Option<&TrainingValueRef> {
        self.numerical_policy.as_ref()
    }

    /// Returns the optional provenance reference.
    #[must_use]
    pub fn provenance(&self) -> Option<&TrainingValueRef> {
        self.provenance.as_ref()
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, TrainingValueRef> {
        &self.metadata
    }

    /// Validates the complete locally owned gradient contract.
    ///
    /// This does not establish mathematical differentiability,
    /// symbol resolution, target compatibility or execution feasibility.
    pub fn validate(&self) -> AiResult<()> {

        validate_text(
            "gradient contract namespace",
            &self.identity.namespace,
        )?;

        validate_text(
            "gradient contract name",
            &self.identity.name,
        )?;

        self.request.validate()?;

        if !self.requirements.is_complete() {
            return Err(
                gradient_error(
                    AiErrorKind::RequirementUnsatisfied,
                    "gradient requirement analysis is incomplete",
                ),
            );
        }

        let mut bound_variables = BTreeSet::new();

        for output in &self.outputs {

            validate_reference(
                "gradient output variable",
                &output.variable,
            )?;

            validate_reference(
                "gradient output reference",
                &output.output,
            )?;

            let declared = self.request.variables.iter().any(
                |variable| variable == &output.variable,
            );

            if !declared {
                return Err(
                    invalid_gradient_input(
                        format!(
                            "gradient output references an undeclared variable: {}",
                            output.variable.as_str(),
                        ),
                    ),
                );
            }

            if !bound_variables.insert(
                output.variable.as_str(),
            ) {
                return Err(
                    invalid_gradient_input(
                        "duplicate gradient output variable",
                    ),
                );
            }
        }

        if let Some(policy) = &self.numerical_policy {
            validate_reference(
                "gradient numerical policy",
                policy,
            )?;
        }

        if let Some(provenance) = &self.provenance {
            validate_reference(
                "gradient provenance",
                provenance,
            )?;
        }

        for (key, value) in &self.metadata {
            validate_text(
                "gradient metadata key",
                key,
            )?;

            validate_reference(
                "gradient metadata value",
                value,
            )?;
        }

        Ok(())
    }

    /// Converts this contract into the existing generic training
    /// operation reference.
    ///
    /// This permits training steps to reference a gradient contract
    /// without introducing a second training operation representation.
    ///
    /// The returned operation is symbolic. It does not execute
    /// differentiation or establish that an implementation exists.
    pub fn training_operation(&self) -> AiResult<TrainingOperation> {

        let identity = qualified_name_key(
            &self.identity,
        );

        TrainingOperation::new(identity)
    }
}


// ============================================================================
// GRADIENT VALIDATION OUTCOME
// ============================================================================

/// Describes the scope of validation performed on a gradient contract.
///
/// Structural validation must not be confused with mathematical
/// differentiability or execution feasibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GradientValidationLevel {

    /// Only locally owned structural invariants were checked.
    Structural,

    /// Canonical semantic analysis also resolved the relevant references.
    Semantic,

    /// The differentiation subsystem established mathematical legality.
    Mathematical,

    /// A downstream execution planner evaluated realization feasibility.
    Feasibility,
}

/// A report describing the validation performed on a gradient contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GradientValidationReport {

    contract: GradientContractId,

    level: GradientValidationLevel,

    valid: bool,

    diagnostics: Vec<String>,
}

impl GradientValidationReport {

    /// Creates a structural validation report.
    ///
    /// This constructor deliberately cannot claim semantic,
    /// mathematical or execution validation.
    pub fn structural(
        contract: GradientContractId,
        result: AiResult<()>,
    ) -> Self {

        match result {
            Ok(()) => Self {
                contract,
                level: GradientValidationLevel::Structural,
                valid: true,
                diagnostics: Vec::new(),
            },

            Err(error) => Self {
                contract,
                level: GradientValidationLevel::Structural,
                valid: false,
                diagnostics: vec![
                    error.to_string(),
                ],
            },
        }
    }

    /// Returns the validated contract identity.
    #[must_use]
    pub fn contract(&self) -> &GradientContractId {
        &self.contract
    }

    /// Returns the validation level.
    #[must_use]
    pub const fn level(&self) -> GradientValidationLevel {
        self.level
    }

    /// Returns whether this validation stage succeeded.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns validation diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }
}


// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for GradientResultKind {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {
            Self::Gradient => formatter.write_str("gradient"),
            Self::Jacobian => formatter.write_str("jacobian"),
            Self::Hessian => formatter.write_str("hessian"),
            Self::HigherOrder => formatter.write_str("higher_order"),
            Self::VectorJacobianProduct => {
                formatter.write_str("vector_jacobian_product")
            }
            Self::JacobianVectorProduct => {
                formatter.write_str("jacobian_vector_product")
            }
            Self::HessianVectorProduct => {
                formatter.write_str("hessian_vector_product")
            }
            Self::Custom(reference) => {
                write!(formatter, "custom({})", reference)
            }
        }
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    fn reference(
        name: &str,
    ) -> TrainingValueRef {

        TrainingValueRef::new(name)
            .expect("valid training value reference")
    }

    fn identity() -> QualifiedName {

        QualifiedName::new(
            "learning.gradient",
            "loss_gradient",
        )
        .expect("valid qualified name")
    }

    fn request() -> GradientRequest {

        GradientRequest::new(
            reference("training.loss"),
            vec![
                reference("model.weights"),
            ],
            GradientResultKind::Gradient,
        )
        .expect("valid gradient request")
    }

    fn contract() -> GradientContract {

        GradientContract::new(
            identity(),
            VersionTriple::new(1, 0, 0),
            request(),
        )
    }

    #[test]
    fn gradient_request_accepts_symbolic_references() {

        let request = request();

        assert_eq!(
            request.target().as_str(),
            "training.loss",
        );

        assert_eq!(
            request.variables().len(),
            1,
        );
    }

    #[test]
    fn empty_differentiation_variables_are_rejected() {

        let result = GradientRequest::new(
            reference("training.loss"),
            Vec::new(),
            GradientResultKind::Gradient,
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_variables_are_rejected() {

        let result = GradientRequest::new(
            reference("training.loss"),
            vec![
                reference("weights"),
                reference("weights"),
            ],
            GradientResultKind::Gradient,
        );

        assert!(result.is_err());
    }

    #[test]
    fn symbolic_derivative_order_is_preserved() {

        let request = request()
            .with_order(reference("derivative.order"))
            .expect("valid symbolic order");

        assert_eq!(
            request.order()
                .expect("order exists")
                .as_str(),
            "derivative.order",
        );
    }

    #[test]
    fn extensible_strategy_names_are_supported() {

        let request = request()
            .with_strategy(
                GradientStrategyRequest::new(
                    reference("future.differentiation.method"),
                    StrategyIntent::Preferred,
                )
                .expect("valid strategy"),
            )
            .expect("strategy insertion");

        assert_eq!(
            request.strategies().len(),
            1,
        );
    }

    #[test]
    fn duplicate_options_are_rejected() {

        let result = request()
            .with_option(
                "precision",
                reference("numeric.precision"),
            )
            .expect("first option")
            .with_option(
                "precision",
                reference("numeric.other"),
            );

        assert!(result.is_err());
    }

    #[test]
    fn gradient_contract_validates() {

        let gradient = contract();

        assert!(
            gradient.validate().is_ok()
        );
    }

    #[test]
    fn output_binding_requires_declared_variable() {

        let gradient = contract()
            .with_output(
                GradientOutputBinding::new(
                    reference("unknown.variable"),
                    reference("gradient.output"),
                )
                .expect("valid binding"),
            )
            .expect("binding insertion");

        assert!(
            gradient.validate().is_err()
        );
    }

    #[test]
    fn valid_output_binding_is_accepted() {

        let gradient = contract()
            .with_output(
                GradientOutputBinding::new(
                    reference("model.weights"),
                    reference("gradient.weights"),
                )
                .expect("valid binding"),
            )
            .expect("binding insertion");

        assert!(
            gradient.validate().is_ok()
        );
    }

    #[test]
    fn duplicate_output_bindings_are_rejected() {

        let gradient = contract()
            .with_output(
                GradientOutputBinding::new(
                    reference("model.weights"),
                    reference("gradient.first"),
                )
                .expect("first binding"),
            )
            .expect("first insertion");

        let result = gradient
            .with_output(
                GradientOutputBinding::new(
                    reference("model.weights"),
                    reference("gradient.second"),
                )
                .expect("second binding"),
            );

        assert!(result.is_err());
    }

    #[test]
    fn requirements_are_preserved() {

        let requirements = AiRequirements::new();

        let gradient = contract()
            .with_requirements(requirements.clone());

        assert_eq!(
            gradient.requirements(),
            &requirements,
        );

        assert!(
            gradient.validate().is_ok()
        );
    }

    #[test]
    fn training_interoperability_uses_existing_operation_type() {

        let gradient = contract();

        let operation = gradient
            .training_operation()
            .expect("training operation");

        assert_eq!(
            operation.identity().as_str(),
            "learning.gradient::loss_gradient",
        );
    }

    #[test]
    fn validation_report_does_not_claim_mathematical_validation() {

        let gradient = contract();

        let report = GradientValidationReport::structural(
            gradient.identity().clone(),
            gradient.validate(),
        );

        assert!(report.is_valid());

        assert_eq!(
            report.level(),
            GradientValidationLevel::Structural,
        );
    }

    #[test]
    fn gradient_result_kinds_are_extensible() {

        let custom = GradientResultKind::Custom(
            reference("extension.custom_gradient"),
        );

        assert!(custom.validate().is_ok());

        assert_eq!(
            custom.to_string(),
            "custom(extension.custom_gradient)",
        );
    }
}
