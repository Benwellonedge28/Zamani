
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Compilation Policy Contracts.
//!
//! Path: src/compiler/configuration/policy.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides compiler-side policy representation, structural validation,
//! policy classification, conditional evaluation and integration contracts.
//!
//! # Architectural ownership
//!
//! The canonical policy grammar owns policy syntax.
//!
//! The frontend owns AST construction.
//!
//! The semantic subsystem owns:
//! - Policy name resolution.
//! - Policy semantic validation.
//! - Policy composition semantics.
//! - Expression meaning.
//! - Authorization semantics.
//! - Source-level policy enforcement meaning.
//!
//! This module owns:
//! - Compiler-side policy contracts.
//! - Structural policy validation.
//! - Policy intent classification.
//! - Conditional evaluation interfaces.
//! - Evaluation result representation.
//! - Fail-closed handling of indeterminate conditions.
//!
//! Other authorities:
//!
//! configuration/budget.rs:
//!     Operational resource budgets and accounting.
//!
//! configuration/target.rs:
//!     Target descriptors.
//!
//! configuration/config.rs:
//!     CompilerConfig construction and validation.
//!
//! analysis/requirements.rs:
//!     Compiler-side requirements.
//!
//! planning/:
//!     Feasibility and realization planning.
//!
//! security/:
//!     Security verification and enforcement integration.
//!
//! optimization/:
//!     Optimization policy consumption.
//!
//! runtime/:
//!     Runtime policy enforcement.
//!
//! # Important boundaries
//!
//! This module does not:
//! - Parse Zamani expressions.
//! - Construct AST nodes.
//! - Resolve policy names.
//! - Discover hardware.
//! - Allocate resources.
//! - Select physical devices.
//! - Perform authorization.
//! - Execute compiler stages.
//! - Schedule runtime work.
//! - Define a competing semantic policy engine.
//!
//! # POCO-REAF
//!
//! Policies describe governing intent independently of physical hardware.
//!
//! No universal hardware capacity ceilings are defined here.
//!
//! Operational limits belong to explicitly configured budgets.
//!
//! Actual feasibility belongs to planning and capability evaluation.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Determinism
//!
//! Policy declaration order is preserved.
//!
//! Conditional rules are evaluated in declaration order.
//!
//! Composition references are validated deterministically.
//!
//! # Expression ownership
//!
//! Expressions remain opaque.
//!
//! Their syntax and meaning belong to the canonical frontend and semantic
//! expression subsystem.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

// ============================================================================
// VERSION
// ============================================================================

/// Version of the compiler-side policy contract.
///
/// Independent of the language, grammar, semantic-model, IR and artifact
/// versions.
pub const POLICY_API_VERSION: u16 = 2;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// A qualified, open-world policy identifier.
///
/// Examples:
///
/// - execution::portable
/// - deployment::production
/// - quantum::fault_tolerant
/// - organization::security::strict
///
/// Identity resolution remains the responsibility of semantic analysis.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

impl PolicyId {
    /// Constructs a policy identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, PolicyValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PolicyValidationError::EmptyIdentifier);
        }

        if value.chars().any(char::is_control) {
            return Err(
                PolicyValidationError::ControlCharacter {
                    field: "policy identifier",
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the original identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EXPRESSIONS
// ============================================================================

/// An opaque Zamani policy expression.
///
/// This type preserves source expression text without interpreting it.
///
/// Parsing and semantic meaning remain owned by the frontend.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolicyExpression(String);

impl PolicyExpression {
    /// Constructs an expression.
    ///
    /// Newlines and ordinary whitespace are preserved.
    /// NUL characters are rejected.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, PolicyValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PolicyValidationError::EmptyExpression);
        }

        if value.contains('\0') {
            return Err(
                PolicyValidationError::ControlCharacter {
                    field: "policy expression",
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the original expression text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the expression.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyExpression {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PROPERTY KEYS
// ============================================================================

/// An open-world policy property key.
///
/// New property names do not require new Rust enum variants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyPropertyKey(String);

impl PolicyPropertyKey {
    /// Constructs a property key.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, PolicyValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(PolicyValidationError::EmptyPropertyKey);
        }

        if value.chars().any(char::is_control) {
            return Err(
                PolicyValidationError::ControlCharacter {
                    field: "policy property key",
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the original property key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyPropertyKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// POLICY CLASSIFICATION
// ============================================================================

/// Broad classification of policy intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyClass {
    /// Requirements, constraints, capabilities and resources.
    MandatoryIntent,

    /// Advisory optimization preferences.
    AdvisoryIntent,

    /// Permission, prohibition and sandbox declarations.
    AuthorizationIntent,

    /// Selection, negotiation, fallback and adaptation.
    RealizationIntent,

    /// Determinism, reproducibility, effects and provenance.
    SemanticContractIntent,

    /// Open-ended metadata.
    MetadataIntent,
}

/// Strength of a policy directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyStrength {
    /// Must be satisfied.
    Mandatory,

    /// Advisory and unable to override mandatory conditions.
    Advisory,

    /// Requires interpretation by its governing authority.
    Governing,

    /// Informational metadata.
    Informational,
}

// ============================================================================
// DIRECTIVE KINDS
// ============================================================================

/// Canonical policy directive categories.
///
/// This enum follows the current canonical policy grammar.
///
/// It does not enumerate computing domains, hardware, vendors or devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyDirectiveKind {
    /// Requirement declaration.
    Requirement,

    /// Constraint declaration.
    Constraint,

    /// Capability declaration.
    Capability,

    /// Resource declaration.
    Resource,

    /// Permission declaration.
    Permission,

    /// Prohibition declaration.
    Prohibition,

    /// Advisory preference.
    Preference,

    /// Alternative realization declaration.
    Fallback,

    /// Selection intent.
    Selection,

    /// Negotiation intent.
    Negotiation,

    /// Retry intent.
    Retry,

    /// Recovery intent.
    Recovery,

    /// Escalation intent.
    Escalation,

    /// Rejection intent.
    Rejection,

    /// Simulation intent.
    Simulation,

    /// Adaptation intent.
    Adaptation,

    /// Sandboxing intent.
    Sandbox,

    /// Determinism declaration.
    Determinism,

    /// Reproducibility declaration.
    Reproducibility,

    /// Effect declaration.
    Effect,

    /// Provenance declaration.
    Provenance,

    /// Contract reference.
    ContractReference,
}

impl PolicyDirectiveKind {
    /// Returns the policy classification.
    pub const fn class(self) -> PolicyClass {
        match self {
            Self::Requirement
            | Self::Constraint
            | Self::Capability
            | Self::Resource => PolicyClass::MandatoryIntent,

            Self::Preference => PolicyClass::AdvisoryIntent,

            Self::Permission
            | Self::Prohibition
            | Self::Sandbox => PolicyClass::AuthorizationIntent,

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
            | Self::ContractReference => {
                PolicyClass::SemanticContractIntent
            }
        }
    }

    /// Returns the policy strength.
    pub const fn strength(self) -> PolicyStrength {
        match self {
            Self::Requirement
            | Self::Constraint
            | Self::Capability
            | Self::Resource => PolicyStrength::Mandatory,

            Self::Preference => PolicyStrength::Advisory,

            Self::Permission
            | Self::Prohibition
            | Self::Sandbox
            | Self::Fallback
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

    /// Returns whether an expression is optional for this directive.
    ///
    /// The canonical grammar permits optional expressions only for
    /// determinism and reproducibility declarations.
    pub const fn permits_expression_omission(self) -> bool {
        matches!(
            self,
            Self::Determinism | Self::Reproducibility
        )
    }

    /// Returns whether this directive is legal as a conditional rule action.
    ///
    /// This follows policyRuleAction in grammar/policies/policy.g4.
    pub const fn is_valid_rule_action(self) -> bool {
        !matches!(
            self,
            Self::Requirement
                | Self::Constraint
                | Self::Capability
                | Self::Resource
        )
    }

    /// Returns whether downstream resolution is required.
    ///
    /// Preferences are advisory, but they must still be considered.
    /// Unsupported preferences must not silently become mandatory or
    /// disappear without an explicit downstream decision.
    pub const fn requires_downstream_resolution(self) -> bool {
        true
    }

    /// Returns whether the directive is advisory.
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::Preference)
    }
}

// ============================================================================
// DIRECTIVES
// ============================================================================

/// One policy directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDirective {
    kind: PolicyDirectiveKind,
    expression: Option<PolicyExpression>,
}

impl PolicyDirective {
    /// Constructs a directive from its kind and expression.
    pub fn new(
        kind: PolicyDirectiveKind,
        expression: Option<PolicyExpression>,
    ) -> Result<Self, PolicyValidationError> {
        if expression.is_none()
            && !kind.permits_expression_omission()
        {
            return Err(
                PolicyValidationError::ExpressionRequired { kind },
            );
        }

        Ok(Self { kind, expression })
    }

    /// Constructs a directive with an expression.
    pub fn with_expression(
        kind: PolicyDirectiveKind,
        expression: PolicyExpression,
    ) -> Self {
        Self {
            kind,
            expression: Some(expression),
        }
    }

    /// Constructs an expression-free determinism directive.
    pub fn deterministic() -> Self {
        Self {
            kind: PolicyDirectiveKind::Determinism,
            expression: None,
        }
    }

    /// Constructs an expression-free reproducibility directive.
    pub fn reproducible() -> Self {
        Self {
            kind: PolicyDirectiveKind::Reproducibility,
            expression: None,
        }
    }

    /// Returns the directive kind.
    pub const fn kind(&self) -> PolicyDirectiveKind {
        self.kind
    }

    /// Returns its classification.
    pub const fn class(&self) -> PolicyClass {
        self.kind.class()
    }

    /// Returns its strength.
    pub const fn strength(&self) -> PolicyStrength {
        self.kind.strength()
    }

    /// Returns the optional expression.
    pub fn expression(&self) -> Option<&PolicyExpression> {
        self.expression.as_ref()
    }
}

// ============================================================================
// POLICY ACTIONS
// ============================================================================

/// An action associated with a conditional policy rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyAction {
    /// A structured policy directive.
    Directive(PolicyDirective),

    /// An opaque canonical policy expression.
    Expression(PolicyExpression),
}

impl PolicyAction {
    /// Returns the directive, if present.
    pub fn as_directive(&self) -> Option<&PolicyDirective> {
        match self {
            Self::Directive(directive) => Some(directive),
            Self::Expression(_) => None,
        }
    }
}

// ============================================================================
// CONDITIONAL RULES
// ============================================================================

/// An additional clause in a conditional rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyRuleClause {
    /// An additional conjunctive condition.
    Condition(PolicyExpression),

    /// An open-world property assignment.
    ///
    /// Property assignments are retained for semantic resolution.
    /// They are not interpreted as boolean conditions here.
    Property {
        /// Property key.
        key: PolicyPropertyKey,

        /// Opaque property value.
        value: PolicyExpression,
    },
}

/// One conditional policy rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    condition: PolicyExpression,
    action: PolicyAction,
    clauses: Vec<PolicyRuleClause>,
}

impl PolicyRule {
    /// Constructs a conditional rule.
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

    /// Adds an additional conjunctive condition.
    pub fn with_condition(
        mut self,
        condition: PolicyExpression,
    ) -> Self {
        self.clauses.push(
            PolicyRuleClause::Condition(condition),
        );

        self
    }

    /// Adds a property assignment.
    pub fn with_property(
        mut self,
        key: PolicyPropertyKey,
        value: PolicyExpression,
    ) -> Self {
        self.clauses.push(
            PolicyRuleClause::Property { key, value },
        );

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

    /// Returns clauses in declaration order.
    pub fn clauses(&self) -> &[PolicyRuleClause] {
        &self.clauses
    }
}

// ============================================================================
// POLICY MEMBERS
// ============================================================================

/// One member of a policy document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyMember {
    /// A direct policy directive.
    Directive(PolicyDirective),

    /// A conditional rule.
    Rule(PolicyRule),

    /// Policy extension reference.
    Extends(PolicyId),

    /// Explicit policy override reference.
    Override(PolicyId),

    /// Open-world policy property.
    Property {
        /// Property key.
        key: PolicyPropertyKey,

        /// Property value.
        value: PolicyExpression,
    },
}

// ============================================================================
// POLICY DOCUMENT
// ============================================================================

/// A structurally represented policy document.
///
/// This is not the authoritative semantic policy model.
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

    /// Creates a named policy document.
    pub fn named(
        id: impl Into<String>,
    ) -> Result<Self, PolicyValidationError> {
        Ok(Self::new(PolicyId::new(id)?))
    }

    /// Returns the policy identifier.
    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    /// Returns members in source declaration order.
    pub fn members(&self) -> &[PolicyMember] {
        &self.members
    }

    /// Returns the number of members.
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns whether the policy is empty.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Adds a policy member.
    ///
    /// No arbitrary member-count ceiling is imposed.
    pub fn add_member(&mut self, member: PolicyMember) {
        self.members.push(member);
    }

    /// Adds a direct directive.
    pub fn add_directive(
        &mut self,
        directive: PolicyDirective,
    ) {
        self.add_member(PolicyMember::Directive(directive));
    }

    /// Adds a conditional rule.
    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.add_member(PolicyMember::Rule(rule));
    }

    /// Adds an extension reference.
    pub fn extends(&mut self, parent: PolicyId) {
        self.add_member(PolicyMember::Extends(parent));
    }

    /// Adds an override reference.
    pub fn overrides(&mut self, parent: PolicyId) {
        self.add_member(PolicyMember::Override(parent));
    }

    /// Adds an open-world property.
    pub fn add_property(
        &mut self,
        key: PolicyPropertyKey,
        value: PolicyExpression,
    ) {
        self.add_member(
            PolicyMember::Property { key, value },
        );
    }

    /// Validates structural invariants.
    ///
    /// Name resolution, composition-cycle detection across documents,
    /// authorization and expression semantics remain external.
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
                        return Err(
                            PolicyValidationError::SelfComposition {
                                policy: self.id.clone(),
                            },
                        );
                    }

                    if !composition_references.insert(parent.clone()) {
                        return Err(
                            PolicyValidationError::
                                DuplicateCompositionReference {
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

    /// Evaluates conditional rules using an authoritative expression
    /// evaluator.
    ///
    /// Directives are retained in declaration order.
    ///
    /// Rules activate only when their primary condition and all additional
    /// condition clauses evaluate to True.
    ///
    /// Unknown is an error. It is never silently interpreted as False.
    ///
    /// Property assignments are preserved in the resulting action.
    ///
    /// This function does not perform authorization, capability matching,
    /// resource feasibility, target selection or enforcement.
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

        for (member_index, member) in
            self.members.iter().enumerate()
        {
            match member {
                PolicyMember::Directive(directive) => {
                    active.push(EvaluatedPolicyAction {
                        member_index,
                        action: PolicyAction::Directive(
                            directive.clone(),
                        ),
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
                        TruthValue::True => {}

                        TruthValue::False => {
                            inactive_rules += 1;
                            continue;
                        }

                        TruthValue::Unknown => {
                            return Err(
                                PolicyEvaluationError::
                                    IndeterminateCondition {
                                        policy: self.id.clone(),
                                        expression:
                                            rule.condition().clone(),
                                    },
                            );
                        }
                    }

                    let mut matches = true;

                    for clause in rule.clauses() {
                        if let PolicyRuleClause::Condition(
                            condition,
                        ) = clause
                        {
                            match evaluator.evaluate(
                                condition,
                                context,
                            )? {
                                TruthValue::True => {}

                                TruthValue::False => {
                                    matches = false;
                                    break;
                                }

                                TruthValue::Unknown => {
                                    return Err(
                                        PolicyEvaluationError::
                                            IndeterminateCondition {
                                                policy: self.id.clone(),
                                                expression:
                                                    condition.clone(),
                                            },
                                    );
                                }
                            }
                        }
                    }

                    if matches {
                        active.push(EvaluatedPolicyAction {
                            member_index,
                            action: rule.action().clone(),
                            clauses: rule.clauses().to_vec(),
                            origin:
                                PolicyActionOrigin::ConditionalRule,
                        });
                    } else {
                        inactive_rules += 1;
                    }
                }

                // These members require semantic policy composition
                // or property resolution. They are deliberately not
                // interpreted as executable actions here.
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
    if directive.expression().is_none()
        && !directive
            .kind()
            .permits_expression_omission()
    {
        return Err(
            PolicyValidationError::ExpressionRequired {
                kind: directive.kind(),
            },
        );
    }

    Ok(())
}

fn validate_rule(
    rule: &PolicyRule,
) -> Result<(), PolicyValidationError> {
    if let PolicyAction::Directive(directive) = rule.action() {
        validate_directive(directive)?;

        if !directive.kind().is_valid_rule_action() {
            return Err(
                PolicyValidationError::InvalidRuleAction {
                    kind: directive.kind(),
                },
            );
        }
    }

    if let PolicyAction::Expression(expression) = rule.action() {
        validate_expression(expression)?;
    }

    validate_expression(rule.condition())?;

    for clause in rule.clauses() {
        match clause {
            PolicyRuleClause::Condition(condition) => {
                validate_expression(condition)?;
            }

            PolicyRuleClause::Property { key, value } => {
                validate_property(key, value)?;
            }
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

    validate_expression(value)
}

fn validate_expression(
    expression: &PolicyExpression,
) -> Result<(), PolicyValidationError> {
    if expression.as_str().trim().is_empty() {
        return Err(PolicyValidationError::EmptyExpression);
    }

    if expression.as_str().contains('\0') {
        return Err(
            PolicyValidationError::ControlCharacter {
                field: "policy expression",
            },
        );
    }

    Ok(())
}

// ============================================================================
// THREE-VALUED EVALUATION
// ============================================================================

/// Three-valued condition result.
///
/// Unknown is distinct from False.
///
/// An unresolved mandatory policy condition must fail closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruthValue {
    /// The expression is true.
    True,

    /// The expression is false.
    False,

    /// The expression cannot currently be determined.
    Unknown,
}

/// Evaluates opaque policy expressions.
///
/// Implementations belong to the authoritative semantic expression
/// subsystem or an explicitly authorized evaluator.
///
/// The compiler policy module does not prescribe the context representation.
pub trait PolicyExpressionEvaluator<Context> {
    /// Evaluates an expression.
    fn evaluate(
        &self,
        expression: &PolicyExpression,
        context: &Context,
    ) -> Result<TruthValue, PolicyEvaluationError>;
}

// ============================================================================
// EVALUATED ACTIONS
// ============================================================================

/// Origin of an evaluated action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyActionOrigin {
    /// Direct policy declaration.
    Direct,

    /// Conditional rule.
    ConditionalRule,
}

/// An action whose conditions evaluated successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedPolicyAction {
    member_index: usize,
    action: PolicyAction,
    clauses: Vec<PolicyRuleClause>,
    origin: PolicyActionOrigin,
}

impl EvaluatedPolicyAction {
    /// Returns the zero-based source member position.
    pub const fn member_index(&self) -> usize {
        self.member_index
    }

    /// Returns the action.
    pub fn action(&self) -> &PolicyAction {
        &self.action
    }

    /// Returns associated clauses.
    ///
    /// Property clauses are retained without reinterpretation.
    pub fn clauses(&self) -> &[PolicyRuleClause] {
        &self.clauses
    }

    /// Returns the action origin.
    pub const fn origin(&self) -> PolicyActionOrigin {
        self.origin
    }

    /// Returns the directive kind.
    pub fn directive_kind(&self) -> Option<PolicyDirectiveKind> {
        self.action
            .as_directive()
            .map(PolicyDirective::kind)
    }

    /// Returns the policy classification.
    pub fn class(&self) -> Option<PolicyClass> {
        self.action
            .as_directive()
            .map(PolicyDirective::class)
    }

    /// Returns the policy strength.
    pub fn strength(&self) -> Option<PolicyStrength> {
        self.action
            .as_directive()
            .map(PolicyDirective::strength)
    }
}

// ============================================================================
// EVALUATION RESULT
// ============================================================================

/// Result of evaluating a policy document.
///
/// This result is not proof of authorization, feasibility or execution.
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

    /// Returns the inactive conditional-rule count.
    pub const fn inactive_rule_count(&self) -> usize {
        self.inactive_rules
    }

    /// Returns whether no actions were activated.
    pub fn is_empty(&self) -> bool {
        self.active.is_empty()
    }

    /// Returns mandatory actions.
    pub fn mandatory_actions(
        &self,
    ) -> impl Iterator<Item = &EvaluatedPolicyAction> {
        self.active.iter().filter(|action| {
            action.strength() == Some(PolicyStrength::Mandatory)
        })
    }

    /// Returns advisory actions.
    pub fn advisory_actions(
        &self,
    ) -> impl Iterator<Item = &EvaluatedPolicyAction> {
        self.active.iter().filter(|action| {
            action.strength() == Some(PolicyStrength::Advisory)
        })
    }

    /// Returns governing actions.
    pub fn governing_actions(
        &self,
    ) -> impl Iterator<Item = &EvaluatedPolicyAction> {
        self.active.iter().filter(|action| {
            action.strength() == Some(PolicyStrength::Governing)
        })
    }
}

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Structural policy validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyValidationError {
    /// Empty identifier.
    EmptyIdentifier,

    /// Empty expression.
    EmptyExpression,

    /// Empty property key.
    EmptyPropertyKey,

    /// Invalid control character.
    ControlCharacter {
        /// Affected input field.
        field: &'static str,
    },

    /// An expression is required for this directive.
    ExpressionRequired {
        /// Directive kind.
        kind: PolicyDirectiveKind,
    },

    /// A directive is not legal as a conditional rule action.
    InvalidRuleAction {
        /// Invalid directive kind.
        kind: PolicyDirectiveKind,
    },

    /// A policy references itself.
    SelfComposition {
        /// Policy identifier.
        policy: PolicyId,
    },

    /// A composition reference is duplicated.
    DuplicateCompositionReference {
        /// Policy identifier.
        policy: PolicyId,

        /// Referenced policy.
        referenced: PolicyId,
    },
}

impl fmt::Display for PolicyValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str(
                    "policy identifier cannot be empty",
                )
            }

            Self::EmptyExpression => {
                formatter.write_str(
                    "policy expression cannot be empty",
                )
            }

            Self::EmptyPropertyKey => {
                formatter.write_str(
                    "policy property key cannot be empty",
                )
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} contains an invalid control character"
                )
            }

            Self::ExpressionRequired { kind } => {
                write!(
                    formatter,
                    "policy directive {kind:?} requires an expression"
                )
            }

            Self::InvalidRuleAction { kind } => {
                write!(
                    formatter,
                    "policy directive {kind:?} is not valid \
                     as a conditional rule action"
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
                    "policy `{policy}` contains duplicate composition \
                     reference `{referenced}`"
                )
            }
        }
    }
}

impl Error for PolicyValidationError {}

// ============================================================================
// EVALUATION ERRORS
// ============================================================================

/// Policy evaluation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEvaluationError {
    /// The document is structurally invalid.
    InvalidDocument(PolicyValidationError),

    /// A required condition is indeterminate.
    IndeterminateCondition {
        /// Policy identifier.
        policy: PolicyId,

        /// Unresolved expression.
        expression: PolicyExpression,
    },

    /// Expression evaluation failed.
    ExpressionFailure {
        /// Expression.
        expression: PolicyExpression,

        /// Failure description.
        message: String,
    },

    /// Evaluation is unavailable.
    EvaluationUnavailable {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for PolicyEvaluationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDocument(error) => {
                write!(
                    formatter,
                    "invalid policy document: {error}"
                )
            }

            Self::IndeterminateCondition {
                policy,
                expression,
            } => {
                write!(
                    formatter,
                    "policy `{policy}` has an indeterminate \
                     condition: `{expression}`"
                )
            }

            Self::ExpressionFailure {
                expression,
                message,
            } => {
                write!(
                    formatter,
                    "failed to evaluate policy expression \
                     `{expression}`: {message}"
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

                other => Err(
                    PolicyEvaluationError::ExpressionFailure {
                        expression: PolicyExpression::new(other)
                            .expect("test expression must be non-empty"),
                        message: "unknown test expression".to_owned(),
                    },
                ),
            }
        }
    }

    fn expression(value: &str) -> PolicyExpression {
        PolicyExpression::new(value)
            .expect("valid test expression")
    }

    fn identifier(value: &str) -> PolicyId {
        PolicyId::new(value)
            .expect("valid test identifier")
    }

    fn directive(
        kind: PolicyDirectiveKind,
        value: &str,
    ) -> PolicyDirective {
        PolicyDirective::with_expression(
            kind,
            expression(value),
        )
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(POLICY_API_VERSION, 0);
    }

    #[test]
    fn identifiers_support_future_domains() {
        assert!(
            PolicyId::new(
                "future::computing::new_domain"
            ).is_ok()
        );
    }

    #[test]
    fn empty_identifiers_are_rejected() {
        assert!(PolicyId::new(" ").is_err());
    }

    #[test]
    fn control_characters_in_identifiers_are_rejected() {
        assert!(PolicyId::new("policy\ninvalid").is_err());
    }

    #[test]
    fn expressions_preserve_source_text() {
        let source = " execution::deterministic ";

        assert_eq!(
            expression(source).as_str(),
            source
        );
    }

    #[test]
    fn expressions_reject_nul() {
        assert!(PolicyExpression::new("a\0b").is_err());
    }

    #[test]
    fn property_keys_are_open_world() {
        assert!(
            PolicyPropertyKey::new(
                "future::optimization::strategy"
            ).is_ok()
        );
    }

    #[test]
    fn empty_property_keys_are_rejected() {
        assert!(PolicyPropertyKey::new(" ").is_err());
    }

    #[test]
    fn requirements_are_mandatory() {
        assert_eq!(
            PolicyDirectiveKind::Requirement.strength(),
            PolicyStrength::Mandatory
        );
    }

    #[test]
    fn constraints_are_mandatory() {
        assert_eq!(
            PolicyDirectiveKind::Constraint.strength(),
            PolicyStrength::Mandatory
        );
    }

    #[test]
    fn preferences_remain_advisory() {
        assert_eq!(
            PolicyDirectiveKind::Preference.strength(),
            PolicyStrength::Advisory
        );
    }

    #[test]
    fn preferences_still_require_downstream_resolution() {
        assert!(
            PolicyDirectiveKind::Preference
                .requires_downstream_resolution()
        );
    }

    #[test]
    fn permissions_are_not_requirements() {
        assert_eq!(
            PolicyDirectiveKind::Permission.class(),
            PolicyClass::AuthorizationIntent
        );
    }

    #[test]
    fn prohibitions_are_not_requirements() {
        assert_eq!(
            PolicyDirectiveKind::Prohibition.class(),
            PolicyClass::AuthorizationIntent
        );
    }

    #[test]
    fn expression_omission_matches_grammar() {
        assert!(
            PolicyDirective::new(
                PolicyDirectiveKind::Determinism,
                None,
            ).is_ok()
        );

        assert!(
            PolicyDirective::new(
                PolicyDirectiveKind::Reproducibility,
                None,
            ).is_ok()
        );

        assert!(
            PolicyDirective::new(
                PolicyDirectiveKind::Requirement,
                None,
            ).is_err()
        );
    }

    #[test]
    fn conditional_action_matches_grammar() {
        assert!(
            PolicyDirectiveKind::Preference
                .is_valid_rule_action()
        );

        assert!(
            !PolicyDirectiveKind::Requirement
                .is_valid_rule_action()
        );

        assert!(
            !PolicyDirectiveKind::Constraint
                .is_valid_rule_action()
        );

        assert!(
            !PolicyDirectiveKind::Capability
                .is_valid_rule_action()
        );

        assert!(
            !PolicyDirectiveKind::Resource
                .is_valid_rule_action()
        );
    }

    #[test]
    fn self_composition_is_rejected() {
        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.extends(identifier("execution::portable"));

        assert!(matches!(
            policy.validate(),
            Err(PolicyValidationError::SelfComposition { .. })
        ));
    }

    #[test]
    fn duplicate_composition_is_rejected() {
        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.extends(identifier("execution::base"));
        policy.overrides(identifier("execution::base"));

        assert!(matches!(
            policy.validate(),
            Err(
                PolicyValidationError::
                    DuplicateCompositionReference { .. }
            )
        ));
    }

    #[test]
    fn declaration_order_is_preserved() {
        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_directive(directive(
            PolicyDirectiveKind::Requirement,
            "classical::compute",
        ));

        policy.add_directive(directive(
            PolicyDirectiveKind::Preference,
            "execution::portable",
        ));

        assert_eq!(policy.len(), 2);

        assert_eq!(
            match &policy.members()[0] {
                PolicyMember::Directive(value) => value.kind(),
                _ => unreachable!(),
            },
            PolicyDirectiveKind::Requirement
        );
    }

    #[test]
    fn false_condition_disables_rule() {
        let rule = PolicyRule::new(
            expression("false"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "portable",
            )),
        );

        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_rule(rule);

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert!(result.is_empty());
        assert_eq!(result.inactive_rule_count(), 1);
    }

    #[test]
    fn true_condition_activates_rule() {
        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "portable",
            )),
        );

        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_rule(rule);

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert_eq!(result.active_actions().len(), 1);
    }

    #[test]
    fn unknown_condition_fails_closed() {
        let rule = PolicyRule::new(
            expression("unknown"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Requirement,
                "required",
            )),
        );

        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_rule(rule);

        assert!(matches!(
            policy.evaluate(&(), &TestEvaluator),
            Err(
                PolicyEvaluationError::
                    IndeterminateCondition { .. }
            )
        ));
    }

    #[test]
    fn additional_conditions_are_conjunctive() {
        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "portable",
            )),
        )
        .with_condition(expression("false"));

        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_rule(rule);

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn property_clauses_are_preserved() {
        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Preference,
                "portable",
            )),
        )
        .with_property(
            PolicyPropertyKey::new("priority").unwrap(),
            expression("10"),
        );

        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_rule(rule);

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert_eq!(
            result.active_actions()[0].clauses().len(),
            1
        );
    }

    #[test]
    fn direct_directives_are_retained() {
        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_directive(
            PolicyDirective::deterministic()
        );

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert_eq!(result.active_actions().len(), 1);
    }

    #[test]
    fn invalid_conditional_directive_is_rejected() {
        let rule = PolicyRule::new(
            expression("true"),
            PolicyAction::Directive(directive(
                PolicyDirectiveKind::Requirement,
                "classical::compute",
            )),
        );

        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_rule(rule);

        assert!(matches!(
            policy.validate(),
            Err(
                PolicyValidationError::InvalidRuleAction { .. }
            )
        ));
    }

    #[test]
    fn active_actions_preserve_source_order() {
        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_directive(directive(
            PolicyDirectiveKind::Requirement,
            "first",
        ));

        policy.add_directive(directive(
            PolicyDirectiveKind::Preference,
            "second",
        ));

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert_eq!(result.active_actions().len(), 2);

        assert_eq!(
            result.active_actions()[0].member_index(),
            0
        );

        assert_eq!(
            result.active_actions()[1].member_index(),
            1
        );
    }

    #[test]
    fn mandatory_and_advisory_actions_remain_distinct() {
        let mut policy = PolicyDocument::named(
            "execution::portable",
        ).unwrap();

        policy.add_directive(directive(
            PolicyDirectiveKind::Requirement,
            "required",
        ));

        policy.add_directive(directive(
            PolicyDirectiveKind::Preference,
            "preferred",
        ));

        let result = policy.evaluate(
            &(),
            &TestEvaluator,
        ).unwrap();

        assert_eq!(
            result.mandatory_actions().count(),
            1
        );

        assert_eq!(
            result.advisory_actions().count(),
            1
        );
    }

    #[test]
    fn empty_policy_is_valid() {
        let policy = PolicyDocument::named(
            "execution::empty",
        ).unwrap();

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn future_computing_domains_are_supported() {
        let mut policy = PolicyDocument::named(
            "future::universal",
        ).unwrap();

        policy.add_directive(directive(
            PolicyDirectiveKind::Capability,
            "future::photonic::compute",
        ));

        assert!(policy.validate().is_ok());
    }
}
