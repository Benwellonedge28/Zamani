
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Symbolic Reasoning
//!
//! Path: `src/ai/reasoning/symbolic.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the AI-domain symbolic reasoning representation and validation
//! interfaces for Zamani.
//!
//! Symbolic reasoning represents mathematical, logical, algebraic and other
//! symbolic computational intentions without requiring their immediate
//! numerical evaluation.
//!
//! This module supports:
//!
//! - Symbolic declarations.
//! - Symbolic expression references.
//! - Symbolic operations.
//! - Symbolic relations.
//! - Assumptions.
//! - Substitutions.
//! - Rewrite rules.
//! - Symbolic constraints.
//! - Symbolic goals.
//! - Symbolic regions.
//! - Extensible operation names.
//! - Canonical AST interoperability.
//! - Structural validation.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITY
//! ============================================================================
//!
//! Source syntax:
//!     grammar/ai/symbolic.g4
//!
//! AI grammar composition:
//!     grammar/ai/ai.g4
//!
//! Canonical AST:
//!     src/frontend/ast/
//!
//! Canonical AST identity:
//!     src/frontend/ast/node/node_id.rs
//!
//! AI semantic authority:
//!     src/frontend/semantic/domains/ai.rs
//!
//! Canonical semantic type system:
//!     src/frontend/semantic/types/
//!
//! Canonical semantic provenance:
//!     src/frontend/semantic/model/provenance.rs
//!
//! AI provenance:
//!     src/ai/provenance.rs
//!
//! Canonical universal IR:
//!     src/zuir/
//!
//! AI IR:
//!     src/zuir/dialects/ai/
//!
//! Knowledge operations:
//!     grammar/expressions/knowledge.g4
//!
//! Knowledge storage:
//!     Existing knowledge infrastructure.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS FILE OWNS:
//!
//! - Symbolic computational intent.
//! - Symbolic operation requests.
//! - Symbolic relations.
//! - Symbolic assumptions.
//! - Symbolic substitutions.
//! - Symbolic rewrite-rule descriptions.
//! - Symbolic constraint descriptions.
//! - Symbolic goal descriptions.
//! - Symbolic region descriptions.
//! - Symbolic structural validation.
//!
//! THIS FILE DOES NOT OWN:
//!
//! - Source syntax.
//! - Lexical tokens.
//! - Expression parsing.
//! - Canonical AST construction.
//! - General semantic type checking.
//! - General name resolution.
//! - Theorem proving.
//! - Symbolic solver implementations.
//! - Rewrite engine execution.
//! - Numerical computation.
//! - Automatic differentiation.
//! - Knowledge graph storage.
//! - Knowledge persistence.
//! - Tensor storage.
//! - Universal intermediate representation.
//! - Hardware discovery.
//! - Device allocation.
//! - Routing.
//! - Scheduling.
//! - Distributed transport.
//! - Runtime execution.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! This module introduces no language-level maximum for:
//!
//! - symbolic declarations;
//! - expressions;
//! - variables;
//! - operations;
//! - relations;
//! - assumptions;
//! - substitutions;
//! - rewrite rules;
//! - constraints;
//! - goals;
//! - regions;
//! - mathematical dimensions;
//! - polynomial degrees;
//! - reasoning participants;
//! - computational targets.
//!
//! Collections grow according to supplied input and available resources.
//!
//! Rust's finite address space and allocation behavior remain implementation
//! constraints. Literal infinite computation is not promised.
//!
//! Operational budgets belong to the responsible compiler or execution layer.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! grammar/ai/symbolic.g4
//!       |
//!       v
//! Canonical frontend AST
//!       |
//!       v
//! AST expression references
//!       |
//!       v
//! Symbolic semantic adapter
//!       |
//!       v
//! This module
//!       |
//!       v
//! AI semantic validation
//!       |
//!       v
//! ZUIR AI dialect
//!       |
//!       v
//! Optimization / lowering
//!       |
//!       v
//! Appropriate computational realization
//!
//! The adapter must preserve canonical AST identities. It must not translate
//! ordinary source expressions into a competing symbolic AST.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! - Ordered collections are used for deterministic validation.
//! - Operation names are explicit and extensible.
//! - No global mutable state is introduced.
//! - No randomness or wall-clock dependency is introduced.
//! - Validation does not execute symbolic operations.
//!
//! ============================================================================

use std::collections::BTreeSet;
use std::fmt;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
};

use crate::frontend::ast::node::node_id::NodeId;

/// Result type used by symbolic reasoning operations.
pub type SymbolicResult<T> = Result<T, AiError>;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn symbolic_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

// ============================================================================
// QUALIFIED SYMBOLIC NAMES
// ============================================================================

/// An extensible qualified symbolic identity.
///
/// Examples:
///
///     core.algebra.add
///     core.logic.equal
///     mathematics.polynomial.factor
///     mathematics.calculus.integrate
///     user.custom.symbolic_operation
///
/// The name is an identity, not an executable operation.
///
/// Its semantics must be established by the appropriate semantic registry
/// or downstream implementation.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct SymbolicName {
    namespace: String,
    name: String,
}

impl SymbolicName {
    /// Construct a qualified symbolic name.
    ///
    /// Names must be nonempty and must not contain control characters.
    ///
    /// Whitespace is not silently normalized.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> SymbolicResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_name_component("namespace", &namespace)?;
        validate_name_component("name", &name)?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the symbolic name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return a qualified textual representation.
    #[must_use]
    pub fn qualified(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

fn validate_name_component(
    component: &'static str,
    value: &str,
) -> SymbolicResult<()> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(symbolic_error(
            AiErrorKind::InvalidInput,
            format!(
                "symbolic {component} must be nonempty and contain no control characters"
            ),
        ));
    }

    Ok(())
}

impl fmt::Display for SymbolicName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// CANONICAL EXPRESSION REFERENCES
// ============================================================================

/// A reference to an expression in the canonical frontend AST.
///
/// This type deliberately does not contain a second expression tree.
///
/// The referenced node must belong to the AST graph associated with the
/// current symbolic analysis.
///
/// Node identity does not prove that the referenced node is an expression.
/// That must be checked through `SymbolicNodeResolver`.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct SymbolicExpressionRef {
    node_id: NodeId,
}

impl SymbolicExpressionRef {
    /// Construct a reference to an existing or expected canonical AST node.
    ///
    /// Actual node existence and node category are checked during validation.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
        }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

// ============================================================================
// SYMBOLIC DECLARATIONS
// ============================================================================

/// A symbolic declaration.
///
/// The declaration references the canonical AST node representing its
/// optional initializer.
///
/// Its type is owned by the canonical semantic type system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicDeclaration {
    source_node: NodeId,
    role: SymbolicName,
    name: SymbolicName,
    initializer: Option<SymbolicExpressionRef>,
}

impl SymbolicDeclaration {
    /// Construct a symbolic declaration.
    pub fn new(
        source_node: NodeId,
        role: SymbolicName,
        name: SymbolicName,
        initializer: Option<SymbolicExpressionRef>,
    ) -> Self {
        Self {
            source_node,
            role,
            name,
            initializer,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub fn role(&self) -> &SymbolicName {
        &self.role
    }

    #[must_use]
    pub fn name(&self) -> &SymbolicName {
        &self.name
    }

    #[must_use]
    pub const fn initializer(
        &self,
    ) -> Option<SymbolicExpressionRef> {
        self.initializer
    }
}

// ============================================================================
// SYMBOLIC OPERATION REQUESTS
// ============================================================================

/// An argument supplied to a symbolic operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicArgument {
    expression: SymbolicExpressionRef,
    name: Option<SymbolicName>,
}

impl SymbolicArgument {
    /// Construct a positional argument.
    #[must_use]
    pub const fn positional(
        expression: SymbolicExpressionRef,
    ) -> Self {
        Self {
            expression,
            name: None,
        }
    }

    /// Construct a named argument.
    #[must_use]
    pub fn named(
        name: SymbolicName,
        expression: SymbolicExpressionRef,
    ) -> Self {
        Self {
            expression,
            name: Some(name),
        }
    }

    #[must_use]
    pub const fn expression(
        &self,
    ) -> SymbolicExpressionRef {
        self.expression
    }

    #[must_use]
    pub fn name(&self) -> Option<&SymbolicName> {
        self.name.as_ref()
    }
}

/// An extensible symbolic operation request.
///
/// Examples:
///
///     simplify(expression)
///     expand(expression)
///     factor(expression)
///     solve(equation, variable)
///     substitute(expression, variable, replacement)
///     custom_namespace.operation(expression)
///
/// The operation name does not imply that an implementation exists.
///
/// This type records intent only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicOperationRequest {
    source_node: NodeId,
    operation: SymbolicName,
    arguments: Vec<SymbolicArgument>,
    clauses: Vec<SymbolicClause>,
}

impl SymbolicOperationRequest {
    pub fn new(
        source_node: NodeId,
        operation: SymbolicName,
        arguments: Vec<SymbolicArgument>,
    ) -> Self {
        Self {
            source_node,
            operation,
            arguments,
            clauses: Vec::new(),
        }
    }

    /// Attach an extensible semantic clause.
    #[must_use]
    pub fn with_clause(
        mut self,
        clause: SymbolicClause,
    ) -> Self {
        self.clauses.push(clause);
        self
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub fn operation(&self) -> &SymbolicName {
        &self.operation
    }

    #[must_use]
    pub fn arguments(&self) -> &[SymbolicArgument] {
        &self.arguments
    }

    #[must_use]
    pub fn clauses(&self) -> &[SymbolicClause] {
        &self.clauses
    }
}

/// An extensible symbolic operation clause.
///
/// The clause value is a reference to a canonical expression.
///
/// This module does not reinterpret the expression's type or meaning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicClause {
    name: SymbolicName,
    value: SymbolicExpressionRef,
}

impl SymbolicClause {
    #[must_use]
    pub fn new(
        name: SymbolicName,
        value: SymbolicExpressionRef,
    ) -> Self {
        Self {
            name,
            value,
        }
    }

    #[must_use]
    pub fn name(&self) -> &SymbolicName {
        &self.name
    }

    #[must_use]
    pub const fn value(
        &self,
    ) -> SymbolicExpressionRef {
        self.value
    }
}

// ============================================================================
// SYMBOLIC RELATIONS
// ============================================================================

/// A symbolic relation.
///
/// The relation identity is open-world. It can represent equality,
/// inequality, ordering, equivalence, membership, or an extension-defined
/// mathematical relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicRelation {
    source_node: NodeId,
    relation: SymbolicName,
    operands: Vec<SymbolicExpressionRef>,
}

impl SymbolicRelation {
    pub fn new(
        source_node: NodeId,
        relation: SymbolicName,
        operands: Vec<SymbolicExpressionRef>,
    ) -> Self {
        Self {
            source_node,
            relation,
            operands,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub fn relation(&self) -> &SymbolicName {
        &self.relation
    }

    #[must_use]
    pub fn operands(&self) -> &[SymbolicExpressionRef] {
        &self.operands
    }
}

// ============================================================================
// SYMBOLIC ASSUMPTIONS
// ============================================================================

/// The polarity of a symbolic assumption.
///
/// An assumption is not automatically a proven fact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssumptionPolarity {
    Assume,
    Negate,
}

/// A symbolic assumption.
///
/// The expression remains a canonical AST expression reference.
///
/// Consistency and satisfiability are delegated to semantic analysis
/// and the appropriate reasoning implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicAssumption {
    source_node: NodeId,
    expression: SymbolicExpressionRef,
    polarity: AssumptionPolarity,
}

impl SymbolicAssumption {
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        expression: SymbolicExpressionRef,
        polarity: AssumptionPolarity,
    ) -> Self {
        Self {
            source_node,
            expression,
            polarity,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub const fn expression(
        &self,
    ) -> SymbolicExpressionRef {
        self.expression
    }

    #[must_use]
    pub const fn polarity(&self) -> AssumptionPolarity {
        self.polarity
    }
}

// ============================================================================
// SYMBOLIC SUBSTITUTIONS
// ============================================================================

/// A symbolic substitution request.
///
/// Substitution is expressed as a relationship between canonical
/// expressions, not as an independently parsed expression tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicSubstitution {
    source_node: NodeId,
    target: SymbolicExpressionRef,
    replacement: SymbolicExpressionRef,
}

impl SymbolicSubstitution {
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        target: SymbolicExpressionRef,
        replacement: SymbolicExpressionRef,
    ) -> Self {
        Self {
            source_node,
            target,
            replacement,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub const fn target(
        &self,
    ) -> SymbolicExpressionRef {
        self.target
    }

    #[must_use]
    pub const fn replacement(
        &self,
    ) -> SymbolicExpressionRef {
        self.replacement
    }
}

// ============================================================================
// SYMBOLIC REWRITE RULES
// ============================================================================

/// The direction in which a symbolic rewrite rule may be applied.
///
/// This is declarative metadata. It does not execute a rewrite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewriteDirection {
    Forward,
    Reverse,
    Bidirectional,
}

/// A symbolic rewrite rule.
///
/// Whether a rewrite is valid, terminating, confluent, or semantics
/// preserving must be established by the appropriate reasoning engine.
///
/// No rewrite is performed by constructing this value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicRewriteRule {
    source_node: NodeId,
    name: SymbolicName,
    left: SymbolicExpressionRef,
    right: SymbolicExpressionRef,
    direction: RewriteDirection,
}

impl SymbolicRewriteRule {
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        name: SymbolicName,
        left: SymbolicExpressionRef,
        right: SymbolicExpressionRef,
        direction: RewriteDirection,
    ) -> Self {
        Self {
            source_node,
            name,
            left,
            right,
            direction,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub fn name(&self) -> &SymbolicName {
        &self.name
    }

    #[must_use]
    pub const fn left(&self) -> SymbolicExpressionRef {
        self.left
    }

    #[must_use]
    pub const fn right(&self) -> SymbolicExpressionRef {
        self.right
    }

    #[must_use]
    pub const fn direction(&self) -> RewriteDirection {
        self.direction
    }
}

// ============================================================================
// SYMBOLIC CONSTRAINTS
// ============================================================================

/// The semantic classification of a symbolic obligation.
///
/// These distinctions must remain explicit throughout compilation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicObligationKind {
    /// Necessary for correctness.
    Requirement,

    /// A restriction that must be respected.
    Constraint,

    /// A desired but non-binding characteristic.
    Preference,

    /// Optional optimization guidance.
    Hint,
}

/// A symbolic constraint or obligation.
///
/// Its satisfiability is not established by this structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicObligation {
    source_node: NodeId,
    kind: SymbolicObligationKind,
    expression: SymbolicExpressionRef,
}

impl SymbolicObligation {
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        kind: SymbolicObligationKind,
        expression: SymbolicExpressionRef,
    ) -> Self {
        Self {
            source_node,
            kind,
            expression,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub const fn kind(&self) -> SymbolicObligationKind {
        self.kind
    }

    #[must_use]
    pub const fn expression(
        &self,
    ) -> SymbolicExpressionRef {
        self.expression
    }
}

// ============================================================================
// SYMBOLIC GOALS
// ============================================================================

/// The kind of symbolic goal.
///
/// Custom goal types remain possible through `SymbolicName`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolicGoalKind {
    Prove,
    Satisfy,
    Optimize,
    Evaluate,
    Custom(SymbolicName),
}

/// A symbolic goal.
///
/// A goal describes what should be established or achieved.
///
/// It does not claim that the goal is achievable or proven.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicGoal {
    source_node: NodeId,
    kind: SymbolicGoalKind,
    expression: SymbolicExpressionRef,
}

impl SymbolicGoal {
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        kind: SymbolicGoalKind,
        expression: SymbolicExpressionRef,
    ) -> Self {
        Self {
            source_node,
            kind,
            expression,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub fn kind(&self) -> &SymbolicGoalKind {
        &self.kind
    }

    #[must_use]
    pub const fn expression(
        &self,
    ) -> SymbolicExpressionRef {
        self.expression
    }
}

// ============================================================================
// SYMBOLIC REGIONS
// ============================================================================

/// A symbolic region declaration.
///
/// The region is represented by its canonical AST node identity.
///
/// Parent relationships are explicit and can be validated without
/// recursively traversing an independently constructed symbolic AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicRegion {
    source_node: NodeId,
    name: SymbolicName,
    parent: Option<NodeId>,
}

impl SymbolicRegion {
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        name: SymbolicName,
        parent: Option<NodeId>,
    ) -> Self {
        Self {
            source_node,
            name,
            parent,
        }
    }

    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    #[must_use]
    pub fn name(&self) -> &SymbolicName {
        &self.name
    }

    #[must_use]
    pub const fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

// ============================================================================
// CANONICAL AST RESOLUTION
// ============================================================================

/// Canonical AST lookup interface.
///
/// The frontend adapter implements this interface using the actual AST
/// belonging to the current compilation.
///
/// This module does not create an AST registry or AST storage system.
pub trait SymbolicNodeResolver {
    /// Return true only if the node exists in the current AST and represents
    /// a canonical expression node.
    fn is_expression_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true only if the node exists and represents a canonical
    /// symbolic construct or declaration.
    fn is_symbolic_construct_node(
        &self,
        node_id: NodeId,
    ) -> bool;
}

// ============================================================================
// SYMBOLIC PROGRAM
// ============================================================================

/// A collection of symbolic computational intentions.
///
/// This is not an AST and is not an executable IR.
///
/// All expression references point to canonical frontend AST nodes.
///
/// The vectors impose no language-level item ceiling.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolicProgram {
    declarations: Vec<SymbolicDeclaration>,
    operations: Vec<SymbolicOperationRequest>,
    relations: Vec<SymbolicRelation>,
    assumptions: Vec<SymbolicAssumption>,
    substitutions: Vec<SymbolicSubstitution>,
    rewrite_rules: Vec<SymbolicRewriteRule>,
    obligations: Vec<SymbolicObligation>,
    goals: Vec<SymbolicGoal>,
    regions: Vec<SymbolicRegion>,
}

impl SymbolicProgram {
    /// Construct an empty symbolic program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn declarations(&self) -> &[SymbolicDeclaration] {
        &self.declarations
    }

    #[must_use]
    pub fn operations(&self) -> &[SymbolicOperationRequest] {
        &self.operations
    }

    #[must_use]
    pub fn relations(&self) -> &[SymbolicRelation] {
        &self.relations
    }

    #[must_use]
    pub fn assumptions(&self) -> &[SymbolicAssumption] {
        &self.assumptions
    }

    #[must_use]
    pub fn substitutions(&self) -> &[SymbolicSubstitution] {
        &self.substitutions
    }

    #[must_use]
    pub fn rewrite_rules(&self) -> &[SymbolicRewriteRule] {
        &self.rewrite_rules
    }

    #[must_use]
    pub fn obligations(&self) -> &[SymbolicObligation] {
        &self.obligations
    }

    #[must_use]
    pub fn goals(&self) -> &[SymbolicGoal] {
        &self.goals
    }

    #[must_use]
    pub fn regions(&self) -> &[SymbolicRegion] {
        &self.regions
    }

    /// Add a symbolic declaration.
    pub fn add_declaration(
        &mut self,
        declaration: SymbolicDeclaration,
    ) {
        self.declarations.push(declaration);
    }

    /// Add a symbolic operation request.
    pub fn add_operation(
        &mut self,
        operation: SymbolicOperationRequest,
    ) {
        self.operations.push(operation);
    }

    /// Add a symbolic relation.
    pub fn add_relation(
        &mut self,
        relation: SymbolicRelation,
    ) {
        self.relations.push(relation);
    }

    /// Add an assumption.
    pub fn add_assumption(
        &mut self,
        assumption: SymbolicAssumption,
    ) {
        self.assumptions.push(assumption);
    }

    /// Add a substitution request.
    pub fn add_substitution(
        &mut self,
        substitution: SymbolicSubstitution,
    ) {
        self.substitutions.push(substitution);
    }

    /// Add a rewrite rule.
    pub fn add_rewrite_rule(
        &mut self,
        rule: SymbolicRewriteRule,
    ) {
        self.rewrite_rules.push(rule);
    }

    /// Add a symbolic obligation.
    pub fn add_obligation(
        &mut self,
        obligation: SymbolicObligation,
    ) {
        self.obligations.push(obligation);
    }

    /// Add a symbolic goal.
    pub fn add_goal(
        &mut self,
        goal: SymbolicGoal,
    ) {
        self.goals.push(goal);
    }

    /// Add a symbolic region.
    pub fn add_region(
        &mut self,
        region: SymbolicRegion,
    ) {
        self.regions.push(region);
    }

    /// Validate structural integrity and canonical AST references.
    ///
    /// This method:
    ///
    /// - rejects duplicate symbolic construct identities;
    /// - checks expression references against the canonical AST;
    /// - checks symbolic construct references;
    /// - rejects missing region parents;
    /// - detects symbolic region parent cycles.
    ///
    /// It does not establish mathematical truth, satisfiability,
    /// type correctness, operation availability, or hardware feasibility.
    pub fn validate<R: SymbolicNodeResolver>(
        &self,
        resolver: &R,
    ) -> SymbolicResult<SymbolicValidationReport> {
        let mut report = SymbolicValidationReport::default();

        let mut construct_ids = BTreeSet::new();

        let mut region_ids = BTreeSet::new();

        // -----------------------------------------------------------------
        // Declarations
        // -----------------------------------------------------------------

        for declaration in &self.declarations {
            register_construct(
                &mut construct_ids,
                declaration.source_node,
            )?;

            validate_construct_node(
                resolver,
                declaration.source_node,
            )?;

            if let Some(expression) = declaration.initializer {
                validate_expression(
                    resolver,
                    expression,
                    &mut report,
                )?;
            }

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Operations
        // -----------------------------------------------------------------

        for operation in &self.operations {
            register_construct(
                &mut construct_ids,
                operation.source_node,
            )?;

            validate_construct_node(
                resolver,
                operation.source_node,
            )?;

            for argument in &operation.arguments {
                validate_expression(
                    resolver,
                    argument.expression,
                    &mut report,
                )?;
            }

            for clause in &operation.clauses {
                validate_expression(
                    resolver,
                    clause.value,
                    &mut report,
                )?;
            }

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Relations
        // -----------------------------------------------------------------

        for relation in &self.relations {
            register_construct(
                &mut construct_ids,
                relation.source_node,
            )?;

            validate_construct_node(
                resolver,
                relation.source_node,
            )?;

            for operand in &relation.operands {
                validate_expression(
                    resolver,
                    *operand,
                    &mut report,
                )?;
            }

            if relation.operands.is_empty() {
                return Err(symbolic_error(
                    AiErrorKind::InvalidInput,
                    "a symbolic relation must reference at least one operand",
                ));
            }

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Assumptions
        // -----------------------------------------------------------------

        for assumption in &self.assumptions {
            register_construct(
                &mut construct_ids,
                assumption.source_node,
            )?;

            validate_construct_node(
                resolver,
                assumption.source_node,
            )?;

            validate_expression(
                resolver,
                assumption.expression,
                &mut report,
            )?;

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Substitutions
        // -----------------------------------------------------------------

        for substitution in &self.substitutions {
            register_construct(
                &mut construct_ids,
                substitution.source_node,
            )?;

            validate_construct_node(
                resolver,
                substitution.source_node,
            )?;

            validate_expression(
                resolver,
                substitution.target,
                &mut report,
            )?;

            validate_expression(
                resolver,
                substitution.replacement,
                &mut report,
            )?;

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Rewrite rules
        // -----------------------------------------------------------------

        for rule in &self.rewrite_rules {
            register_construct(
                &mut construct_ids,
                rule.source_node,
            )?;

            validate_construct_node(
                resolver,
                rule.source_node,
            )?;

            validate_expression(
                resolver,
                rule.left,
                &mut report,
            )?;

            validate_expression(
                resolver,
                rule.right,
                &mut report,
            )?;

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Obligations
        // -----------------------------------------------------------------

        for obligation in &self.obligations {
            register_construct(
                &mut construct_ids,
                obligation.source_node,
            )?;

            validate_construct_node(
                resolver,
                obligation.source_node,
            )?;

            validate_expression(
                resolver,
                obligation.expression,
                &mut report,
            )?;

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Goals
        // -----------------------------------------------------------------

        for goal in &self.goals {
            register_construct(
                &mut construct_ids,
                goal.source_node,
            )?;

            validate_construct_node(
                resolver,
                goal.source_node,
            )?;

            validate_expression(
                resolver,
                goal.expression,
                &mut report,
            )?;

            report.record_construct()?;
        }

        // -----------------------------------------------------------------
        // Regions
        // -----------------------------------------------------------------

        for region in &self.regions {
            register_construct(
                &mut construct_ids,
                region.source_node,
            )?;

            validate_construct_node(
                resolver,
                region.source_node,
            )?;

            if !region_ids.insert(region.source_node) {
                return Err(symbolic_error(
                    AiErrorKind::Conflict,
                    "duplicate symbolic region identity",
                ));
            }

            if let Some(parent) = region.parent {
                if !region_ids.contains(&parent)
                    && !self.regions.iter().any(
                        |candidate| candidate.source_node == parent
                    )
                {
                    return Err(symbolic_error(
                        AiErrorKind::NotFound,
                        "symbolic region parent does not exist",
                    ));
                }
            }

            report.record_construct()?;
        }

        validate_region_cycles(&self.regions)?;

        report.complete = true;

        Ok(report)
    }
}

// ============================================================================
// STRUCTURAL VALIDATION HELPERS
// ============================================================================

fn register_construct(
    identities: &mut BTreeSet<NodeId>,
    node_id: NodeId,
) -> SymbolicResult<()> {
    if !identities.insert(node_id) {
        return Err(symbolic_error(
            AiErrorKind::Conflict,
            "a canonical AST node is registered as multiple symbolic constructs",
        ));
    }

    Ok(())
}

fn validate_construct_node<R: SymbolicNodeResolver>(
    resolver: &R,
    node_id: NodeId,
) -> SymbolicResult<()> {
    if !resolver.is_symbolic_construct_node(node_id) {
        return Err(symbolic_error(
            AiErrorKind::InvalidInput,
            "symbolic construct does not reference a valid canonical AST construct",
        ));
    }

    Ok(())
}

fn validate_expression<R: SymbolicNodeResolver>(
    resolver: &R,
    expression: SymbolicExpressionRef,
    report: &mut SymbolicValidationReport,
) -> SymbolicResult<()> {
    if !resolver.is_expression_node(expression.node_id) {
        return Err(symbolic_error(
            AiErrorKind::InvalidInput,
            "symbolic expression reference does not identify a canonical AST expression",
        ));
    }

    report.record_expression_reference()?;

    Ok(())
}

/// Validate parent relationships without recursive traversal.
///
/// This prevents deep region nesting from consuming the Rust call stack.
fn validate_region_cycles(
    regions: &[SymbolicRegion],
) -> SymbolicResult<()> {
    use std::collections::BTreeMap;

    let mut parents = BTreeMap::new();

    for region in regions {
        parents.insert(
            region.source_node,
            region.parent,
        );
    }

    let mut completed = BTreeSet::new();

    for start in parents.keys().copied() {
        if completed.contains(&start) {
            continue;
        }

        let mut current_path = BTreeSet::new();
        let mut current = Some(start);

        while let Some(node) = current {
            if completed.contains(&node) {
                break;
            }

            if !current_path.insert(node) {
                return Err(symbolic_error(
                    AiErrorKind::InvalidInput,
                    "symbolic region hierarchy contains a parent cycle",
                ));
            }

            current = parents.get(&node).copied().flatten();
        }

        completed.extend(current_path);
    }

    Ok(())
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic symbolic structural-validation report.
///
/// A successful report establishes only the structural properties checked
/// by this module.
///
/// It is not a mathematical proof or a complete semantic validation result.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolicValidationReport {
    constructs_checked: usize,
    expression_references_checked: usize,
    complete: bool,
}

impl SymbolicValidationReport {
    fn record_construct(&mut self) -> SymbolicResult<()> {
        self.constructs_checked = self.constructs_checked
            .checked_add(1)
            .ok_or_else(|| {
                symbolic_error(
                    AiErrorKind::ResourceExhausted,
                    "symbolic validation counter exhausted its representation",
                )
            })?;

        Ok(())
    }

    fn record_expression_reference(
        &mut self,
    ) -> SymbolicResult<()> {
        self.expression_references_checked =
            self.expression_references_checked
                .checked_add(1)
                .ok_or_else(|| {
                    symbolic_error(
                        AiErrorKind::ResourceExhausted,
                        "symbolic expression reference counter exhausted its representation",
                    )
                })?;

        Ok(())
    }

    #[must_use]
    pub const fn constructs_checked(&self) -> usize {
        self.constructs_checked
    }

    #[must_use]
    pub const fn expression_references_checked(
        &self,
    ) -> usize {
        self.expression_references_checked
    }

    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    struct TestResolver {
        expressions: BTreeSet<NodeId>,
        constructs: BTreeSet<NodeId>,
    }

    impl TestResolver {
        fn new() -> Self {
            Self {
                expressions: BTreeSet::new(),
                constructs: BTreeSet::new(),
            }
        }

        fn expression(&mut self, id: u64) -> NodeId {
            let id = NodeId::new(id)
                .expect("test AST node identity must be nonzero");

            self.expressions.insert(id);

            id
        }

        fn construct(&mut self, id: u64) -> NodeId {
            let id = NodeId::new(id)
                .expect("test AST node identity must be nonzero");

            self.constructs.insert(id);

            id
        }
    }

    impl SymbolicNodeResolver for TestResolver {
        fn is_expression_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.expressions.contains(&node_id)
        }

        fn is_symbolic_construct_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.constructs.contains(&node_id)
        }
    }

    fn name(
        namespace: &str,
        name: &str,
    ) -> SymbolicName {
        SymbolicName::new(namespace, name).unwrap()
    }

    fn expression(
        id: NodeId,
    ) -> SymbolicExpressionRef {
        SymbolicExpressionRef::new(id)
    }

    #[test]
    fn accepts_extensible_symbolic_names() {
        let operation = name(
            "custom.mathematics",
            "arbitrary_operation",
        );

        assert_eq!(
            operation.qualified(),
            "custom.mathematics.arbitrary_operation"
        );
    }

    #[test]
    fn rejects_empty_symbolic_names() {
        assert!(
            SymbolicName::new("", "operation").is_err()
        );

        assert!(
            SymbolicName::new("namespace", "").is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_names() {
        assert!(
            SymbolicName::new(
                "namespace",
                "invalid\nname"
            ).is_err()
        );
    }

    #[test]
    fn validates_a_symbolic_operation() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);
        let input = resolver.expression(2);

        let operation = SymbolicOperationRequest::new(
            source,
            name("mathematics", "simplify"),
            vec![
                SymbolicArgument::positional(
                    expression(input)
                ),
            ],
        );

        let mut program = SymbolicProgram::new();

        program.add_operation(operation);

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(report.constructs_checked(), 1);

        assert_eq!(
            report.expression_references_checked(),
            1
        );
    }

    #[test]
    fn validates_symbolic_assumptions() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);
        let condition = resolver.expression(2);

        let mut program = SymbolicProgram::new();

        program.add_assumption(
            SymbolicAssumption::new(
                source,
                expression(condition),
                AssumptionPolarity::Assume,
            ),
        );

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());
    }

    #[test]
    fn rejects_unknown_expression_references() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);

        let missing = NodeId::new(2).unwrap();

        let mut program = SymbolicProgram::new();

        program.add_assumption(
            SymbolicAssumption::new(
                source,
                expression(missing),
                AssumptionPolarity::Assume,
            ),
        );

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_unknown_symbolic_constructs() {
        let resolver = TestResolver::new();

        let source = NodeId::new(1).unwrap();

        let mut program = SymbolicProgram::new();

        program.add_declaration(
            SymbolicDeclaration::new(
                source,
                name("symbolic", "variable"),
                name("user", "x"),
                None,
            ),
        );

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_duplicate_construct_identities() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);

        let mut program = SymbolicProgram::new();

        program.add_declaration(
            SymbolicDeclaration::new(
                source,
                name("symbolic", "variable"),
                name("user", "x"),
                None,
            ),
        );

        program.add_declaration(
            SymbolicDeclaration::new(
                source,
                name("symbolic", "variable"),
                name("user", "y"),
                None,
            ),
        );

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_empty_relations() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);

        let mut program = SymbolicProgram::new();

        program.add_relation(
            SymbolicRelation::new(
                source,
                name("logic", "equal"),
                Vec::new(),
            ),
        );

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn validates_substitution_references() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);
        let target = resolver.expression(2);
        let replacement = resolver.expression(3);

        let mut program = SymbolicProgram::new();

        program.add_substitution(
            SymbolicSubstitution::new(
                source,
                expression(target),
                expression(replacement),
            ),
        );

        assert!(
            program.validate(&resolver).unwrap().is_complete()
        );
    }

    #[test]
    fn validates_rewrite_rule_references() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);
        let left = resolver.expression(2);
        let right = resolver.expression(3);

        let mut program = SymbolicProgram::new();

        program.add_rewrite_rule(
            SymbolicRewriteRule::new(
                source,
                name("algebra", "identity"),
                expression(left),
                expression(right),
                RewriteDirection::Bidirectional,
            ),
        );

        assert!(
            program.validate(&resolver).unwrap().is_complete()
        );
    }

    #[test]
    fn validates_symbolic_goals() {
        let mut resolver = TestResolver::new();

        let source = resolver.construct(1);
        let target = resolver.expression(2);

        let mut program = SymbolicProgram::new();

        program.add_goal(
            SymbolicGoal::new(
                source,
                SymbolicGoalKind::Prove,
                expression(target),
            ),
        );

        assert!(
            program.validate(&resolver).unwrap().is_complete()
        );
    }

    #[test]
    fn accepts_extensible_custom_goals() {
        let custom = SymbolicGoalKind::Custom(
            name("custom.reasoning", "establish_invariant")
        );

        assert!(
            matches!(
                custom,
                SymbolicGoalKind::Custom(_)
            )
        );
    }

    #[test]
    fn rejects_symbolic_region_cycles() {
        let mut resolver = TestResolver::new();

        let first = resolver.construct(1);
        let second = resolver.construct(2);

        let mut program = SymbolicProgram::new();

        program.add_region(
            SymbolicRegion::new(
                first,
                name("symbolic", "first"),
                Some(second),
            ),
        );

        program.add_region(
            SymbolicRegion::new(
                second,
                name("symbolic", "second"),
                Some(first),
            ),
        );

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn accepts_large_symbolic_collections_without_domain_ceilings() {
        let mut resolver = TestResolver::new();

        let mut program = SymbolicProgram::new();

        for index in 1_u64..=1024 {
            let source = resolver.construct(index);

            let declaration_name = SymbolicName::new(
                "user",
                format!("symbol_{index}"),
            ).unwrap();

            program.add_declaration(
                SymbolicDeclaration::new(
                    source,
                    name("symbolic", "variable"),
                    declaration_name,
                    None,
                ),
            );
        }

        let report = program.validate(&resolver).unwrap();

        assert_eq!(
            report.constructs_checked(),
            1024
        );

        assert!(report.is_complete());
    }

    #[test]
    fn empty_symbolic_program_is_valid() {
        let resolver = TestResolver::new();

        let program = SymbolicProgram::new();

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(report.constructs_checked(), 0);
    }

    #[test]
    fn symbolic_obligation_categories_remain_distinct() {
        assert_ne!(
            SymbolicObligationKind::Requirement,
            SymbolicObligationKind::Preference,
        );

        assert_ne!(
            SymbolicObligationKind::Constraint,
            SymbolicObligationKind::Hint,
        );
    }
}
