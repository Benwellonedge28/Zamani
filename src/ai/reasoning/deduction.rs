
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Deductive Reasoning Domain
//!
//! Path: `src/ai/reasoning/deduction.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the AI-domain representation of deductive reasoning constructs
//! declared by `grammar/ai/deduction.g4`.
//!
//! This module preserves references to the canonical frontend AST. It does
//! not construct a second AST, implement a theorem prover, or execute
//! deduction.
//!
//! # Grammar authority
//!
//! The source grammar owns the syntax of:
//!
//! - deduction declarations;
//! - deduction invocations;
//! - deduction expressions;
//! - direct derivations;
//! - structured deduction statements;
//! - premises and conclusions;
//! - assumptions, observations and evidence;
//! - named rules and conditions;
//! - derivations;
//! - explanations and provenance;
//! - metadata and open directives;
//! - statement modifiers;
//! - proof, verification and validation boundaries.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - deduction-domain declaration descriptions;
//! - deduction invocation descriptions;
//! - deduction argument descriptions;
//! - deduction statement/member descriptions;
//! - deduction-specific AST reference validation;
//! - deterministic structural validation reports.
//!
//! This module does not own:
//!
//! - parsing or lexical analysis;
//! - AST construction;
//! - general expression or type representations;
//! - name resolution;
//! - theorem proving;
//! - SAT/SMT solving;
//! - constraint solving;
//! - knowledge storage;
//! - evidence storage;
//! - probabilistic reasoning;
//! - inference execution;
//! - model execution;
//! - tensor storage;
//! - hardware discovery;
//! - resource allocation;
//! - scheduling;
//! - distributed transport;
//! - ZUIR construction;
//! - runtime execution.
//!
//! # POCO-REAF
//!
//! This module imposes no language-level maximum on:
//!
//! - deduction declarations;
//! - deduction invocations;
//! - parameters;
//! - arguments;
//! - premises;
//! - evidence references;
//! - assumptions;
//! - observations;
//! - rules;
//! - conditions;
//! - derivations;
//! - modifiers;
//! - metadata;
//! - directives;
//! - nested directive members;
//! - proof obligations;
//! - AST expression references.
//!
//! Collections grow according to supplied input and available resources.
//! Rust's finite address space and allocation behavior remain implementation
//! constraints. Literal infinite computation is not promised.
//!
//! Operational budgets belong to the compiler, configuration, resource
//! analysis and execution infrastructure.
//!
//! # Integration pathway
//!
//! `grammar/ai/deduction.g4`
//!       |
//!       v
//! Canonical frontend AST
//!       |
//!       v
//! Deduction AST adapter
//!       |
//!       v
//! `DeductionProgram`
//!       |
//!       v
//! Canonical semantic validation
//!       |
//!       v
//! ZUIR AI dialect
//!       |
//!       v
//! Optimization and lowering
//!       |
//!       v
//! Execution infrastructure
//!
//! The adapter must preserve canonical AST identities and source provenance.
//!
//! # Important semantic boundary
//!
//! Structural validation establishes that references identify nodes of the
//! expected AST categories. It does not establish that a deduction is
//! mathematically valid. Logical validity belongs to the appropriate
//! registered reasoning implementation and semantic services.

use std::collections::BTreeSet;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::frontend::ast::node::node_id::NodeId;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn deduction_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

// ============================================================================
// CANONICAL AST REFERENCES
// ============================================================================

/// Reference to an expression in the canonical frontend AST.
///
/// This type contains only the canonical node identity. It does not contain
/// or recreate an expression tree.
///
/// Existence and expression category are checked by
/// [`DeductionNodeResolver`].
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
pub struct DeductionExpressionRef {
    node_id: NodeId,
}

impl DeductionExpressionRef {
    /// Construct a reference to a canonical expression node.
    ///
    /// This constructor does not establish that the referenced node exists.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

/// Reference to a type expression in the canonical frontend AST.
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
pub struct DeductionTypeRef {
    node_id: NodeId,
}

impl DeductionTypeRef {
    /// Construct a reference to a canonical type-expression node.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

/// Reference to a deduction construct in the canonical frontend AST.
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
pub struct DeductionConstructRef {
    node_id: NodeId,
}

impl DeductionConstructRef {
    /// Construct a reference to a canonical deduction construct.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

/// Reference to an ordinary canonical Zamani statement.
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
pub struct DeductionStatementRef {
    node_id: NodeId,
}

impl DeductionStatementRef {
    /// Construct a reference to a canonical statement.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

// ============================================================================
// OPEN-ENDED IDENTIFIERS
// ============================================================================

/// An extensible deduction-domain identifier.
///
/// Names are deliberately not restricted to a fixed set of rules, algorithms,
/// directives, providers, or logical systems.
///
/// Semantic services determine whether an identifier is registered and what
/// it means.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct DeductionName(String);

impl DeductionName {
    /// Construct a nonempty identifier without control characters.
    ///
    /// Whitespace is preserved rather than silently normalized.
    pub fn new(value: impl Into<String>) -> AiResult<Self> {
        let value = value.into();

        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(deduction_error(
                AiErrorKind::InvalidInput,
                "deduction identifier must be nonempty and contain no control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// ARGUMENTS
// ============================================================================

/// A positional or named deduction argument.
///
/// The argument expression remains a reference to the canonical AST.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum DeductionArgument {
    /// A positional expression argument.
    Positional(DeductionExpressionRef),

    /// A named expression argument.
    Named {
        /// Argument name.
        name: DeductionName,

        /// Canonical expression supplying the argument value.
        value: DeductionExpressionRef,
    },
}

impl DeductionArgument {
    /// Construct a positional argument.
    #[must_use]
    pub const fn positional(
        expression: DeductionExpressionRef,
    ) -> Self {
        Self::Positional(expression)
    }

    /// Construct a named argument.
    #[must_use]
    pub fn named(
        name: DeductionName,
        value: DeductionExpressionRef,
    ) -> Self {
        Self::Named { name, value }
    }

    /// Return the canonical expression referenced by this argument.
    #[must_use]
    pub const fn expression(&self) -> DeductionExpressionRef {
        match self {
            Self::Positional(expression) => *expression,
            Self::Named { value, .. } => *value,
        }
    }

    /// Return the optional argument name.
    #[must_use]
    pub fn name(&self) -> Option<&DeductionName> {
        match self {
            Self::Positional(_) => None,
            Self::Named { name, .. } => Some(name),
        }
    }
}

/// A collection of deduction arguments in source order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeductionArguments {
    arguments: Vec<DeductionArgument>,
}

impl DeductionArguments {
    /// Construct an argument collection.
    #[must_use]
    pub fn new(arguments: Vec<DeductionArgument>) -> Self {
        Self { arguments }
    }

    /// Return arguments in source order.
    #[must_use]
    pub fn as_slice(&self) -> &[DeductionArgument] {
        &self.arguments
    }

    /// Return the number of arguments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.arguments.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arguments.is_empty()
    }
}

// ============================================================================
// DECLARATION PARAMETERS
// ============================================================================

/// A parameter in a deduction declaration.
///
/// The grammar supports an identifier, an optional type expression, or an
/// optional initializer expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionParameter {
    name: DeductionName,
    declared_type: Option<DeductionTypeRef>,
    initializer: Option<DeductionExpressionRef>,
}

impl DeductionParameter {
    /// Construct a deduction parameter.
    #[must_use]
    pub const fn new(
        name: DeductionName,
        declared_type: Option<DeductionTypeRef>,
        initializer: Option<DeductionExpressionRef>,
    ) -> Self {
        Self {
            name,
            declared_type,
            initializer,
        }
    }

    /// Return the parameter name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return the optional canonical type reference.
    #[must_use]
    pub const fn declared_type(&self) -> Option<DeductionTypeRef> {
        self.declared_type
    }

    /// Return the optional initializer expression.
    #[must_use]
    pub const fn initializer(&self) -> Option<DeductionExpressionRef> {
        self.initializer
    }
}

// ============================================================================
// DECLARATIONS AND INVOCATIONS
// ============================================================================

/// A deduction declaration.
///
/// Represents the declaration form in `deduction.g4`.
///
/// Parameters, type references and body members preserve their source-level
/// relationships without constructing a new AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionDeclaration {
    source: DeductionConstructRef,
    name: DeductionName,
    parameters: Vec<DeductionParameter>,
    declared_type: Option<DeductionTypeRef>,
    members: Vec<DeductionMember>,
}

impl DeductionDeclaration {
    /// Construct a deduction declaration.
    #[must_use]
    pub fn new(
        source: DeductionConstructRef,
        name: DeductionName,
        parameters: Vec<DeductionParameter>,
        declared_type: Option<DeductionTypeRef>,
        members: Vec<DeductionMember>,
    ) -> Self {
        Self {
            source,
            name,
            parameters,
            declared_type,
            members,
        }
    }

    /// Return the canonical source construct.
    #[must_use]
    pub const fn source(&self) -> DeductionConstructRef {
        self.source
    }

    /// Return the declaration name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return declaration parameters in source order.
    #[must_use]
    pub fn parameters(&self) -> &[DeductionParameter] {
        &self.parameters
    }

    /// Return the optional declared type.
    #[must_use]
    pub const fn declared_type(&self) -> Option<DeductionTypeRef> {
        self.declared_type
    }

    /// Return declaration body members in source order.
    #[must_use]
    pub fn members(&self) -> &[DeductionMember] {
        &self.members
    }
}

/// A deduction invocation.
///
/// Represents `deduce(argument, ...)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionInvocation {
    source: DeductionConstructRef,
    arguments: DeductionArguments,
}

impl DeductionInvocation {
    /// Construct a deduction invocation.
    #[must_use]
    pub fn new(
        source: DeductionConstructRef,
        arguments: DeductionArguments,
    ) -> Self {
        Self { source, arguments }
    }

    /// Return the source construct.
    #[must_use]
    pub const fn source(&self) -> DeductionConstructRef {
        self.source
    }

    /// Return invocation arguments.
    #[must_use]
    pub fn arguments(&self) -> &DeductionArguments {
        &self.arguments
    }
}

/// A deduction expression.
///
/// Represents the expression form of the deduction grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionExpression {
    source: DeductionConstructRef,
    arguments: DeductionArguments,
}

impl DeductionExpression {
    /// Construct a deduction expression.
    #[must_use]
    pub fn new(
        source: DeductionConstructRef,
        arguments: DeductionArguments,
    ) -> Self {
        Self { source, arguments }
    }

    /// Return the source construct.
    #[must_use]
    pub const fn source(&self) -> DeductionConstructRef {
        self.source
    }

    /// Return expression arguments.
    #[must_use]
    pub fn arguments(&self) -> &DeductionArguments {
        &self.arguments
    }
}

// ============================================================================
// DIRECT DERIVATION
// ============================================================================

/// A direct deduction derivation.
///
/// Represents the grammar's direct form:
///
/// `deduce conclusion from premise, ...`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectDerivation {
    conclusion: DeductionExpressionRef,
    premises: Vec<DeductionExpressionRef>,
    modifiers: Vec<DeductionModifier>,
}

impl DirectDerivation {
    /// Construct a direct derivation.
    #[must_use]
    pub fn new(
        conclusion: DeductionExpressionRef,
        premises: Vec<DeductionExpressionRef>,
        modifiers: Vec<DeductionModifier>,
    ) -> Self {
        Self {
            conclusion,
            premises,
            modifiers,
        }
    }

    /// Return the conclusion expression.
    #[must_use]
    pub const fn conclusion(&self) -> DeductionExpressionRef {
        self.conclusion
    }

    /// Return premise expressions in source order.
    #[must_use]
    pub fn premises(&self) -> &[DeductionExpressionRef] {
        &self.premises
    }

    /// Return statement modifiers.
    #[must_use]
    pub fn modifiers(&self) -> &[DeductionModifier] {
        &self.modifiers
    }
}

// ============================================================================
// STRUCTURED STATEMENT MEMBERS
// ============================================================================

/// A premise clause containing one or more expressions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionPremises {
    expressions: Vec<DeductionExpressionRef>,
}

impl DeductionPremises {
    /// Construct a premise collection.
    #[must_use]
    pub fn new(expressions: Vec<DeductionExpressionRef>) -> Self {
        Self { expressions }
    }

    /// Return premise expressions.
    #[must_use]
    pub fn expressions(&self) -> &[DeductionExpressionRef] {
        &self.expressions
    }
}

/// A named deduction rule reference.
///
/// The rule name is open-ended. Its meaning is determined by the semantic
/// environment and registered reasoning capabilities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionRule {
    name: DeductionName,
    expression: DeductionExpressionRef,
}

impl DeductionRule {
    /// Construct a named rule reference.
    #[must_use]
    pub const fn new(
        name: DeductionName,
        expression: DeductionExpressionRef,
    ) -> Self {
        Self { name, expression }
    }

    /// Return the rule name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return the rule expression.
    #[must_use]
    pub const fn expression(&self) -> DeductionExpressionRef {
        self.expression
    }
}

/// A named derivation assignment.
///
/// Represents a named derivation target assigned an expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionDerivation {
    name: DeductionName,
    target: DeductionName,
    expression: DeductionExpressionRef,
}

impl DeductionDerivation {
    /// Construct a named derivation.
    #[must_use]
    pub const fn new(
        name: DeductionName,
        target: DeductionName,
        expression: DeductionExpressionRef,
    ) -> Self {
        Self {
            name,
            target,
            expression,
        }
    }

    /// Return the derivation name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return the derivation target.
    #[must_use]
    pub fn target(&self) -> &DeductionName {
        &self.target
    }

    /// Return the derivation expression.
    #[must_use]
    pub const fn expression(&self) -> DeductionExpressionRef {
        self.expression
    }
}

/// A payload used by explanation and provenance clauses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeductionPayload {
    /// A single canonical expression.
    Expression(DeductionExpressionRef),

    /// An ordered argument group.
    Arguments(DeductionArguments),
}

/// A named metadata payload.
///
/// Metadata can refer to an argument group, a type, or an expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeductionMetadataPayload {
    /// Argument-group metadata.
    Arguments(DeductionArguments),

    /// Type metadata.
    Type(DeductionTypeRef),

    /// Expression metadata.
    Expression(DeductionExpressionRef),
}

/// A deduction metadata declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionMetadata {
    name: DeductionName,
    payload: Option<DeductionMetadataPayload>,
}

impl DeductionMetadata {
    /// Construct deduction metadata.
    #[must_use]
    pub const fn new(
        name: DeductionName,
        payload: Option<DeductionMetadataPayload>,
    ) -> Self {
        Self { name, payload }
    }

    /// Return metadata name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return metadata payload.
    #[must_use]
    pub fn payload(&self) -> Option<&DeductionMetadataPayload> {
        self.payload.as_ref()
    }
}

/// An open deduction directive payload.
///
/// The grammar permits argument groups, type references, expressions and
/// nested directive regions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeductionDirectivePayload {
    /// An argument group.
    Arguments(DeductionArguments),

    /// A type reference.
    Type(DeductionTypeRef),

    /// An expression reference.
    Expression(DeductionExpressionRef),

    /// Nested deduction members.
    Region(Vec<DeductionMember>),
}

/// An open deduction directive.
///
/// Directive names remain extensible. Their semantics must be established by
/// the registered semantic environment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionDirective {
    name: DeductionName,
    payload: Option<DeductionDirectivePayload>,
}

impl DeductionDirective {
    /// Construct a deduction directive.
    #[must_use]
    pub const fn new(
        name: DeductionName,
        payload: Option<DeductionDirectivePayload>,
    ) -> Self {
        Self { name, payload }
    }

    /// Return the directive name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return the optional payload.
    #[must_use]
    pub fn payload(&self) -> Option<&DeductionDirectivePayload> {
        self.payload.as_ref()
    }
}

/// A statement modifier.
///
/// Modifier names and payloads are extensible and are not interpreted here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionModifier {
    name: DeductionName,
    payload: Option<DeductionMetadataPayload>,
}

impl DeductionModifier {
    /// Construct a deduction modifier.
    #[must_use]
    pub const fn new(
        name: DeductionName,
        payload: Option<DeductionMetadataPayload>,
    ) -> Self {
        Self { name, payload }
    }

    /// Return modifier name.
    #[must_use]
    pub fn name(&self) -> &DeductionName {
        &self.name
    }

    /// Return modifier payload.
    #[must_use]
    pub fn payload(&self) -> Option<&DeductionMetadataPayload> {
        self.payload.as_ref()
    }
}

/// A proof, verification, or validation target.
///
/// These are declarations of obligations or requests. They do not execute a
/// proof engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeductionProofObligation {
    /// A proof target.
    Prove(DeductionExpressionRef),

    /// A verification target.
    Verify(DeductionExpressionRef),

    /// A validation target.
    Validate(DeductionExpressionRef),
}

/// A structured deduction statement member.
///
/// This enum models the member forms declared by `deduction.g4`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeductionMember {
    /// Ordinary canonical Zamani statement.
    Statement(DeductionStatementRef),

    /// One or more premises.
    Premises(DeductionPremises),

    /// An evidence expression.
    Evidence(DeductionExpressionRef),

    /// An assumption expression.
    Assumption(DeductionExpressionRef),

    /// An observation expression.
    Observation(DeductionExpressionRef),

    /// A named rule reference.
    Rule(DeductionRule),

    /// A condition introduced by `when` or `where`.
    Condition(DeductionExpressionRef),

    /// A named derivation.
    Derivation(DeductionDerivation),

    /// A conclusion expression.
    Conclusion(DeductionExpressionRef),

    /// An explanation payload.
    Explanation(DeductionPayload),

    /// A provenance payload.
    Provenance(DeductionPayload),

    /// Metadata.
    Metadata(DeductionMetadata),

    /// An open directive.
    Directive(DeductionDirective),

    /// A proof, verification or validation obligation.
    ProofObligation(DeductionProofObligation),
}

/// The payload of a deduction statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeductionStatementPayload {
    /// Direct conclusion-from-premises form.
    Direct(DirectDerivation),

    /// Structured deduction members.
    Structured(Vec<DeductionMember>),
}

/// A deduction statement.
///
/// This represents the statement-level public grammar construct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeductionStatement {
    source: DeductionConstructRef,
    payload: DeductionStatementPayload,
}

impl DeductionStatement {
    /// Construct a deduction statement.
    #[must_use]
    pub const fn new(
        source: DeductionConstructRef,
        payload: DeductionStatementPayload,
    ) -> Self {
        Self { source, payload }
    }

    /// Return the canonical source construct.
    #[must_use]
    pub const fn source(&self) -> DeductionConstructRef {
        self.source
    }

    /// Return the statement payload.
    #[must_use]
    pub const fn payload(&self) -> &DeductionStatementPayload {
        &self.payload
    }
}

// ============================================================================
// PROGRAM
// ============================================================================

/// A deduction-domain projection of canonical AST constructs.
///
/// This is not an AST and is not an executable intermediate representation.
///
/// Declarations, invocations, expressions and statements retain their
/// canonical source identities. Their order is preserved.
///
/// The program imposes no language-level limit on the number of constructs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeductionProgram {
    declarations: Vec<DeductionDeclaration>,
    invocations: Vec<DeductionInvocation>,
    expressions: Vec<DeductionExpression>,
    statements: Vec<DeductionStatement>,
}

impl DeductionProgram {
    /// Construct an empty deduction program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return declarations in insertion/source order.
    #[must_use]
    pub fn declarations(&self) -> &[DeductionDeclaration] {
        &self.declarations
    }

    /// Return invocations in insertion/source order.
    #[must_use]
    pub fn invocations(&self) -> &[DeductionInvocation] {
        &self.invocations
    }

    /// Return expressions in insertion/source order.
    #[must_use]
    pub fn expressions(&self) -> &[DeductionExpression] {
        &self.expressions
    }

    /// Return statements in insertion/source order.
    #[must_use]
    pub fn statements(&self) -> &[DeductionStatement] {
        &self.statements
    }

    /// Add a declaration.
    ///
    /// A canonical source construct may be registered only once.
    pub fn add_declaration(
        &mut self,
        declaration: DeductionDeclaration,
    ) -> AiResult<()> {
        self.ensure_unique_source(declaration.source())?;
        self.declarations.push(declaration);
        Ok(())
    }

    /// Add an invocation.
    pub fn add_invocation(
        &mut self,
        invocation: DeductionInvocation,
    ) -> AiResult<()> {
        self.ensure_unique_source(invocation.source())?;
        self.invocations.push(invocation);
        Ok(())
    }

    /// Add a deduction expression.
    pub fn add_expression(
        &mut self,
        expression: DeductionExpression,
    ) -> AiResult<()> {
        self.ensure_unique_source(expression.source())?;
        self.expressions.push(expression);
        Ok(())
    }

    /// Add a deduction statement.
    pub fn add_statement(
        &mut self,
        statement: DeductionStatement,
    ) -> AiResult<()> {
        self.ensure_unique_source(statement.source())?;
        self.statements.push(statement);
        Ok(())
    }

    fn ensure_unique_source(
        &self,
        source: DeductionConstructRef,
    ) -> AiResult<()> {
        let node_id = source.node_id();

        let duplicate = self.declarations.iter()
            .any(|item| item.source().node_id() == node_id)
            || self.invocations.iter()
                .any(|item| item.source().node_id() == node_id)
            || self.expressions.iter()
                .any(|item| item.source().node_id() == node_id)
            || self.statements.iter()
                .any(|item| item.source().node_id() == node_id);

        if duplicate {
            return Err(deduction_error(
                AiErrorKind::Conflict,
                format!(
                    "deduction source construct {node_id} is already registered"
                ),
            ));
        }

        Ok(())
    }

    /// Validate every stored reference against the canonical frontend AST.
    ///
    /// This validates AST node categories and duplicate source references.
    /// It does not establish logical validity, name resolution, type
    /// compatibility, authorization, capability availability, resource
    /// feasibility, or execution success.
    pub fn validate<R: DeductionNodeResolver>(
        &self,
        resolver: &R,
    ) -> AiResult<DeductionValidationReport> {
        let mut report = DeductionValidationReport::default();
        let mut registered_sources = BTreeSet::new();

        for declaration in &self.declarations {
            register_source(
                &mut registered_sources,
                declaration.source(),
            )?;

            validate_construct(resolver, declaration.source())?;

            if let Some(declared_type) = declaration.declared_type() {
                validate_type(resolver, declared_type, &mut report)?;
            }

            for parameter in declaration.parameters() {
                if let Some(declared_type) = parameter.declared_type() {
                    validate_type(resolver, declared_type, &mut report)?;
                }

                if let Some(initializer) = parameter.initializer() {
                    validate_expression(resolver, initializer, &mut report)?;
                }
            }

            validate_members(
                resolver,
                declaration.members(),
                &mut registered_sources,
                &mut report,
            )?;

            report.record_declaration()?;
        }

        for invocation in &self.invocations {
            register_source(
                &mut registered_sources,
                invocation.source(),
            )?;

            validate_construct(resolver, invocation.source())?;

            validate_arguments(
                resolver,
                invocation.arguments(),
                &mut report,
            )?;

            report.record_invocation()?;
        }

        for expression in &self.expressions {
            register_source(
                &mut registered_sources,
                expression.source(),
            )?;

            validate_construct(resolver, expression.source())?;

            validate_arguments(
                resolver,
                expression.arguments(),
                &mut report,
            )?;

            report.record_expression_construct()?;
        }

        for statement in &self.statements {
            register_source(
                &mut registered_sources,
                statement.source(),
            )?;

            validate_construct(resolver, statement.source())?;

            match statement.payload() {
                DeductionStatementPayload::Direct(direct) => {
                    validate_expression(
                        resolver,
                        direct.conclusion(),
                        &mut report,
                    )?;

                    if direct.premises().is_empty() {
                        return Err(deduction_error(
                            AiErrorKind::InvalidInput,
                            "direct deduction derivation requires at least one premise",
                        ));
                    }

                    for premise in direct.premises() {
                        validate_expression(resolver, *premise, &mut report)?;
                    }

                    validate_modifiers(
                        resolver,
                        direct.modifiers(),
                        &mut report,
                    )?;
                }

                DeductionStatementPayload::Structured(members) => {
                    validate_members(
                        resolver,
                        members,
                        &mut registered_sources,
                        &mut report,
                    )?;
                }
            }

            report.record_statement()?;
        }

        Ok(report)
    }
}

// ============================================================================
// AST RESOLVER CONTRACT
// ============================================================================

/// Resolves deduction references against the canonical frontend AST.
///
/// Implementations must query the AST belonging to the current compilation
/// or semantic-analysis session.
///
/// A numerically valid `NodeId` is not sufficient evidence that a node exists
/// or belongs to the required AST category.
///
/// The resolver must not construct or mutate AST nodes.
pub trait DeductionNodeResolver {
    /// Return true if the node represents a public deduction construct:
    /// declaration, invocation, expression or statement.
    fn is_deduction_construct_node(&self, node_id: NodeId) -> bool;

    /// Return true if the node represents a canonical expression.
    fn is_expression_node(&self, node_id: NodeId) -> bool;

    /// Return true if the node represents a canonical type expression.
    fn is_type_node(&self, node_id: NodeId) -> bool;

    /// Return true if the node represents an ordinary canonical statement.
    fn is_statement_node(&self, node_id: NodeId) -> bool;
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic structural validation statistics.
///
/// These counters describe the references inspected by this module. They
/// do not certify mathematical correctness or successful execution.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeductionValidationReport {
    declarations: usize,
    invocations: usize,
    expressions: usize,
    statements: usize,
    expression_references: usize,
    type_references: usize,
    statement_references: usize,
}

impl DeductionValidationReport {
    /// Number of validated declarations.
    #[must_use]
    pub const fn declarations(&self) -> usize {
        self.declarations
    }

    /// Number of validated invocations.
    #[must_use]
    pub const fn invocations(&self) -> usize {
        self.invocations
    }

    /// Number of validated deduction expression constructs.
    #[must_use]
    pub const fn expressions(&self) -> usize {
        self.expressions
    }

    /// Number of validated deduction statements.
    #[must_use]
    pub const fn statements(&self) -> usize {
        self.statements
    }

    /// Number of validated expression references.
    #[must_use]
    pub const fn expression_references(&self) -> usize {
        self.expression_references
    }

    /// Number of validated type references.
    #[must_use]
    pub const fn type_references(&self) -> usize {
        self.type_references
    }

    /// Number of validated ordinary statement references.
    #[must_use]
    pub const fn statement_references(&self) -> usize {
        self.statement_references
    }

    fn increment(
        counter: &mut usize,
        description: &str,
    ) -> AiResult<()> {
        *counter = counter.checked_add(1).ok_or_else(|| {
            deduction_error(
                AiErrorKind::ResourceExhausted,
                format!(
                    "deduction validation counter overflow: {description}"
                ),
            )
        })?;

        Ok(())
    }

    fn record_declaration(&mut self) -> AiResult<()> {
        Self::increment(&mut self.declarations, "declarations")
    }

    fn record_invocation(&mut self) -> AiResult<()> {
        Self::increment(&mut self.invocations, "invocations")
    }

    fn record_expression_construct(&mut self) -> AiResult<()> {
        Self::increment(&mut self.expressions, "expressions")
    }

    fn record_statement(&mut self) -> AiResult<()> {
        Self::increment(&mut self.statements, "statements")
    }

    fn record_expression_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.expression_references,
            "expression references",
        )
    }

    fn record_type_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.type_references,
            "type references",
        )
    }

    fn record_statement_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.statement_references,
            "statement references",
        )
    }
}

// ============================================================================
// REFERENCE VALIDATION
// ============================================================================

fn register_source(
    registered: &mut BTreeSet<NodeId>,
    source: DeductionConstructRef,
) -> AiResult<()> {
    if !registered.insert(source.node_id()) {
        return Err(deduction_error(
            AiErrorKind::Conflict,
            format!(
                "duplicate deduction source construct reference {}",
                source.node_id()
            ),
        ));
    }

    Ok(())
}

fn validate_construct<R: DeductionNodeResolver>(
    resolver: &R,
    source: DeductionConstructRef,
) -> AiResult<()> {
    if !resolver.is_deduction_construct_node(source.node_id()) {
        return Err(deduction_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid deduction construct",
                source.node_id()
            ),
        ));
    }

    Ok(())
}

fn validate_expression<R: DeductionNodeResolver>(
    resolver: &R,
    expression: DeductionExpressionRef,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    if !resolver.is_expression_node(expression.node_id()) {
        return Err(deduction_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid expression",
                expression.node_id()
            ),
        ));
    }

    report.record_expression_reference()
}

fn validate_type<R: DeductionNodeResolver>(
    resolver: &R,
    declared_type: DeductionTypeRef,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    if !resolver.is_type_node(declared_type.node_id()) {
        return Err(deduction_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid type expression",
                declared_type.node_id()
            ),
        ));
    }

    report.record_type_reference()
}

fn validate_arguments<R: DeductionNodeResolver>(
    resolver: &R,
    arguments: &DeductionArguments,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    for argument in arguments.as_slice() {
        validate_expression(
            resolver,
            argument.expression(),
            report,
        )?;
    }

    Ok(())
}

fn validate_payload<R: DeductionNodeResolver>(
    resolver: &R,
    payload: &DeductionPayload,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    match payload {
        DeductionPayload::Expression(expression) => {
            validate_expression(resolver, *expression, report)
        }

        DeductionPayload::Arguments(arguments) => {
            validate_arguments(resolver, arguments, report)
        }
    }
}

fn validate_metadata_payload<R: DeductionNodeResolver>(
    resolver: &R,
    payload: &DeductionMetadataPayload,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    match payload {
        DeductionMetadataPayload::Arguments(arguments) => {
            validate_arguments(resolver, arguments, report)
        }

        DeductionMetadataPayload::Type(declared_type) => {
            validate_type(resolver, *declared_type, report)
        }

        DeductionMetadataPayload::Expression(expression) => {
            validate_expression(resolver, *expression, report)
        }
    }
}

fn validate_modifiers<R: DeductionNodeResolver>(
    resolver: &R,
    modifiers: &[DeductionModifier],
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    for modifier in modifiers {
        if let Some(payload) = modifier.payload() {
            validate_metadata_payload(resolver, payload, report)?;
        }
    }

    Ok(())
}

fn validate_directive_payload<R: DeductionNodeResolver>(
    resolver: &R,
    payload: &DeductionDirectivePayload,
    registered: &mut BTreeSet<NodeId>,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    match payload {
        DeductionDirectivePayload::Arguments(arguments) => {
            validate_arguments(resolver, arguments, report)
        }

        DeductionDirectivePayload::Type(declared_type) => {
            validate_type(resolver, *declared_type, report)
        }

        DeductionDirectivePayload::Expression(expression) => {
            validate_expression(resolver, *expression, report)
        }

        DeductionDirectivePayload::Region(members) => {
            validate_members(
                resolver,
                members,
                registered,
                report,
            )
        }
    }
}

fn validate_members<R: DeductionNodeResolver>(
    resolver: &R,
    members: &[DeductionMember],
    registered: &mut BTreeSet<NodeId>,
    report: &mut DeductionValidationReport,
) -> AiResult<()> {
    for member in members {
        match member {
            DeductionMember::Statement(statement) => {
                if !resolver.is_statement_node(statement.node_id()) {
                    return Err(deduction_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "AST node {} is not a valid ordinary statement",
                            statement.node_id()
                        ),
                    ));
                }

                report.record_statement_reference()?;
            }

            DeductionMember::Premises(premises) => {
                if premises.expressions().is_empty() {
                    return Err(deduction_error(
                        AiErrorKind::InvalidInput,
                        "a premise clause must contain at least one expression",
                    ));
                }

                for expression in premises.expressions() {
                    validate_expression(resolver, *expression, report)?;
                }
            }

            DeductionMember::Evidence(expression)
            | DeductionMember::Assumption(expression)
            | DeductionMember::Observation(expression)
            | DeductionMember::Condition(expression)
            | DeductionMember::Conclusion(expression) => {
                validate_expression(resolver, *expression, report)?;
            }

            DeductionMember::Rule(rule) => {
                validate_expression(
                    resolver,
                    rule.expression(),
                    report,
                )?;
            }

            DeductionMember::Derivation(derivation) => {
                validate_expression(
                    resolver,
                    derivation.expression(),
                    report,
                )?;
            }

            DeductionMember::Explanation(payload)
            | DeductionMember::Provenance(payload) => {
                validate_payload(resolver, payload, report)?;
            }

            DeductionMember::Metadata(metadata) => {
                if let Some(payload) = metadata.payload() {
                    validate_metadata_payload(
                        resolver,
                        payload,
                        report,
                    )?;
                }
            }

            DeductionMember::Directive(directive) => {
                if let Some(payload) = directive.payload() {
                    validate_directive_payload(
                        resolver,
                        payload,
                        registered,
                        report,
                    )?;
                }
            }

            DeductionMember::ProofObligation(obligation) => {
                let expression = match obligation {
                    DeductionProofObligation::Prove(expression)
                    | DeductionProofObligation::Verify(expression)
                    | DeductionProofObligation::Validate(expression) => {
                        *expression
                    }
                };

                validate_expression(resolver, expression, report)?;
            }
        }
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    struct TestResolver {
        constructs: BTreeSet<NodeId>,
        expressions: BTreeSet<NodeId>,
        types: BTreeSet<NodeId>,
        statements: BTreeSet<NodeId>,
    }

    impl TestResolver {
        fn new() -> Self {
            Self {
                constructs: BTreeSet::new(),
                expressions: BTreeSet::new(),
                types: BTreeSet::new(),
                statements: BTreeSet::new(),
            }
        }

        fn with_construct(mut self, id: u64) -> Self {
            self.constructs.insert(NodeId::new(id).expect("valid ID"));
            self
        }

        fn with_expression(mut self, id: u64) -> Self {
            self.expressions.insert(NodeId::new(id).expect("valid ID"));
            self
        }

        fn with_type(mut self, id: u64) -> Self {
            self.types.insert(NodeId::new(id).expect("valid ID"));
            self
        }

        fn with_statement(mut self, id: u64) -> Self {
            self.statements.insert(NodeId::new(id).expect("valid ID"));
            self
        }
    }

    impl DeductionNodeResolver for TestResolver {
        fn is_deduction_construct_node(&self, node_id: NodeId) -> bool {
            self.constructs.contains(&node_id)
        }

        fn is_expression_node(&self, node_id: NodeId) -> bool {
            self.expressions.contains(&node_id)
        }

        fn is_type_node(&self, node_id: NodeId) -> bool {
            self.types.contains(&node_id)
        }

        fn is_statement_node(&self, node_id: NodeId) -> bool {
            self.statements.contains(&node_id)
        }
    }

    fn node(id: u64) -> NodeId {
        NodeId::new(id).expect("valid node ID")
    }

    fn expression(id: u64) -> DeductionExpressionRef {
        DeductionExpressionRef::new(node(id))
    }

    fn construct(id: u64) -> DeductionConstructRef {
        DeductionConstructRef::new(node(id))
    }

    #[test]
    fn deduction_name_rejects_empty_values() {
        assert!(DeductionName::new("").is_err());
    }

    #[test]
    fn deduction_name_rejects_control_characters() {
        assert!(DeductionName::new("rule\nname").is_err());
    }

    #[test]
    fn deduction_name_preserves_extension_names() {
        let name = DeductionName::new("custom.logic.rule").expect("valid name");
        assert_eq!(name.as_str(), "custom.logic.rule");
    }

    #[test]
    fn expression_reference_preserves_node_identity() {
        let reference = expression(42);
        assert_eq!(reference.node_id(), node(42));
    }

    #[test]
    fn named_argument_preserves_name_and_expression() {
        let name = DeductionName::new("premise").expect("name");
        let argument = DeductionArgument::named(name, expression(5));

        assert_eq!(argument.name().expect("named").as_str(), "premise");
        assert_eq!(argument.expression(), expression(5));
    }

    #[test]
    fn positional_argument_has_no_name() {
        let argument = DeductionArgument::positional(expression(5));
        assert!(argument.name().is_none());
    }

    #[test]
    fn program_accepts_distinct_constructs() {
        let mut program = DeductionProgram::new();

        let invocation = DeductionInvocation::new(
            construct(1),
            DeductionArguments::default(),
        );

        let statement = DeductionStatement::new(
            construct(2),
            DeductionStatementPayload::Structured(Vec::new()),
        );

        assert!(program.add_invocation(invocation).is_ok());
        assert!(program.add_statement(statement).is_ok());
        assert_eq!(program.invocations().len(), 1);
        assert_eq!(program.statements().len(), 1);
    }

    #[test]
    fn program_rejects_duplicate_source_constructs() {
        let mut program = DeductionProgram::new();

        let first = DeductionInvocation::new(
            construct(1),
            DeductionArguments::default(),
        );

        let second = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Structured(Vec::new()),
        );

        assert!(program.add_invocation(first).is_ok());
        assert!(program.add_statement(second).is_err());
    }

    #[test]
    fn validates_invocation_expression_references() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_expression(2);

        let mut program = DeductionProgram::new();

        let invocation = DeductionInvocation::new(
            construct(1),
            DeductionArguments::new(vec![
                DeductionArgument::positional(expression(2)),
            ]),
        );

        program.add_invocation(invocation).expect("add");

        let report = program.validate(&resolver).expect("valid");

        assert_eq!(report.invocations(), 1);
        assert_eq!(report.expression_references(), 1);
    }

    #[test]
    fn rejects_expression_reference_to_non_expression_node() {
        let resolver = TestResolver::new()
            .with_construct(1);

        let mut program = DeductionProgram::new();

        let invocation = DeductionInvocation::new(
            construct(1),
            DeductionArguments::new(vec![
                DeductionArgument::positional(expression(2)),
            ]),
        );

        program.add_invocation(invocation).expect("add");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn validates_direct_derivation() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_expression(2)
            .with_expression(3);

        let direct = DirectDerivation::new(
            expression(2),
            vec![expression(3)],
            Vec::new(),
        );

        let statement = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Direct(direct),
        );

        let mut program = DeductionProgram::new();
        program.add_statement(statement).expect("add");

        let report = program.validate(&resolver).expect("valid");

        assert_eq!(report.statements(), 1);
        assert_eq!(report.expression_references(), 2);
    }

    #[test]
    fn direct_derivation_requires_a_premise() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_expression(2);

        let direct = DirectDerivation::new(
            expression(2),
            Vec::new(),
            Vec::new(),
        );

        let statement = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Direct(direct),
        );

        let mut program = DeductionProgram::new();
        program.add_statement(statement).expect("add");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn validates_nested_directive_region() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_expression(2)
            .with_expression(3);

        let nested = DeductionMember::Directive(
            DeductionDirective::new(
                DeductionName::new("custom").expect("name"),
                Some(DeductionDirectivePayload::Region(vec![
                    DeductionMember::Assumption(expression(2)),
                    DeductionMember::Conclusion(expression(3)),
                ])),
            ),
        );

        let statement = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Structured(vec![nested]),
        );

        let mut program = DeductionProgram::new();
        program.add_statement(statement).expect("add");

        let report = program.validate(&resolver).expect("valid");
        assert_eq!(report.expression_references(), 2);
    }

    #[test]
    fn validates_type_references_in_declarations() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_type(2);

        let declaration = DeductionDeclaration::new(
            construct(1),
            DeductionName::new("derive").expect("name"),
            Vec::new(),
            Some(DeductionTypeRef::new(node(2))),
            Vec::new(),
        );

        let mut program = DeductionProgram::new();
        program.add_declaration(declaration).expect("add");

        let report = program.validate(&resolver).expect("valid");
        assert_eq!(report.declarations(), 1);
        assert_eq!(report.type_references(), 1);
    }

    #[test]
    fn validates_ordinary_statement_references() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_statement(2);

        let statement = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Structured(vec![
                DeductionMember::Statement(
                    DeductionStatementRef::new(node(2)),
                ),
            ]),
        );

        let mut program = DeductionProgram::new();
        program.add_statement(statement).expect("add");

        let report = program.validate(&resolver).expect("valid");
        assert_eq!(report.statement_references(), 1);
    }

    #[test]
    fn empty_structured_statement_is_structurally_valid() {
        let resolver = TestResolver::new()
            .with_construct(1);

        let statement = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Structured(Vec::new()),
        );

        let mut program = DeductionProgram::new();
        program.add_statement(statement).expect("add");

        assert!(program.validate(&resolver).is_ok());
    }

    #[test]
    fn premise_clause_must_not_be_empty() {
        let resolver = TestResolver::new()
            .with_construct(1);

        let statement = DeductionStatement::new(
            construct(1),
            DeductionStatementPayload::Structured(vec![
                DeductionMember::Premises(
                    DeductionPremises::new(Vec::new()),
                ),
            ]),
        );

        let mut program = DeductionProgram::new();
        program.add_statement(statement).expect("add");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn report_counters_are_deterministic() {
        let resolver = TestResolver::new()
            .with_construct(1)
            .with_expression(2);

        let mut program = DeductionProgram::new();

        program.add_invocation(
            DeductionInvocation::new(
                construct(1),
                DeductionArguments::new(vec![
                    DeductionArgument::positional(expression(2)),
                ]),
            ),
        ).expect("add");

        let first = program.validate(&resolver).expect("first");
        let second = program.validate(&resolver).expect("second");

        assert_eq!(first, second);
    }
}
