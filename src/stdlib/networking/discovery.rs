
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Networking Service Discovery
//!
//! Path: `src/stdlib/networking/discovery.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, extensible contracts for logical service discovery.
//!
//! A discovery declaration describes what logical service or communication
//! resource a program needs. A discovery request carries that intent to an
//! implementation. A discovery provider reports actual results.
//!
//! This module does not itself perform network operations, query registries,
//! resolve DNS, allocate resources, select hardware, or execute programs.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Discovery declaration identities.
//! - Discovery expressions and properties.
//! - Discovery attributes.
//! - Ordered nested discovery-member representations.
//! - Typed logical discovery references.
//! - Discovery requests and continuation cursors.
//! - Discovery records and result pages.
//! - Discovery outcomes.
//! - Discovery provider metadata.
//! - Discovery provider contracts.
//! - Discovery-specific validation and errors.
//!
//! This module does not own:
//!
//! - Source parsing or AST construction.
//! - Semantic name resolution.
//! - Expression evaluation.
//! - Canonical requirements, capabilities, constraints or policies.
//! - Endpoint declarations.
//! - Address declarations or address resolution.
//! - Protocol declarations or protocol negotiation.
//! - Transport implementation.
//! - DNS implementation.
//! - Service-registry implementation.
//! - Authentication or authorization implementation.
//! - Cryptographic implementation.
//! - Network topology or routing.
//! - Distributed placement or scheduling.
//! - Runtime execution.
//! - Physical resource allocation.
//! - Hardware discovery or control.
//!
//! ## Grammar authority
//!
//! Canonical discovery grammar:
//! `grammar/networking/service-discovery.g4`
//!
//! Networking composition:
//! `grammar/networking/networking.g4`
//!
//! Endpoint grammar:
//! `grammar/networking/endpoints.g4`
//!
//! Address grammar:
//! `grammar/networking/addresses.g4`
//!
//! Protocol grammar:
//! `grammar/networking/protocols.g4`
//!
//! Normative specification:
//! `grammar/spec/networking.md`
//!
//! The frontend translates validated grammar and AST structures into these
//! public contracts. This module does not parse Zamani source.
//!
//! ## POCO-REAF and scalability
//!
//! Discovery identities and references are open-world.
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Discovery declarations.
//! - Discovery properties.
//! - Nested discovery blocks.
//! - Discovery references.
//! - Attributes.
//! - Candidate records.
//! - Providers.
//! - Capabilities.
//! - Continuation pages.
//! - Service identities.
//! - Metadata.
//! - Expression text.
//!
//! Nested members use a flat ordered representation with parent indices.
//! This avoids recursive data structures and recursive traversal.
//!
//! Actual limits are determined by available memory, processing resources,
//! provider capabilities, configured operational budgets and physical reality.
//! Literal infinite storage or computation is not promised.
//!
//! ## Integration
//!
//! Address contracts:
//! `crate::stdlib::networking::addresses`
//!
//! Transport contracts:
//! `crate::stdlib::networking::transports`
//!
//! Protocol contracts:
//! `crate::stdlib::networking::protocols`
//!
//! Networking grammar:
//! `grammar/networking/`
//!
//! Semantic analysis:
//! `crate::frontend::semantic`
//!
//! Distributed execution:
//! `crate::distributed`
//!
//! Runtime execution:
//! `crate::runtime`
//!
//! Actual provider implementations belong to the appropriate runtime,
//! networking, distributed or external-service integration.
//!
//! ## Discovery semantics
//!
//! A discovery declaration is not an instruction to immediately contact a
//! physical service.
//!
//! A discovery request is not proof that a service exists.
//!
//! A discovery record is evidence returned by a provider, not proof that a
//! connection can be established or that a resource has been allocated.
//!
//! A provider must distinguish:
//!
//! - A successful page containing records.
//! - An authoritative absence of matching records.
//! - A deferred result.
//! - An operational failure.
//!
//! An empty successful page is not automatically equivalent to an
//! authoritative `NotFound` result.
//!
//! Continuation cursors are opaque provider-owned values. This module does
//! not interpret them or prescribe a pagination algorithm.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No network I/O, filesystem access, global mutable state, hidden provider
//! execution, unsafe code or operating-system calls are performed here.

use core::fmt;
use std::error::Error as StdError;
use std::num::NonZeroUsize;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates an open-world qualified identifier.
///
/// Components are separated by `::`.
///
/// Each component begins with an ASCII letter, digit or underscore and
/// continues with ASCII letters, digits, underscores, hyphens or dots.
///
/// This is a naming convention, not a catalogue of supported services,
/// providers, protocols or technologies.
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

/// Validates an opaque expression representation.
///
/// Parsing, type checking, name resolution and evaluation remain the
/// responsibility of the canonical frontend and semantic subsystems.
fn validate_expression(value: &str) -> DiscoveryResult<()> {
    if value.trim().is_empty() {
        return Err(DiscoveryError::new(
            DiscoveryErrorKind::InvalidExpression,
            "discovery expression cannot be empty",
        ));
    }

    if value.contains('\0') {
        return Err(DiscoveryError::new(
            DiscoveryErrorKind::InvalidExpression,
            "discovery expression contains a NUL character",
        ));
    }

    Ok(())
}

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Classification of service-discovery failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DiscoveryErrorKind {
    /// A discovery identifier is invalid.
    InvalidIdentifier,

    /// A qualified name is invalid.
    InvalidName,

    /// An expression representation is invalid.
    InvalidExpression,

    /// A discovery property is invalid.
    InvalidProperty,

    /// A discovery attribute is invalid.
    InvalidAttribute,

    /// A nested member has an invalid parent relationship.
    InvalidMemberHierarchy,

    /// A discovery reference is invalid.
    InvalidReference,

    /// A discovery declaration is structurally invalid.
    InvalidDeclaration,

    /// A discovery request is invalid.
    InvalidRequest,

    /// A continuation cursor is invalid.
    InvalidCursor,

    /// A provider identity or version is invalid.
    InvalidProvider,

    /// The requested discovery operation is unsupported.
    UnsupportedOperation,

    /// No discovery provider is available.
    ProviderUnavailable,

    /// A provider failed while performing discovery.
    ProviderFailure,

    /// A provider returned an invalid result.
    InvalidProviderResult,

    /// The discovery operation was cancelled.
    Cancelled,

    /// The discovery operation timed out.
    TimedOut,

    /// The caller is not authorized to perform the operation.
    Unauthorized,

    /// A discovery policy rejected the operation.
    PolicyRejected,

    /// The discovery operation exceeded an operational budget.
    BudgetExceeded,

    /// An internal discovery operation failed.
    Internal,
}

impl DiscoveryErrorKind {
    /// Returns a stable textual classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "invalid_identifier",
            Self::InvalidName => "invalid_name",
            Self::InvalidExpression => "invalid_expression",
            Self::InvalidProperty => "invalid_property",
            Self::InvalidAttribute => "invalid_attribute",
            Self::InvalidMemberHierarchy => "invalid_member_hierarchy",
            Self::InvalidReference => "invalid_reference",
            Self::InvalidDeclaration => "invalid_declaration",
            Self::InvalidRequest => "invalid_request",
            Self::InvalidCursor => "invalid_cursor",
            Self::InvalidProvider => "invalid_provider",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderFailure => "provider_failure",
            Self::InvalidProviderResult => "invalid_provider_result",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Unauthorized => "unauthorized",
            Self::PolicyRejected => "policy_rejected",
            Self::BudgetExceeded => "budget_exceeded",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for DiscoveryErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured service-discovery error.
///
/// The error preserves a stable classification, a human-readable message,
/// related diagnostic details and an optional causal error.
#[derive(Debug)]
pub struct DiscoveryError {
    kind: DiscoveryErrorKind,
    message: String,
    related: Vec<String>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl DiscoveryError {
    /// Creates a discovery error.
    #[must_use]
    pub fn new(
        kind: DiscoveryErrorKind,
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
    pub const fn kind(&self) -> DiscoveryErrorKind {
        self.kind
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns related diagnostic details.
    #[must_use]
    pub fn related(&self) -> &[String] {
        &self.related
    }

    /// Adds a related diagnostic detail.
    #[must_use]
    pub fn with_related(mut self, value: impl Into<String>) -> Self {
        self.related.push(value.into());
        self
    }

    /// Attaches an underlying causal error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns whether a causal error is attached.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "network service-discovery error [{}]: {}",
            self.kind,
            self.message
        )
    }
}

impl StdError for DiscoveryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for discovery operations.
pub type DiscoveryResult<T> = Result<T, DiscoveryError>;

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

/// A validated qualified discovery identity.
///
/// This type is used for logical discovery declaration names, provider
/// identities, service identities and other open-world discovery names.
///
/// It does not assert that the named entity exists.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiscoveryId(String);

impl DiscoveryId {
    /// Creates a validated discovery identity.
    pub fn new(value: impl Into<String>) -> DiscoveryResult<Self> {
        let value = value.into();

        if !is_valid_qualified_identifier(&value) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidIdentifier,
                "invalid qualified discovery identity",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the complete qualified identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks equality with a textual identity.
    #[must_use]
    pub fn matches(&self, value: &str) -> bool {
        self.0 == value
    }
}

impl fmt::Display for DiscoveryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::str::FromStr for DiscoveryId {
    type Err = DiscoveryError;

    fn from_str(value: &str) -> DiscoveryResult<Self> {
        Self::new(value)
    }
}

// ============================================================================
// EXPRESSIONS AND PROPERTIES
// ============================================================================

/// Opaque representation of a validated discovery expression.
///
/// The frontend owns expression parsing and semantic interpretation.
/// This type preserves expression text for later semantic or provider
/// integration without creating a second expression language.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiscoveryExpression(String);

impl DiscoveryExpression {
    /// Creates an expression representation.
    ///
    /// This performs only basic representation validation. It does not
    /// establish that the expression is syntactically or semantically valid.
    pub fn new(value: impl Into<String>) -> DiscoveryResult<Self> {
        let value = value.into();
        validate_expression(&value)?;
        Ok(Self(value))
    }

    /// Returns the original expression representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiscoveryExpression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A named discovery property.
///
/// Property names are open-world qualified names. Their meaning is
/// established by the semantic subsystem or a registered dialect.
///
/// Repeated property names are preserved because the grammar preserves
/// declaration order and semantic analysis owns duplicate-property rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryProperty {
    name: DiscoveryId,
    value: DiscoveryExpression,
}

impl DiscoveryProperty {
    /// Creates a discovery property.
    pub fn new(
        name: DiscoveryId,
        value: DiscoveryExpression,
    ) -> Self {
        Self { name, value }
    }

    /// Returns the property name.
    #[must_use]
    pub fn name(&self) -> &DiscoveryId {
        &self.name
    }

    /// Returns the property expression.
    #[must_use]
    pub fn value(&self) -> &DiscoveryExpression {
        &self.value
    }
}

/// A source-level attribute attached to a discovery declaration or member.
///
/// Attribute semantics remain owned by the canonical frontend and semantic
/// authorities. The optional expression is preserved without evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryAttribute {
    name: DiscoveryId,
    value: Option<DiscoveryExpression>,
}

impl DiscoveryAttribute {
    /// Creates a discovery attribute.
    #[must_use]
    pub fn new(
        name: DiscoveryId,
        value: Option<DiscoveryExpression>,
    ) -> Self {
        Self { name, value }
    }

    /// Returns the attribute name.
    #[must_use]
    pub fn name(&self) -> &DiscoveryId {
        &self.name
    }

    /// Returns the optional attribute expression.
    #[must_use]
    pub fn value(&self) -> Option<&DiscoveryExpression> {
        self.value.as_ref()
    }
}

// ============================================================================
// FLAT NESTED MEMBER MODEL
// ============================================================================

/// The kind of a discovery member.
///
/// A property stores a qualified property name and an opaque expression.
/// A block stores an open-world qualified block name.
///
/// Nested blocks are represented by parent indices in
/// [`DiscoveryMemberNode`], not by recursive Rust values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiscoveryMemberKind {
    /// A property declaration.
    Property(DiscoveryProperty),

    /// A nested configuration block.
    Block {
        /// The qualified block name.
        name: DiscoveryId,
    },
}

/// An ordered node in a discovery declaration.
///
/// `parent` is `None` for a top-level member. Otherwise it is the zero-based
/// index of an earlier member whose kind must be `Block`.
///
/// Parent indices must refer to earlier entries. This gives the collection
/// an acyclic, preorder representation and permits iterative validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryMemberNode {
    parent: Option<usize>,
    attributes: Vec<DiscoveryAttribute>,
    kind: DiscoveryMemberKind,
}

impl DiscoveryMemberNode {
    /// Creates a top-level discovery property.
    #[must_use]
    pub fn property(property: DiscoveryProperty) -> Self {
        Self {
            parent: None,
            attributes: Vec::new(),
            kind: DiscoveryMemberKind::Property(property),
        }
    }

    /// Creates a nested discovery property.
    #[must_use]
    pub fn nested_property(
        parent: usize,
        property: DiscoveryProperty,
    ) -> Self {
        Self {
            parent: Some(parent),
            attributes: Vec::new(),
            kind: DiscoveryMemberKind::Property(property),
        }
    }

    /// Creates a top-level nested-block declaration.
    #[must_use]
    pub fn block(name: DiscoveryId) -> Self {
        Self {
            parent: None,
            attributes: Vec::new(),
            kind: DiscoveryMemberKind::Block { name },
        }
    }

    /// Creates a nested block under an earlier block node.
    #[must_use]
    pub fn nested_block(
        parent: usize,
        name: DiscoveryId,
    ) -> Self {
        Self {
            parent: Some(parent),
            attributes: Vec::new(),
            kind: DiscoveryMemberKind::Block { name },
        }
    }

    /// Adds an attribute while preserving source order.
    #[must_use]
    pub fn with_attribute(mut self, attribute: DiscoveryAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Returns the parent node index.
    #[must_use]
    pub const fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// Returns member attributes in source order.
    #[must_use]
    pub fn attributes(&self) -> &[DiscoveryAttribute] {
        &self.attributes
    }

    /// Returns the member kind.
    #[must_use]
    pub fn kind(&self) -> &DiscoveryMemberKind {
        &self.kind
    }

    /// Returns whether this node represents a nested block.
    #[must_use]
    pub fn is_block(&self) -> bool {
        matches!(self.kind, DiscoveryMemberKind::Block { .. })
    }
}

// ============================================================================
// TYPED LOGICAL REFERENCES
// ============================================================================

/// The grammar-defined category of a typed discovery reference.
///
/// These categories identify the kinds of logical declarations that the
/// discovery grammar can reference. They are not a closed list of services,
/// protocols, providers or technologies.
///
/// New service and technology identities remain open-world through
/// [`DiscoveryId`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiscoveryReferenceKind {
    /// A logical service reference.
    Service,

    /// A logical endpoint reference.
    Endpoint,

    /// An address declaration reference.
    Address,

    /// A protocol declaration reference.
    Protocol,

    /// A communication channel reference.
    Channel,

    /// A route declaration reference.
    Route,

    /// A reusable request reference.
    Request,

    /// A reusable response reference.
    Response,

    /// A socket declaration reference.
    Socket,

    /// A capability reference.
    Capability,
}

impl DiscoveryReferenceKind {
    /// Returns the stable textual category.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Service => "service",
            Self::Endpoint => "endpoint",
            Self::Address => "address",
            Self::Protocol => "protocol",
            Self::Channel => "channel",
            Self::Route => "route",
            Self::Request => "request",
            Self::Response => "response",
            Self::Socket => "socket",
            Self::Capability => "capability",
        }
    }
}

impl fmt::Display for DiscoveryReferenceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A typed logical reference appearing in a discovery declaration.
///
/// Construction validates only the qualified name. It does not resolve
/// the name or establish that the referenced declaration exists.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DiscoveryReference {
    kind: DiscoveryReferenceKind,
    target: DiscoveryId,
}

impl DiscoveryReference {
    /// Creates a typed logical reference.
    #[must_use]
    pub fn new(
        kind: DiscoveryReferenceKind,
        target: DiscoveryId,
    ) -> Self {
        Self { kind, target }
    }

    /// Returns the reference category.
    #[must_use]
    pub const fn kind(&self) -> DiscoveryReferenceKind {
        self.kind
    }

    /// Returns the referenced qualified identity.
    #[must_use]
    pub fn target(&self) -> &DiscoveryId {
        &self.target
    }
}

// ============================================================================
// DISCOVERY DECLARATION
// ============================================================================

/// A portable logical service-discovery declaration.
///
/// The declaration preserves the source-level intent and ordered member
/// structure. It does not perform lookup or evaluate expressions.
///
/// The frontend should preserve source spans in its canonical AST and retain
/// a mapping to this representation when diagnostics require source locations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryDeclaration {
    name: DiscoveryId,
    attributes: Vec<DiscoveryAttribute>,
    members: Vec<DiscoveryMemberNode>,
    references: Vec<DiscoveryReference>,
    selector: Option<DiscoveryExpression>,
    query: Option<DiscoveryExpression>,
}

impl DiscoveryDeclaration {
    /// Creates an empty discovery declaration.
    #[must_use]
    pub fn new(name: DiscoveryId) -> Self {
        Self {
            name,
            attributes: Vec::new(),
            members: Vec::new(),
            references: Vec::new(),
            selector: None,
            query: None,
        }
    }

    /// Adds a declaration attribute.
    #[must_use]
    pub fn with_attribute(mut self, attribute: DiscoveryAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Adds an ordered discovery member.
    ///
    /// The member's parent relationship is checked by
    /// [`validate_declaration`].
    #[must_use]
    pub fn with_member(mut self, member: DiscoveryMemberNode) -> Self {
        self.members.push(member);
        self
    }

    /// Adds a typed logical reference.
    #[must_use]
    pub fn with_reference(mut self, reference: DiscoveryReference) -> Self {
        self.references.push(reference);
        self
    }

    /// Sets the optional selector expression.
    #[must_use]
    pub fn with_selector(mut self, selector: DiscoveryExpression) -> Self {
        self.selector = Some(selector);
        self
    }

    /// Sets the optional query expression.
    #[must_use]
    pub fn with_query(mut self, query: DiscoveryExpression) -> Self {
        self.query = Some(query);
        self
    }

    /// Returns the declaration identity.
    #[must_use]
    pub fn name(&self) -> &DiscoveryId {
        &self.name
    }

    /// Returns declaration attributes.
    #[must_use]
    pub fn attributes(&self) -> &[DiscoveryAttribute] {
        &self.attributes
    }

    /// Returns ordered declaration members.
    #[must_use]
    pub fn members(&self) -> &[DiscoveryMemberNode] {
        &self.members
    }

    /// Returns typed logical references.
    #[must_use]
    pub fn references(&self) -> &[DiscoveryReference] {
        &self.references
    }

    /// Returns the optional selector expression.
    #[must_use]
    pub fn selector(&self) -> Option<&DiscoveryExpression> {
        self.selector.as_ref()
    }

    /// Returns the optional query expression.
    #[must_use]
    pub fn query(&self) -> Option<&DiscoveryExpression> {
        self.query.as_ref()
    }

    /// Returns the member nodes directly contained by a parent block.
    ///
    /// Passing `None` returns top-level members. Passing `Some(index)`
    /// returns members nested under the corresponding block.
    ///
    /// The returned iterator preserves declaration order.
    pub fn children_of(
        &self,
        parent: Option<usize>,
    ) -> impl Iterator<Item = (usize, &DiscoveryMemberNode)> {
        self.members
            .iter()
            .enumerate()
            .filter(move |(_, member)| member.parent == parent)
    }
}

// ============================================================================
// DECLARATION VALIDATION
// ============================================================================

/// Validates the local structural integrity of a discovery declaration.
///
/// This function checks:
///
/// - The declaration identity.
/// - Attribute identities and expression representations.
/// - Property and block identities.
/// - Parent indices.
/// - Parent-before-child ordering.
/// - That nested members belong to block nodes.
/// - Selector and query representations.
/// - Typed reference identities.
///
/// It does not perform semantic name resolution, policy evaluation,
/// capability checking, expression evaluation or network access.
pub fn validate_declaration(
    declaration: &DiscoveryDeclaration,
) -> DiscoveryResult<()> {
    if !is_valid_qualified_identifier(declaration.name.as_str()) {
        return Err(DiscoveryError::new(
            DiscoveryErrorKind::InvalidDeclaration,
            "discovery declaration has an invalid identity",
        ));
    }

    validate_attributes(&declaration.attributes)?;

    for (index, member) in declaration.members.iter().enumerate() {
        validate_attributes(&member.attributes)?;

        match &member.kind {
            DiscoveryMemberKind::Property(property) => {
                if !is_valid_qualified_identifier(property.name.as_str()) {
                    return Err(DiscoveryError::new(
                        DiscoveryErrorKind::InvalidProperty,
                        "discovery property has an invalid name",
                    )
                    .with_related(format!("member index: {index}")));
                }

                validate_expression(property.value.as_str())?;
            }
            DiscoveryMemberKind::Block { name } => {
                if !is_valid_qualified_identifier(name.as_str()) {
                    return Err(DiscoveryError::new(
                        DiscoveryErrorKind::InvalidName,
                        "discovery block has an invalid name",
                    )
                    .with_related(format!("member index: {index}")));
                }
            }
        }

        if let Some(parent_index) = member.parent {
            if parent_index >= index {
                return Err(DiscoveryError::new(
                    DiscoveryErrorKind::InvalidMemberHierarchy,
                    "a discovery member must refer to an earlier parent node",
                )
                .with_related(format!("member index: {index}"))
                .with_related(format!("parent index: {parent_index}")));
            }

            if !declaration.members[parent_index].is_block() {
                return Err(DiscoveryError::new(
                    DiscoveryErrorKind::InvalidMemberHierarchy,
                    "a discovery member's parent must be a nested block",
                )
                .with_related(format!("member index: {index}"))
                .with_related(format!("parent index: {parent_index}")));
            }
        }
    }

    for reference in &declaration.references {
        if !is_valid_qualified_identifier(reference.target.as_str()) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidReference,
                "discovery reference has an invalid target",
            )
            .with_related(reference.kind.to_string()));
        }
    }

    if let Some(selector) = &declaration.selector {
        validate_expression(selector.as_str())?;
    }

    if let Some(query) = &declaration.query {
        validate_expression(query.as_str())?;
    }

    Ok(())
}

fn validate_attributes(attributes: &[DiscoveryAttribute]) -> DiscoveryResult<()> {
    for attribute in attributes {
        if !is_valid_qualified_identifier(attribute.name.as_str()) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidAttribute,
                "discovery attribute has an invalid name",
            ));
        }

        if let Some(value) = &attribute.value {
            validate_expression(value.as_str())?;
        }
    }

    Ok(())
}

// ============================================================================
// CONTINUATION CURSORS
// ============================================================================

/// An opaque provider-owned discovery continuation cursor.
///
/// The cursor may represent a database position, registry token, snapshot
/// reference or another provider-specific continuation mechanism.
///
/// Its internal representation must not be interpreted by consumers.
/// A cursor is meaningful only to the provider and discovery scope that
/// produced it.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DiscoveryCursor(String);

impl DiscoveryCursor {
    /// Creates a continuation cursor.
    ///
    /// Empty strings and control characters are rejected. No arbitrary
    /// maximum cursor length is imposed.
    pub fn new(value: impl Into<String>) -> DiscoveryResult<Self> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidCursor,
                "discovery cursor cannot be empty or have surrounding whitespace",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidCursor,
                "discovery cursor contains control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque cursor representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// A request to perform logical service discovery.
///
/// The request carries a validated discovery declaration and optional
/// pagination information. It does not choose a provider or transport.
///
/// The optional page size is a caller preference, not a universal maximum.
/// A provider may return fewer records according to its capabilities,
/// operational budgets or backend behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryRequest {
    declaration: DiscoveryDeclaration,
    continuation: Option<DiscoveryCursor>,
    preferred_page_size: Option<NonZeroUsize>,
}

impl DiscoveryRequest {
    /// Creates a discovery request from a declaration.
    ///
    /// The declaration is structurally validated before the request is
    /// created.
    pub fn new(declaration: DiscoveryDeclaration) -> DiscoveryResult<Self> {
        validate_declaration(&declaration)?;

        Ok(Self {
            declaration,
            continuation: None,
            preferred_page_size: None,
        })
    }

    /// Sets an opaque continuation cursor.
    #[must_use]
    pub fn with_continuation(mut self, cursor: DiscoveryCursor) -> Self {
        self.continuation = Some(cursor);
        self
    }

    /// Sets a preferred page size.
    ///
    /// This is a preference, not a guaranteed result count or a global
    /// capacity limit.
    #[must_use]
    pub fn with_preferred_page_size(mut self, size: NonZeroUsize) -> Self {
        self.preferred_page_size = Some(size);
        self
    }

    /// Returns the discovery declaration.
    #[must_use]
    pub fn declaration(&self) -> &DiscoveryDeclaration {
        &self.declaration
    }

    /// Returns the optional continuation cursor.
    #[must_use]
    pub fn continuation(&self) -> Option<&DiscoveryCursor> {
        self.continuation.as_ref()
    }

    /// Returns the optional preferred page size.
    #[must_use]
    pub const fn preferred_page_size(&self) -> Option<NonZeroUsize> {
        self.preferred_page_size
    }
}

// ============================================================================
// DISCOVERY RECORDS
// ============================================================================

/// A record describing one discovered logical service or resource.
///
/// A record is provider-reported information. It does not establish that
/// the service is reachable, authorized for use, healthy, executable or
/// physically allocated.
///
/// Addresses and endpoints remain typed logical references here. Actual
/// address resolution belongs to the address subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryRecord {
    identity: DiscoveryId,
    references: Vec<DiscoveryReference>,
    properties: Vec<DiscoveryProperty>,
}

impl DiscoveryRecord {
    /// Creates a discovery record.
    #[must_use]
    pub fn new(identity: DiscoveryId) -> Self {
        Self {
            identity,
            references: Vec::new(),
            properties: Vec::new(),
        }
    }

    /// Adds a typed logical reference.
    #[must_use]
    pub fn with_reference(mut self, reference: DiscoveryReference) -> Self {
        self.references.push(reference);
        self
    }

    /// Adds a provider-reported property.
    #[must_use]
    pub fn with_property(mut self, property: DiscoveryProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Returns the logical record identity.
    #[must_use]
    pub fn identity(&self) -> &DiscoveryId {
        &self.identity
    }

    /// Returns typed logical references.
    #[must_use]
    pub fn references(&self) -> &[DiscoveryReference] {
        &self.references
    }

    /// Returns ordered provider-reported properties.
    #[must_use]
    pub fn properties(&self) -> &[DiscoveryProperty] {
        &self.properties
    }

    /// Returns all properties with the specified qualified name.
    pub fn properties_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a DiscoveryProperty> + 'a {
        self.properties
            .iter()
            .filter(move |property| property.name().as_str() == name)
    }
}

/// Validates the local structure of a discovery record.
///
/// This does not establish that the provider's claims are true. Provider
/// identity, trust, authorization and evidence validation are separate
/// responsibilities.
pub fn validate_record(record: &DiscoveryRecord) -> DiscoveryResult<()> {
    if !is_valid_qualified_identifier(record.identity.as_str()) {
        return Err(DiscoveryError::new(
            DiscoveryErrorKind::InvalidProviderResult,
            "discovery record has an invalid identity",
        ));
    }

    for reference in &record.references {
        if !is_valid_qualified_identifier(reference.target.as_str()) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidProviderResult,
                "discovery record contains an invalid reference",
            ));
        }
    }

    for property in &record.properties {
        if !is_valid_qualified_identifier(property.name.as_str()) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidProviderResult,
                "discovery record contains an invalid property name",
            ));
        }

        validate_expression(property.value.as_str()).map_err(|error| {
            DiscoveryError::new(
                DiscoveryErrorKind::InvalidProviderResult,
                "discovery record contains an invalid property expression",
            )
            .with_related(error.to_string())
        })?;
    }

    Ok(())
}

// ============================================================================
// DISCOVERY PAGE AND OUTCOME
// ============================================================================

/// A page of discovered records.
///
/// A page may contain fewer records than requested. A provider may return
/// an empty page with a continuation cursor when its backend requires
/// additional traversal.
///
/// `next_cursor == None` means the provider reports that no continuation
/// is currently available for this result sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryPage {
    records: Vec<DiscoveryRecord>,
    next_cursor: Option<DiscoveryCursor>,
}

impl DiscoveryPage {
    /// Creates a discovery page.
    ///
    /// Records are validated before the page is accepted.
    pub fn new(
        records: Vec<DiscoveryRecord>,
        next_cursor: Option<DiscoveryCursor>,
    ) -> DiscoveryResult<Self> {
        for record in &records {
            validate_record(record)?;
        }

        Ok(Self {
            records,
            next_cursor,
        })
    }

    /// Returns records in provider-defined order.
    #[must_use]
    pub fn records(&self) -> &[DiscoveryRecord] {
        &self.records
    }

    /// Returns the next continuation cursor, if any.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&DiscoveryCursor> {
        self.next_cursor.as_ref()
    }

    /// Returns whether this page contains records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns the number of records in this page.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Consumes the page and returns its records.
    #[must_use]
    pub fn into_records(self) -> Vec<DiscoveryRecord> {
        self.records
    }
}

/// The outcome of a discovery operation.
///
/// `Found` represents a successful provider response, including a valid
/// empty page. `NotFound` represents an authoritative absence within the
/// provider's declared search scope. `Deferred` means that discovery has
/// not yet produced an authoritative result.
///
/// Operational failures are represented by [`DiscoveryError`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DiscoveryOutcome {
    /// Discovery returned a page successfully.
    Found(DiscoveryPage),

    /// The provider authoritatively found no matching service in its scope.
    NotFound,

    /// Discovery must be continued or completed by a later operation.
    Deferred,
}

impl DiscoveryOutcome {
    /// Returns whether the outcome contains a successful page.
    #[must_use]
    pub const fn is_found(&self) -> bool {
        matches!(self, Self::Found(_))
    }

    /// Returns the page when discovery succeeded.
    #[must_use]
    pub fn page(&self) -> Option<&DiscoveryPage> {
        match self {
            Self::Found(page) => Some(page),
            Self::NotFound | Self::Deferred => None,
        }
    }
}

// ============================================================================
// PROVIDER INFORMATION
// ============================================================================

/// Descriptive metadata about a discovery provider.
///
/// This structure describes a provider; it does not register, load,
/// initialize or execute that provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryProviderInfo {
    identity: DiscoveryId,
    version: String,
    capabilities: Vec<DiscoveryId>,
}

impl DiscoveryProviderInfo {
    /// Creates provider metadata.
    ///
    /// Provider versions are opaque identifiers. This module does not
    /// assume semantic versioning or impose a version compatibility policy.
    pub fn new(
        identity: DiscoveryId,
        version: impl Into<String>,
    ) -> DiscoveryResult<Self> {
        let version = version.into();

        if version.trim().is_empty() || version.chars().any(char::is_control) {
            return Err(DiscoveryError::new(
                DiscoveryErrorKind::InvalidProvider,
                "discovery provider version is invalid",
            ));
        }

        Ok(Self {
            identity,
            version,
            capabilities: Vec::new(),
        })
    }

    /// Adds a declared provider capability.
    #[must_use]
    pub fn with_capability(mut self, capability: DiscoveryId) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn identity(&self) -> &DiscoveryId {
        &self.identity
    }

    /// Returns the provider version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns declared provider capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[DiscoveryId] {
        &self.capabilities
    }

    /// Checks whether a capability is declared.
    #[must_use]
    pub fn supports_capability(&self, capability: &DiscoveryId) -> bool {
        self.capabilities
            .iter()
            .any(|declared| declared == capability)
    }
}

// ============================================================================
// DISCOVERY PROVIDER CONTRACT
// ============================================================================

/// Contract for an implementation that performs service discovery.
///
/// Implementations may integrate with logical registries, distributed
/// discovery systems, local service catalogues, cloud services, embedded
/// service maps or future discovery technologies.
///
/// This trait does not define how providers are registered or selected.
/// Registration and provider lifecycle belong to the repository's provider
/// registry and runtime architecture.
///
/// Implementations must:
///
/// - Respect the discovery request and its semantic scope.
/// - Enforce their applicable authorization and security boundaries.
/// - Report actual results rather than fabricated success.
/// - Distinguish authoritative absence from deferred discovery.
/// - Return valid provider metadata.
/// - Preserve the meaning of continuation cursors.
/// - Document consistency, freshness and ordering guarantees.
/// - Report operational failures as structured errors.
///
/// The synchronous method is an integration boundary. Async execution,
/// cancellation orchestration, deadlines, retries and scheduling belong
/// to the runtime executor and provider adapter.
pub trait ServiceDiscoveryProvider: Send + Sync {
    /// Returns provider identity, version and declared capabilities.
    fn info(&self) -> DiscoveryProviderInfo;

    /// Performs one discovery operation or continuation request.
    ///
    /// This operation may contact an external system in a concrete provider.
    /// The standard library itself does not perform that operation.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the operation cannot be completed.
    /// A provider must not convert authorization failure, timeout,
    /// cancellation, provider failure or unsupported functionality into
    /// a successful empty result.
    fn discover(
        &self,
        request: &DiscoveryRequest,
    ) -> DiscoveryResult<DiscoveryOutcome>;
}

// ============================================================================
// PROVIDER RESULT VALIDATION
// ============================================================================

/// Validates a provider-reported discovery outcome.
///
/// This checks the local structure of returned records. It cannot prove
/// record authenticity, service availability, authorization, freshness or
/// reachability. Those checks require the applicable trust, security,
/// semantic and runtime authorities.
pub fn validate_outcome(outcome: &DiscoveryOutcome) -> DiscoveryResult<()> {
    if let DiscoveryOutcome::Found(page) = outcome {
        for record in page.records() {
            validate_record(record).map_err(|error| {
                DiscoveryError::new(
                    DiscoveryErrorKind::InvalidProviderResult,
                    "discovery provider returned an invalid record",
                )
                .with_related(error.to_string())
            })?;
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

    fn id(value: &str) -> DiscoveryId {
        DiscoveryId::new(value).expect("valid discovery identity")
    }

    fn expression(value: &str) -> DiscoveryExpression {
        DiscoveryExpression::new(value).expect("valid expression representation")
    }

    fn property(name: &str, value: &str) -> DiscoveryProperty {
        DiscoveryProperty::new(id(name), expression(value))
    }

    #[test]
    fn accepts_open_world_qualified_identities() {
        let identity = id("future::fabric::compute_service");

        assert_eq!(identity.as_str(), "future::fabric::compute_service");
    }

    #[test]
    fn rejects_invalid_qualified_identities() {
        assert!(DiscoveryId::new("").is_err());
        assert!(DiscoveryId::new("network::::service").is_err());
        assert!(DiscoveryId::new(" network::service").is_err());
        assert!(DiscoveryId::new("network::service ").is_err());
        assert!(DiscoveryId::new("network::bad/name").is_err());
    }

    #[test]
    fn preserves_discovery_expression_text() {
        let value = expression("capability(\"compute.general\")");

        assert_eq!(value.as_str(), "capability(\"compute.general\")");
    }

    #[test]
    fn rejects_empty_and_nul_expressions() {
        assert!(DiscoveryExpression::new(" ").is_err());
        assert!(DiscoveryExpression::new("value\0invalid").is_err());
    }

    #[test]
    fn preserves_ordered_properties_and_nested_blocks() {
        let declaration = DiscoveryDeclaration::new(id("compute"))
            .with_member(DiscoveryMemberNode::property(property(
                "service",
                "compute::general",
            )))
            .with_member(DiscoveryMemberNode::block(id("selector")))
            .with_member(DiscoveryMemberNode::nested_property(
                1,
                property("capability", "compute::general"),
            ))
            .with_member(DiscoveryMemberNode::nested_block(
                1,
                id("locality"),
            ))
            .with_member(DiscoveryMemberNode::nested_property(
                3,
                property("preference", "locality::near"),
            ));

        validate_declaration(&declaration).expect("valid declaration");

        let root_members: Vec<_> = declaration.children_of(None).collect();
        assert_eq!(root_members.len(), 2);

        let selector_members: Vec<_> = declaration.children_of(Some(1)).collect();
        assert_eq!(selector_members.len(), 2);

        let locality_members: Vec<_> = declaration.children_of(Some(3)).collect();
        assert_eq!(locality_members.len(), 1);
    }

    #[test]
    fn rejects_a_parent_that_appears_after_its_child() {
        let declaration = DiscoveryDeclaration::new(id("compute"))
            .with_member(DiscoveryMemberNode::nested_property(
                1,
                property("capability", "compute::general"),
            ))
            .with_member(DiscoveryMemberNode::block(id("selector")));

        let error = validate_declaration(&declaration)
            .expect_err("parent must precede child");

        assert_eq!(
            error.kind(),
            DiscoveryErrorKind::InvalidMemberHierarchy
        );
    }

    #[test]
    fn rejects_a_property_as_a_parent() {
        let declaration = DiscoveryDeclaration::new(id("compute"))
            .with_member(DiscoveryMemberNode::property(property(
                "service",
                "compute::general",
            )))
            .with_member(DiscoveryMemberNode::nested_property(
                0,
                property("capability", "compute::general"),
            ));

        let error = validate_declaration(&declaration)
            .expect_err("parent must be a block");

        assert_eq!(
            error.kind(),
            DiscoveryErrorKind::InvalidMemberHierarchy
        );
    }

    #[test]
    fn preserves_typed_grammar_references() {
        let declaration = DiscoveryDeclaration::new(id("compute"))
            .with_reference(DiscoveryReference::new(
                DiscoveryReferenceKind::Service,
                id("services::compute"),
            ))
            .with_reference(DiscoveryReference::new(
                DiscoveryReferenceKind::Protocol,
                id("network::reliable"),
            ))
            .with_reference(DiscoveryReference::new(
                DiscoveryReferenceKind::Capability,
                id("compute::general"),
            ));

        validate_declaration(&declaration).expect("valid references");

        assert_eq!(
            declaration.references()[0].kind(),
            DiscoveryReferenceKind::Service
        );
        assert_eq!(
            declaration.references()[1].target().as_str(),
            "network::reliable"
        );
    }

    #[test]
    fn request_validates_declaration() {
        let declaration = DiscoveryDeclaration::new(id("compute"))
            .with_selector(expression("capability(\"compute.general\")"))
            .with_query(expression("service.status == \"ready\""));

        let request = DiscoveryRequest::new(declaration)
            .expect("valid request")
            .with_preferred_page_size(
                NonZeroUsize::new(32).expect("non-zero page size"),
            );

        assert_eq!(
            request.preferred_page_size().map(NonZeroUsize::get),
            Some(32)
        );
        assert!(request.continuation().is_none());
    }

    #[test]
    fn cursor_is_opaque_and_validated() {
        let cursor = DiscoveryCursor::new("provider-specific-token")
            .expect("valid cursor");

        assert_eq!(cursor.as_str(), "provider-specific-token");
        assert!(DiscoveryCursor::new("").is_err());
        assert!(DiscoveryCursor::new("invalid\ncursor").is_err());
    }

    #[test]
    fn discovery_page_preserves_records_and_continuation() {
        let record = DiscoveryRecord::new(id("services::worker"))
            .with_reference(DiscoveryReference::new(
                DiscoveryReferenceKind::Endpoint,
                id("endpoints::worker"),
            ))
            .with_property(property("status", "\"ready\""));

        let cursor = DiscoveryCursor::new("next-page")
            .expect("valid cursor");

        let page = DiscoveryPage::new(vec![record], Some(cursor))
            .expect("valid page");

        assert_eq!(page.len(), 1);
        assert!(!page.is_empty());
        assert_eq!(
            page.next_cursor().map(DiscoveryCursor::as_str),
            Some("next-page")
        );
    }

    #[test]
    fn empty_successful_page_is_not_not_found() {
        let page = DiscoveryPage::new(Vec::new(), None)
            .expect("valid empty page");

        let outcome = DiscoveryOutcome::Found(page);

        assert!(outcome.is_found());
        assert!(outcome.page().expect("page exists").is_empty());
        assert!(!matches!(outcome, DiscoveryOutcome::NotFound));
    }

    #[test]
    fn provider_metadata_is_open_world() {
        let info = DiscoveryProviderInfo::new(
            id("vendor::future_registry"),
            "revision-alpha",
        )
        .expect("valid provider")
        .with_capability(id("discovery::distributed"))
        .with_capability(id("discovery::incremental"));

        assert_eq!(info.identity().as_str(), "vendor::future_registry");
        assert!(info.supports_capability(&id("discovery::distributed")));
        assert!(!info.supports_capability(&id("discovery::unknown")));
    }

    #[test]
    fn validates_provider_outcomes() {
        let record = DiscoveryRecord::new(id("services::quantum_backend"))
            .with_reference(DiscoveryReference::new(
                DiscoveryReferenceKind::Service,
                id("quantum::backend"),
            ));

        let page = DiscoveryPage::new(vec![record], None)
            .expect("valid page");

        validate_outcome(&DiscoveryOutcome::Found(page))
            .expect("valid outcome");
    }

    #[test]
    fn error_exposes_stable_classification_and_related_details() {
        let error = DiscoveryError::new(
            DiscoveryErrorKind::ProviderFailure,
            "provider could not complete discovery",
        )
        .with_related("provider: registry::primary");

        assert_eq!(error.kind().as_str(), "provider_failure");
        assert_eq!(error.related().len(), 1);
        assert!(error.to_string().contains("provider_failure"));
    }
}
