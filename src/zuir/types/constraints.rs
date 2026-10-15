#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR symbolic constraints
//!
//! Path: `src/zuir/types/constraints.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provide a target-independent representation and evaluation interface for
//! symbolic constraints used by semantic types, generics, tensor dimensions,
//! integer widths, hardware descriptions, quantum programs, and extensions.
//!
//! OWNS
//!
//! - Constraint identities within a ConstraintSet.
//! - Constraint expressions and their validation.
//! - Constraint collection, composition, and evaluation outcomes.
//! - An extensible evaluator interface.
//! - A deterministic evaluator for explicitly supplied literal facts.
//! - Conversion that preserves the meaning of existing TypeConstraint values.
//!
//! DOES_NOT_OWN
//!
//! - Canonical TypeId allocation, type registration, or type interning.
//! - Type definitions or type compatibility.
//! - Source-language expression parsing.
//! - Generic inference or specialization policy.
//! - Physical type layout, ABI, allocation, or device discovery.
//! - Target feasibility or execution authorization.
//! - Dialect registration or plugin loading.
//! - Whole-program verification.
//!
//! INTEGRATION
//!
//! - Uses TypeConstraint, SymbolicExpression, TypeRef, and Dimension from
//!   `types/type_def.rs`.
//! - Uses QualifiedName from `schema/types.rs`.
//! - `types/type_def.rs` remains the owner of semantic type declarations.
//! - `types/layout.rs` preserves symbolic dimensions and does not solve them.
//! - `types/compatibility.rs` owns equivalence and conversion legality.
//! - The semantic verifier decides which constraints must be discharged.
//! - A dialect registry or caller supplies evaluators for extension constraints.
//! - Target-specific stages evaluate resource and capability obligations.
//!
//! SCALABILITY
//!
//! No universal maximum is imposed on tensor rank, integer width, qubit
//! count, hardware signal width, aggregate size, or resource dimensions.
//! Decimal integer facts are represented as strings to avoid host-width
//! truncation. Collections grow according to available resources.
//!
//! The implementation does not promise literal infinite computation.
//! Memory, time, recursion, and all concrete executions remain finite.
//! Operational budgets belong to the caller and must produce explicit
//! incomplete results rather than false success.
//!
//! SEMANTIC RULE
//!
//! Unknown, unsupported, and incomplete are not equivalent to satisfied.
//! An evaluator must not claim that an opaque expression is true unless it
//! has sufficient semantic information to establish that result.
//!
//! FREEZE CRITERIA
//!
//! - Existing TypeConstraint variants are preserved without reinterpretation.
//! - Unknown extensions remain explicit.
//! - Evaluator outcomes distinguish uncertainty from failure.
//! - Numeric comparisons avoid fixed-width overflow.
//! - No physical target assumptions are introduced.
//! - Tests cover parsing, evaluation, aggregation, and invalid inputs.

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::{
    Dimension, SymbolicExpression, TypeConstraint, TypeRef,
};

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// A stable name for a symbolic variable.
///
/// This is an identifier within a constraint environment, not a type ID,
/// symbol-table entry, or proof of source-level name resolution.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConstraintVariable(String);

impl ConstraintVariable {
    /// Construct a non-empty variable name.
    ///
    /// This checks only the local representation invariant. Source-language
    /// name resolution and namespace authorization belong elsewhere.
    pub fn new(name: impl Into<String>) -> Result<Self, ConstraintError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ConstraintError::EmptyVariableName);
        }

        if name.chars().any(char::is_control) {
            return Err(ConstraintError::ControlCharacterInName);
        }

        Ok(Self(name))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConstraintVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A non-negative arbitrary-width integer represented in canonical decimal.
///
/// This avoids silently truncating symbolic widths and resource quantities
/// to the host's integer width. The representation is finite and must still
/// fit in available memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Natural(String);

impl Natural {
    /// Parse a non-negative decimal integer.
    ///
    /// Leading zeroes are normalized. Signs, separators, and non-decimal
    /// characters are rejected.
    pub fn parse(value: &str) -> Result<Self, ConstraintError> {
        if value.is_empty() {
            return Err(ConstraintError::InvalidNatural(value.to_owned()));
        }

        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(ConstraintError::InvalidNatural(value.to_owned()));
        }

        let normalized = value.trim_start_matches('0');

        Ok(Self(if normalized.is_empty() {
            "0".to_owned()
        } else {
            normalized.to_owned()
        }))
    }

    #[must_use]
    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    #[must_use]
    pub fn one() -> Self {
        Self("1".to_owned())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compare values without converting them to a fixed-width integer.
    #[must_use]
    pub fn compare(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }

    /// Validate values that may have been deserialized without parsing.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        let parsed = Self::parse(&self.0)?;

        if parsed.0 != self.0 {
            return Err(ConstraintError::NonCanonicalNatural(
                self.0.clone(),
            ));
        }

        Ok(())
    }
}

impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

/// A symbolic expression.
///
/// `Source` preserves the expression text supplied by the existing semantic
/// type model. It is intentionally not parsed or executed by this module.
/// `Literal` provides a small, explicitly defined, portable expression form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConstraintExpression {
    /// An opaque expression owned by another semantic subsystem.
    Source(SymbolicExpression),

    /// An arbitrary-width non-negative integer literal.
    Integer(Natural),

    /// A Boolean literal.
    Boolean(bool),

    /// A symbolic variable resolved through an evaluation environment.
    Variable(ConstraintVariable),
}

impl ConstraintExpression {
    /// Construct an opaque expression after checking its local invariants.
    pub fn source(
        expression: SymbolicExpression,
    ) -> Result<Self, ConstraintError> {
        if expression.expression.trim().is_empty() {
            return Err(ConstraintError::EmptyExpression);
        }

        Ok(Self::Source(expression))
    }

    /// Construct an integer literal without host-width truncation.
    pub fn integer(value: &str) -> Result<Self, ConstraintError> {
        Ok(Self::Integer(Natural::parse(value)?))
    }

    /// Construct a symbolic variable reference.
    pub fn variable(
        name: impl Into<String>,
    ) -> Result<Self, ConstraintError> {
        Ok(Self::Variable(ConstraintVariable::new(name)?))
    }

    pub fn validate(&self) -> Result<(), ConstraintError> {
        match self {
            Self::Source(source) if source.expression.trim().is_empty() => {
                Err(ConstraintError::EmptyExpression)
            }
            Self::Source(_) | Self::Boolean(_) => Ok(()),
            Self::Integer(value) => value.validate(),
            Self::Variable(variable) => {
                ConstraintVariable::new(variable.0.clone()).map(|_| ())
            }
        }
    }
}

/// An explicitly represented relation between expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

/// A constraint that can be stored and passed between compiler stages.
///
/// Existing type constraints are preserved as declarations. A consumer must
/// use an evaluator that understands their semantics before claiming that
/// they have been satisfied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Constraint {
    /// Preserve an existing constraint from the semantic type system.
    DeclaredTypeConstraint {
        constraint: TypeConstraint,
    },

    /// Compare two expressions using a defined relation.
    Compare {
        left: ConstraintExpression,
        relation: Relation,
        right: ConstraintExpression,
    },

    /// Require a symbolic expression to evaluate to true.
    Predicate {
        expression: ConstraintExpression,
    },

    /// A requirement that a type implements a named semantic contract.
    Implements {
        ty: TypeRef,
        contract: QualifiedName,
    },

    /// An extension-defined constraint.
    ///
    /// The namespace and constraint name identify its meaning. They do not
    /// automatically provide an implementation or make it executable.
    Extension {
        namespace: QualifiedName,
        name: QualifiedName,
        arguments: Vec<ConstraintExpression>,
    },
}

impl Constraint {
    /// Validate the locally enforceable parts of a constraint.
    ///
    /// This does not resolve names, prove type equality, or evaluate opaque
    /// expressions.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        match self {
            Self::DeclaredTypeConstraint { constraint } => {
                validate_type_constraint(constraint)
            }

            Self::Compare { left, right, .. } => {
                left.validate()?;
                right.validate()
            }

            Self::Predicate { expression } => expression.validate(),

            Self::Implements { contract, .. } => contract
                .validate()
                .map_err(|error| {
                    ConstraintError::InvalidQualifiedName(error.to_string())
                }),

            Self::Extension {
                namespace,
                name,
                arguments,
            } => {
                namespace.validate().map_err(|error| {
                    ConstraintError::InvalidQualifiedName(error.to_string())
                })?;

                name.validate().map_err(|error| {
                    ConstraintError::InvalidQualifiedName(error.to_string())
                })?;

                for argument in arguments {
                    argument.validate()?;
                }

                Ok(())
            }
        }
    }
}

/// Validate an existing type constraint without assigning new semantics.
fn validate_type_constraint(
    constraint: &TypeConstraint,
) -> Result<(), ConstraintError> {
    match constraint {
        TypeConstraint::Implements(name) => validate_name(name),

        TypeConstraint::EqualTo(TypeRef::Named(name)) => validate_name(name),

        TypeConstraint::EqualTo(TypeRef::Inline(_)) => Ok(()),

        TypeConstraint::IntegerWidthAtLeast(expression)
        | TypeConstraint::Shape(expression) => {
            ConstraintExpression::source(expression.clone()).map(|_| ())
        }

        TypeConstraint::Dialect {
            namespace,
            constraint,
        } => {
            validate_name(namespace)?;
            validate_name(constraint)
        }
    }
}

fn validate_name(name: &QualifiedName) -> Result<(), ConstraintError> {
    name.validate()
        .map_err(|error| ConstraintError::InvalidQualifiedName(error.to_string()))
}

/// Convert an existing type constraint into the extensible constraint model.
///
/// The conversion preserves the original constraint instead of guessing what
/// an expression means or deciding whether a dialect constraint is true.
impl From<TypeConstraint> for Constraint {
    fn from(constraint: TypeConstraint) -> Self {
        Self::DeclaredTypeConstraint { constraint }
    }
}

/// Constraint-evaluation result.
///
/// `Unknown` means the available facts were insufficient.
/// `Unsupported` means no evaluator can interpret the constraint.
/// `Incomplete` means evaluation could not finish under the applicable
/// operational policy. None of these outcomes proves satisfaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "detail", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConstraintOutcome {
    Satisfied,
    Violated {
        explanation: String,
    },
    Unknown {
        explanation: String,
    },
    Unsupported {
        explanation: String,
    },
    Incomplete {
        explanation: String,
    },
}

impl ConstraintOutcome {
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        matches!(self, Self::Satisfied)
    }

    #[must_use]
    pub fn is_decisive(&self) -> bool {
        matches!(self, Self::Satisfied | Self::Violated { .. })
    }
}

/// Facts supplied to constraint evaluators.
///
/// These are compile-time facts, not physical allocations. The environment
/// does not discover hardware or infer values that have not been supplied.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintEnvironment {
    integers: BTreeMap<ConstraintVariable, Natural>,
    booleans: BTreeMap<ConstraintVariable, bool>,
}

impl ConstraintEnvironment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_integer(
        &mut self,
        variable: ConstraintVariable,
        value: Natural,
    ) -> Result<Option<Natural>, ConstraintError> {
        value.validate()?;
        Ok(self.integers.insert(variable, value))
    }

    pub fn bind_boolean(
        &mut self,
        variable: ConstraintVariable,
        value: bool,
    ) -> Option<bool> {
        self.booleans.insert(variable, value)
    }

    #[must_use]
    pub fn integer(
        &self,
        variable: &ConstraintVariable,
    ) -> Option<&Natural> {
        self.integers.get(variable)
    }

    #[must_use]
    pub fn boolean(
        &self,
        variable: &ConstraintVariable,
    ) -> Option<bool> {
        self.booleans.get(variable).copied()
    }
}

/// An evaluator that understands a defined subset of constraint semantics.
///
/// Implementations must be deterministic for deterministic inputs when their
/// contract declares deterministic behavior. They must not convert an
/// unsupported or unknown constraint into `Satisfied`.
pub trait ConstraintEvaluator {
    fn evaluate(
        &self,
        constraint: &Constraint,
        environment: &ConstraintEnvironment,
    ) -> ConstraintOutcome;
}

/// Evaluates literal comparisons and Boolean facts.
///
/// This evaluator intentionally does not parse arbitrary expression strings,
/// resolve types, infer generic arguments, or interpret dialect constraints.
/// Such tasks belong to specialized evaluators.
#[derive(Debug, Default, Clone, Copy)]
pub struct LiteralConstraintEvaluator;

impl ConstraintEvaluator for LiteralConstraintEvaluator {
    fn evaluate(
        &self,
        constraint: &Constraint,
        environment: &ConstraintEnvironment,
    ) -> ConstraintOutcome {
        if let Err(error) = constraint.validate() {
            return ConstraintOutcome::Violated {
                explanation: error.to_string(),
            };
        }

        match constraint {
            Constraint::Compare {
                left,
                relation,
                right,
            } => {
                let left = evaluate_expression(left, environment);
                let right = evaluate_expression(right, environment);

                match (left, right) {
                    (EvaluatedValue::Integer(left), EvaluatedValue::Integer(right)) => {
                        compare_values(left.cmp(&right), *relation)
                    }
                    (EvaluatedValue::Boolean(left), EvaluatedValue::Boolean(right)) => {
                        if *relation == Relation::Equal {
                            boolean_outcome(left == right)
                        } else if *relation == Relation::NotEqual {
                            boolean_outcome(left != right)
                        } else {
                            ConstraintOutcome::Unsupported {
                                explanation:
                                    "ordered comparisons are not defined for Boolean values"
                                        .to_owned(),
                            }
                        }
                    }
                    (EvaluatedValue::Unknown(reason), _)
                    | (_, EvaluatedValue::Unknown(reason)) => {
                        ConstraintOutcome::Unknown {
                            explanation: reason,
                        }
                    }
                    _ => ConstraintOutcome::Unsupported {
                        explanation:
                            "comparison operands have incompatible value categories"
                                .to_owned(),
                    },
                }
            }

            Constraint::Predicate { expression } => {
                match evaluate_expression(expression, environment) {
                    EvaluatedValue::Boolean(value) => boolean_outcome(value),
                    EvaluatedValue::Unknown(reason) => {
                        ConstraintOutcome::Unknown {
                            explanation: reason,
                        }
                    }
                    EvaluatedValue::Integer(_) => ConstraintOutcome::Unsupported {
                        explanation:
                            "a predicate must evaluate to a Boolean value".to_owned(),
                    },
                }
            }

            Constraint::DeclaredTypeConstraint { .. } => {
                ConstraintOutcome::Unsupported {
                    explanation:
                        "the literal evaluator does not interpret semantic type constraints"
                            .to_owned(),
                }
            }

            Constraint::Implements { .. } => ConstraintOutcome::Unsupported {
                explanation:
                    "type-contract satisfaction requires the semantic type registry"
                        .to_owned(),
            },

            Constraint::Extension { namespace, name, .. } => {
                ConstraintOutcome::Unsupported {
                    explanation: format!(
                        "no evaluator registered for extension constraint {namespace}::{name}"
                    ),
                }
            }
        }
    }
}

enum EvaluatedValue {
    Integer(Natural),
    Boolean(bool),
    Unknown(String),
}

fn evaluate_expression(
    expression: &ConstraintExpression,
    environment: &ConstraintEnvironment,
) -> EvaluatedValue {
    match expression {
        ConstraintExpression::Integer(value) => {
            EvaluatedValue::Integer(value.clone())
        }
        ConstraintExpression::Boolean(value) => {
            EvaluatedValue::Boolean(*value)
        }
        ConstraintExpression::Variable(variable) => {
            if let Some(value) = environment.integer(variable) {
                EvaluatedValue::Integer(value.clone())
            } else if let Some(value) = environment.boolean(variable) {
                EvaluatedValue::Boolean(value)
            } else {
                EvaluatedValue::Unknown(format!(
                    "no value is bound for variable `{variable}`"
                ))
            }
        }
        ConstraintExpression::Source(source) => {
            EvaluatedValue::Unknown(format!(
                "expression `{}` requires its owning semantic evaluator",
                source.expression
            ))
        }
    }
}

fn compare_values(
    ordering: Ordering,
    relation: Relation,
) -> ConstraintOutcome {
    let satisfied = match relation {
        Relation::Equal => ordering == Ordering::Equal,
        Relation::NotEqual => ordering != Ordering::Equal,
        Relation::LessThan => ordering == Ordering::Less,
        Relation::LessThanOrEqual => ordering != Ordering::Greater,
        Relation::GreaterThan => ordering == Ordering::Greater,
        Relation::GreaterThanOrEqual => ordering != Ordering::Less,
    };

    boolean_outcome(satisfied)
}

fn boolean_outcome(value: bool) -> ConstraintOutcome {
    if value {
        ConstraintOutcome::Satisfied
    } else {
        ConstraintOutcome::Violated {
            explanation: "constraint evaluated to false".to_owned(),
        }
    }
}

/// The aggregate outcome of evaluating a set of constraints.
///
/// Any violation makes the set violated. Otherwise, any incomplete evaluation
/// takes precedence over unsupported and unknown evaluations. The set is
/// satisfied only when every member is satisfied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "details", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConstraintSetOutcome {
    Satisfied,
    Violated {
        failures: Vec<ConstraintOutcome>,
    },
    Unknown {
        details: Vec<ConstraintOutcome>,
    },
    Unsupported {
        details: Vec<ConstraintOutcome>,
    },
    Incomplete {
        details: Vec<ConstraintOutcome>,
    },
}

/// A dynamically sized collection of constraints.
///
/// The insertion order is preserved for diagnostics. No fixed constraint
/// count is encoded in this type.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintSet {
    constraints: Vec<Constraint>,
}

impl ConstraintSet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_constraints(
        constraints: Vec<Constraint>,
    ) -> Result<Self, ConstraintError> {
        let set = Self { constraints };
        set.validate()?;
        Ok(set)
    }

    pub fn push(
        &mut self,
        constraint: Constraint,
    ) -> Result<(), ConstraintError> {
        constraint.validate()?;
        self.constraints.push(constraint);
        Ok(())
    }

    pub fn extend<I>(
        &mut self,
        constraints: I,
    ) -> Result<(), ConstraintError>
    where
        I: IntoIterator<Item = Constraint>,
    {
        // Validate all incoming items before mutating the existing set.
        let incoming: Vec<Constraint> = constraints.into_iter().collect();

        for constraint in &incoming {
            constraint.validate()?;
        }

        self.constraints.extend(incoming);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ConstraintError> {
        for constraint in &self.constraints {
            constraint.validate()?;
        }

        Ok(())
    }

    #[must_use]
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Evaluate all constraints without short-circuiting.
    ///
    /// Evaluating every item produces more useful diagnostics and preserves
    /// deterministic result ordering. The caller remains responsible for
    /// applying any operational work budget before evaluation.
    pub fn evaluate<E: ConstraintEvaluator>(
        &self,
        evaluator: &E,
        environment: &ConstraintEnvironment,
    ) -> ConstraintSetOutcome {
        let outcomes: Vec<ConstraintOutcome> = self
            .constraints
            .iter()
            .map(|constraint| evaluator.evaluate(constraint, environment))
            .collect();

        aggregate_outcomes(outcomes)
    }
}

fn aggregate_outcomes(
    outcomes: Vec<ConstraintOutcome>,
) -> ConstraintSetOutcome {
    if outcomes.iter().any(|outcome| {
        matches!(outcome, ConstraintOutcome::Violated { .. })
    }) {
        return ConstraintSetOutcome::Violated {
            failures: outcomes
                .into_iter()
                .filter(|outcome| {
                    matches!(outcome, ConstraintOutcome::Violated { .. })
                })
                .collect(),
        };
    }

    if outcomes.iter().any(|outcome| {
        matches!(outcome, ConstraintOutcome::Incomplete { .. })
    }) {
        return ConstraintSetOutcome::Incomplete {
            details: outcomes
                .into_iter()
                .filter(|outcome| {
                    !matches!(outcome, ConstraintOutcome::Satisfied)
                })
                .collect(),
        };
    }

    if outcomes.iter().any(|outcome| {
        matches!(outcome, ConstraintOutcome::Unsupported { .. })
    }) {
        return ConstraintSetOutcome::Unsupported {
            details: outcomes
                .into_iter()
                .filter(|outcome| {
                    !matches!(outcome, ConstraintOutcome::Satisfied)
                })
                .collect(),
        };
    }

    if outcomes.iter().any(|outcome| {
        matches!(outcome, ConstraintOutcome::Unknown { .. })
    }) {
        return ConstraintSetOutcome::Unknown {
            details: outcomes
                .into_iter()
                .filter(|outcome| {
                    !matches!(outcome, ConstraintOutcome::Satisfied)
                })
                .collect(),
        };
    }

    ConstraintSetOutcome::Satisfied
}

/// Errors caused by invalid local constraint representations.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ConstraintError {
    #[error("constraint variable name cannot be empty")]
    EmptyVariableName,

    #[error("constraint name contains a control character")]
    ControlCharacterInName,

    #[error("invalid non-negative decimal integer: `{0}`")]
    InvalidNatural(String),

    #[error("non-canonical non-negative decimal integer: `{0}`")]
    NonCanonicalNatural(String),

    #[error("symbolic expression cannot be empty")]
    EmptyExpression,

    #[error("invalid qualified name: {0}")]
    InvalidQualifiedName(String),
}

/// Preserve a semantic dimension as a constraint expression when it is
/// useful to downstream analyses.
///
/// A dynamic dimension stays dynamic: it is not silently changed into an
/// arbitrary constant or treated as an unbounded allocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DimensionConstraint {
    Known(Natural),
    Symbolic(ConstraintExpression),
    Dynamic,
}

impl TryFrom<&Dimension> for DimensionConstraint {
    type Error = ConstraintError;

    fn try_from(dimension: &Dimension) -> Result<Self, Self::Error> {
        match dimension {
            Dimension::Known(value) => {
                Ok(Self::Known(Natural::parse(&value.to_string())?))
            }
            Dimension::Symbolic(expression) => Ok(Self::Symbolic(
                ConstraintExpression::source(expression.clone())?,
            )),
            Dimension::Dynamic => Ok(Self::Dynamic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::types::type_def::TypeConstraint;

    fn variable(name: &str) -> ConstraintVariable {
        ConstraintVariable::new(name).expect("valid variable")
    }

    fn integer(value: &str) -> ConstraintExpression {
        ConstraintExpression::integer(value).expect("valid integer")
    }

    #[test]
    fn arbitrary_width_integer_comparison_does_not_truncate() {
        let left = Natural::parse(
            "340282366920938463463374607431768211456",
        )
        .expect("large integer");

        let right = Natural::parse(
            "340282366920938463463374607431768211455",
        )
        .expect("large integer");

        assert_eq!(left.compare(&right), Ordering::Greater);
    }

    #[test]
    fn natural_normalizes_leading_zeroes() {
        let value = Natural::parse("00000042").expect("integer");

        assert_eq!(value.as_str(), "42");
    }

    #[test]
    fn rejects_signed_and_non_decimal_naturals() {
        assert!(Natural::parse("-1").is_err());
        assert!(Natural::parse("+1").is_err());
        assert!(Natural::parse("1_000").is_err());
        assert!(Natural::parse("").is_err());
    }

    #[test]
    fn evaluates_literal_integer_relations() {
        let evaluator = LiteralConstraintEvaluator;
        let environment = ConstraintEnvironment::new();

        let constraint = Constraint::Compare {
            left: integer("100000000000000000000000000000000000000"),
            relation: Relation::GreaterThan,
            right: integer("99999999999999999999999999999999999999"),
        };

        assert_eq!(
            evaluator.evaluate(&constraint, &environment),
            ConstraintOutcome::Satisfied
        );
    }

    #[test]
    fn evaluates_symbolic_integer_relations_from_supplied_facts() {
        let evaluator = LiteralConstraintEvaluator;
        let mut environment = ConstraintEnvironment::new();

        environment
            .bind_integer(
                variable("tensor_width"),
                Natural::parse("4096").expect("integer"),
            )
            .expect("bind");

        let constraint = Constraint::Compare {
            left: ConstraintExpression::variable("tensor_width")
                .expect("variable"),
            relation: Relation::GreaterThanOrEqual,
            right: integer("1024"),
        };

        assert_eq!(
            evaluator.evaluate(&constraint, &environment),
            ConstraintOutcome::Satisfied
        );
    }

    #[test]
    fn missing_facts_produce_unknown_not_satisfied() {
        let evaluator = LiteralConstraintEvaluator;
        let environment = ConstraintEnvironment::new();

        let constraint = Constraint::Predicate {
            expression: ConstraintExpression::variable("required_width")
                .expect("variable"),
        };

        assert!(matches!(
            evaluator.evaluate(&constraint, &environment),
            ConstraintOutcome::Unknown { .. }
        ));
    }

    #[test]
    fn opaque_expressions_are_not_guessed() {
        let evaluator = LiteralConstraintEvaluator;
        let environment = ConstraintEnvironment::new();

        let constraint = Constraint::Predicate {
            expression: ConstraintExpression::source(
                SymbolicExpression {
                    expression: "required_qubits * 2".to_owned(),
                },
            )
            .expect("expression"),
        };

        assert!(matches!(
            evaluator.evaluate(&constraint, &environment),
            ConstraintOutcome::Unknown { .. }
        ));
    }

    #[test]
    fn unknown_extension_constraints_are_not_satisfied() {
        let evaluator = LiteralConstraintEvaluator;
        let environment = ConstraintEnvironment::new();

        let constraint = Constraint::Extension {
            namespace: QualifiedName::parse("future::domain")
                .expect("namespace"),
            name: QualifiedName::parse("requires_topology")
                .expect("constraint name"),
            arguments: Vec::new(),
        };

        assert!(matches!(
            evaluator.evaluate(&constraint, &environment),
            ConstraintOutcome::Unsupported { .. }
        ));
    }

    #[test]
    fn preserves_existing_type_constraints() {
        let declared = TypeConstraint::IntegerWidthAtLeast(
            SymbolicExpression {
                expression: "minimum_width".to_owned(),
            },
        );

        let constraint = Constraint::from(declared.clone());

        assert_eq!(
            constraint,
            Constraint::DeclaredTypeConstraint {
                constraint: declared,
            }
        );
    }

    #[test]
    fn an_unsatisfied_constraint_violates_the_set() {
        let mut set = ConstraintSet::new();

        set.push(Constraint::Compare {
            left: integer("2"),
            relation: Relation::GreaterThan,
            right: integer("10"),
        })
        .expect("valid constraint");

        let result = set.evaluate(
            &LiteralConstraintEvaluator,
            &ConstraintEnvironment::new(),
        );

        assert!(matches!(
            result,
            ConstraintSetOutcome::Violated { .. }
        ));
    }

    #[test]
    fn an_unknown_constraint_prevents_set_satisfaction() {
        let mut set = ConstraintSet::new();

        set.push(Constraint::Predicate {
            expression: ConstraintExpression::variable("unresolved")
                .expect("variable"),
        })
        .expect("valid constraint");

        let result = set.evaluate(
            &LiteralConstraintEvaluator,
            &ConstraintEnvironment::new(),
        );

        assert!(matches!(
            result,
            ConstraintSetOutcome::Unknown { .. }
        ));
    }

    #[test]
    fn empty_constraint_set_is_satisfied() {
        let result = ConstraintSet::new().evaluate(
            &LiteralConstraintEvaluator,
            &ConstraintEnvironment::new(),
        );

        assert_eq!(result, ConstraintSetOutcome::Satisfied);
    }

    #[test]
    fn dimension_conversion_preserves_dynamic_semantics() {
        let converted = DimensionConstraint::try_from(&Dimension::Dynamic)
            .expect("dynamic dimension");

        assert_eq!(converted, DimensionConstraint::Dynamic);
    }

    #[test]
    fn constraint_set_extension_is_atomic_on_validation_failure() {
        let mut set = ConstraintSet::new();

        set.push(Constraint::Predicate {
            expression: ConstraintExpression::Boolean(true),
        })
        .expect("valid constraint");

        // Construct malformed serialized data to exercise validation.
        let invalid: Constraint = serde_json::from_str(
            r#"{"kind":"predicate","expression":{"kind":"source","value":{"expression":""}}}"#,
        )
        .expect("deserialize representation");

        assert!(set.extend(vec![invalid]).is_err());
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn environment_keeps_integer_and_boolean_names_separate() {
        let mut environment = ConstraintEnvironment::new();
        let key = variable("flag");

        environment.bind_boolean(key.clone(), true);

        assert_eq!(environment.boolean(&key), Some(true));
        assert_eq!(environment.integer(&key), None);
    }
}