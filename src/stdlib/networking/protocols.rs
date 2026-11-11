
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Networking Protocols
//!
//! Path: `src/stdlib/networking/protocols.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, extensible protocol contracts and deterministic
//! protocol-offer negotiation.
//!
//! A protocol describes logical communication behavior and obligations.
//! It does not select or implement a physical communication mechanism.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Protocol identity and revision identifiers.
//! - Protocol refinement references.
//! - Generic protocol parameters.
//! - Logical participant roles.
//! - Message and operation references.
//! - Protocol operation signatures.
//! - Protocol contract clauses.
//! - Capability references and declarations.
//! - Protocol properties.
//! - Effects, policies, evidence and provenance references.
//! - Abstract protocol states and transitions.
//! - Logical interaction-flow descriptions.
//! - Protocol offers.
//! - Protocol compatibility policies.
//! - Negotiated protocol results.
//!
//! This module does not own:
//!
//! - Source grammar or parsing.
//! - AST construction.
//! - Name resolution or semantic type checking.
//! - Message schema definitions.
//! - Address or endpoint declarations.
//! - Transport implementations.
//! - Serialization or compression.
//! - Cryptography or authentication.
//! - Authorization or policy enforcement.
//! - Service discovery.
//! - Routing or topology.
//! - Scheduling or placement.
//! - Distributed execution.
//! - Runtime queues.
//! - Hardware discovery or allocation.
//! - Physical communication.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ## Grammar authority
//!
//! Canonical protocol grammar:
//! `grammar/networking/protocols.g4`
//!
//! Networking composition:
//! `grammar/networking/networking.g4`
//!
//! Networking architecture:
//! `grammar/networking/README.md`
//!
//! Normative networking specification:
//! `grammar/spec/networking.md`
//!
//! The frontend must translate validated grammar/AST structures into
//! these types. This module does not parse Zamani source.
//!
//! ## POCO-REAF
//!
//! Protocol and capability identities are open-world identifiers.
//! New protocol technologies must not require adding Rust enum variants.
//!
//! This module imposes no arbitrary universal limits on:
//!
//! - Protocols.
//! - Roles.
//! - Messages.
//! - Operations.
//! - Generic parameters.
//! - Capabilities.
//! - Properties.
//! - States.
//! - Transitions.
//! - Flows.
//! - Flow steps.
//! - Refinement depth.
//! - Qualified-name depth.
//!
//! Actual limits arise from available resources, provider capabilities,
//! explicit budgets and physical reality.
//!
//! ## Integration
//!
//! Address contracts:
//! `crate::stdlib::networking::addresses`
//!
//! Transport contracts:
//! `crate::stdlib::networking::transports`
//!
//! Security grammar:
//! `grammar/networking/security.g4`
//!
//! Message grammar:
//! `grammar/networking/messages.g4`
//!
//! Endpoint grammar:
//! `grammar/networking/endpoints.g4`
//!
//! Capability grammar:
//! `grammar/networking/network-capabilities.g4`
//!
//! Service discovery grammar:
//! `grammar/networking/service-discovery.g4`
//!
//! Runtime:
//! `crate::runtime`
//!
//! Distributed execution:
//! `crate::distributed`
//!
//! Quantum computation:
//! `crate::quantum`
//!
//! Hardware description:
//! `crate::hdl` and its authoritative hardware subsystem.
//!
//! This module has no dependency on those execution implementations.
//!
//! ## Negotiation
//!
//! Negotiation is a deterministic operation over explicit protocol offers.
//! Revision compatibility is supplied through a policy interface.
//!
//! This module does not assume that matching protocol names alone prove
//! compatibility, nor does it silently satisfy missing capabilities.
//!
//! ## Safety
//!
//! Safe Rust only. No network I/O, global mutable state, unsafe code,
//! operating-system calls or hidden provider execution.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error as StdError;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates an open-world qualified identifier.
///
/// Components are separated by `::`.
///
/// Each component starts with an ASCII letter, digit or underscore and
/// continues with ASCII letters, digits, underscores, hyphens or dots.
///
/// This is an identifier convention, not a catalogue of known protocols.
fn is_valid_qualified_identifier(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    if value.chars().any(char::is_control) {
        return false;
    }

    value.split("::").all(|component| {
        let mut bytes = component.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        (first.is_ascii_alphanumeric() || first == b'_')
            && bytes.all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
                    || byte == b'.'
            })
    })
}

/// Validates a single, unqualified identifier.
fn is_valid_identifier(value: &str) -> bool {
    !value.contains("::") && is_valid_qualified_identifier(value)
}

/// Validates opaque expression or type text.
///
/// The frontend remains responsible for parsing and validating expressions
/// and types. This function only checks basic representation integrity.
fn validate_expression_text(value: &str) -> Result<(), ProtocolError> {
    if value.trim().is_empty() {
        return Err(ProtocolError::new(
            ProtocolErrorKind::InvalidExpression,
            "protocol expression cannot be empty",
        ));
    }

    if value.contains('\0') {
        return Err(ProtocolError::new(
            ProtocolErrorKind::InvalidExpression,
            "protocol expression contains a NUL character",
        ));
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of protocol errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProtocolErrorKind {
    /// A protocol identifier is invalid.
    InvalidProtocolId,

    /// A protocol revision is invalid.
    InvalidRevision,

    /// A role identifier is invalid.
    InvalidRole,

    /// A message reference is invalid.
    InvalidMessage,

    /// An operation declaration is invalid.
    InvalidOperation,

    /// A generic parameter is invalid.
    InvalidTypeParameter,

    /// A capability reference is invalid.
    InvalidCapability,

    /// A property declaration is invalid.
    InvalidProperty,

    /// A state declaration is invalid.
    InvalidState,

    /// A transition declaration is invalid.
    InvalidTransition,

    /// A flow declaration is invalid.
    InvalidFlow,

    /// An expression representation is invalid.
    InvalidExpression,

    /// A protocol definition contains duplicate declarations.
    DuplicateDeclaration,

    /// A protocol definition refers to an unknown local state.
    UnknownState,

    /// Two protocol offers identify different protocols.
    ProtocolMismatch,

    /// The offered revisions are incompatible.
    IncompatibleRevision,

    /// A required capability is not provided by the peer.
    MissingCapability,

    /// A protocol offer is invalid.
    InvalidOffer,

    /// A provider or compatibility policy failed.
    NegotiationFailed,
}

impl ProtocolErrorKind {
    /// Returns a stable textual error classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidProtocolId => "invalid_protocol_id",
            Self::InvalidRevision => "invalid_revision",
            Self::InvalidRole => "invalid_role",
            Self::InvalidMessage => "invalid_message",
            Self::InvalidOperation => "invalid_operation",
            Self::InvalidTypeParameter => "invalid_type_parameter",
            Self::InvalidCapability => "invalid_capability",
            Self::InvalidProperty => "invalid_property",
            Self::InvalidState => "invalid_state",
            Self::InvalidTransition => "invalid_transition",
            Self::InvalidFlow => "invalid_flow",
            Self::InvalidExpression => "invalid_expression",
            Self::DuplicateDeclaration => "duplicate_declaration",
            Self::UnknownState => "unknown_state",
            Self::ProtocolMismatch => "protocol_mismatch",
            Self::IncompatibleRevision => "incompatible_revision",
            Self::MissingCapability => "missing_capability",
            Self::InvalidOffer => "invalid_offer",
            Self::NegotiationFailed => "negotiation_failed",
        }
    }
}

impl fmt::Display for ProtocolErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured protocol error.
#[derive(Debug)]
pub struct ProtocolError {
    kind: ProtocolErrorKind,
    message: String,
    related: Vec<String>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl ProtocolError {
    /// Creates a protocol error.
    #[must_use]
    pub fn new(
        kind: ProtocolErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            related: Vec::new(),
            source: None,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> ProtocolErrorKind {
        self.kind
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns related identifiers or diagnostic details.
    #[must_use]
    pub fn related(&self) -> &[String] {
        &self.related
    }

    /// Adds a related diagnostic detail.
    #[must_use]
    pub fn with_related(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.related.push(value.into());
        self
    }

    /// Attaches an underlying error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns whether an underlying error is attached.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "networking protocol error [{}]: {}",
            self.kind,
            self.message
        )
    }
}

impl StdError for ProtocolError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for protocol operations.
pub type ProtocolResult<T> = Result<T, ProtocolError>;

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

macro_rules! protocol_identifier {
    (
        $(#[$meta:meta])*
        $name:ident,
        $error_kind:expr
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            pub fn new(value: impl Into<String>) -> ProtocolResult<Self> {
                let value = value.into();

                if !is_valid_qualified_identifier(&value) {
                    return Err(ProtocolError::new(
                        $error_kind,
                        concat!("invalid ", stringify!($name), " identifier"),
                    ));
                }

                Ok(Self(value))
            }

            /// Returns the identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Checks equality with a textual identifier.
            #[must_use]
            pub fn matches(&self, value: &str) -> bool {
                self.0 == value
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = ProtocolError;

            fn from_str(value: &str) -> ProtocolResult<Self> {
                Self::new(value)
            }
        }
    };
}

protocol_identifier!(
    /// Open-world protocol identity.
    ///
    /// Examples include `network::reliable`, `application::request_response`
    /// and names introduced by future protocol ecosystems.
    ProtocolId,
    ProtocolErrorKind::InvalidProtocolId
);

protocol_identifier!(
    /// Logical participant role identity.
    RoleId,
    ProtocolErrorKind::InvalidRole
);

protocol_identifier!(
    /// Message-schema reference.
    ///
    /// Message schema ownership remains with the networking message subsystem.
    MessageId,
    ProtocolErrorKind::InvalidMessage
);

protocol_identifier!(
    /// Protocol operation identity.
    OperationId,
    ProtocolErrorKind::InvalidOperation
);

protocol_identifier!(
    /// Protocol capability identity.
    CapabilityId,
    ProtocolErrorKind::InvalidCapability
);

protocol_identifier!(
    /// Abstract protocol state identity.
    StateId,
    ProtocolErrorKind::InvalidState
);

protocol_identifier!(
    /// Protocol effect reference.
    EffectId,
    ProtocolErrorKind::InvalidProperty
);

protocol_identifier!(
    /// Protocol policy reference.
    PolicyId,
    ProtocolErrorKind::InvalidProperty
);

protocol_identifier!(
    /// Protocol evidence reference.
    EvidenceId,
    ProtocolErrorKind::InvalidProperty
);

protocol_identifier!(
    /// Protocol provenance reference.
    ProvenanceId,
    ProtocolErrorKind::InvalidProperty
);

// ============================================================================
// PROTOCOL REVISION
// ============================================================================

/// Opaque protocol revision identity.
///
/// The standard library deliberately does not impose semantic-versioning
/// rules. A language, protocol authority or compatibility provider may
/// define its own revision scheme.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProtocolRevision(String);

impl ProtocolRevision {
    /// Creates a revision identity.
    pub fn new(value: impl Into<String>) -> ProtocolResult<Self> {
        let value = value.into();

        if value.trim().is_empty() || value.contains('\0') {
            return Err(ProtocolError::new(
                ProtocolErrorKind::InvalidRevision,
                "protocol revision cannot be empty or contain NUL",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque revision.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProtocolRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EXPRESSIONS AND TYPE PARAMETERS
// ============================================================================

/// Opaque source expression or type-expression representation.
///
/// This is not a second expression or type system. The frontend must
/// construct it from the canonical AST and semantic model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProtocolExpression(String);

impl ProtocolExpression {
    /// Creates a non-empty expression representation.
    pub fn new(value: impl Into<String>) -> ProtocolResult<Self> {
        let value = value.into();
        validate_expression_text(&value)?;
        Ok(Self(value))
    }

    /// Returns the represented source expression.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Generic protocol type parameter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolTypeParameter {
    name: String,
    bound: Option<ProtocolExpression>,
}

impl ProtocolTypeParameter {
    /// Creates a generic parameter.
    pub fn new(
        name: impl Into<String>,
        bound: Option<ProtocolExpression>,
    ) -> ProtocolResult<Self> {
        let name = name.into();

        if !is_valid_identifier(&name) {
            return Err(ProtocolError::new(
                ProtocolErrorKind::InvalidTypeParameter,
                "invalid protocol type parameter name",
            ));
        }

        Ok(Self { name, bound })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional type bound.
    #[must_use]
    pub fn bound(&self) -> Option<&ProtocolExpression> {
        self.bound.as_ref()
    }
}

/// Parameter of a protocol operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolParameter {
    name: String,
    type_expression: ProtocolExpression,
    default_value: Option<ProtocolExpression>,
}

impl ProtocolParameter {
    /// Creates an operation parameter.
    pub fn new(
        name: impl Into<String>,
        type_expression: ProtocolExpression,
        default_value: Option<ProtocolExpression>,
    ) -> ProtocolResult<Self> {
        let name = name.into();

        if !is_valid_identifier(&name) {
            return Err(ProtocolError::new(
                ProtocolErrorKind::InvalidOperation,
                "invalid protocol operation parameter name",
            ));
        }

        Ok(Self {
            name,
            type_expression,
            default_value,
        })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the parameter type expression.
    #[must_use]
    pub fn type_expression(&self) -> &ProtocolExpression {
        &self.type_expression
    }

    /// Returns the optional default expression.
    #[must_use]
    pub fn default_value(&self) -> Option<&ProtocolExpression> {
        self.default_value.as_ref()
    }
}

// ============================================================================
// ROLES, MESSAGES AND OPERATIONS
// ============================================================================

/// Logical participant role declaration.
///
/// A role is not a physical machine, process, device, thread or endpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolRole {
    id: RoleId,
    role_type: Option<ProtocolExpression>,
}

impl ProtocolRole {
    /// Creates a logical role.
    #[must_use]
    pub fn new(id: RoleId) -> Self {
        Self {
            id,
            role_type: None,
        }
    }

    /// Sets the optional role type expression.
    #[must_use]
    pub fn with_type(mut self, value: ProtocolExpression) -> Self {
        self.role_type = Some(value);
        self
    }

    /// Returns the role identity.
    #[must_use]
    pub fn id(&self) -> &RoleId {
        &self.id
    }

    /// Returns the optional role type.
    #[must_use]
    pub fn role_type(&self) -> Option<&ProtocolExpression> {
        self.role_type.as_ref()
    }
}

/// Reference to a message schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolMessage {
    id: MessageId,
    direction: Option<ProtocolExpression>,
}

impl ProtocolMessage {
    /// Creates a message reference.
    #[must_use]
    pub fn new(id: MessageId) -> Self {
        Self {
            id,
            direction: None,
        }
    }

    /// Sets an optional direction expression.
    #[must_use]
    pub fn with_direction(mut self, value: ProtocolExpression) -> Self {
        self.direction = Some(value);
        self
    }

    /// Returns the message identity.
    #[must_use]
    pub fn id(&self) -> &MessageId {
        &self.id
    }

    /// Returns the optional direction expression.
    #[must_use]
    pub fn direction(&self) -> Option<&ProtocolExpression> {
        self.direction.as_ref()
    }
}

/// Protocol operation declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolOperation {
    id: OperationId,
    type_parameters: Vec<ProtocolTypeParameter>,
    parameters: Vec<ProtocolParameter>,
    return_type: Option<ProtocolExpression>,
    clauses: Vec<ProtocolClause>,
}

impl ProtocolOperation {
    /// Creates an operation declaration.
    #[must_use]
    pub fn new(id: OperationId) -> Self {
        Self {
            id,
            type_parameters: Vec::new(),
            parameters: Vec::new(),
            return_type: None,
            clauses: Vec::new(),
        }
    }

    /// Adds a generic parameter.
    #[must_use]
    pub fn with_type_parameter(
        mut self,
        parameter: ProtocolTypeParameter,
    ) -> Self {
        self.type_parameters.push(parameter);
        self
    }

    /// Adds an operation parameter.
    #[must_use]
    pub fn with_parameter(
        mut self,
        parameter: ProtocolParameter,
    ) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// Sets the optional return type.
    #[must_use]
    pub fn with_return_type(
        mut self,
        return_type: ProtocolExpression,
    ) -> Self {
        self.return_type = Some(return_type);
        self
    }

    /// Adds an operation contract clause.
    #[must_use]
    pub fn with_clause(mut self, clause: ProtocolClause) -> Self {
        self.clauses.push(clause);
        self
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn id(&self) -> &OperationId {
        &self.id
    }

    /// Returns generic parameters.
    #[must_use]
    pub fn type_parameters(&self) -> &[ProtocolTypeParameter] {
        &self.type_parameters
    }

    /// Returns operation parameters.
    #[must_use]
    pub fn parameters(&self) -> &[ProtocolParameter] {
        &self.parameters
    }

    /// Returns the optional return type.
    #[must_use]
    pub fn return_type(&self) -> Option<&ProtocolExpression> {
        self.return_type.as_ref()
    }

    /// Returns operation clauses.
    #[must_use]
    pub fn clauses(&self) -> &[ProtocolClause] {
        &self.clauses
    }
}

// ============================================================================
// CONTRACTS, REQUIREMENTS AND CAPABILITIES
// ============================================================================

/// Kind of protocol contract clause.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProtocolClauseKind {
    /// Preconditions.
    Requires,

    /// Postconditions.
    Ensures,

    /// Invariants.
    Invariant,

    /// Assumptions.
    Assume,

    /// Guarantees.
    Guarantee,
}

/// A protocol contract clause.
///
/// The expression is preserved for the semantic authority to validate.
/// This module does not evaluate it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolClause {
    kind: ProtocolClauseKind,
    expression: ProtocolExpression,
}

impl ProtocolClause {
    /// Creates a contract clause.
    #[must_use]
    pub fn new(
        kind: ProtocolClauseKind,
        expression: ProtocolExpression,
    ) -> Self {
        Self { kind, expression }
    }

    /// Returns the clause kind.
    #[must_use]
    pub const fn kind(&self) -> ProtocolClauseKind {
        self.kind
    }

    /// Returns the expression.
    #[must_use]
    pub fn expression(&self) -> &ProtocolExpression {
        &self.expression
    }
}

/// Protocol capability declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolCapability {
    id: CapabilityId,
    value: Option<ProtocolExpression>,
}

impl ProtocolCapability {
    /// Creates a capability declaration.
    #[must_use]
    pub fn new(id: CapabilityId) -> Self {
        Self { id, value: None }
    }

    /// Sets an optional capability expression.
    #[must_use]
    pub fn with_value(mut self, value: ProtocolExpression) -> Self {
        self.value = Some(value);
        self
    }

    /// Returns the capability identity.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the optional value expression.
    #[must_use]
    pub fn value(&self) -> Option<&ProtocolExpression> {
        self.value.as_ref()
    }
}

/// Generic protocol property.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolProperty {
    name: String,
    type_expression: Option<ProtocolExpression>,
    initializer: Option<ProtocolExpression>,
}

impl ProtocolProperty {
    /// Creates a property declaration.
    pub fn new(
        name: impl Into<String>,
        type_expression: Option<ProtocolExpression>,
        initializer: Option<ProtocolExpression>,
    ) -> ProtocolResult<Self> {
        let name = name.into();

        if !is_valid_identifier(&name) {
            return Err(ProtocolError::new(
                ProtocolErrorKind::InvalidProperty,
                "invalid protocol property name",
            ));
        }

        Ok(Self {
            name,
            type_expression,
            initializer,
        })
    }

    /// Returns the property name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional type expression.
    #[must_use]
    pub fn type_expression(&self) -> Option<&ProtocolExpression> {
        self.type_expression.as_ref()
    }

    /// Returns the optional initializer.
    #[must_use]
    pub fn initializer(&self) -> Option<&ProtocolExpression> {
        self.initializer.as_ref()
    }
}

// ============================================================================
// ABSTRACT STATE MACHINES
// ============================================================================

/// Abstract protocol state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolState {
    id: StateId,
}

impl ProtocolState {
    /// Creates an abstract protocol state.
    #[must_use]
    pub fn new(id: StateId) -> Self {
        Self { id }
    }

    /// Returns the state identity.
    #[must_use]
    pub fn id(&self) -> &StateId {
        &self.id
    }
}

/// Abstract state transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolTransition {
    from: StateId,
    to: StateId,
    event: Option<ProtocolExpression>,
    conditions: Vec<ProtocolExpression>,
}

impl ProtocolTransition {
    /// Creates a transition between abstract states.
    #[must_use]
    pub fn new(from: StateId, to: StateId) -> Self {
        Self {
            from,
            to,
            event: None,
            conditions: Vec::new(),
        }
    }

    /// Sets the optional transition event.
    #[must_use]
    pub fn with_event(mut self, event: ProtocolExpression) -> Self {
        self.event = Some(event);
        self
    }

    /// Adds a transition condition.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: ProtocolExpression,
    ) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Returns the source state.
    #[must_use]
    pub fn from(&self) -> &StateId {
        &self.from
    }

    /// Returns the destination state.
    #[must_use]
    pub fn to(&self) -> &StateId {
        &self.to
    }

    /// Returns the optional event expression.
    #[must_use]
    pub fn event(&self) -> Option<&ProtocolExpression> {
        self.event.as_ref()
    }

    /// Returns transition conditions.
    #[must_use]
    pub fn conditions(&self) -> &[ProtocolExpression] {
        &self.conditions
    }
}

// ============================================================================
// LOGICAL INTERACTION FLOWS
// ============================================================================

/// Contract clause embedded in a protocol flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlowContract {
    kind: ProtocolClauseKind,
    expression: ProtocolExpression,
}

impl FlowContract {
    /// Creates a flow contract.
    #[must_use]
    pub fn new(
        kind: ProtocolClauseKind,
        expression: ProtocolExpression,
    ) -> Self {
        Self { kind, expression }
    }

    /// Returns the contract kind.
    #[must_use]
    pub const fn kind(&self) -> ProtocolClauseKind {
        self.kind
    }

    /// Returns the contract expression.
    #[must_use]
    pub fn expression(&self) -> &ProtocolExpression {
        &self.expression
    }
}

/// One logical protocol interaction step.
///
/// These steps describe interaction intent. They do not perform I/O,
/// create tasks, establish connections or schedule physical operations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProtocolFlowStep {
    /// Send a logical message or expression.
    Send(ProtocolExpression),

    /// Receive a logical message or expression.
    Receive(ProtocolExpression),

    /// Emit a logical event or expression.
    Emit(ProtocolExpression),

    /// Invoke another named flow.
    Call {
        /// Target flow identity.
        target: ProtocolId,

        /// Opaque argument expressions.
        arguments: Vec<ProtocolExpression>,
    },

    /// Execute a sequence of logical steps.
    Sequence(Vec<ProtocolFlowStep>),

    /// Select one of several alternative branches.
    Choice(Vec<Vec<ProtocolFlowStep>>),

    /// Describe logically parallel interaction.
    ///
    /// Runtime and compiler planning decide whether this is implemented
    /// concurrently, pipelined, multiplexed, serialized or distributed.
    Parallel(Vec<ProtocolFlowStep>),

    /// Repeat a logical interaction while a condition applies.
    Repeat {
        /// Optional repetition condition.
        condition: Option<ProtocolExpression>,

        /// Repeated steps.
        steps: Vec<ProtocolFlowStep>,
    },

    /// Describe an optional interaction.
    Optional(Vec<ProtocolFlowStep>),

    /// Wait for a logical condition or event.
    Wait(ProtocolExpression),

    /// Attach a contract to the flow.
    Contract(FlowContract),

    /// Preserve a canonical expression step.
    Expression(ProtocolExpression),
}

/// Named or anonymous protocol flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolFlow {
    name: Option<String>,
    steps: Vec<ProtocolFlowStep>,
}

impl ProtocolFlow {
    /// Creates a flow with an optional local identifier.
    pub fn new(name: Option<String>) -> ProtocolResult<Self> {
        if let Some(name) = &name {
            if !is_valid_identifier(name) {
                return Err(ProtocolError::new(
                    ProtocolErrorKind::InvalidFlow,
                    "invalid protocol flow name",
                ));
            }
        }

        Ok(Self {
            name,
            steps: Vec::new(),
        })
    }

    /// Adds a logical flow step.
    #[must_use]
    pub fn with_step(mut self, step: ProtocolFlowStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Appends a logical flow step.
    pub fn add_step(&mut self, step: ProtocolFlowStep) {
        self.steps.push(step);
    }

    /// Returns the optional flow name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the ordered flow steps.
    #[must_use]
    pub fn steps(&self) -> &[ProtocolFlowStep] {
        &self.steps
    }
}

// ============================================================================
// PROTOCOL DEFINITION
// ============================================================================

/// Complete portable protocol contract.
///
/// This structure preserves protocol-level declarations from
/// `grammar/networking/protocols.g4`.
///
/// Semantic interpretation, type checking, name resolution, contract
/// verification and policy enforcement remain outside this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolDefinition {
    id: ProtocolId,
    revision: Option<ProtocolRevision>,
    refinements: Vec<ProtocolId>,
    type_parameters: Vec<ProtocolTypeParameter>,
    roles: Vec<ProtocolRole>,
    messages: Vec<ProtocolMessage>,
    operations: Vec<ProtocolOperation>,
    clauses: Vec<ProtocolClause>,
    capabilities: Vec<ProtocolCapability>,
    properties: Vec<ProtocolProperty>,
    effects: Vec<EffectId>,
    policies: Vec<PolicyId>,
    evidence: Vec<EvidenceId>,
    provenance: Vec<ProvenanceId>,
    states: Vec<ProtocolState>,
    transitions: Vec<ProtocolTransition>,
    flows: Vec<ProtocolFlow>,
    nested: Vec<ProtocolDefinition>,
}

impl ProtocolDefinition {
    /// Creates a protocol definition.
    #[must_use]
    pub fn new(id: ProtocolId) -> Self {
        Self {
            id,
            revision: None,
            refinements: Vec::new(),
            type_parameters: Vec::new(),
            roles: Vec::new(),
            messages: Vec::new(),
            operations: Vec::new(),
            clauses: Vec::new(),
            capabilities: Vec::new(),
            properties: Vec::new(),
            effects: Vec::new(),
            policies: Vec::new(),
            evidence: Vec::new(),
            provenance: Vec::new(),
            states: Vec::new(),
            transitions: Vec::new(),
            flows: Vec::new(),
            nested: Vec::new(),
        }
    }

    /// Sets the protocol revision.
    #[must_use]
    pub fn with_revision(mut self, revision: ProtocolRevision) -> Self {
        self.revision = Some(revision);
        self
    }

    /// Adds a protocol refinement reference.
    #[must_use]
    pub fn with_refinement(mut self, id: ProtocolId) -> Self {
        self.refinements.push(id);
        self
    }

    /// Adds a generic parameter.
    #[must_use]
    pub fn with_type_parameter(
        mut self,
        parameter: ProtocolTypeParameter,
    ) -> Self {
        self.type_parameters.push(parameter);
        self
    }

    /// Adds a logical role.
    #[must_use]
    pub fn with_role(mut self, role: ProtocolRole) -> Self {
        self.roles.push(role);
        self
    }

    /// Adds a message reference.
    #[must_use]
    pub fn with_message(mut self, message: ProtocolMessage) -> Self {
        self.messages.push(message);
        self
    }

    /// Adds an operation.
    #[must_use]
    pub fn with_operation(mut self, operation: ProtocolOperation) -> Self {
        self.operations.push(operation);
        self
    }

    /// Adds a protocol contract clause.
    #[must_use]
    pub fn with_clause(mut self, clause: ProtocolClause) -> Self {
        self.clauses.push(clause);
        self
    }

    /// Adds a capability declaration.
    #[must_use]
    pub fn with_capability(mut self, capability: ProtocolCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Adds a property declaration.
    #[must_use]
    pub fn with_property(mut self, property: ProtocolProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Adds an effect reference.
    #[must_use]
    pub fn with_effect(mut self, effect: EffectId) -> Self {
        self.effects.push(effect);
        self
    }

    /// Adds a policy reference.
    #[must_use]
    pub fn with_policy(mut self, policy: PolicyId) -> Self {
        self.policies.push(policy);
        self
    }

    /// Adds an evidence reference.
    #[must_use]
    pub fn with_evidence(mut self, evidence: EvidenceId) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Adds a provenance reference.
    #[must_use]
    pub fn with_provenance(mut self, provenance: ProvenanceId) -> Self {
        self.provenance.push(provenance);
        self
    }

    /// Adds an abstract state.
    #[must_use]
    pub fn with_state(mut self, state: ProtocolState) -> Self {
        self.states.push(state);
        self
    }

    /// Adds a state transition.
    #[must_use]
    pub fn with_transition(mut self, transition: ProtocolTransition) -> Self {
        self.transitions.push(transition);
        self
    }

    /// Adds a logical interaction flow.
    #[must_use]
    pub fn with_flow(mut self, flow: ProtocolFlow) -> Self {
        self.flows.push(flow);
        self
    }

    /// Adds a nested protocol declaration.
    #[must_use]
    pub fn with_nested(mut self, protocol: ProtocolDefinition) -> Self {
        self.nested.push(protocol);
        self
    }

    /// Returns the protocol identity.
    #[must_use]
    pub fn id(&self) -> &ProtocolId {
        &self.id
    }

    /// Returns the optional revision.
    #[must_use]
    pub fn revision(&self) -> Option<&ProtocolRevision> {
        self.revision.as_ref()
    }

    /// Returns protocol refinements.
    #[must_use]
    pub fn refinements(&self) -> &[ProtocolId] {
        &self.refinements
    }

    /// Returns generic parameters.
    #[must_use]
    pub fn type_parameters(&self) -> &[ProtocolTypeParameter] {
        &self.type_parameters
    }

    /// Returns logical roles.
    #[must_use]
    pub fn roles(&self) -> &[ProtocolRole] {
        &self.roles
    }

    /// Returns message references.
    #[must_use]
    pub fn messages(&self) -> &[ProtocolMessage] {
        &self.messages
    }

    /// Returns operations.
    #[must_use]
    pub fn operations(&self) -> &[ProtocolOperation] {
        &self.operations
    }

    /// Returns contract clauses.
    #[must_use]
    pub fn clauses(&self) -> &[ProtocolClause] {
        &self.clauses
    }

    /// Returns capability declarations.
    #[must_use]
    pub fn capabilities(&self) -> &[ProtocolCapability] {
        &self.capabilities
    }

    /// Returns properties.
    #[must_use]
    pub fn properties(&self) -> &[ProtocolProperty] {
        &self.properties
    }

    /// Returns effect references.
    #[must_use]
    pub fn effects(&self) -> &[EffectId] {
        &self.effects
    }

    /// Returns policy references.
    #[must_use]
    pub fn policies(&self) -> &[PolicyId] {
        &self.policies
    }

    /// Returns evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceId] {
        &self.evidence
    }

    /// Returns provenance references.
    #[must_use]
    pub fn provenance(&self) -> &[ProvenanceId] {
        &self.provenance
    }

    /// Returns abstract states.
    #[must_use]
    pub fn states(&self) -> &[ProtocolState] {
        &self.states
    }

    /// Returns state transitions.
    #[must_use]
    pub fn transitions(&self) -> &[ProtocolTransition] {
        &self.transitions
    }

    /// Returns interaction flows.
    #[must_use]
    pub fn flows(&self) -> &[ProtocolFlow] {
        &self.flows
    }

    /// Returns nested protocols.
    #[must_use]
    pub fn nested(&self) -> &[ProtocolDefinition] {
        &self.nested
    }

    /// Performs local structural validation.
    ///
    /// This checks duplicate local declarations, state references and
    /// structural flow conditions. It does not perform semantic name
    /// resolution, type checking, capability feasibility or policy checks.
    pub fn validate_structure(&self) -> ProtocolResult<()> {
        validate_unique(
            self.type_parameters.iter().map(|item| item.name.as_str()),
            "generic type parameter",
        )?;

        validate_unique(
            self.roles.iter().map(|item| item.id.as_str()),
            "protocol role",
        )?;

        validate_unique(
            self.messages.iter().map(|item| item.id.as_str()),
            "message reference",
        )?;

        validate_unique(
            self.operations.iter().map(|item| item.id.as_str()),
            "protocol operation",
        )?;

        validate_unique(
            self.capabilities.iter().map(|item| item.id.as_str()),
            "protocol capability",
        )?;

        validate_unique(
            self.properties.iter().map(|item| item.name.as_str()),
            "protocol property",
        )?;

        validate_unique(
            self.states.iter().map(|item| item.id.as_str()),
            "protocol state",
        )?;

        let state_ids: BTreeSet<&str> = self
            .states
            .iter()
            .map(|state| state.id.as_str())
            .collect();

        for transition in &self.transitions {
            if !state_ids.contains(transition.from.as_str()) {
                return Err(
                    ProtocolError::new(
                        ProtocolErrorKind::UnknownState,
                        "transition references an unknown source state",
                    )
                    .with_related(transition.from.to_string()),
                );
            }

            if !state_ids.contains(transition.to.as_str()) {
                return Err(
                    ProtocolError::new(
                        ProtocolErrorKind::UnknownState,
                        "transition references an unknown destination state",
                    )
                    .with_related(transition.to.to_string()),
                );
            }
        }

        validate_unique(
            self.flows.iter().filter_map(|flow| flow.name()),
            "named protocol flow",
        )?;

        for flow in &self.flows {
            validate_flow_steps(flow.steps())?;
        }

        for operation in &self.operations {
            validate_unique(
                operation
                    .type_parameters
                    .iter()
                    .map(|item| item.name.as_str()),
                "operation type parameter",
            )?;

            validate_unique(
                operation
                    .parameters
                    .iter()
                    .map(|item| item.name.as_str()),
                "operation parameter",
            )?;
        }

        for nested in &self.nested {
            nested.validate_structure()?;
        }

        Ok(())
    }
}

fn validate_unique<'a>(
    values: impl IntoIterator<Item = &'a str>,
    description: &str,
) -> ProtocolResult<()> {
    let mut seen = BTreeSet::new();

    for value in values {
        if !seen.insert(value) {
            return Err(
                ProtocolError::new(
                    ProtocolErrorKind::DuplicateDeclaration,
                    format!("duplicate {description} declaration"),
                )
                .with_related(value.to_owned()),
            );
        }
    }

    Ok(())
}

/// Validates nested flow structures iteratively.
///
/// An explicit work stack avoids adding a language-defined flow-depth
/// limit or relying on recursive validation of arbitrarily nested flows.
fn validate_flow_steps(
    steps: &[ProtocolFlowStep],
) -> ProtocolResult<()> {
    let mut pending: Vec<&[ProtocolFlowStep]> = vec![steps];

    while let Some(current) = pending.pop() {
        for step in current {
            match step {
                ProtocolFlowStep::Sequence(children)
                | ProtocolFlowStep::Parallel(children)
                | ProtocolFlowStep::Optional(children) => {
                    pending.push(children);
                }

                ProtocolFlowStep::Choice(branches) => {
                    if branches.is_empty() {
                        return Err(ProtocolError::new(
                            ProtocolErrorKind::InvalidFlow,
                            "a choice flow must contain at least one branch",
                        ));
                    }

                    for branch in branches {
                        if branch.is_empty() {
                            return Err(ProtocolError::new(
                                ProtocolErrorKind::InvalidFlow,
                                "a choice branch cannot be empty",
                            ));
                        }

                        pending.push(branch);
                    }
                }

                ProtocolFlowStep::Repeat { steps, .. } => {
                    pending.push(steps);
                }

                ProtocolFlowStep::Send(_)
                | ProtocolFlowStep::Receive(_)
                | ProtocolFlowStep::Emit(_)
                | ProtocolFlowStep::Call { .. }
                | ProtocolFlowStep::Wait(_)
                | ProtocolFlowStep::Contract(_)
                | ProtocolFlowStep::Expression(_) => {}
            }
        }
    }

    Ok(())
}

// ============================================================================
// PROTOCOL OFFERS AND NEGOTIATION
// ============================================================================

/// A protocol offer presented by one participant.
///
/// An offer advertises supported logical roles, messages and operations,
/// capabilities it provides, and capabilities it requires.
///
/// It does not advertise a socket, endpoint, transport or hardware device.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolOffer {
    protocol: ProtocolId,
    revision: Option<ProtocolRevision>,
    roles: Vec<RoleId>,
    messages: Vec<MessageId>,
    operations: Vec<OperationId>,
    required_capabilities: Vec<CapabilityId>,
    provided_capabilities: Vec<CapabilityId>,
}

impl ProtocolOffer {
    /// Creates an offer for a protocol.
    #[must_use]
    pub fn new(protocol: ProtocolId) -> Self {
        Self {
            protocol,
            revision: None,
            roles: Vec::new(),
            messages: Vec::new(),
            operations: Vec::new(),
            required_capabilities: Vec::new(),
            provided_capabilities: Vec::new(),
        }
    }

    /// Sets the offered protocol revision.
    #[must_use]
    pub fn with_revision(mut self, revision: ProtocolRevision) -> Self {
        self.revision = Some(revision);
        self
    }

    /// Adds a supported role.
    pub fn add_role(&mut self, id: RoleId) -> ProtocolResult<()> {
        insert_unique(&mut self.roles, id, "role")
    }

    /// Adds a supported message.
    pub fn add_message(&mut self, id: MessageId) -> ProtocolResult<()> {
        insert_unique(&mut self.messages, id, "message")
    }

    /// Adds a supported operation.
    pub fn add_operation(&mut self, id: OperationId) -> ProtocolResult<()> {
        insert_unique(&mut self.operations, id, "operation")
    }

    /// Adds a required capability.
    pub fn require_capability(
        &mut self,
        id: CapabilityId,
    ) -> ProtocolResult<()> {
        insert_unique(
            &mut self.required_capabilities,
            id,
            "required capability",
        )
    }

    /// Adds a provided capability.
    pub fn provide_capability(
        &mut self,
        id: CapabilityId,
    ) -> ProtocolResult<()> {
        insert_unique(
            &mut self.provided_capabilities,
            id,
            "provided capability",
        )
    }

    /// Returns the protocol identity.
    #[must_use]
    pub fn protocol(&self) -> &ProtocolId {
        &self.protocol
    }

    /// Returns the optional revision.
    #[must_use]
    pub fn revision(&self) -> Option<&ProtocolRevision> {
        self.revision.as_ref()
    }

    /// Returns supported roles.
    #[must_use]
    pub fn roles(&self) -> &[RoleId] {
        &self.roles
    }

    /// Returns supported messages.
    #[must_use]
    pub fn messages(&self) -> &[MessageId] {
        &self.messages
    }

    /// Returns supported operations.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns required capabilities.
    #[must_use]
    pub fn required_capabilities(&self) -> &[CapabilityId] {
        &self.required_capabilities
    }

    /// Returns provided capabilities.
    #[must_use]
    pub fn provided_capabilities(&self) -> &[CapabilityId] {
        &self.provided_capabilities
    }
}

fn insert_unique<T>(
    values: &mut Vec<T>,
    value: T,
    description: &str,
) -> ProtocolResult<()>
where
    T: PartialEq + fmt::Display,
{
    if values.contains(&value) {
        return Err(
            ProtocolError::new(
                ProtocolErrorKind::DuplicateDeclaration,
                format!("duplicate {description} in protocol offer"),
            )
            .with_related(value.to_string()),
        );
    }

    values.push(value);
    Ok(())
}

/// Policy for deciding whether two opaque protocol revisions are compatible.
///
/// The policy is deliberately injected. The standard library does not
/// assume that every protocol uses the same versioning rules.
pub trait RevisionCompatibility: Send + Sync {
    /// Returns whether two revisions may interoperate.
    ///
    /// `None` represents an unspecified revision. The policy decides
    /// whether unspecified revisions are compatible.
    fn is_compatible(
        &self,
        local: Option<&ProtocolRevision>,
        remote: Option<&ProtocolRevision>,
    ) -> bool;
}

/// Exact revision compatibility policy.
///
/// Both revisions must be equal. Two unspecified revisions are equal.
/// This is a conservative default, not a universal versioning rule.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExactRevisionCompatibility;

impl RevisionCompatibility for ExactRevisionCompatibility {
    fn is_compatible(
        &self,
        local: Option<&ProtocolRevision>,
        remote: Option<&ProtocolRevision>,
    ) -> bool {
        local == remote
    }
}

/// Successful protocol negotiation result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegotiatedProtocol {
    protocol: ProtocolId,
    local_revision: Option<ProtocolRevision>,
    remote_revision: Option<ProtocolRevision>,
    roles: Vec<RoleId>,
    messages: Vec<MessageId>,
    operations: Vec<OperationId>,
    common_capabilities: Vec<CapabilityId>,
}

impl NegotiatedProtocol {
    /// Returns the negotiated protocol identity.
    #[must_use]
    pub fn protocol(&self) -> &ProtocolId {
        &self.protocol
    }

    /// Returns the local revision.
    #[must_use]
    pub fn local_revision(&self) -> Option<&ProtocolRevision> {
        self.local_revision.as_ref()
    }

    /// Returns the remote revision.
    #[must_use]
    pub fn remote_revision(&self) -> Option<&ProtocolRevision> {
        self.remote_revision.as_ref()
    }

    /// Returns the common supported roles.
    #[must_use]
    pub fn roles(&self) -> &[RoleId] {
        &self.roles
    }

    /// Returns the common supported messages.
    #[must_use]
    pub fn messages(&self) -> &[MessageId] {
        &self.messages
    }

    /// Returns the common supported operations.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns capabilities provided by both participants.
    #[must_use]
    pub fn common_capabilities(&self) -> &[CapabilityId] {
        &self.common_capabilities
    }
}

/// Negotiates two protocol offers.
///
/// The operation:
///
/// 1. Requires matching protocol identities.
/// 2. Applies the supplied revision-compatibility policy.
/// 3. Verifies that each participant's required capabilities are provided
///    by the other participant.
/// 4. Computes common roles, messages, operations and provided capabilities.
///
/// The function does not open a connection, select a transport, authenticate
/// a peer, enforce policies or verify semantic expressions.
pub fn negotiate(
    local: &ProtocolOffer,
    remote: &ProtocolOffer,
    revision_policy: &dyn RevisionCompatibility,
) -> ProtocolResult<NegotiatedProtocol> {
    if local.protocol != remote.protocol {
        return Err(
            ProtocolError::new(
                ProtocolErrorKind::ProtocolMismatch,
                "protocol identities do not match",
            )
            .with_related(local.protocol.to_string())
            .with_related(remote.protocol.to_string()),
        );
    }

    if !revision_policy.is_compatible(
        local.revision.as_ref(),
        remote.revision.as_ref(),
    ) {
        return Err(
            ProtocolError::new(
                ProtocolErrorKind::IncompatibleRevision,
                "protocol revisions are not compatible",
            )
            .with_related(
                local
                    .revision
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<unspecified>".to_owned()),
            )
            .with_related(
                remote
                    .revision
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<unspecified>".to_owned()),
            ),
        );
    }

    let remote_provides: BTreeSet<&str> = remote
        .provided_capabilities
        .iter()
        .map(CapabilityId::as_str)
        .collect();

    let local_provides: BTreeSet<&str> = local
        .provided_capabilities
        .iter()
        .map(CapabilityId::as_str)
        .collect();

    let missing_local: Vec<String> = local
        .required_capabilities
        .iter()
        .filter(|id| !remote_provides.contains(id.as_str()))
        .map(ToString::to_string)
        .collect();

    let missing_remote: Vec<String> = remote
        .required_capabilities
        .iter()
        .filter(|id| !local_provides.contains(id.as_str()))
        .map(ToString::to_string)
        .collect();

    if !missing_local.is_empty() || !missing_remote.is_empty() {
        let mut error = ProtocolError::new(
            ProtocolErrorKind::MissingCapability,
            "one or more required protocol capabilities are not provided",
        );

        for id in missing_local {
            error = error.with_related(format!("local requires: {id}"));
        }

        for id in missing_remote {
            error = error.with_related(format!("remote requires: {id}"));
        }

        return Err(error);
    }

    let remote_roles: BTreeSet<&str> = remote
        .roles
        .iter()
        .map(RoleId::as_str)
        .collect();

    let remote_messages: BTreeSet<&str> = remote
        .messages
        .iter()
        .map(MessageId::as_str)
        .collect();

    let remote_operations: BTreeSet<&str> = remote
        .operations
        .iter()
        .map(OperationId::as_str)
        .collect();

    let remote_capabilities: BTreeSet<&str> = remote
        .provided_capabilities
        .iter()
        .map(CapabilityId::as_str)
        .collect();

    let roles = local
        .roles
        .iter()
        .filter(|id| remote_roles.contains(id.as_str()))
        .cloned()
        .collect();

    let messages = local
        .messages
        .iter()
        .filter(|id| remote_messages.contains(id.as_str()))
        .cloned()
        .collect();

    let operations = local
        .operations
        .iter()
        .filter(|id| remote_operations.contains(id.as_str()))
        .cloned()
        .collect();

    let common_capabilities = local
        .provided_capabilities
        .iter()
        .filter(|id| remote_capabilities.contains(id.as_str()))
        .cloned()
        .collect();

    Ok(NegotiatedProtocol {
        protocol: local.protocol.clone(),
        local_revision: local.revision.clone(),
        remote_revision: remote.revision.clone(),
        roles,
        messages,
        operations,
        common_capabilities,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn protocol_id(value: &str) -> ProtocolId {
        ProtocolId::new(value).expect("valid protocol identifier")
    }

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability identifier")
    }

    fn role(value: &str) -> RoleId {
        RoleId::new(value).expect("valid role identifier")
    }

    fn message(value: &str) -> MessageId {
        MessageId::new(value).expect("valid message identifier")
    }

    fn operation(value: &str) -> OperationId {
        OperationId::new(value).expect("valid operation identifier")
    }

    #[test]
    fn accepts_open_world_protocol_identifiers() {
        assert_eq!(
            protocol_id("future::fabric::protocol").as_str(),
            "future::fabric::protocol"
        );
    }

    #[test]
    fn rejects_invalid_protocol_identifiers() {
        assert_eq!(
            ProtocolId::new("network::::tcp")
                .unwrap_err()
                .kind(),
            ProtocolErrorKind::InvalidProtocolId
        );

        assert!(ProtocolId::new("").is_err());
        assert!(ProtocolId::new(" network::tcp").is_err());
    }

    #[test]
    fn supports_opaque_protocol_revisions() {
        let revision = ProtocolRevision::new("vendor-revision/2035")
            .expect("valid opaque revision");

        assert_eq!(revision.as_str(), "vendor-revision/2035");
    }

    #[test]
    fn creates_protocol_contracts() {
        let definition = ProtocolDefinition::new(protocol_id("network::request"))
            .with_role(ProtocolRole::new(role("client")))
            .with_role(ProtocolRole::new(role("server")))
            .with_message(ProtocolMessage::new(message("messages::request")))
            .with_operation(ProtocolOperation::new(operation("send")));

        assert_eq!(definition.roles().len(), 2);
        assert_eq!(definition.messages().len(), 1);
        assert_eq!(definition.operations().len(), 1);
        assert!(definition.validate_structure().is_ok());
    }

    #[test]
    fn detects_duplicate_roles() {
        let definition = ProtocolDefinition::new(protocol_id("network::duplicate"))
            .with_role(ProtocolRole::new(role("client")))
            .with_role(ProtocolRole::new(role("client")));

        assert_eq!(
            definition.validate_structure().unwrap_err().kind(),
            ProtocolErrorKind::DuplicateDeclaration
        );
    }

    #[test]
    fn validates_transition_state_references() {
        let definition = ProtocolDefinition::new(protocol_id("network::session"))
            .with_state(ProtocolState::new(StateId::new("idle").unwrap()))
            .with_transition(ProtocolTransition::new(
                StateId::new("idle").unwrap(),
                StateId::new("connected").unwrap(),
            ));

        assert_eq!(
            definition.validate_structure().unwrap_err().kind(),
            ProtocolErrorKind::UnknownState
        );
    }

    #[test]
    fn accepts_valid_state_transitions() {
        let definition = ProtocolDefinition::new(protocol_id("network::session"))
            .with_state(ProtocolState::new(StateId::new("idle").unwrap()))
            .with_state(ProtocolState::new(StateId::new("connected").unwrap()))
            .with_transition(ProtocolTransition::new(
                StateId::new("idle").unwrap(),
                StateId::new("connected").unwrap(),
            ));

        assert!(definition.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_choice_branches() {
        let flow = ProtocolFlow::new(Some("exchange".to_owned()))
            .unwrap()
            .with_step(ProtocolFlowStep::Choice(vec![Vec::new()]));

        let definition = ProtocolDefinition::new(protocol_id("network::flow"))
            .with_flow(flow);

        assert_eq!(
            definition.validate_structure().unwrap_err().kind(),
            ProtocolErrorKind::InvalidFlow
        );
    }

    #[test]
    fn accepts_nested_flow_composition() {
        let flow = ProtocolFlow::new(Some("exchange".to_owned()))
            .unwrap()
            .with_step(ProtocolFlowStep::Sequence(vec![
                ProtocolFlowStep::Send(
                    ProtocolExpression::new("request").unwrap(),
                ),
                ProtocolFlowStep::Receive(
                    ProtocolExpression::new("response").unwrap(),
                ),
            ]))
            .with_step(ProtocolFlowStep::Optional(vec![
                ProtocolFlowStep::Emit(
                    ProtocolExpression::new("telemetry").unwrap(),
                ),
            ]));

        let definition = ProtocolDefinition::new(protocol_id("network::flow"))
            .with_flow(flow);

        assert!(definition.validate_structure().is_ok());
    }

    #[test]
    fn negotiates_matching_protocol_offers() {
        let mut local = ProtocolOffer::new(protocol_id("network::request"));
        local.add_role(role("client")).unwrap();
        local.add_message(message("messages::request")).unwrap();
        local.add_operation(operation("send")).unwrap();
        local.provide_capability(capability("network::reliable")).unwrap();

        let mut remote = ProtocolOffer::new(protocol_id("network::request"));
        remote.add_role(role("client")).unwrap();
        remote.add_message(message("messages::request")).unwrap();
        remote.add_operation(operation("send")).unwrap();
        remote.provide_capability(capability("network::reliable")).unwrap();

        let result = negotiate(
            &local,
            &remote,
            &ExactRevisionCompatibility,
        )
        .expect("offers are compatible");

        assert_eq!(result.roles().len(), 1);
        assert_eq!(result.messages().len(), 1);
        assert_eq!(result.operations().len(), 1);
        assert_eq!(result.common_capabilities().len(), 1);
    }

    #[test]
    fn rejects_protocol_identity_mismatch() {
        let local = ProtocolOffer::new(protocol_id("network::one"));
        let remote = ProtocolOffer::new(protocol_id("network::two"));

        assert_eq!(
            negotiate(
                &local,
                &remote,
                &ExactRevisionCompatibility,
            )
            .unwrap_err()
            .kind(),
            ProtocolErrorKind::ProtocolMismatch
        );
    }

    #[test]
    fn rejects_incompatible_revisions() {
        let local = ProtocolOffer::new(protocol_id("network::request"))
            .with_revision(ProtocolRevision::new("revision-a").unwrap());

        let remote = ProtocolOffer::new(protocol_id("network::request"))
            .with_revision(ProtocolRevision::new("revision-b").unwrap());

        assert_eq!(
            negotiate(
                &local,
                &remote,
                &ExactRevisionCompatibility,
            )
            .unwrap_err()
            .kind(),
            ProtocolErrorKind::IncompatibleRevision
        );
    }

    #[test]
    fn rejects_missing_required_capabilities() {
        let mut local = ProtocolOffer::new(protocol_id("network::secure"));
        local.require_capability(capability("network::integrity")).unwrap();

        let remote = ProtocolOffer::new(protocol_id("network::secure"));

        let error = negotiate(
            &local,
            &remote,
            &ExactRevisionCompatibility,
        )
        .unwrap_err();

        assert_eq!(
            error.kind(),
            ProtocolErrorKind::MissingCapability
        );

        assert_eq!(error.related().len(), 1);
    }

    #[test]
    fn checks_capabilities_in_both_directions() {
        let mut local = ProtocolOffer::new(protocol_id("network::mutual"));
        local.require_capability(capability("network::remote")).unwrap();
        local.provide_capability(capability("network::local")).unwrap();

        let mut remote = ProtocolOffer::new(protocol_id("network::mutual"));
        remote.require_capability(capability("network::local")).unwrap();
        remote.provide_capability(capability("network::remote")).unwrap();

        assert!(
            negotiate(
                &local,
                &remote,
                &ExactRevisionCompatibility,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_duplicate_offer_entries() {
        let mut offer = ProtocolOffer::new(protocol_id("network::duplicate"));
        offer.add_role(role("client")).unwrap();

        assert_eq!(
            offer.add_role(role("client")).unwrap_err().kind(),
            ProtocolErrorKind::DuplicateDeclaration
        );
    }

    #[test]
    fn supports_custom_revision_compatibility() {
        struct CompatibleRevisions;

        impl RevisionCompatibility for CompatibleRevisions {
            fn is_compatible(
                &self,
                local: Option<&ProtocolRevision>,
                remote: Option<&ProtocolRevision>,
            ) -> bool {
                match (local, remote) {
                    (Some(_), Some(_)) => true,
                    (None, None) => true,
                    _ => false,
                }
            }
        }

        let local = ProtocolOffer::new(protocol_id("network::custom"))
            .with_revision(ProtocolRevision::new("v1").unwrap());

        let remote = ProtocolOffer::new(protocol_id("network::custom"))
            .with_revision(ProtocolRevision::new("v2").unwrap());

        assert!(
            negotiate(&local, &remote, &CompatibleRevisions).is_ok()
        );
    }

    #[test]
    fn preserves_structured_error_details() {
        let error = ProtocolError::new(
            ProtocolErrorKind::MissingCapability,
            "capability is unavailable",
        )
        .with_related("network::integrity");

        assert_eq!(
            error.kind(),
            ProtocolErrorKind::MissingCapability
        );
        assert_eq!(
            error.related(),
            &["network::integrity".to_owned()]
        );
    }

    #[test]
    fn validates_expression_representation() {
        assert!(ProtocolExpression::new("delivery_complete").is_ok());
        assert!(ProtocolExpression::new("  ").is_err());
        assert!(ProtocolExpression::new("\0").is_err());
    }
}
