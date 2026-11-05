
#![forbid(unsafe_code)]

//! Zamani Universal Meta-Compiler — Compilation Policy Contracts.
//!
//! File:
//!     src/compiler/configuration/policy.rs
//!
//! Purpose:
//!     Define the compiler-side representation, validation, classification,
//!     and evaluation boundary for compilation policies.
//!
//! Rust:
//!     Edition 2021.
//!     Minimum version 1.97.
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! Architectural ownership:
//!
//!     grammar/policies/
//!         Owns policy syntax.
//!
//!     semantic policy subsystem
//!         Owns source-level policy meaning, name resolution, semantic
//!         validation, policy composition, authorization, and enforcement
//!         semantics.
//!
//!     compiler::configuration::policy
//!         Owns compiler-side policy contracts, structural validation,
//!         policy intent classification, and the expression-evaluation
//!         boundary used by compiler planning.
//!
//!     compiler::configuration::budget
//!         Owns resource-budget definitions and accounting.
//!
//!     compiler::configuration::target
//!         Owns target descriptors and target-specific configuration.
//!
//!     compiler::configuration::config
//!         Owns construction and validation of the existing CompilerConfig.
//!
//!     compiler::planning
//!         Consumes resolved policy information when producing realization
//!         plans.
//!
//!     runtime / security
//!         Own actual runtime policy enforcement and authorization.
//!
//! This module deliberately does not create a competing semantic policy
//! engine, resource model, capability registry, target registry, security
//! subsystem, or runtime.
//!
//! POCO-REAF:
//!     Policy must govern permitted realizations without making the semantic
//!     meaning of a program dependent on one physical machine.
//!
//! Scalability:
//!     No arbitrary maximum is imposed on policies, members, rules, clauses,
//!     properties, identifiers, or expression text. Actual memory and
//!     execution limits are implementation/resource concerns.
//!
//! Determinism:
//!     Members, rules, clauses, and evaluation results preserve declaration
//!     order. This module does not use nondeterministic iteration to determine
//!     policy precedence.
//!
//! Expression ownership:
//!     Expressions are retained as opaque source-level expressions. This
//!     module does not implement a second Zamani expression parser.
//!
//! Integration:
//!     The evaluator is generic over its context. Resource, capability,
//!     semantic, target, and security information can be supplied by their
//!     authoritative subsystems without making this module own those models.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

// ============================================================================
// VERSION
// ============================================================================

/// Version of this compiler-side policy contract.
///
/// This is not the Zamani language version, grammar version, semantic-model
/// version, compiler version, or artifact version.
pub const POLICY_API_VERSION: u16 = 1;

// ============================================================================
// IDENTIFIERS AND EXPRESSIONS
// ============================================================================

/// A qualified policy identifier.
///
/// Examples:
///
///     execution::portable
///     deployment::production
///     quantum::fault_tolerant
///     organization::security::strict
///
/// Identifiers are open-world. This type does not enumerate policy names or
/// computational domains.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

impl PolicyId {
    /// Creates a policy identifier.
    ///
    /// Empty or whitespace-only identifiers are rejected.
    /// No arbitrary identifier-length or namespace-depth ceiling is imposed.
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PolicyValidationError::EmptyIdentifier);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its text.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An opaque Zamani policy expression.
///
/// Expression syntax and meaning belong to the canonical grammar and
/// semantic-expression subsystems. This type preserves the expression for
/// downstream evaluation without interpreting or rewriting it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolicyExpression(String);

impl PolicyExpression {
    /// Creates an opaque policy expression.
    ///
    /// Whitespace-only expressions are rejected.
    /// The original expression text is preserved.
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PolicyValidationError::EmptyExpression);
        }

        Ok(Self(value))
    }

    /// Returns the original expression text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the expression and returns its original text.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyExpression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A policy property key.
///
/// Property names are open-world. A new policy property must not require
/// adding a new Rust enum variant.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyPropertyKey(String);

impl PolicyPropertyKey {
    /// Creates a property key.
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PolicyValidationError::EmptyPropertyKey);
        }

        Ok(Self(value))
    }

    /// Returns the property key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyPropertyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// POLICY CLASSIFICATION
// ============================================================================

/// The broad role played by a policy directive.
///
/// Classification does not execute or authorize the directive. It allows
/// downstream semantic, planning, security, and runtime systems to handle
/// different kinds of policy intent without conflating their meanings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyClass {
    /// Mandatory program requirements, constraints, capability demands,
    /// and resource demands.
    MandatoryIntent,

    /// Advisory optimization preferences.
    AdvisoryIntent,

    /// Permission and prohibition declarations.
    ///
    /// These are policy declarations, not proof of security authorization.
    AuthorizationIntent,

    /// Selection, negotiation, fallback, recovery, retry, escalation,
    /// simulation, and adaptation declarations.
    RealizationIntent,

    /// Determinism, reproducibility, effects, provenance, and contract
    /// declarations.
    SemanticContractIntent,

    /// Open-ended policy metadata.
    MetadataIntent,
}

/// Indicates whether a directive expresses a mandatory condition or
/// advisory intent.
///
/// This is deliberately not a complete feasibility result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyStrength {
    /// Must be satisfied by any permitted realization.
    Mandatory,

    /// Desirable, but cannot override mandatory requirements or constraints.
    Advisory,

    /// Describes governing or authorization intent that requires evaluation
    /// by the appropriate authoritative subsystem.
    Governing,

    /// Provides metadata or contextual information.
    Informational,
}

// ============================================================================
// DIRECTIVE KINDS
// ============================================================================

/// Domain-neutral policy directive kinds.
///
/// These correspond to the conceptual policy forms in the canonical
/// `grammar/policies/policy.g4` grammar.
///
/// The enum classifies syntax-level intent. It does not replace the semantic
/// policy model and does not enumerate hardware, vendor, device, or domain
/// capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyDirectiveKind {
    Requirement,
    Constraint,
    Capability,
    Resource,

    Permission,
    Prohibition,
    Preference,

    Fallback,
    Selection,
    Negotiation,
    Retry,
    Recovery,
    Escalation,
    Rejection,

    Simulation,
    Adaptation,
    Sandbox,

    Determinism,
    Reproducibility,

    Effect,
    Provenance,
    ContractReference,
}

impl PolicyDirectiveKind {
    /// Returns the classification of this directive.
    pub const fn class(self) -> PolicyClass {
        match self {
            Self::Requirement
            | Self::Constraint
            | Self::Capability
            | Self::Resource => PolicyClass::MandatoryIntent,

            Self::Preference => PolicyClass::AdvisoryIntent,

            Self::Permission | Self::Prohibition | Self::Sandbox => {
                PolicyClass::AuthorizationIntent
            }

            Self::Fallback
            | Self::Selection
            | Self::Negotiation
            | Self::Retry
            | Self::Recovery
            | Self::Escalation
            | Self::Rejection
            | Self::Simulation
            | Self::Adaptation => PolicyClass::RealizationIntent,

            Self::Determinism
            | Self::Reproducibility
            | Self::Effect
            | Self::Provenance
            | Self::ContractReference => PolicyClass::SemanticContractIntent,
        }
    }

    /// Returns the strength of this directive.
    pub const fn strength(self) -> PolicyStrength {
        match self {
            Self::Requirement
            | Self::Constraint
            | Self::Capability
            | Self::Resource => PolicyStrength::Mandatory,

            Self::Preference => PolicyStrength::Advisory,

            Self::Permission
            | Self::Prohibition
            | Self::Sandbox => PolicyStrength::Governing,

            Self::Fallback
            | Self::Selection
            | Self::Negotiation
            | Self::Retry
            | Self::Recovery
            | Self::Escalation
            | Self::Rejection
            | Self::Simulation
            | Self::Adaptation
            | Self::Determinism
            | Self::Reproducibility
            | Self::Effect
            | Self::Provenance
            | Self::ContractReference => PolicyStrength::Governing,
        }
    }

    /// Returns whether this directive must not be silently ignored by a
    /// conforming downstream policy resolver.
    ///
    /// This does not mean that this module performs the resolution.
    pub const fn requires_downstream_resolution(self) -> bool {
        !matches!(self, Self::Preference)
    }
}

// ============================================================================
// DIRECTIVES AND ACTIONS
// ============================================================================

/// A policy directive and its optional expression.
///
/// Determinism and reproducibility declarations may be expression-free in
/// the grammar. Other directive kinds require an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDirective {
    kind: PolicyDirectiveKind,
    expression: Option<PolicyExpression>,
}

impl PolicyDirective {
    /// Creates a directive with an expression.
    pub fn with_expression(
        kind: PolicyDirectiveKind,
        expression: PolicyExpression,
    ) -> Self {
        Self {
            kind,
            expression: Some(expression),
        }
    }

    /// Creates an expression-free determinism declaration.
    pub fn deterministic() -> Self {
        Self {
            kind: PolicyDirectiveKind::Determinism,
            expression: None,
        }
    }

    /// Creates an expression-free reproducibility declaration.
    pub fn reproducible() -> Self {
        Self {
            kind: PolicyDirectiveKind::Reproducibility,
            expression: None,
        }
    }

    /// Creates a directive from its kind and optional expression.
    pub fn new(
        kind: PolicyDirectiveKind,
        expression: Option<PolicyExpression>,
    ) -> Result<Self, PolicyValidationError> {
        let permits_empty_expression = matches!(
            kind,
            PolicyDirectiveKind::Determinism
                | PolicyDirectiveKind::Reproducibility
        );

        if expression.is_none() && !permits_empty_expression {
            return Err(PolicyValidationError::ExpressionRequired { kind });
        }

        Ok(Self { kind, expression })
    }

    /// Returns the directive kind.
    pub const fn kind(&self) -> PolicyDirectiveKind {
        self.kind
    }

    /// Returns the directive classification.
    pub const fn class(&self) -> PolicyClass {
        self.kind.class()
    }

    /// Returns the directive strength.
    pub const fn strength(&self) -> PolicyStrength {
        self.kind.strength()
    }

    /// Returns the optional expression.
    pub fn expression(&self) -> Option<&PolicyExpression> {
        self.expression.as_ref()
    }
}

/// The action performed when a conditional policy rule matches.
///
/// The action is either a policy directive or an opaque expression whose
/// meaning is resolved by the canonical semantic policy subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyAction {
    Directive(PolicyDirective),
    Expression(PolicyExpression),
}

impl PolicyAction {
    /// Returns the directive when the action is a directive.
    pub fn as_directive(&self) -> Option<&PolicyDirective> {
        match self {
            Self::Directive(directive) => Some(directive),
            Self::Expression(_) => None,
        }
    }
}

// ============================================================================
// RULE CLAUSES AND RULES
// ============================================================================

/// Additional clauses attached to a conditional policy rule.
///
/// Conditions are conjunctive in this compiler-side representation.
/// Property clauses are preserved as data and interpreted downstream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyRuleClause {
    Condition(PolicyExpression),

    Property {
        key: PolicyPropertyKey,
        value: PolicyExpression,
    },
}

/// A conditional policy rule.
///
/// The condition is an opaque expression. The expression evaluator supplied
/// by the semantic layer determines its truth value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    condition: PolicyExpression,
    action: PolicyAction,
    clauses: Vec<PolicyRuleClause>,
}

impl PolicyRule {
    /// Creates a conditional policy rule.
    pub fn new(
        condition: PolicyExpression,
        action: PolicyAction,
    ) -> Self {
        Self {
            condition,
            action,
            clauses: Vec::new(),
        }
    }

    /// Adds an additional condition.
    pub fn with_condition(mut self, condition: PolicyExpression) -> Self {
        self.clauses
            .push(PolicyRuleClause::Condition(condition));
        self
    }

    /// Adds an open-world property clause.
    pub fn with_property(
        mut self,
        key: PolicyPropertyKey,
        value: PolicyExpression,
    ) -> Self {
        self.clauses
            .push(PolicyRuleClause::Property { key, value });
        self
    }

    /// Returns the primary condition.
    pub fn condition(&self) -> &PolicyExpression {
        &self.condition
    }

    /// Returns the action.
    pub fn action(&self) -> &PolicyAction {
        &self.action
    }

    /// Returns all additional clauses in declaration order.
    pub fn clauses(&self) -> &[PolicyRuleClause] {
        &self.clauses
    }
}

// ============================================================================
// POLICY MEMBERS
// ============================================================================

/// One member of a policy declaration.
///
/// The variants preserve the policy grammar's open-world structure without
/// creating separate policy languages for quantum, AI, HDL, distributed,
/// classical, networking, or future computing domains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyMember {
    Directive(PolicyDirective),
    Rule(PolicyRule),

    Extends(PolicyId),
    Override(PolicyId),

    Property {
        key: PolicyPropertyKey,
        value: PolicyExpression,
    },
}

// ============================================================================
// POLICY DOCUMENT
// ============================================================================

/// A compiler-side policy document.
///
/// This is a structural representation of one named policy declaration.
/// It is not the authoritative semantic policy model.
///
/// Declaration order is preserved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDocument {
    id: PolicyId,
    members: Vec<PolicyMember>,
}

impl PolicyDocument {
    /// Creates an empty policy document.
    pub fn new(id: PolicyId) -> Self {
        Self {
            id,
            members: Vec::new(),
        }
    }

    /// Creates a policy document from an identifier string.
    pub fn named(
        id: impl Into<String>,
    ) -> Result<Self, PolicyValidationError> {
        Ok(Self::new(PolicyId::new(id)?))
    }

    /// Returns the policy identifier.
    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    /// Returns policy members in declaration order.
    pub fn members(&self) -> &[PolicyMember] {
        &self.members
    }

    /// Adds a policy member.
    ///
    /// No arbitrary member-count ceiling is imposed.
    pub fn add_member(&mut self, member: PolicyMember) {
        self.members.push(member);
    }

    /// Adds a directive.
    pub fn add_directive(&mut self, directive: PolicyDirective) {
        self.add_member(PolicyMember::Directive(directive));
    }

    /// Adds a conditional rule.
    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.add_member(PolicyMember::Rule(rule));
    }

    /// Adds a policy extension reference.
    pub fn extends(&mut self, parent: PolicyId) {
        self.add_member(PolicyMember::Extends(parent));
    }

    /// Adds an explicit policy override reference.
    pub fn overrides(&mut self, parent: PolicyId) {
        self.add_member(PolicyMember::Override(parent));
    }

    /// Adds an open-world property.
    pub fn add_property(
        &mut self,
        key: PolicyPropertyKey,
        value: PolicyExpression,
    ) {
        self.add_member(PolicyMember::Property { key, value });
    }

    /// Validates structural invariants.
    ///
    /// This method does not resolve names, evaluate expressions, determine
    /// authorization, check hardware availability, or establish feasibility.
    pub fn validate(&self) -> Result<(), PolicyValidationError> {
        let mut composition_references = BTreeSet::new();

        for member in &self.members {
            match member {
                PolicyMember::Directive(directive) => {
                    validate_directive(directive)?;
                }

                PolicyMember::Rule(rule) => {
                    validate_rule(rule)?;
                }

                PolicyMember::Extends(parent)
                | PolicyMember::Override(parent) => {
                    if parent == &self.id {
                        return Err(PolicyValidationError::SelfComposition {
                            policy: self.id.clone(),
                        });
                    }

                    if !composition_references.insert(parent.clone()) {
                        return Err(
                            PolicyValidationError::DuplicateCompositionReference {
                                policy: self.id.clone(),
                                referenced: parent.clone(),
                            },
                        );
                    }
                }

                PolicyMember::Property { key, value } => {
                    validate_property(key, value)?;
                }
            }
        }

        Ok(())
    }

    /// Evaluates the conditions of this policy document.
    ///
    /// Directives are preserved as active declarations.
    ///
    /// A rule is active only when its primary condition and every additional
    /// condition evaluate to `True`.
    ///
    /// An `Unknown` condition is an error. It is never treated as `False`,
    /// because silently dropping an unresolved mandatory policy could change
    /// program meaning or permit an invalid realization.
    ///
    /// This method does not enforce the resulting directives.
    pub fn evaluate<C, E>(
        &self,
        context: &C,
        evaluator: &E,
    ) -> Result<PolicyEvaluation, PolicyEvaluationError>
    where
        E: PolicyExpressionEvaluator<C>,
    {
        self.validate()
            .map_err(PolicyEvaluationError::InvalidDocument)?;

        let mut active = Vec::new();
        let mut inactive_rules = 0usize;

        for (member_index, member) in self.members.iter().enumerate() {
            match member {
                PolicyMember::Directive(directive) => {
                    active.push(EvaluatedPolicyAction {
                        member_index,
                        action: PolicyAction::Directive(directive.clone()),
                        clauses: Vec::new(),
                        origin: PolicyActionOrigin::Direct,
                    });
                }

                PolicyMember::Rule(rule) => {
                    let primary = evaluator.evaluate(
                        rule.condition(),
                        context,
                    )?;

                    match primary {
                        TruthValue::False => {
                            inactive_rules += 1;
                            continue;
                        }

                        TruthValue::Unknown => {
                            return Err(
                                PolicyEvaluationError::IndeterminateCondition {
                                    policy: self.id.clone(),
                                    expression: rule.condition().clone(),
                                },
                            );
                        }

                        TruthValue::True => {}
                    }

                    let mut rule_matches = true;

                    for clause in rule.clauses() {
                        if let PolicyRuleClause::Condition(condition) = clause {
                            match evaluator.evaluate(condition, context)? {
                                TruthValue::True => {}
                                TruthValue::False => {
                                    rule_matches = false;
                                    break;
                                }
                                TruthValue::Unknown => {
                                    return Err(
                                        PolicyEvaluationError::IndeterminateCondition {
                                            policy: self.id.clone(),
                                            expression: condition.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }

                    if rule_matches {
                        active.push(EvaluatedPolicyAction {
                            member_index,
                            action: rule.action().clone(),
                            clauses: rule.clauses().to_vec(),
                            origin: PolicyActionOrigin::ConditionalRule,
                        });
                    } else {
                        inactive_rules += 1;
                    }
                }

                // Composition and properties are retained in the document.
                // Their semantic resolution belongs to the policy subsystem.
                PolicyMember::Extends(_)
                | PolicyMember::Override(_)
                | PolicyMember::Property { .. } => {}
            }
        }

        Ok(PolicyEvaluation {
            policy: self.id.clone(),
            active,
            inactive_rules,
        })
    }
}

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

fn validate_directive(
    directive: &PolicyDirective,
) -> Result<(), PolicyValidationError> {
    let expression_required = !matches!(
        directive.kind(),
        PolicyDirectiveKind::Determinism
            | PolicyDirectiveKind::Reproducibility
    );

    if expression_required && directive.expression().is_none() {
        return Err(PolicyValidationError::ExpressionRequired {
            kind: directive.kind(),
        });
    }

    Ok(())
}

fn validate_rule(rule: &PolicyRule) -> Result<(), PolicyValidationError> {
    match &rule.action {
        PolicyAction::Directive(directive) => {
            validate_directive(directive)?;
        }
        PolicyAction::Expression(expression) => {
            if expression.as_str().trim().is_empty() {
                return Err(PolicyValidationError::EmptyExpression);
            }
        }
    }

    for clause in &rule.clauses {
        if let PolicyRuleClause::Property { key, value } = clause {
            validate_property(key, value)?;
        }
    }

    Ok(())
}

fn validate_property(
    key: &PolicyPropertyKey,
    value: &PolicyExpression,
) -> Result<(), PolicyValidationError> {
    if key.as_str().trim().is_empty() {
        return Err(PolicyValidationError::EmptyPropertyKey);
    }

    if value.as_str().trim().is_empty() {
        return Err(PolicyValidationError::EmptyExpression);
    }

    Ok(())
}

// ============================================================================
// EVALUATION CONTRACT
// ============================================================================

/// Three-valued expression evaluation result.
///
/// `Unknown` is intentionally distinct from `False`.
///
/// An unresolved condition must not silently disable a policy rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruthValue {
    True,
    False,
    Unknown,
}

/// Evaluates an opaque policy expression using an authoritative context.
///
/// Implementations belong to the semantic/expression subsystem or another
/// explicitly authorized policy evaluator.
///
/// The context can provide access to resolved semantic facts, capability
/// information, target information, or resource information through existing
/// repository-owned interfaces.
///
/// This trait does not prescribe or duplicate those interfaces.
pub trait PolicyExpressionEvaluator<Context> {
    /// Evaluates an expression.
    fn evaluate(
        &self,
        expression: &PolicyExpression,
        context: &Context,
    ) -> Result<TruthValue, PolicyEvaluationError>;
}

/// Origin of an evaluated policy action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyActionOrigin {
    Direct,
    ConditionalRule,
}

/// An action whose condition has been evaluated successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedPolicyAction {
    member_index: usize,
    action: PolicyAction,
    clauses: Vec<PolicyRuleClause>,
    origin: PolicyActionOrigin,
}

impl EvaluatedPolicyAction {
    /// Returns the zero-based declaration position of the originating member.
    pub const fn member_index(&self) -> usize {
        self.member_index
    }

    /// Returns the active action.
    pub fn action(&self) -> &PolicyAction {
        &self.action
    }

    /// Returns the clauses associated with the action.
    pub fn clauses(&self) -> &[PolicyRuleClause] {
        &self.clauses
    }

    /// Returns whether the action came from a conditional rule.
    pub const fn origin(&self) -> PolicyActionOrigin {
        self.origin
    }

    /// Returns the directive kind, if the action is a directive.
    pub fn directive_kind(&self) -> Option<PolicyDirectiveKind> {
        self.action
            .as_directive()
            .map(PolicyDirective::kind)
    }

    /// Returns the policy class, if the action is a directive.
    pub fn class(&self) -> Option<PolicyClass> {
        self.action
            .as_directive()
            .map(PolicyDirective::class)
    }

    /// Returns the policy strength, if the action is a directive.
    pub fn strength(&self) -> Option<PolicyStrength> {
        self.action
            .as_directive()
            .map(PolicyDirective::strength)
    }
}

/// The result of evaluating a policy document's conditions.
///
/// This is not a feasibility result and is not proof that an action has been
/// authorized, satisfied, or implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvaluation {
    policy: PolicyId,
    active: Vec<EvaluatedPolicyAction>,
    inactive_rules: usize,
}

impl PolicyEvaluation {
    /// Returns the policy identifier.
    pub fn policy(&self) -> &PolicyId {
        &self.policy
    }

    /// Returns active actions in declaration order.
    pub fn active_actions(&self) -> &[EvaluatedPolicyAction] {
        &self.active
    }

    /// Returns the number of conditional rules that did not match.
    pub const fn inactive_rule_count(&self) -> usize {
        self.inactive_rules
    }

    /// Returns whether no active action was produced.
    pub fn is_empty(&self) -> bool {
        self.active.is_empty()
    }

    /// Returns active actions with mandatory strength.
    pub fn mandatory_actions(
        &self,
    ) -> impl Iterator<Item = &EvaluatedPolicyAction> {
        self.active
            .iter()
            .filter(|action| {
                action.strength() == Some(PolicyStrength::Mandatory)
            })
    }

    /// Returns active advisory actions.
    pub fn advisory_actions(
        &self,
    ) -> impl Iterator<Item = &EvaluatedPolicyAction> {
        self.active
            .iter()
            .filter(|action| {
                action.strength() == Some(PolicyStrength::Advisory)
            })
    }

    /// Returns active governing actions.
    pub fn governing_actions(
        &self,
    ) -> impl Iterator<Item = &EvaluatedPolicyAction> {
        self.active
            .iter()
            .filter(|action| {
                action.strength() == Some(PolicyStrength::Governing)
            })
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structural policy validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyValidationError {
    EmptyIdentifier,
    EmptyExpression,
    EmptyPropertyKey,

    ExpressionRequired {
        kind: PolicyDirectiveKind,
    },

    SelfComposition {
        policy: PolicyId,
    },

    DuplicateCompositionReference {
        policy: PolicyId,
        referenced: PolicyId,
    },
}

impl fmt::Display for PolicyValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("policy identifier cannot be empty")
            }

            Self::EmptyExpression => {
                formatter.write_str("policy expression cannot be empty")
            }

            Self::EmptyPropertyKey => {
                formatter.write_str("policy property key cannot be empty")
            }

            Self::ExpressionRequired { kind } => {
                write!(
                    formatter,
                    "policy directive {kind:?} requires an expression"
                )
            }

            Self::SelfComposition { policy } => {
                write!(
                    formatter,
                    "policy `{policy}` cannot extend or override itself"
                )
            }

            Self::DuplicateCompositionReference {
                policy,
                referenced,
            } => {
                write!(
                    formatter,
                    "policy `{policy}` contains a duplicate composition \
                     reference to `{referenced}`"
                )
            }
        }
    }
}

impl Error for PolicyValidationError {}

/// Policy evaluation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEvaluationError {
    InvalidDocument(PolicyValidationError),

    IndeterminateCondition {
        policy: PolicyId,
        expression: PolicyExpression,
    },

    ExpressionFailure {
        expression: PolicyExpression,
        message: String,
    },

    EvaluationUnavailable {
        message: String,
    },
}

impl fmt::Display for PolicyEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDocument(error) => {
                write!(formatter, "invalid policy document: {error}")
            }

            Self::IndeterminateCondition {
                policy,
                expression,
            } => {
                write!(
                    formatter,
                    "policy `{policy}` has an indeterminate condition: \
                     `{expression}`"
                )
            }

            Self::ExpressionFailure {
                expression,
                message,
            } => {
                write!(
                    formatter,
                    "failed to evaluate policy expression `{expression}`: \
                     {message}"
                )
            }

            Self::EvaluationUnavailable { message } => {
                write!(
                    formatter,
                    "policy evaluation is unavailable: {message}"
                )
            }
        }
    }
}

impl Error for PolicyEvaluationError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct TestEvaluator;

    impl PolicyExpressionEvaluator<()> for TestEvaluator {
        fn evaluate(
            &self,
            expression: &PolicyExpression,
            _context: &(),
        ) -> Result<TruthValue, PolicyEvaluationError> {
            match expression.as_str() {
                "true" => Ok(TruthValue::True),
                "false" => Ok(TruthValue::False),
                "unknown" => Ok(TruthValue::Unknown),
                other => Err(PolicyEvaluationError::ExpressionFailure {
                    expression: PolicyExpression::new(other)
                        .expect("test expression is non-empty"),
                    message: "unknown test expression".to_owned(),
                }),
            }
        }
    }

    fn expression(value: &str) -> PolicyExpression {
        PolicyExpression::new(value)
            .expect("test expression must be non-empty")
    }

    fn identifier(value: &str) -> PolicyId {
        PolicyId::new(value)
            .expect("test identifier must be non-empty")
    }

    fn directive(
        kind: PolicyDirectiveKind,
        value: &str,
    ) -> PolicyDirective {
        PolicyDirective::with_expression(kind, expression(value))
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(POLICY_API_VERSION, 0);
    }

    #[test]
    fn policy_identifier_rejects_empty_values() {
        assert!(matches!(
            PolicyId::new("  "),
            Err(PolicyValidationError::EmptyIdentifier)
        ));
    }

    #[test]
    fn policy_identifier_preserves_qualified_names() {
        let id = identifier("organization::deployment::production");

        assert_eq!(
            id.as_str(),
            "organization::deployment::production"
        );
    }

    #[test]
    fn expression_rejects_empty_values() {
        assert!(matches!(
            PolicyExpression::new(" \n "),
            Err(PolicyValidationError::EmptyExpression)
        ));
    }

    #[test]
    fn expression_preserves_original_text() {
        let value = " execution::deterministic ";

        let parsed = expression(value);

        assert_eq!(parsed.as_str(), value);
    }

    #[test]
    fn property_keys_are_open_world() {
        let key = PolicyPropertyKey::new(
            "future::compiler::optimization_strategy",
        )
        .expect("property key should be accepted");

        assert_eq!(
            key.as_str(),
            "future::compiler::optimization_strategy"
        );
    }

    #[test]
    fn directive_classification_keeps_preferences_advisory() {
        assert_eq!(
            PolicyDirectiveKind::Preference.strength(),
            PolicyStrength::Advisory
        );

        assert_eq!(
            PolicyDirectiveKind::Requirement.strength(),
            PolicyStrength::Mandatory
        );

        assert_eq!(
            PolicyDirectiveKind::Constraint.strength(),
            PolicyStrength::Mandatory
        );
    }

    #[test]
    fn permission_and_prohibition_are_not_requirements() {
        assert_eq!(
            PolicyDirectiveKind::Permission.class(),
            PolicyClass::AuthorizationIntent
        );

        assert_eq!(
            PolicyDirectiveKind::Prohibition.class(),
            PolicyClass::AuthorizationIntent
        );

        assert_ne!(
            PolicyDirectiveKind::Permission.class(),
            PolicyClass::MandatoryIntent
        );
    }

    #[test]
    fn expression_free_determinism_is_supported() {
        let directive = PolicyDirective::deterministic();

        assert_eq!(
            directive.kind(),
            PolicyDirectiveKind::Determinism
        );

        assert!(directive.expression().is_none());
    }

    #[test]
    fn expression_free_reproducibility_is_supported() {
        let directive = PolicyDirective::reproducible();

        assert_eq!(
            directive.kind(),
            PolicyDirectiveKind::Reproducibility
        );

        assert!(directive.expression().is_none());
    }

    #[test]
    fn other_directives_require_expressions() {
        let result = PolicyDirective::new(
            PolicyDirectiveKind::Requirement,
            None,
        );

        assert!(matches!(
            result,
            Err(PolicyValidationError::ExpressionRequired { .. })
        ));
    }

    #[test]
    fn document_preserves_declaration_order() {
        let mut document = PolicyDocument::named("deployment::portable")
            .expect("document should be created");

        document.add_directive(directive(
            PolicyDirectiveKind::Requirement,
            "execution::portable",
        ));

        document.add_directive(directive(
            PolicyDirectiveKind::Preference,
            "lower_energy",
        ));

        assert_eq!(document.members().len(), 2);

        assert!(matches!(
            document.members()[0],
            PolicyMember::Directive(_)
        ));

        assert!(matches!(
            document.members()[1],
            PolicyMember::Directive(_)
        ));
    }

    #[test]
    fn document_rejects_self_extension() {
        let mut document = PolicyDocument::named("deployment::portable")
            .expect("document should be created");

        document.extends(identifier("deployment::portable"));

        assert!(matches!(
            document.validate(),
            Err(PolicyValidationError::SelfComposition { .. })
        ));
    }

    #[test]
    fn document_rejects_duplicate_composition_references() {
        let mut document = PolicyDocument::named("deployment::portable")
            .expect("document should be created");

        document.extends(identifier("deployment::base"));
        document.overrides(identifier("deployment::base"));

        assert!(matches!(
            document.validate(),
            Err(PolicyValidationError::DuplicateCompositionReference { .. })
        ));
    }

    #[test]
    fn document_accepts_distinct_composition_references() {
        let mut document = PolicyDocument::named("deployment::portable")
            .expect("document should be created");

        document.extends(identifier("deployment::base"));
        document.overrides(identifier("deployment::legacy"));

        assert!(document.validate().is_ok());
    }

    #[test]
    fn conditional_rule_is_active_when_condition_is_true() {
        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "lower_energy",
            )),
        );

        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_rule(rule);

        let result = document
            .evaluate(&(), &TestEvaluator)
            .expect("evaluation should succeed");

        assert_eq!(result.active_actions().len(), 1);
        assert_eq!(result.inactive_rule_count(), 0);
    }

    #[test]
    fn conditional_rule_is_inactive_when_condition_is_false() {
        let rule = PolicyRule::new(
            expression("false"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "lower_energy",
            )),
        );

        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_rule(rule);

        let result = document
            .evaluate(&(), &TestEvaluator)
            .expect("evaluation should succeed");

        assert!(result.is_empty());
        assert_eq!(result.inactive_rule_count(), 1);
    }

    #[test]
    fn unknown_rule_condition_fails_closed() {
        let rule = PolicyRule::new(
            expression("unknown"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "lower_energy",
            )),
        );

        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_rule(rule);

        assert!(matches!(
            document.evaluate(&(), &TestEvaluator),
            Err(PolicyEvaluationError::IndeterminateCondition { .. })
        ));
    }

    #[test]
    fn additional_conditions_are_conjunctive() {
        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "lower_energy",
            )),
        )
        .with_condition(expression("false"));

        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_rule(rule);

        let result = document
            .evaluate(&(), &TestEvaluator)
            .expect("evaluation should succeed");

        assert!(result.is_empty());
        assert_eq!(result.inactive_rule_count(), 1);
    }

    #[test]
    fn active_actions_preserve_original_member_indices() {
        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_directive(directive(
            PolicyDirectiveKind::Preference,
            "lower_energy",
        ));

        document.add_rule(PolicyRule::new(
            expression("false"),
            PolicyAction::Expression(expression("unused")),
        ));

        document.add_directive(directive(
            PolicyDirectiveKind::Requirement,
            "execution::deterministic",
        ));

        let result = document
            .evaluate(&(), &TestEvaluator)
            .expect("evaluation should succeed");

        assert_eq!(result.active_actions()[0].member_index(), 0);
        assert_eq!(result.active_actions()[1].member_index(), 2);
    }

    #[test]
    fn evaluation_does_not_turn_preferences_into_requirements() {
        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_directive(directive(
            PolicyDirectiveKind::Preference,
            "lower_latency",
        ));

        document.add_directive(directive(
            PolicyDirectiveKind::Requirement,
            "execution::deterministic",
        ));

        let result = document
            .evaluate(&(), &TestEvaluator)
            .expect("evaluation should succeed");

        assert_eq!(result.advisory_actions().count(), 1);
        assert_eq!(result.mandatory_actions().count(), 1);
    }

    #[test]
    fn direct_directives_are_preserved_without_interpretation() {
        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_directive(directive(
            PolicyDirectiveKind::Capability,
            "future::accelerator::compute",
        ));

        let result = document
            .evaluate(&(), &TestEvaluator)
            .expect("evaluation should succeed");

        assert_eq!(result.active_actions().len(), 1);
        assert_eq!(
            result.active_actions()[0].directive_kind(),
            Some(PolicyDirectiveKind::Capability)
        );
    }

    #[test]
    fn policy_document_supports_open_world_domains() {
        let domains = [
            "classical::vector",
            "quantum::measurement",
            "ai::inference",
            "hdl::synthesis",
            "distributed::communication",
            "networking::transport",
            "future::computing::domain",
        ];

        let mut document = PolicyDocument::named("execution::universal")
            .expect("document should be created");

        for domain in domains {
            document.add_directive(directive(
                PolicyDirectiveKind::Requirement,
                domain,
            ));
        }

        assert_eq!(document.members().len(), domains.len());
        assert!(document.validate().is_ok());
    }

    #[test]
    fn property_clauses_are_preserved() {
        let key = PolicyPropertyKey::new("priority")
            .expect("property key should be valid");

        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Expression(expression("select target")),
        )
        .with_property(key, expression("high"));

        assert_eq!(rule.clauses().len(), 1);

        assert!(matches!(
            rule.clauses()[0],
            PolicyRuleClause::Property { .. }
        ));
    }

    #[test]
    fn evaluation_failure_is_propagated() {
        let mut document = PolicyDocument::named("execution::portable")
            .expect("document should be created");

        document.add_rule(PolicyRule::new(
            expression("unsupported_expression"),
            PolicyAction::Expression(expression("select target")),
        ));

        assert!(matches!(
            document.evaluate(&(), &TestEvaluator),
            Err(PolicyEvaluationError::ExpressionFailure { .. })
        ));
    }

    #[test]
    fn no_fixed_policy_member_limit_is_encoded() {
        let mut document = PolicyDocument::named("execution::large")
            .expect("document should be created");

        for index in 0..10_000usize {
            document.add_directive(directive(
                PolicyDirectiveKind::Preference,
                &format!("preference::{index}"),
            ));
        }

        assert_eq!(document.members().len(), 10_000);
        assert!(document.validate().is_ok());
    }
}
