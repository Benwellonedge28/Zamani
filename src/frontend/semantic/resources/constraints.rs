#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Resource Constraints
//!
//! Path: `src/frontend/semantic/resources/constraints.rs`
//!
//! ## PURPOSE
//!
//! Evaluate resource requirements and resource constraints using explicitly
//! supplied semantic evidence. Preserve unresolved conditions as deferred
//! assessments instead of guessing, truncating, or treating them as satisfied.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/resources/constraints.g4`
//! - `grammar/resources/resources.g4`
//! - `src/frontend/ast/node/resources/resource.rs`
//! - `src/frontend/ast/node/resources/resource_kind.rs`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/symbolic.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//!
//! ## OWNS
//!
//! - Constraint evaluation against caller-supplied semantic evidence.
//! - Requirement evaluation against caller-supplied resource evidence.
//! - Three-valued evaluation plus explicit invalid and inapplicable states.
//! - Structured assessments and deterministic batch reports.
//! - Separation of binding conditions from advisory intent.
//! - Explicit handling of symbolic quantities and incompatible units.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing or grammar interpretation.
//! - AST construction, identity, or traversal.
//! - Resource-kind identity or the canonical resource model.
//! - Name resolution, general constant evaluation, or type inference.
//! - Capability discovery or authorization.
//! - Physical resource inventory or allocation.
//! - Target selection, scheduling, routing, lowering, or execution.
//! - Unit conversion tables or dimensional-analysis policy.
//!
//! ## INTEGRATION CONTRACT
//!
//! `model.rs` owns `ResourceConstraint`, `ResourceRequirement`,
//! `ConstraintOperator`, `ConstraintOperand`, `ResourceQuantity`,
//! `ExactQuantity`, `ResourceUnit`, and `ResourceIntent`.
//!
//! A semantic context implements `ResourceConstraintResolver` and supplies
//! resolved values. Expression evaluation remains owned by the canonical
//! expression/constant-evaluation subsystem. Unit conversion may be supplied
//! by overriding `compare_quantities`.
//!
//! This module never inspects hardware itself. A downstream target-feasibility
//! service may provide resource evidence, but that evidence must be explicit.
//!
//! ## SCALABILITY
//!
//! No machine-capacity ceilings are defined. Quantities remain decimal
//! strings through `ExactQuantity`; this module does not convert them to
//! `usize`, `u64`, `u128`, or floating-point values.
//!
//! Collection processing grows with the supplied collection and available
//! memory. Callers may enforce explicit analysis budgets outside this module.
//!
//! ## DETERMINISM
//!
//! Items are assessed in caller-supplied order. No randomized iteration,
//! global mutable state, clock, environment lookup, or hardware probing occurs.
//!
//! ## FAILURE POLICY
//!
//! - Satisfied: sufficient evidence proves the condition.
//! - Violated: sufficient evidence disproves the condition.
//! - Deferred: required evidence is unavailable or remains symbolic.
//! - Invalid: evaluation encountered an invalid value or resolver failure.
//! - NotApplicable: the requirement intent does not impose a testable condition.
//!
//! Deferred and invalid assessments are never represented as satisfied.
//!
//! ## SAFETY
//!
//! Safe Rust only. No source expression is executed by this module.

use core::cmp::Ordering;
use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::resources::resource_kind::ResourceKind;

use super::model::{
    ConstraintOperand,
    ConstraintOperator,
    ExactQuantity,
    ResourceConstraint,
    ResourceIntent,
    ResourceQuantity,
    ResourceRequirement,
    ResourceUnit,
};

/// Version of this module's in-memory assessment contract.
///
/// This is not the language version or a serialized-artifact version.
pub const RESOURCE_CONSTRAINTS_SCHEMA_VERSION: u16 = 1;

/// The result of checking a resource condition.
///
/// `Deferred` means that the analyzer does not currently have enough
/// information to decide the condition. It does not mean the condition holds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ConstraintOutcome {
    /// The condition was established from available evidence.
    Satisfied,

    /// Available evidence proves that the condition does not hold.
    Violated,

    /// The condition cannot yet be decided.
    Deferred,

    /// Evaluation encountered invalid input or a resolver failure.
    Invalid,

    /// The requirement intent does not impose a testable condition.
    NotApplicable,
}

impl ConstraintOutcome {
    /// Returns whether evaluation established a definitive result.
    ///
    /// Invalid and inapplicable assessments are not successful proofs.
    #[must_use]
    pub const fn is_decisive(self) -> bool {
        matches!(self, Self::Satisfied | Self::Violated)
    }

    /// Returns whether this outcome proves that its condition holds.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }
}

/// How the language-level intent affects enforcement of an assessment.
///
/// Advisory conditions may be useful to an optimizer or target planner but
/// must not silently become mandatory requirements.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ConstraintEnforcement {
    /// The condition is mandatory under its declared semantics.
    Binding,

    /// The condition is advisory and may be unsatisfied without invalidating
    /// the program solely for that reason.
    Advisory,

    /// The condition records permission rather than requiring consumption.
    Permission,

    /// The condition prohibits the relevant resource use.
    Prohibition,

    /// The condition expresses a budget.
    Budget,
}

impl ConstraintEnforcement {
    /// Maps source-level intent to its enforcement category.
    #[must_use]
    pub const fn from_intent(intent: ResourceIntent) -> Self {
        match intent {
            ResourceIntent::Required => Self::Binding,
            ResourceIntent::Preferred => Self::Advisory,
            ResourceIntent::Allowed => Self::Permission,
            ResourceIntent::Forbidden => Self::Prohibition,
            ResourceIntent::Hint => Self::Advisory,
            ResourceIntent::Budget => Self::Budget,
        }
    }

    /// Returns whether a violation of this condition is mandatory to resolve.
    #[must_use]
    pub const fn is_binding(self) -> bool {
        matches!(
            self,
            Self::Binding | Self::Prohibition | Self::Budget
        )
    }
}

/// A resource value whose meaning has been established by a semantic resolver.
///
/// The value is target-independent in representation. A resolver may supply
/// target-derived evidence when explicitly performing feasibility analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedResourceValue {
    quantity: ExactQuantity,
    unit: Option<ResourceUnit>,
}

impl ResolvedResourceValue {
    /// Constructs a resolved value from a canonical exact quantity and unit.
    #[must_use]
    pub fn new(
        quantity: ExactQuantity,
        unit: Option<ResourceUnit>,
    ) -> Self {
        Self { quantity, unit }
    }

    /// Constructs a resolved value from decimal text.
    ///
    /// Returns an error if the quantity is not a valid exact non-negative
    /// decimal quantity.
    pub fn from_decimal(
        quantity: impl AsRef<str>,
        unit: Option<ResourceUnit>,
    ) -> Result<Self, ConstraintEvaluationError> {
        let quantity = ExactQuantity::new(quantity).map_err(|error| {
            ConstraintEvaluationError::InvalidQuantity(error.to_string())
        })?;

        Ok(Self { quantity, unit })
    }

    /// Returns the exact quantity.
    #[must_use]
    pub fn quantity(&self) -> &ExactQuantity {
        &self.quantity
    }

    /// Returns the unit, if known.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }
}

/// Resolution state for a resource value.
///
/// `Unavailable` means that the resolver cannot currently provide a value.
/// It does not mean that the resource is absent or has zero capacity.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceValueResolution {
    /// The quantity and optional unit are known.
    Known(ResolvedResourceValue),

    /// The value exists but remains symbolic.
    Symbolic {
        /// The source expression, when available.
        expression: Option<NodeId>,
    },

    /// No usable value is currently available.
    Unavailable,
}

/// Resolution state for a constraint operand backed by an AST expression.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExpressionValueResolution {
    /// Evaluation produced an exact non-negative quantity.
    Known(ExactQuantity),

    /// The expression is valid but its value remains symbolic or unresolved.
    Symbolic,

    /// The expression is runtime-dependent.
    Runtime,

    /// The expression cannot produce a valid resource quantity.
    Invalid(String),
}

/// Services required to evaluate resource constraints.
///
/// Implementations should delegate to the canonical semantic context,
/// symbol resolver, constant evaluator, and unit registry. They must not
/// create independent symbol tables or silently invent values.
///
/// This trait does not require an implementation to inspect a physical target.
/// For source-only semantic analysis, unavailable values remain deferred.
pub trait ResourceConstraintResolver {
    /// Resolves the current semantic value of a resource kind.
    ///
    /// `Ok(None)` is not used: the resolution state is explicit in the
    /// returned `ResourceValueResolution`.
    fn resource_value(
        &self,
        kind: &ResourceKind,
    ) -> Result<ResourceValueResolution, String>;

    /// Resolves an AST expression to an exact resource quantity if possible.
    ///
    /// `Symbolic` and `Runtime` are not errors; they indicate that exact
    /// compile-time evaluation is not currently possible.
    fn expression_value(
        &self,
        expression: NodeId,
    ) -> Result<ExpressionValueResolution, String>;

    /// Compares two quantities, accounting for their units.
    ///
    /// Returns:
    /// - `Ok(Some(ordering))` when the quantities are comparable;
    /// - `Ok(None)` when the relationship cannot be established, for example
    ///   because units differ and no conversion is registered;
    /// - `Err(message)` when the resolver detects invalid unit semantics.
    ///
    /// The default implementation compares quantities only when both units
    /// are absent or when both units have the same canonical identity.
    /// Implementations with a validated unit registry may override this
    /// method to convert compatible units exactly.
    fn compare_quantities(
        &self,
        left: &ExactQuantity,
        left_unit: Option<&ResourceUnit>,
        right: &ExactQuantity,
        right_unit: Option<&ResourceUnit>,
    ) -> Result<Option<Ordering>, String> {
        match (left_unit, right_unit) {
            (None, None) => Ok(Some(left.numeric_cmp(right))),
            (Some(left_unit), Some(right_unit))
                if left_unit == right_unit =>
            {
                Ok(Some(left.numeric_cmp(right)))
            }
            _ => Ok(None),
        }
    }
}

/// The source category associated with an assessment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AssessmentKind {
    /// A `ResourceConstraint` from the semantic resource model.
    Constraint,

    /// A `ResourceRequirement` from the semantic resource model.
    Requirement,
}

/// One structured result from checking a resource condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintAssessment {
    kind: AssessmentKind,
    outcome: ConstraintOutcome,
    enforcement: ConstraintEnforcement,
    intent: ResourceIntent,
    source_node: Option<NodeId>,
    resource_kind: String,
    explanation: String,
}

impl ConstraintAssessment {
    fn new(
        kind: AssessmentKind,
        outcome: ConstraintOutcome,
        intent: ResourceIntent,
        source_node: Option<NodeId>,
        resource_kind: &ResourceKind,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            outcome,
            enforcement: ConstraintEnforcement::from_intent(intent),
            intent,
            source_node,
            resource_kind: resource_kind.as_str().to_owned(),
            explanation: explanation.into(),
        }
    }

    /// Returns the assessed source category.
    #[must_use]
    pub const fn kind(&self) -> AssessmentKind {
        self.kind
    }

    /// Returns the outcome.
    #[must_use]
    pub const fn outcome(&self) -> ConstraintOutcome {
        self.outcome
    }

    /// Returns the enforcement category.
    #[must_use]
    pub const fn enforcement(&self) -> ConstraintEnforcement {
        self.enforcement
    }

    /// Returns the source-level intent.
    #[must_use]
    pub const fn intent(&self) -> ResourceIntent {
        self.intent
    }

    /// Returns the originating AST node, when available.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }

    /// Returns the canonical resource-kind spelling.
    #[must_use]
    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    /// Returns the human-readable explanation.
    ///
    /// Consumers should use `outcome()` and `enforcement()` for programmatic
    /// decisions instead of parsing this text.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    /// Returns whether this assessment blocks acceptance of the condition.
    ///
    /// Invalid assessments always block acceptance because the analyzer could
    /// not establish correctness. A deferred assessment means analysis is
    /// incomplete, but is not itself proof of a violation.
    #[must_use]
    pub const fn blocks_acceptance(&self) -> bool {
        match self.outcome {
            ConstraintOutcome::Invalid => true,
            ConstraintOutcome::Violated => self.enforcement.is_binding(),
            ConstraintOutcome::Satisfied
            | ConstraintOutcome::Deferred
            | ConstraintOutcome::NotApplicable => false,
        }
    }

    /// Returns whether this assessment requires additional semantic evidence.
    #[must_use]
    pub const fn needs_more_evidence(&self) -> bool {
        matches!(
            self.outcome,
            ConstraintOutcome::Deferred | ConstraintOutcome::Invalid
        )
    }
}

/// Errors constructing an assessment from invalid local input.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConstraintEvaluationError {
    /// An exact resource quantity could not be parsed.
    InvalidQuantity(String),

    /// A resolver returned an invalid result.
    ResolverFailure(String),

    /// An internal evaluation invariant was not satisfied.
    InternalInvariant(String),
}

impl fmt::Display for ConstraintEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuantity(message) => {
                write!(formatter, "invalid resource quantity: {message}")
            }
            Self::ResolverFailure(message) => {
                write!(formatter, "resource constraint resolver failed: {message}")
            }
            Self::InternalInvariant(message) => {
                write!(formatter, "resource constraint evaluation invariant failed: {message}")
            }
        }
    }
}

impl Error for ConstraintEvaluationError {}

/// A deterministic report containing resource-condition assessments.
///
/// The report preserves the input order. A report with deferred or invalid
/// assessments is not complete, and must not be treated as proof that every
/// condition holds.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceConstraintReport {
    assessments: Vec<ConstraintAssessment>,
}

impl ResourceConstraintReport {
    /// Creates an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            assessments: Vec::new(),
        }
    }

    /// Returns assessments in input order.
    #[must_use]
    pub fn assessments(&self) -> &[ConstraintAssessment] {
        &self.assessments
    }

    /// Returns the number of assessments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.assessments.len()
    }

    /// Returns whether the report has no assessments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assessments.is_empty()
    }

    /// Returns whether all assessments have decisive outcomes.
    ///
    /// `NotApplicable` is decisive in the sense that no test was required;
    /// callers must still inspect it if their workflow requires every
    /// requirement to have a directly evaluated condition.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.assessments.iter().all(|assessment| {
            !matches!(
                assessment.outcome(),
                ConstraintOutcome::Deferred | ConstraintOutcome::Invalid
            )
        })
    }

    /// Returns whether any assessment blocks acceptance.
    #[must_use]
    pub fn has_blocking_issues(&self) -> bool {
        self.assessments
            .iter()
            .any(ConstraintAssessment::blocks_acceptance)
    }

    /// Returns whether any condition remains unresolved or invalid.
    #[must_use]
    pub fn needs_more_evidence(&self) -> bool {
        self.assessments
            .iter()
            .any(ConstraintAssessment::needs_more_evidence)
    }

    /// Returns whether a binding condition was violated.
    #[must_use]
    pub fn has_binding_violations(&self) -> bool {
        self.assessments.iter().any(|assessment| {
            assessment.outcome() == ConstraintOutcome::Violated
                && assessment.enforcement().is_binding()
        })
    }

    /// Returns whether any advisory condition was violated.
    #[must_use]
    pub fn has_advisory_violations(&self) -> bool {
        self.assessments.iter().any(|assessment| {
            assessment.outcome() == ConstraintOutcome::Violated
                && !assessment.enforcement().is_binding()
        })
    }

    fn push(&mut self, assessment: ConstraintAssessment) {
        self.assessments.push(assessment);
    }
}

/// Stateless evaluator for canonical semantic resource conditions.
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceConstraintEvaluator;

impl ResourceConstraintEvaluator {
    /// Creates an evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates one resource constraint.
    ///
    /// A missing value, symbolic expression, or incompatible unit produces
    /// `Deferred`, not `Satisfied`. Resolver failures produce `Invalid`.
    pub fn evaluate_constraint<R: ResourceConstraintResolver + ?Sized>(
        &self,
        constraint: &ResourceConstraint,
        resolver: &R,
    ) -> ConstraintAssessment {
        let kind = constraint.resource_kind();
        let intent = constraint.intent();
        let source_node = constraint.source_node();

        let actual = match resolver.resource_value(kind) {
            Ok(ResourceValueResolution::Known(value)) => value,
            Ok(ResourceValueResolution::Symbolic { expression }) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    match expression {
                        Some(id) => format!(
                            "resource value remains symbolic at AST expression {id:?}"
                        ),
                        None => "resource value remains symbolic".to_owned(),
                    },
                );
            }
            Ok(ResourceValueResolution::Unavailable) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "no resource value is currently available",
                );
            }
            Err(message) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Invalid,
                    intent,
                    source_node,
                    kind,
                    format!("could not resolve resource value: {message}"),
                );
            }
        };

        let (expected, expected_unit) = match resolve_operand(
            constraint.operand(),
            constraint.unit(),
            resolver,
        ) {
            Ok(Some(value)) => value,
            Ok(None) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "constraint operand remains symbolic or runtime-dependent",
                );
            }
            Err(message) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Invalid,
                    intent,
                    source_node,
                    kind,
                    format!("could not resolve constraint operand: {message}"),
                );
            }
        };

        let ordering = match resolver.compare_quantities(
            actual.quantity(),
            actual.unit(),
            &expected,
            expected_unit,
        ) {
            Ok(Some(ordering)) => ordering,
            Ok(None) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "resource quantities cannot be compared with the available unit information",
                );
            }
            Err(message) => {
                return assessment(
                    AssessmentKind::Constraint,
                    ConstraintOutcome::Invalid,
                    intent,
                    source_node,
                    kind,
                    format!("unit comparison failed: {message}"),
                );
            }
        };

        let holds = operator_holds(constraint.operator(), ordering);
        let outcome = if holds {
            ConstraintOutcome::Satisfied
        } else {
            ConstraintOutcome::Violated
        };

        assessment(
            AssessmentKind::Constraint,
            outcome,
            intent,
            source_node,
            kind,
            format!(
                "resource value {} the constraint operator {} the supplied operand",
                if holds { "satisfies" } else { "does not satisfy" },
                operator_name(constraint.operator())
            ),
        )
    }

    /// Evaluates one resource requirement against supplied resource evidence.
    ///
    /// Interpretation:
    ///
    /// - `Required`: available quantity must be at least the requested amount.
    /// - `Preferred`: the requested amount is advisory.
    /// - `Budget`: available quantity must not exceed the declared budget.
    /// - `Forbidden`: the resource quantity must be exactly zero.
    /// - `Allowed` and `Hint`: no mandatory quantity test is imposed here.
    ///
    /// These interpretations operate on already-resolved semantic quantities.
    /// The caller must ensure they match the normative language semantics
    /// for the specific resource declaration.
    pub fn evaluate_requirement<R: ResourceConstraintResolver + ?Sized>(
        &self,
        requirement: &ResourceRequirement,
        resolver: &R,
    ) -> ConstraintAssessment {
        let kind = requirement.kind();
        let intent = requirement.intent();
        let source_node = requirement.source_node();

        if matches!(intent, ResourceIntent::Allowed | ResourceIntent::Hint) {
            return assessment(
                AssessmentKind::Requirement,
                ConstraintOutcome::NotApplicable,
                intent,
                source_node,
                kind,
                "this intent does not impose a mandatory quantity comparison",
            );
        }

        let actual = match resolver.resource_value(kind) {
            Ok(ResourceValueResolution::Known(value)) => value,
            Ok(ResourceValueResolution::Symbolic { expression }) => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    match expression {
                        Some(id) => format!(
                            "available resource quantity remains symbolic at AST expression {id:?}"
                        ),
                        None => "available resource quantity remains symbolic".to_owned(),
                    },
                );
            }
            Ok(ResourceValueResolution::Unavailable) => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "available resource quantity is unknown",
                );
            }
            Err(message) => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Invalid,
                    intent,
                    source_node,
                    kind,
                    format!("could not resolve resource value: {message}"),
                );
            }
        };

        if intent == ResourceIntent::Forbidden {
            let zero = match ExactQuantity::new("0") {
                Ok(value) => value,
                Err(error) => {
                    return assessment(
                        AssessmentKind::Requirement,
                        ConstraintOutcome::Invalid,
                        intent,
                        source_node,
                        kind,
                        format!("could not construct zero quantity: {error}"),
                    );
                }
            };

            let ordering = match resolver.compare_quantities(
                actual.quantity(),
                actual.unit(),
                &zero,
                actual.unit(),
            ) {
                Ok(Some(ordering)) => ordering,
                Ok(None) => {
                    return assessment(
                        AssessmentKind::Requirement,
                        ConstraintOutcome::Deferred,
                        intent,
                        source_node,
                        kind,
                        "cannot determine whether the forbidden resource is in use",
                    );
                }
                Err(message) => {
                    return assessment(
                        AssessmentKind::Requirement,
                        ConstraintOutcome::Invalid,
                        intent,
                        source_node,
                        kind,
                        format!("resource comparison failed: {message}"),
                    );
                }
            };

            let outcome = if ordering == Ordering::Equal {
                ConstraintOutcome::Satisfied
            } else {
                ConstraintOutcome::Violated
            };

            return assessment(
                AssessmentKind::Requirement,
                outcome,
                intent,
                source_node,
                kind,
                if outcome == ConstraintOutcome::Satisfied {
                    "the forbidden resource has a known zero quantity"
                } else {
                    "the forbidden resource has a non-zero quantity"
                },
            );
        }

        let requested = match requirement.quantity() {
            ResourceQuantity::Known(quantity) => quantity.clone(),
            ResourceQuantity::Symbolic(expression) => {
                match resolver.expression_value(*expression) {
                    Ok(ExpressionValueResolution::Known(quantity)) => quantity,
                    Ok(
                        ExpressionValueResolution::Symbolic
                        | ExpressionValueResolution::Runtime,
                    ) => {
                        return assessment(
                            AssessmentKind::Requirement,
                            ConstraintOutcome::Deferred,
                            intent,
                            source_node,
                            kind,
                            "requested resource quantity remains unresolved",
                        );
                    }
                    Ok(ExpressionValueResolution::Invalid(message)) => {
                        return assessment(
                            AssessmentKind::Requirement,
                            ConstraintOutcome::Invalid,
                            intent,
                            source_node,
                            kind,
                            format!("requested quantity is invalid: {message}"),
                        );
                    }
                    Err(message) => {
                        return assessment(
                            AssessmentKind::Requirement,
                            ConstraintOutcome::Invalid,
                            intent,
                            source_node,
                            kind,
                            format!("could not evaluate requested quantity: {message}"),
                        );
                    }
                }
            }
            ResourceQuantity::Runtime(_) => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "requested resource quantity is runtime-dependent",
                );
            }
            ResourceQuantity::Unspecified => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "no explicit resource quantity was supplied",
                );
            }
        };

        let ordering = match resolver.compare_quantities(
            actual.quantity(),
            actual.unit(),
            &requested,
            requirement.unit(),
        ) {
            Ok(Some(ordering)) => ordering,
            Ok(None) => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Deferred,
                    intent,
                    source_node,
                    kind,
                    "available and requested quantities cannot be compared with the available unit information",
                );
            }
            Err(message) => {
                return assessment(
                    AssessmentKind::Requirement,
                    ConstraintOutcome::Invalid,
                    intent,
                    source_node,
                    kind,
                    format!("resource comparison failed: {message}"),
                );
            }
        };

        let holds = match intent {
            ResourceIntent::Required | ResourceIntent::Preferred => {
                ordering != Ordering::Less
            }
            ResourceIntent::Budget => ordering != Ordering::Greater,
            ResourceIntent::Allowed
            | ResourceIntent::Forbidden
            | ResourceIntent::Hint => true,
        };

        let outcome = if holds {
            ConstraintOutcome::Satisfied
        } else {
            ConstraintOutcome::Violated
        };

        assessment(
            AssessmentKind::Requirement,
            outcome,
            intent,
            source_node,
            kind,
            match intent {
                ResourceIntent::Required => {
                    if holds {
                        "available quantity meets the required minimum"
                    } else {
                        "available quantity is below the required minimum"
                    }
                }
                ResourceIntent::Preferred => {
                    if holds {
                        "available quantity meets the preferred amount"
                    } else {
                        "available quantity is below the preferred amount"
                    }
                }
                ResourceIntent::Budget => {
                    if holds {
                        "available quantity is within the declared budget"
                    } else {
                        "available quantity exceeds the declared budget"
                    }
                }
                ResourceIntent::Allowed
                | ResourceIntent::Forbidden
                | ResourceIntent::Hint => {
                    "no mandatory quantity comparison applies"
                }
            },
        )
    }

    /// Evaluates constraints in caller-supplied order.
    ///
    /// Every input produces exactly one assessment, including unresolved and
    /// invalid inputs. This prevents one resolver failure from silently
    /// dropping a constraint from the report.
    #[must_use]
    pub fn evaluate_constraints<R: ResourceConstraintResolver + ?Sized>(
        &self,
        constraints: &[ResourceConstraint],
        resolver: &R,
    ) -> ResourceConstraintReport {
        let mut report = ResourceConstraintReport::new();

        for constraint in constraints {
            report.push(self.evaluate_constraint(constraint, resolver));
        }

        report
    }

    /// Evaluates requirements in caller-supplied order.
    #[must_use]
    pub fn evaluate_requirements<R: ResourceConstraintResolver + ?Sized>(
        &self,
        requirements: &[ResourceRequirement],
        resolver: &R,
    ) -> ResourceConstraintReport {
        let mut report = ResourceConstraintReport::new();

        for requirement in requirements {
            report.push(self.evaluate_requirement(requirement, resolver));
        }

        report
    }
}

fn assessment(
    kind: AssessmentKind,
    outcome: ConstraintOutcome,
    intent: ResourceIntent,
    source_node: Option<NodeId>,
    resource_kind: &ResourceKind,
    explanation: impl Into<String>,
) -> ConstraintAssessment {
    ConstraintAssessment::new(
        kind,
        outcome,
        intent,
        source_node,
        resource_kind,
        explanation,
    )
}

fn resolve_operand<R: ResourceConstraintResolver + ?Sized>(
    operand: &ConstraintOperand,
    unit: Option<&ResourceUnit>,
    resolver: &R,
) -> Result<Option<(ExactQuantity, Option<&ResourceUnit>)>, String> {
    match operand {
        ConstraintOperand::Known(quantity) => Ok(Some((quantity.clone(), unit))),
        ConstraintOperand::Expression(expression) => {
            match resolver.expression_value(*expression)? {
                ExpressionValueResolution::Known(quantity) => {
                    Ok(Some((quantity, unit)))
                }
                ExpressionValueResolution::Symbolic
                | ExpressionValueResolution::Runtime => Ok(None),
                ExpressionValueResolution::Invalid(message) => Err(message),
            }
        }
    }
}

fn operator_holds(
    operator: ConstraintOperator,
    ordering: Ordering,
) -> bool {
    match operator {
        ConstraintOperator::Equal => ordering == Ordering::Equal,
        ConstraintOperator::NotEqual => ordering != Ordering::Equal,
        ConstraintOperator::LessThan => ordering == Ordering::Less,
        ConstraintOperator::LessThanOrEqual => ordering != Ordering::Greater,
        ConstraintOperator::GreaterThan => ordering == Ordering::Greater,
        ConstraintOperator::GreaterThanOrEqual => ordering != Ordering::Less,
    }
}

const fn operator_name(operator: ConstraintOperator) -> &'static str {
    match operator {
        ConstraintOperator::Equal => "==",
        ConstraintOperator::NotEqual => "!=",
        ConstraintOperator::LessThan => "<",
        ConstraintOperator::LessThanOrEqual => "<=",
        ConstraintOperator::GreaterThan => ">",
        ConstraintOperator::GreaterThanOrEqual => ">=",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResolver {
        resource: ResourceValueResolution,
    }

    impl ResourceConstraintResolver for TestResolver {
        fn resource_value(
            &self,
            _kind: &ResourceKind,
        ) -> Result<ResourceValueResolution, String> {
            Ok(self.resource.clone())
        }

        fn expression_value(
            &self,
            _expression: NodeId,
        ) -> Result<ExpressionValueResolution, String> {
            Ok(ExpressionValueResolution::Symbolic)
        }
    }

    fn kind() -> ResourceKind {
        ResourceKind::new("test", "resource")
            .expect("valid resource-kind identity")
    }

    fn known(value: &str) -> ResolvedResourceValue {
        ResolvedResourceValue::from_decimal(value, None)
            .expect("valid exact quantity")
    }

    fn resolver(value: &str) -> TestResolver {
        TestResolver {
            resource: ResourceValueResolution::Known(known(value)),
        }
    }

    fn constraint(
        operator: ConstraintOperator,
        operand: &str,
        intent: ResourceIntent,
    ) -> ResourceConstraint {
        ResourceConstraint::new(
            kind(),
            operator,
            ConstraintOperand::Known(
                ExactQuantity::new(operand).expect("valid operand"),
            ),
            None,
            intent,
            None,
        )
        .expect("valid constraint")
    }

    #[test]
    fn exact_decimal_comparison_does_not_need_machine_integer_conversion() {
        let evaluator = ResourceConstraintEvaluator::new();
        let condition = constraint(
            ConstraintOperator::GreaterThanOrEqual,
            "999999999999999999999999999999999999999999",
            ResourceIntent::Required,
        );

        let actual = resolver(
            "1000000000000000000000000000000000000000000",
        );

        let result = evaluator.evaluate_constraint(&condition, &actual);

        assert_eq!(result.outcome(), ConstraintOutcome::Satisfied);
    }

    #[test]
    fn false_binding_constraint_is_a_blocking_issue() {
        let evaluator = ResourceConstraintEvaluator::new();
        let condition = constraint(
            ConstraintOperator::GreaterThanOrEqual,
            "100",
            ResourceIntent::Required,
        );

        let result = evaluator.evaluate_constraint(&condition, &resolver("12"));

        assert_eq!(result.outcome(), ConstraintOutcome::Violated);
        assert!(result.blocks_acceptance());
    }

    #[test]
    fn false_preference_is_advisory() {
        let evaluator = ResourceConstraintEvaluator::new();
        let condition = constraint(
            ConstraintOperator::GreaterThanOrEqual,
            "100",
            ResourceIntent::Preferred,
        );

        let result = evaluator.evaluate_constraint(&condition, &resolver("12"));

        assert_eq!(result.outcome(), ConstraintOutcome::Violated);
        assert!(!result.blocks_acceptance());
    }

    #[test]
    fn unknown_resource_value_is_deferred_not_satisfied() {
        let evaluator = ResourceConstraintEvaluator::new();
        let condition = constraint(
            ConstraintOperator::GreaterThanOrEqual,
            "1",
            ResourceIntent::Required,
        );
        let unknown = TestResolver {
            resource: ResourceValueResolution::Unavailable,
        };

        let result = evaluator.evaluate_constraint(&condition, &unknown);

        assert_eq!(result.outcome(), ConstraintOutcome::Deferred);
        assert!(result.needs_more_evidence());
        assert!(!result.blocks_acceptance());
    }

    #[test]
    fn different_units_are_deferred_without_a_conversion_service() {
        let evaluator = ResourceConstraintEvaluator::new();
        let left_unit = ResourceUnit::new("length", "meter")
            .expect("valid unit");
        let right_unit = ResourceUnit::new("length", "centimeter")
            .expect("valid unit");

        let actual = TestResolver {
            resource: ResourceValueResolution::Known(
                ResolvedResourceValue::from_decimal(
                    "2",
                    Some(left_unit),
                )
                .expect("valid resource value"),
            ),
        };

        let condition = ResourceConstraint::new(
            kind(),
            ConstraintOperator::GreaterThan,
            ConstraintOperand::Known(
                ExactQuantity::new("100").expect("valid quantity"),
            ),
            Some(right_unit),
            ResourceIntent::Required,
            None,
        )
        .expect("valid constraint");

        let result = evaluator.evaluate_constraint(&condition, &actual);

        assert_eq!(result.outcome(), ConstraintOutcome::Deferred);
    }

    #[test]
    fn batch_report_preserves_input_order_and_unresolved_state() {
        let evaluator = ResourceConstraintEvaluator::new();
        let conditions = vec![
            constraint(
                ConstraintOperator::GreaterThan,
                "1",
                ResourceIntent::Required,
            ),
            constraint(
                ConstraintOperator::LessThan,
                "100",
                ResourceIntent::Required,
            ),
        ];

        let report = evaluator.evaluate_constraints(&conditions, &resolver("5"));

        assert_eq!(report.len(), 2);
        assert_eq!(
            report.assessments()[0].outcome(),
            ConstraintOutcome::Satisfied
        );
        assert_eq!(
            report.assessments()[1].outcome(),
            ConstraintOutcome::Satisfied
        );
        assert!(report.is_complete());
    }

    #[test]
    fn no_input_conditions_produce_an_empty_complete_report() {
        let evaluator = ResourceConstraintEvaluator::new();
        let report = evaluator.evaluate_constraints(&[], &resolver("5"));

        assert!(report.is_empty());
        assert!(report.is_complete());
        assert!(!report.has_blocking_issues());
    }
}