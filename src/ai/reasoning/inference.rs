
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Inference Domain
//!
//! Path: `src/ai/reasoning/inference.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides the AI-domain representation of inference declarations,
//! invocations, directives, bindings and nested inference regions.
//!
//! This module preserves references to the canonical frontend AST.
//! It does not construct a second AST or execute inference operations.
//!
//! # Architectural authority
//!
//! Source grammar:
//!     grammar/ai/inference.g4
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
//! AI error authority:
//!     src/ai/error.rs
//!
//! Model definitions:
//!     src/ai/model/
//!
//! Symbolic reasoning:
//!     src/ai/reasoning/symbolic.rs
//!
//! Universal IR:
//!     src/zuir/
//!
//! AI IR:
//!     src/zuir/dialects/ai/
//!
//! # Ownership
//!
//! This module owns:
//!
//! - inference-domain declarations;
//! - inference invocation descriptions;
//! - inference-local directive descriptions;
//! - inference-local binding descriptions;
//! - inference-specific AST reference validation;
//! - inference structural validation;
//! - deterministic inference validation reports.
//!
//! This module does not own:
//!
//! - parsing;
//! - lexical vocabulary;
//! - AST construction;
//! - general expression representation;
//! - general type representation;
//! - name resolution;
//! - model storage;
//! - model registration;
//! - model loading;
//! - numerical computation;
//! - training;
//! - probabilistic reasoning;
//! - symbolic solving;
//! - knowledge storage;
//! - tensor storage;
//! - hardware discovery;
//! - resource allocation;
//! - device placement;
//! - scheduling;
//! - distributed transport;
//! - IR construction;
//! - runtime execution.
//!
//! # POCO-REAF
//!
//! No language-level maximum is imposed on:
//!
//! - inference declarations;
//! - inference invocations;
//! - input bindings;
//! - output bindings;
//! - arguments;
//! - directives;
//! - nested directive regions;
//! - inference members;
//! - model parameters;
//! - model dimensions;
//! - inference workload size;
//! - execution targets.
//!
//! Collections grow according to supplied input and available resources.
//!
//! Rust's finite address space and allocation behavior remain implementation
//! constraints. Literal infinite computation is not promised.
//!
//! Operational limits must be supplied by the responsible compiler,
//! configuration, resource or execution infrastructure.
//!
//! # Integration
//!
//! grammar/ai/inference.g4
//!       |
//!       v
//! inferenceConstruct
//!       |
//!       v
//! canonical frontend AST
//!       |
//!       v
//! AST adapter
//!       |
//!       v
//! InferenceProgram
//!       |
//!       v
//! AI semantic validation
//!       |
//!       v
//! canonical semantic model
//!       |
//!       v
//! ZUIR AI dialect
//!       |
//!       v
//! optimization and lowering
//!       |
//!       v
//! target realization
//!
//! The adapter must preserve canonical AST identities and source provenance.
//!
//! # Determinism
//!
//! - Validation uses ordered collections for identity checks.
//! - Directive names remain open-ended.
//! - No global mutable state is introduced.
//! - No hardware-specific behavior is introduced.
//! - Validation does not execute inference.
//!
//! # Safety
//!
//! This module uses safe Rust only.

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

fn inference_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Inference)
}

// ============================================================================
// CANONICAL AST REFERENCES
// ============================================================================

/// A reference to an expression in the canonical frontend AST.
///
/// This type does not contain an expression tree.
///
/// The referenced node must be validated against the AST associated with
/// the current semantic analysis.
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
pub struct InferenceExpressionRef {
    node_id: NodeId,
}

impl InferenceExpressionRef {
    /// Construct a reference to a canonical AST expression node.
    ///
    /// This constructor does not establish that the node exists or that
    /// it represents an expression. Those properties are checked during
    /// structural validation.
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

/// A reference to a type expression in the canonical frontend AST.
///
/// Type syntax and type semantics remain owned by the canonical type system.
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
pub struct InferenceTypeRef {
    node_id: NodeId,
}

impl InferenceTypeRef {
    /// Construct a reference to a canonical type-expression node.
    ///
    /// Existence and node category are checked during validation.
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

/// A reference to an inference construct in the canonical frontend AST.
///
/// The referenced node identifies the original source construct.
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
pub struct InferenceConstructRef {
    node_id: NodeId,
}

impl InferenceConstructRef {
    /// Construct a reference to a canonical inference construct.
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

/// A reference to a statement inside an inference body.
///
/// Ordinary Zamani statements remain owned by the universal statement
/// grammar and canonical AST.
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
pub struct InferenceStatementRef {
    node_id: NodeId,
}

impl InferenceStatementRef {
    /// Construct a reference to a canonical statement node.
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
// IDENTIFIERS
// ============================================================================

/// An open-ended inference-domain identifier.
///
/// The canonical frontend remains responsible for resolving its meaning.
///
/// This type deliberately does not contain a fixed list of model names,
/// directive names, providers or inference algorithms.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct InferenceName(String);

impl InferenceName {
    /// Construct a nonempty inference-domain identifier.
    ///
    /// Whitespace is preserved rather than silently normalized.
    ///
    /// Control characters are rejected.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(inference_error(
                AiErrorKind::InvalidInput,
                "inference identifier must be nonempty and contain no control characters",
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
// INVOCATION ARGUMENTS
// ============================================================================

/// An argument supplied to an inference invocation.
///
/// Argument expressions are references to the canonical AST.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
)]
pub struct InferenceArgument {
    expression: InferenceExpressionRef,
}

impl InferenceArgument {
    /// Construct an argument from a canonical expression reference.
    #[must_use]
    pub const fn new(
        expression: InferenceExpressionRef,
    ) -> Self {
        Self { expression }
    }

    /// Return the referenced expression.
    #[must_use]
    pub const fn expression(
        self,
    ) -> InferenceExpressionRef {
        self.expression
    }
}

// ============================================================================
// DECLARATIONS
// ============================================================================

/// An inference declaration.
///
/// Represents the declaration form in `inference.g4`:
///
/// `infer name [: type] [= expression] { members }`
///
/// The declared type and initializer refer to canonical AST nodes.
///
/// The body preserves references to its original inference members.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceDeclaration {
    source: InferenceConstructRef,
    name: InferenceName,
    declared_type: Option<InferenceTypeRef>,
    initializer: Option<InferenceExpressionRef>,
    members: Vec<InferenceMember>,
}

impl InferenceDeclaration {
    /// Construct an inference declaration.
    pub fn new(
        source: InferenceConstructRef,
        name: InferenceName,
        declared_type: Option<InferenceTypeRef>,
        initializer: Option<InferenceExpressionRef>,
        members: Vec<InferenceMember>,
    ) -> Self {
        Self {
            source,
            name,
            declared_type,
            initializer,
            members,
        }
    }

    /// Return the source construct reference.
    #[must_use]
    pub const fn source(&self) -> InferenceConstructRef {
        self.source
    }

    /// Return the declaration name.
    #[must_use]
    pub fn name(&self) -> &InferenceName {
        &self.name
    }

    /// Return the optional declared type reference.
    #[must_use]
    pub const fn declared_type(
        &self,
    ) -> Option<InferenceTypeRef> {
        self.declared_type
    }

    /// Return the optional initializer expression.
    #[must_use]
    pub const fn initializer(
        &self,
    ) -> Option<InferenceExpressionRef> {
        self.initializer
    }

    /// Return the inference body members in source order.
    #[must_use]
    pub fn members(&self) -> &[InferenceMember] {
        &self.members
    }
}

// ============================================================================
// INVOCATIONS
// ============================================================================

/// An inference invocation.
///
/// Represents the invocation form in `inference.g4`:
///
/// `infer expression [(arguments)] ;`
///
/// The target and arguments remain canonical AST expression references.
///
/// The target may be a model, a model handle, a provider, a dynamically
/// selected expression or another valid Zamani expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceInvocation {
    source: InferenceConstructRef,
    target: InferenceExpressionRef,
    arguments: Vec<InferenceArgument>,
}

impl InferenceInvocation {
    /// Construct an inference invocation.
    pub fn new(
        source: InferenceConstructRef,
        target: InferenceExpressionRef,
        arguments: Vec<InferenceArgument>,
    ) -> Self {
        Self {
            source,
            target,
            arguments,
        }
    }

    /// Return the source construct reference.
    #[must_use]
    pub const fn source(&self) -> InferenceConstructRef {
        self.source
    }

    /// Return the invocation target.
    #[must_use]
    pub const fn target(&self) -> InferenceExpressionRef {
        self.target
    }

    /// Return invocation arguments in source order.
    #[must_use]
    pub fn arguments(&self) -> &[InferenceArgument] {
        &self.arguments
    }
}

// ============================================================================
// DIRECTIVE PAYLOADS
// ============================================================================

/// A call payload supplied to an inference directive.
///
/// The call's arguments remain references to canonical AST expressions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InferenceCall {
    arguments: Vec<InferenceArgument>,
}

impl InferenceCall {
    /// Construct a call payload.
    #[must_use]
    pub fn new(
        arguments: Vec<InferenceArgument>,
    ) -> Self {
        Self { arguments }
    }

    /// Return call arguments in source order.
    #[must_use]
    pub fn arguments(&self) -> &[InferenceArgument] {
        &self.arguments
    }
}

/// A typed binding payload.
///
/// The declared type is referenced rather than reimplemented.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceTypedBinding {
    name: InferenceName,
    declared_type: InferenceTypeRef,
    call: Option<InferenceCall>,
}

impl InferenceTypedBinding {
    /// Construct a typed binding.
    #[must_use]
    pub fn new(
        name: InferenceName,
        declared_type: InferenceTypeRef,
        call: Option<InferenceCall>,
    ) -> Self {
        Self {
            name,
            declared_type,
            call,
        }
    }

    /// Return the binding name.
    #[must_use]
    pub fn name(&self) -> &InferenceName {
        &self.name
    }

    /// Return the canonical type reference.
    #[must_use]
    pub const fn declared_type(
        &self,
    ) -> InferenceTypeRef {
        self.declared_type
    }

    /// Return the optional call payload.
    #[must_use]
    pub fn call(&self) -> Option<&InferenceCall> {
        self.call.as_ref()
    }
}

/// An assignment payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceAssignment {
    name: InferenceName,
    value: InferenceExpressionRef,
}

impl InferenceAssignment {
    /// Construct an assignment payload.
    #[must_use]
    pub const fn new(
        name: InferenceName,
        value: InferenceExpressionRef,
    ) -> Self {
        Self { name, value }
    }

    /// Return the assigned name.
    #[must_use]
    pub fn name(&self) -> &InferenceName {
        &self.name
    }

    /// Return the assigned expression.
    #[must_use]
    pub const fn value(
        &self,
    ) -> InferenceExpressionRef {
        self.value
    }
}

/// A named binding payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceNamedBinding {
    name: InferenceName,
    call: Option<InferenceCall>,
}

impl InferenceNamedBinding {
    /// Construct a named binding.
    #[must_use]
    pub fn new(
        name: InferenceName,
        call: Option<InferenceCall>,
    ) -> Self {
        Self { name, call }
    }

    /// Return the binding name.
    #[must_use]
    pub fn name(&self) -> &InferenceName {
        &self.name
    }

    /// Return the optional call payload.
    #[must_use]
    pub fn call(&self) -> Option<&InferenceCall> {
        self.call.as_ref()
    }
}

/// The structural payload of an inference directive.
///
/// Directive names remain open-ended. The semantic registry determines
/// whether a directive is recognized and what it means.
///
/// No closed enum of directive names is introduced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferenceDirectivePayload {
    /// A call with zero or more arguments.
    Call(InferenceCall),

    /// A typed binding with an optional call.
    TypedBinding(InferenceTypedBinding),

    /// An assignment to a named inference binding.
    Assignment(InferenceAssignment),

    /// A general canonical expression.
    Expression(InferenceExpressionRef),

    /// A nested inference region.
    Region(Vec<InferenceMember>),

    /// A named binding with an optional call.
    NamedBinding(InferenceNamedBinding),
}

// ============================================================================
// DIRECTIVES
// ============================================================================

/// An inference-local directive.
///
/// Represents the generic directive form in `inference.g4`:
///
/// `@identifier [payload] ;`
///
/// The directive's meaning is not determined by its spelling alone.
/// Semantic validation and registered extension contracts establish meaning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceDirective {
    source: InferenceConstructRef,
    name: InferenceName,
    payload: Option<InferenceDirectivePayload>,
}

impl InferenceDirective {
    /// Construct an inference directive.
    #[must_use]
    pub fn new(
        source: InferenceConstructRef,
        name: InferenceName,
        payload: Option<InferenceDirectivePayload>,
    ) -> Self {
        Self {
            source,
            name,
            payload,
        }
    }

    /// Return the source construct reference.
    #[must_use]
    pub const fn source(&self) -> InferenceConstructRef {
        self.source
    }

    /// Return the directive name.
    #[must_use]
    pub fn name(&self) -> &InferenceName {
        &self.name
    }

    /// Return the optional directive payload.
    #[must_use]
    pub fn payload(&self) -> Option<&InferenceDirectivePayload> {
        self.payload.as_ref()
    }
}

// ============================================================================
// INFERENCE BODY MEMBERS
// ============================================================================

/// A member of an inference declaration or nested directive region.
///
/// Ordinary statements remain canonical AST references. Inference directives
/// are represented using their inference-specific domain projection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferenceMember {
    /// A generic Zamani statement.
    Statement(InferenceStatementRef),

    /// An inference-local directive.
    Directive(InferenceDirective),
}

// ============================================================================
// INFERENCE PROGRAM
// ============================================================================

/// A collection of inference-domain declarations and invocations.
///
/// This is not a source AST and is not an executable IR.
///
/// It is a domain-specific projection of canonical AST constructs.
///
/// Its vectors impose no language-level item ceiling.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InferenceProgram {
    declarations: Vec<InferenceDeclaration>,
    invocations: Vec<InferenceInvocation>,
}

impl InferenceProgram {
    /// Construct an empty inference program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return inference declarations in insertion/source order.
    #[must_use]
    pub fn declarations(&self) -> &[InferenceDeclaration] {
        &self.declarations
    }

    /// Return inference invocations in insertion/source order.
    #[must_use]
    pub fn invocations(&self) -> &[InferenceInvocation] {
        &self.invocations
    }

    /// Add an inference declaration.
    ///
    /// A source construct may be registered only once in this program.
    pub fn add_declaration(
        &mut self,
        declaration: InferenceDeclaration,
    ) -> AiResult<()> {
        self.ensure_unique_source(declaration.source())?;

        self.declarations.push(declaration);

        Ok(())
    }

    /// Add an inference invocation.
    ///
    /// A source construct may be registered only once in this program.
    pub fn add_invocation(
        &mut self,
        invocation: InferenceInvocation,
    ) -> AiResult<()> {
        self.ensure_unique_source(invocation.source())?;

        self.invocations.push(invocation);

        Ok(())
    }

    fn ensure_unique_source(
        &self,
        source: InferenceConstructRef,
    ) -> AiResult<()> {
        let node_id = source.node_id();

        let declaration_exists = self
            .declarations
            .iter()
            .any(|item| item.source().node_id() == node_id);

        let invocation_exists = self
            .invocations
            .iter()
            .any(|item| item.source().node_id() == node_id);

        if declaration_exists || invocation_exists {
            return Err(inference_error(
                AiErrorKind::InvalidInput,
                format!(
                    "inference source construct {} is already registered",
                    node_id
                ),
            ));
        }

        Ok(())
    }

    /// Validate all inference references against the canonical AST.
    ///
    /// This validates structural references only.
    ///
    /// It does not establish:
    ///
    /// - model existence;
    /// - model compatibility;
    /// - type correctness;
    /// - directive registration;
    /// - operation availability;
    /// - capability satisfaction;
    /// - resource feasibility;
    /// - mathematical correctness;
    /// - execution success.
    ///
    /// Those checks belong to the canonical semantic and execution layers.
    pub fn validate<R: InferenceNodeResolver>(
        &self,
        resolver: &R,
    ) -> AiResult<InferenceValidationReport> {
        let mut report = InferenceValidationReport::default();
        let mut source_nodes = BTreeSet::new();

        for declaration in &self.declarations {
            register_source(
                &mut source_nodes,
                declaration.source(),
            )?;

            validate_construct(
                resolver,
                declaration.source(),
            )?;

            if let Some(declared_type) = declaration.declared_type() {
                validate_type(
                    resolver,
                    declared_type,
                    &mut report,
                )?;
            }

            if let Some(initializer) = declaration.initializer() {
                validate_expression(
                    resolver,
                    initializer,
                    &mut report,
                )?;
            }

            validate_members(
                resolver,
                declaration.members(),
                &mut source_nodes,
                &mut report,
            )?;

            report.record_declaration()?;
        }

        for invocation in &self.invocations {
            register_source(
                &mut source_nodes,
                invocation.source(),
            )?;

            validate_construct(
                resolver,
                invocation.source(),
            )?;

            validate_expression(
                resolver,
                invocation.target(),
                &mut report,
            )?;

            for argument in invocation.arguments() {
                validate_expression(
                    resolver,
                    argument.expression(),
                    &mut report,
                )?;
            }

            report.record_invocation()?;
        }

        Ok(report)
    }
}

// ============================================================================
// AST RESOLVER CONTRACT
// ============================================================================

/// Resolves inference references against the canonical frontend AST.
///
/// Implementations must query the AST belonging to the current compilation
/// or semantic-analysis session.
///
/// They must not accept an identifier merely because it is numerically valid.
///
/// The trait deliberately does not expose AST construction or mutation.
pub trait InferenceNodeResolver {
    /// Return true when the node exists and represents an inference
    /// declaration, invocation or directive construct.
    fn is_inference_construct_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true when the node exists and represents a canonical expression.
    fn is_expression_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true when the node exists and represents a canonical type
    /// expression.
    fn is_type_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true when the node exists and represents an ordinary
    /// canonical Zamani statement.
    fn is_statement_node(
        &self,
        node_id: NodeId,
    ) -> bool;
}

// ============================================================================
// VALIDATION
// ============================================================================

fn register_source(
    registered: &mut BTreeSet<NodeId>,
    source: InferenceConstructRef,
) -> AiResult<()> {
    if !registered.insert(source.node_id()) {
        return Err(inference_error(
            AiErrorKind::InvalidInput,
            format!(
                "duplicate inference construct reference {}",
                source.node_id()
            ),
        ));
    }

    Ok(())
}

fn validate_construct<R: InferenceNodeResolver>(
    resolver: &R,
    source: InferenceConstructRef,
) -> AiResult<()> {
    if !resolver.is_inference_construct_node(source.node_id()) {
        return Err(inference_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid inference construct",
                source.node_id()
            ),
        ));
    }

    Ok(())
}

fn validate_expression<R: InferenceNodeResolver>(
    resolver: &R,
    expression: InferenceExpressionRef,
    report: &mut InferenceValidationReport,
) -> AiResult<()> {
    if !resolver.is_expression_node(expression.node_id()) {
        return Err(inference_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid expression",
                expression.node_id()
            ),
        ));
    }

    report.record_expression_reference()?;

    Ok(())
}

fn validate_type<R: InferenceNodeResolver>(
    resolver: &R,
    declared_type: InferenceTypeRef,
    report: &mut InferenceValidationReport,
) -> AiResult<()> {
    if !resolver.is_type_node(declared_type.node_id()) {
        return Err(inference_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid type expression",
                declared_type.node_id()
            ),
        ));
    }

    report.record_type_reference()?;

    Ok(())
}

fn validate_members<R: InferenceNodeResolver>(
    resolver: &R,
    members: &[InferenceMember],
    registered: &mut BTreeSet<NodeId>,
    report: &mut InferenceValidationReport,
) -> AiResult<()> {
    for member in members {
        match member {
            InferenceMember::Statement(statement) => {
                if !resolver.is_statement_node(statement.node_id()) {
                    return Err(inference_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "AST node {} is not a valid statement",
                            statement.node_id()
                        ),
                    ));
                }

                report.record_statement_reference()?;
            }

            InferenceMember::Directive(directive) => {
                register_source(
                    registered,
                    directive.source(),
                )?;

                validate_construct(
                    resolver,
                    directive.source(),
                )?;

                if let Some(payload) = directive.payload() {
                    validate_payload(
                        resolver,
                        payload,
                        registered,
                        report,
                    )?;
                }

                report.record_directive()?;
            }
        }
    }

    Ok(())
}

fn validate_payload<R: InferenceNodeResolver>(
    resolver: &R,
    payload: &InferenceDirectivePayload,
    registered: &mut BTreeSet<NodeId>,
    report: &mut InferenceValidationReport,
) -> AiResult<()> {
    match payload {
        InferenceDirectivePayload::Call(call) => {
            validate_call(
                resolver,
                call,
                report,
            )?;
        }

        InferenceDirectivePayload::TypedBinding(binding) => {
            validate_type(
                resolver,
                binding.declared_type(),
                report,
            )?;

            if let Some(call) = binding.call() {
                validate_call(
                    resolver,
                    call,
                    report,
                )?;
            }
        }

        InferenceDirectivePayload::Assignment(assignment) => {
            validate_expression(
                resolver,
                assignment.value(),
                report,
            )?;
        }

        InferenceDirectivePayload::Expression(expression) => {
            validate_expression(
                resolver,
                *expression,
                report,
            )?;
        }

        InferenceDirectivePayload::Region(members) => {
            validate_members(
                resolver,
                members,
                registered,
                report,
            )?;
        }

        InferenceDirectivePayload::NamedBinding(binding) => {
            if let Some(call) = binding.call() {
                validate_call(
                    resolver,
                    call,
                    report,
                )?;
            }
        }
    }

    Ok(())
}

fn validate_call<R: InferenceNodeResolver>(
    resolver: &R,
    call: &InferenceCall,
    report: &mut InferenceValidationReport,
) -> AiResult<()> {
    for argument in call.arguments() {
        validate_expression(
            resolver,
            argument.expression(),
            report,
        )?;
    }

    Ok(())
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic summary of inference structural validation.
///
/// Counts describe references examined by this module. They do not represent
/// proof of semantic validity or execution feasibility.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
)]
pub struct InferenceValidationReport {
    declarations_checked: usize,
    invocations_checked: usize,
    directives_checked: usize,
    statements_checked: usize,
    expression_references_checked: usize,
    type_references_checked: usize,
}

impl InferenceValidationReport {
    /// Return the number of declarations checked.
    #[must_use]
    pub const fn declarations_checked(&self) -> usize {
        self.declarations_checked
    }

    /// Return the number of invocations checked.
    #[must_use]
    pub const fn invocations_checked(&self) -> usize {
        self.invocations_checked
    }

    /// Return the number of directives checked.
    #[must_use]
    pub const fn directives_checked(&self) -> usize {
        self.directives_checked
    }

    /// Return the number of ordinary statement references checked.
    #[must_use]
    pub const fn statements_checked(&self) -> usize {
        self.statements_checked
    }

    /// Return the number of expression references checked.
    #[must_use]
    pub const fn expression_references_checked(&self) -> usize {
        self.expression_references_checked
    }

    /// Return the number of type references checked.
    #[must_use]
    pub const fn type_references_checked(&self) -> usize {
        self.type_references_checked
    }

    /// Return the total number of top-level inference constructs checked.
    #[must_use]
    pub const fn constructs_checked(&self) -> usize {
        self.declarations_checked + self.invocations_checked
    }

    fn record_declaration(&mut self) -> AiResult<()> {
        self.declarations_checked = self
            .declarations_checked
            .checked_add(1)
            .ok_or_else(|| {
                inference_error(
                    AiErrorKind::ResourceExhausted,
                    "inference validation declaration counter overflowed",
                )
            })?;

        Ok(())
    }

    fn record_invocation(&mut self) -> AiResult<()> {
        self.invocations_checked = self
            .invocations_checked
            .checked_add(1)
            .ok_or_else(|| {
                inference_error(
                    AiErrorKind::ResourceExhausted,
                    "inference validation invocation counter overflowed",
                )
            })?;

        Ok(())
    }

    fn record_directive(&mut self) -> AiResult<()> {
        self.directives_checked = self
            .directives_checked
            .checked_add(1)
            .ok_or_else(|| {
                inference_error(
                    AiErrorKind::ResourceExhausted,
                    "inference validation directive counter overflowed",
                )
            })?;

        Ok(())
    }

    fn record_statement_reference(&mut self) -> AiResult<()> {
        self.statements_checked = self
            .statements_checked
            .checked_add(1)
            .ok_or_else(|| {
                inference_error(
                    AiErrorKind::ResourceExhausted,
                    "inference validation statement counter overflowed",
                )
            })?;

        Ok(())
    }

    fn record_expression_reference(&mut self) -> AiResult<()> {
        self.expression_references_checked = self
            .expression_references_checked
            .checked_add(1)
            .ok_or_else(|| {
                inference_error(
                    AiErrorKind::ResourceExhausted,
                    "inference validation expression counter overflowed",
                )
            })?;

        Ok(())
    }

    fn record_type_reference(&mut self) -> AiResult<()> {
        self.type_references_checked = self
            .type_references_checked
            .checked_add(1)
            .ok_or_else(|| {
                inference_error(
                    AiErrorKind::ResourceExhausted,
                    "inference validation type counter overflowed",
                )
            })?;

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[derive(Default)]
    struct TestResolver {
        constructs: BTreeSet<NodeId>,
        expressions: BTreeSet<NodeId>,
        types: BTreeSet<NodeId>,
        statements: BTreeSet<NodeId>,
    }

    impl TestResolver {
        fn add_construct(&mut self, id: u64) {
            self.constructs.insert(
                NodeId::new(id).expect("nonzero node ID"),
            );
        }

        fn add_expression(&mut self, id: u64) {
            self.expressions.insert(
                NodeId::new(id).expect("nonzero node ID"),
            );
        }

        fn add_type(&mut self, id: u64) {
            self.types.insert(
                NodeId::new(id).expect("nonzero node ID"),
            );
        }

        fn add_statement(&mut self, id: u64) {
            self.statements.insert(
                NodeId::new(id).expect("nonzero node ID"),
            );
        }
    }

    impl InferenceNodeResolver for TestResolver {
        fn is_inference_construct_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.constructs.contains(&node_id)
        }

        fn is_expression_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.expressions.contains(&node_id)
        }

        fn is_type_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.types.contains(&node_id)
        }

        fn is_statement_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.statements.contains(&node_id)
        }
    }

    fn node(id: u64) -> NodeId {
        NodeId::new(id).expect("nonzero node ID")
    }

    fn name(value: &str) -> InferenceName {
        InferenceName::new(value).expect("valid inference name")
    }

    #[test]
    fn inference_name_rejects_empty_values() {
        assert!(InferenceName::new("").is_err());
    }

    #[test]
    fn inference_name_rejects_control_characters() {
        assert!(InferenceName::new("model\nname").is_err());
    }

    #[test]
    fn inference_name_preserves_valid_text() {
        let value = InferenceName::new("custom_provider")
            .expect("valid name");

        assert_eq!(value.as_str(), "custom_provider");
    }

    #[test]
    fn expression_reference_preserves_canonical_identity() {
        let reference = InferenceExpressionRef::new(node(12));

        assert_eq!(reference.node_id(), node(12));
    }

    #[test]
    fn type_reference_preserves_canonical_identity() {
        let reference = InferenceTypeRef::new(node(13));

        assert_eq!(reference.node_id(), node(13));
    }

    #[test]
    fn declaration_preserves_its_source_references() {
        let declaration = InferenceDeclaration::new(
            InferenceConstructRef::new(node(1)),
            name("classifier"),
            Some(InferenceTypeRef::new(node(2))),
            Some(InferenceExpressionRef::new(node(3))),
            Vec::new(),
        );

        assert_eq!(declaration.source().node_id(), node(1));
        assert_eq!(declaration.name().as_str(), "classifier");
        assert_eq!(
            declaration.declared_type().expect("type").node_id(),
            node(2)
        );
        assert_eq!(
            declaration.initializer().expect("initializer").node_id(),
            node(3)
        );
    }

    #[test]
    fn invocation_preserves_target_and_arguments() {
        let invocation = InferenceInvocation::new(
            InferenceConstructRef::new(node(1)),
            InferenceExpressionRef::new(node(2)),
            vec![
                InferenceArgument::new(
                    InferenceExpressionRef::new(node(3)),
                ),
                InferenceArgument::new(
                    InferenceExpressionRef::new(node(4)),
                ),
            ],
        );

        assert_eq!(invocation.target().node_id(), node(2));
        assert_eq!(invocation.arguments().len(), 2);
        assert_eq!(
            invocation.arguments()[1].expression().node_id(),
            node(4)
        );
    }

    #[test]
    fn directives_support_open_ended_names() {
        let directive = InferenceDirective::new(
            InferenceConstructRef::new(node(1)),
            name("future_provider_extension"),
            None,
        );

        assert_eq!(
            directive.name().as_str(),
            "future_provider_extension"
        );
    }

    #[test]
    fn nested_directive_regions_preserve_members() {
        let nested = InferenceDirective::new(
            InferenceConstructRef::new(node(2)),
            name("precision"),
            Some(InferenceDirectivePayload::Expression(
                InferenceExpressionRef::new(node(3)),
            )),
        );

        let outer = InferenceDirective::new(
            InferenceConstructRef::new(node(1)),
            name("configuration"),
            Some(InferenceDirectivePayload::Region(vec![
                InferenceMember::Directive(nested),
            ])),
        );

        assert!(matches!(
            outer.payload(),
            Some(InferenceDirectivePayload::Region(members))
                if members.len() == 1
        ));
    }

    #[test]
    fn program_rejects_duplicate_top_level_constructs() {
        let mut program = InferenceProgram::new();

        let first = InferenceInvocation::new(
            InferenceConstructRef::new(node(1)),
            InferenceExpressionRef::new(node(2)),
            Vec::new(),
        );

        let duplicate = InferenceInvocation::new(
            InferenceConstructRef::new(node(1)),
            InferenceExpressionRef::new(node(3)),
            Vec::new(),
        );

        program.add_invocation(first).expect("first registration");

        assert!(program.add_invocation(duplicate).is_err());
    }

    #[test]
    fn program_rejects_duplicate_across_construct_categories() {
        let mut program = InferenceProgram::new();

        let declaration = InferenceDeclaration::new(
            InferenceConstructRef::new(node(1)),
            name("classifier"),
            None,
            None,
            Vec::new(),
        );

        let invocation = InferenceInvocation::new(
            InferenceConstructRef::new(node(1)),
            InferenceExpressionRef::new(node(2)),
            Vec::new(),
        );

        program
            .add_declaration(declaration)
            .expect("declaration registration");

        assert!(program.add_invocation(invocation).is_err());
    }

    #[test]
    fn valid_invocation_passes_ast_validation() {
        let mut resolver = TestResolver::default();

        resolver.add_construct(1);
        resolver.add_expression(2);
        resolver.add_expression(3);

        let mut program = InferenceProgram::new();

        program
            .add_invocation(InferenceInvocation::new(
                InferenceConstructRef::new(node(1)),
                InferenceExpressionRef::new(node(2)),
                vec![
                    InferenceArgument::new(
                        InferenceExpressionRef::new(node(3)),
                    ),
                ],
            ))
            .expect("registration");

        let report = program
            .validate(&resolver)
            .expect("valid inference structure");

        assert_eq!(report.invocations_checked(), 1);
        assert_eq!(report.expression_references_checked(), 2);
    }

    #[test]
    fn invalid_invocation_target_is_rejected() {
        let mut resolver = TestResolver::default();

        resolver.add_construct(1);
        resolver.add_expression(3);

        let mut program = InferenceProgram::new();

        program
            .add_invocation(InferenceInvocation::new(
                InferenceConstructRef::new(node(1)),
                InferenceExpressionRef::new(node(2)),
                Vec::new(),
            ))
            .expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn invalid_type_reference_is_rejected() {
        let mut resolver = TestResolver::default();

        resolver.add_construct(1);
        resolver.add_type(3);

        let mut program = InferenceProgram::new();

        program
            .add_declaration(InferenceDeclaration::new(
                InferenceConstructRef::new(node(1)),
                name("classifier"),
                Some(InferenceTypeRef::new(node(2))),
                None,
                Vec::new(),
            ))
            .expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn valid_declaration_and_nested_directive_validate() {
        let mut resolver = TestResolver::default();

        resolver.add_construct(1);
        resolver.add_construct(2);
        resolver.add_expression(3);
        resolver.add_type(4);
        resolver.add_statement(5);

        let nested = InferenceDirective::new(
            InferenceConstructRef::new(node(2)),
            name("input"),
            Some(InferenceDirectivePayload::TypedBinding(
                InferenceTypedBinding::new(
                    name("features"),
                    InferenceTypeRef::new(node(4)),
                    None,
                ),
            )),
        );

        let declaration = InferenceDeclaration::new(
            InferenceConstructRef::new(node(1)),
            name("classifier"),
            None,
            Some(InferenceExpressionRef::new(node(3))),
            vec![
                InferenceMember::Statement(
                    InferenceStatementRef::new(node(5)),
                ),
                InferenceMember::Directive(nested),
            ],
        );

        let mut program = InferenceProgram::new();

        program
            .add_declaration(declaration)
            .expect("registration");

        let report = program
            .validate(&resolver)
            .expect("valid declaration");

        assert_eq!(report.declarations_checked(), 1);
        assert_eq!(report.directives_checked(), 1);
        assert_eq!(report.statements_checked(), 1);
        assert_eq!(report.expression_references_checked(), 1);
        assert_eq!(report.type_references_checked(), 1);
    }

    #[test]
    fn invalid_nested_statement_reference_is_rejected() {
        let mut resolver = TestResolver::default();

        resolver.add_construct(1);
        resolver.add_statement(3);

        let declaration = InferenceDeclaration::new(
            InferenceConstructRef::new(node(1)),
            name("classifier"),
            None,
            None,
            vec![
                InferenceMember::Statement(
                    InferenceStatementRef::new(node(2)),
                ),
            ],
        );

        let mut program = InferenceProgram::new();

        program
            .add_declaration(declaration)
            .expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn empty_program_is_valid_and_deterministic() {
        let resolver = TestResolver::default();
        let program = InferenceProgram::new();

        let first = program.validate(&resolver).expect("validation");
        let second = program.validate(&resolver).expect("validation");

        assert_eq!(first, second);
        assert_eq!(first.constructs_checked(), 0);
    }

    #[test]
    fn no_hardware_limit_is_encoded_in_inference_types() {
        // The inference representation contains no device, worker, memory,
        // accelerator or model-capacity ceiling.
        //
        // This test documents the architectural contract.
        let program = InferenceProgram::new();

        assert!(program.declarations().is_empty());
        assert!(program.invocations().is_empty());
    }
}
