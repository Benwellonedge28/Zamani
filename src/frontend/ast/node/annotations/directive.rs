//! # Zamani Frontend AST — Generic Source Directive
//!
//! This module defines the canonical native Zamani AST representation of a
//! source-level directive.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer
//!      │
//!      ▼
//! parser
//!      │
//!      ▼
//! Native Zamani AST
//!      │
//!      ├── Node
//!      ├── NodeKind::Directive
//!      ├── Directive
//!      └── DirectiveArgument
//!      │
//!      ▼
//! structural validation
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! domain / target / backend lowering
//! ```
//!
//! ## Purpose
//!
//! `Directive` represents source-level directive syntax and source intent.
//!
//! It deliberately does **not** execute a directive.
//!
//! A directive may eventually influence compiler processing, imports, source
//! transformation, diagnostics, optimization policy, resource constraints,
//! extension behavior, or another language-defined concern. Its meaning is
//! determined by a later compiler phase.
//!
//! This type therefore contains only source-level information:
//!
//! - canonical AST node identity;
//! - source span;
//! - AST metadata;
//! - directive namespace;
//! - directive name;
//! - ordered source arguments.
//!
//! It contains no:
//!
//! - hardware;
//! - quantum processor;
//! - qubit count;
//! - machine size;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience implementation;
//! - backend;
//! - vendor;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime state;
//! - process execution;
//! - filesystem access;
//! - network access.
//!
//! ## POCO-REAF
//!
//! A directive must not make the AST dependent on the eventual machine.
//!
//! The same directive representation can therefore survive compilation toward:
//!
//! - a tiny classical system;
//! - a large classical system;
//! - a quantum system;
//! - a heterogeneous system;
//! - a distributed system;
//! - a simulator;
//! - a future computational architecture.
//!
//! The source program expresses intent. Later compilation phases determine how
//! that intent can be realized with the resources and capabilities available to
//! the selected compilation environment.
//!
//! ## Extensibility
//!
//! Directive identity is represented by an open namespace/name pair.
//!
//! This is intentionally **not**:
//!
//! ```text
//! enum DirectiveKind {
//!     Using,
//!     GlobalUsing,
//!     Quantum,
//!     Hardware,
//!     ...
//! }
//! ```
//!
//! Such a closed enumeration would require modification of the native AST every
//! time a new language feature or computational technology introduced a new
//! directive.
//!
//! Instead, directive identity is data:
//!
//! ```text
//! namespace + name
//! ```
//!
//! A compiler extension may subsequently assign semantic meaning to that
//! identity without modifying this file.
//!
//! ## Directive versus annotation versus attribute
//!
//! These concepts are intentionally separate:
//!
//! - `Attribute` is structured metadata attached to an AST construct.
//! - `Annotation` represents source-level metadata/intent.
//! - `Directive` represents a source-level directive construct that may affect
//!   compiler/source processing.
//!
//! The AST stores all three as source-level information. Semantic interpretation
//! belongs downstream.
//!
//! ## Grammar integration
//!
//! The current Zamani grammar contains directive productions including forms
//! equivalent to:
//!
//! ```text
//! usingDirective: 'using' IDENTIFIER ';';
//! ```
//!
//! and global-using directives.
//!
//! This type is deliberately more general than those initial productions so
//! that future directive forms do not require redesigning the AST.
//!
//! The parser is responsible for recognizing the concrete syntax and creating
//! this source-level representation.
//!
//! The parser must not execute the directive.
//!
//! ## Argument representation
//!
//! Directive arguments are retained as ordered source fragments rather than
//! being interpreted here.
//!
//! This provides three important properties:
//!
//! 1. source preservation;
//! 2. forward compatibility with extensions;
//! 3. independence from semantic expression/type systems.
//!
//! A later semantic extension may interpret an argument according to its
//! registered directive schema.
//!
//! This also prevents this low-level file from becoming coupled to the
//! expression subsystem.
//!
//! ## Dependency contract
//!
//! This file may depend only on:
//!
//! - [`super::super::node::AstNode`];
//! - [`super::super::node::Node`];
//! - [`super::super::node::NodeId`];
//! - [`super::super::node_kind::CoreNodeKind`];
//! - [`super::super::node_kind::NodeKind`];
//! - [`super::super::source::Span`];
//! - Rust standard-library functionality;
//! - `serde` for structural serialization.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - symbol resolution;
//! - type inference;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum hardware;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - optimization;
//! - error correction;
//! - resilience;
//! - runtime execution;
//! - backend providers;
//! - filesystem APIs;
//! - network APIs.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!   │
//!   └── Directive::new / Directive::with_arguments
//!             │
//!             ▼
//!       Native AST
//!             │
//!             ▼
//!   structural validation
//!             │
//!             ▼
//!      semantic analysis
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//! recognized     preserved/ignored
//!       │
//!       ▼
//! semantic directive/constraint
//!       │
//!       ▼
//! ZUIR when semantically applicable
//! ```
//!
//! This module never performs the final interpretation itself.
//!
//! ## Visitor and traversal contract
//!
//! `Directive` is an AST node and therefore implements [`AstNode`].
//!
//! Its argument payload is source-level data rather than child AST nodes.
//! Consequently, this concrete node contributes zero child AST nodes to
//! structural AST traversal.
//!
//! If a future directive language introduces nested AST expressions or blocks,
//! that representation belongs in an explicitly defined extension or a future
//! version of the directive model. It must not be silently smuggled into the
//! opaque argument string.
//!
//! ## Validation contract
//!
//! Local validation checks only invariants owned by this type:
//!
//! - the embedded node has `CoreNodeKind::Directive`;
//! - namespace, when present, is non-empty;
//! - directive name is non-empty;
//! - namespace/name contain no control or whitespace characters;
//! - argument spans belong to the same source as the directive;
//! - argument spans are contained within the directive span;
//! - argument source fragments are not structurally invalid.
//!
//! Semantic validity is deliberately excluded.
//!
//! In particular this file does not decide whether a directive is:
//!
//! - recognized;
//! - allowed in a particular context;
//! - deprecated;
//! - supported by a compilation target;
//! - compatible with a quantum device;
//! - compatible with a backend.
//!
//! Those decisions belong to later phases.
//!
//! ## Scalability
//!
//! No fixed number of directives or directive arguments is imposed.
//!
//! `Vec` is used for ordered arguments because source order is significant and
//! the representation must scale with available resources.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_DIRECTIVES
//! MAX_ARGUMENTS
//! MAX_MACHINE_SIZE
//! MAX_QUBITS
//! MAX_BACKENDS
//! ```
//!
//! Hostile-input limits, when required, belong to configurable compiler
//! resource-policy infrastructure.
//!
//! ## Determinism
//!
//! Directive representation is deterministic:
//!
//! - namespace spelling is preserved;
//! - name spelling is preserved;
//! - argument order is preserved;
//! - argument spelling is preserved;
//! - source spans are explicit;
//! - no timestamps are stored;
//! - no random state is stored;
//! - no memory addresses are stored;
//! - no process state is stored;
//! - no hash-map iteration order is used.
//!
//! ## Serialization
//!
//! `serde` serialization preserves:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - namespace;
//! - name;
//! - argument order;
//! - argument names;
//! - argument source values;
//! - argument spans.
//!
//! Global AST schema and serialization-version policy belongs to the enclosing
//! AST serialization layer.
//!
//! ## Security
//!
//! This type treats directive input as untrusted source data.
//!
//! It does not:
//!
//! - execute commands;
//! - spawn processes;
//! - access files;
//! - access the network;
//! - invoke plugins;
//! - access hardware;
//! - evaluate expressions;
//! - interpret directive payloads.
//!
//! No `unsafe` code is used.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021.
//!
//! No nightly features are required.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::node::{AstNode, Node};
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// In-memory schema version of the native directive representation.
///
/// This is independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - global AST schema version;
/// - serialization format version;
/// - extension version.
pub const DIRECTIVE_SCHEMA_VERSION: u16 = 1;

/// A source-level directive in the native Zamani AST.
///
/// `Directive` is deliberately generic. Its namespace and name are data rather
/// than a closed enumeration so new directives can be introduced without
/// modifying the native AST.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Directive {
    /// Canonical AST node identity, kind, source span and metadata.
    node: Node,

    /// Optional directive namespace.
    ///
    /// `None` represents an unqualified directive.
    ///
    /// A namespace is source-level identity only. It does not identify a
    /// backend, vendor, device or machine.
    namespace: Option<String>,

    /// Directive name.
    ///
    /// The exact source spelling is preserved.
    name: String,

    /// Ordered directive arguments.
    ///
    /// Source order is preserved.
    arguments: Vec<DirectiveArgument>,
}

impl Directive {
    /// Creates an argument-free directive.
    ///
    /// The supplied node must eventually validate as
    /// `CoreNodeKind::Directive`.
    ///
    /// This constructor intentionally does not rewrite the node kind.
    pub fn new(
        node: Node,
        namespace: Option<impl Into<String>>,
        name: impl Into<String>,
    ) -> Result<Self, DirectiveError> {
        Self::with_arguments(node, namespace, name, Vec::new())
    }

    /// Creates a directive with ordered source arguments.
    ///
    /// No semantic interpretation is performed.
    pub fn with_arguments(
        node: Node,
        namespace: Option<impl Into<String>>,
        name: impl Into<String>,
        arguments: Vec<DirectiveArgument>,
    ) -> Result<Self, DirectiveError> {
        let namespace = namespace.map(Into::into);
        let name = name.into();

        validate_optional_component("namespace", namespace.as_deref())?;
        validate_component("name", &name)?;

        Ok(Self {
            node,
            namespace,
            name,
            arguments,
        })
    }

    /// Returns the canonical node container.
    #[inline]
    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the canonical node container.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the optional namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the directive name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the directive arguments in source order.
    #[inline]
    #[must_use]
    pub fn arguments(&self) -> &[DirectiveArgument] {
        &self.arguments
    }

    /// Returns mutable access to the directive arguments.
    ///
    /// Mutating arguments changes source-level AST data only. It does not
    /// execute or interpret the directive.
    #[inline]
    pub fn arguments_mut(&mut self) -> &mut Vec<DirectiveArgument> {
        &mut self.arguments
    }

    /// Returns the number of source-level arguments.
    #[inline]
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns whether the directive has no arguments.
    #[inline]
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns the fully qualified directive identity.
    ///
    /// The representation is:
    ///
    /// ```text
    /// namespace.name
    /// ```
    ///
    /// for namespaced directives, and:
    ///
    /// ```text
    /// name
    /// ```
    ///
    /// for unqualified directives.
    ///
    /// This method allocates a `String`. Use [`Self::namespace`] and
    /// [`Self::name`] when allocation is unnecessary.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        match self.namespace() {
            Some(namespace) => {
                let capacity = namespace
                    .len()
                    .saturating_add(1)
                    .saturating_add(self.name.len());

                let mut result = String::with_capacity(capacity);
                result.push_str(namespace);
                result.push('.');
                result.push_str(&self.name);
                result
            }
            None => self.name.clone(),
        }
    }

    /// Returns the canonical native AST node kind expected by this type.
    #[inline]
    #[must_use]
    pub const fn expected_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::Directive)
    }

    /// Returns whether the embedded node has the canonical directive kind.
    #[inline]
    #[must_use]
    pub fn has_expected_kind(&self) -> bool {
        self.node.is_kind(&Self::expected_kind())
    }

    /// Returns the number of child AST nodes directly owned by this node.
    ///
    /// Directive arguments are intentionally source fragments rather than child
    /// AST nodes. Therefore the concrete directive node is a leaf from the
    /// native AST traversal perspective.
    #[inline]
    #[must_use]
    pub const fn child_count(&self) -> usize {
        0
    }

    /// Returns whether this directive is a leaf in the native AST.
    #[inline]
    #[must_use]
    pub const fn is_leaf(&self) -> bool {
        true
    }

    /// Returns the directive schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        DIRECTIVE_SCHEMA_VERSION
    }

    /// Performs local structural validation.
    ///
    /// This method intentionally does not interpret directive semantics.
    pub fn validate_structure(&self) -> Result<(), DirectiveError> {
        if !self.has_expected_kind() {
            return Err(DirectiveError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        validate_optional_component("namespace", self.namespace())?;
        validate_component("name", &self.name)?;

        let directive_span = *self.node.span();

        for argument in &self.arguments {
            argument.validate_structure()?;

            let argument_span = *argument.span();

            if argument_span.source() != directive_span.source() {
                return Err(DirectiveError::ArgumentSpanSourceMismatch {
                    directive_source: directive_span.source(),
                    argument_source: argument_span.source(),
                });
            }

            if !directive_span.contains(argument_span) {
                return Err(DirectiveError::ArgumentSpanOutsideDirective {
                    directive_span,
                    argument_span,
                });
            }
        }

        Ok(())
    }

    /// Returns a compact source-level diagnostic identity.
    ///
    /// No argument payload is included because extension payloads may be large.
    #[inline]
    #[must_use]
    pub fn diagnostic_name(&self) -> String {
        self.qualified_name()
    }
}

impl AstNode for Directive {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// One source-level argument belonging to a [`Directive`].
///
/// The value is intentionally retained as source text instead of being
/// interpreted by the native AST.
///
/// This allows different directive extensions to define their own argument
/// grammar without forcing the native AST to know about every future extension.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DirectiveArgument {
    /// Optional source-level argument name.
    ///
    /// `None` represents a positional argument.
    name: Option<String>,

    /// Source-preserved argument value.
    ///
    /// This is data only. It is never evaluated by this module.
    value: String,

    /// Source span covering this argument.
    span: Span,
}

impl DirectiveArgument {
    /// Creates a positional directive argument.
    pub fn positional(
        value: impl Into<String>,
        span: Span,
    ) -> Result<Self, DirectiveArgumentError> {
        Self::new(None, value, span)
    }

    /// Creates a named directive argument.
    pub fn named(
        name: impl Into<String>,
        value: impl Into<String>,
        span: Span,
    ) -> Result<Self, DirectiveArgumentError> {
        Self::new(Some(name), value, span)
    }

    /// Creates a directive argument with an optional name.
    ///
    /// Empty values are allowed because some directive grammars may assign
    /// meaning to an explicitly empty source payload. Empty argument names are
    /// rejected because a named argument requires an actual name.
    pub fn new(
        name: Option<impl Into<String>>,
        value: impl Into<String>,
        span: Span,
    ) -> Result<Self, DirectiveArgumentError> {
        let name = name.map(Into::into);
        let value = value.into();

        if let Some(name) = name.as_deref() {
            validate_component("argument name", name)?;
        }

        Ok(Self { name, value, span })
    }

    /// Returns the optional argument name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the source-preserved argument value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the source span of the argument.
    #[inline]
    #[must_use]
    pub const fn span(&self) -> &Span {
        &self.span
    }

    /// Returns whether this is a named argument.
    #[inline]
    #[must_use]
    pub fn is_named(&self) -> bool {
        self.name.is_some()
    }

    /// Returns whether this is a positional argument.
    #[inline]
    #[must_use]
    pub fn is_positional(&self) -> bool {
        self.name.is_none()
    }

    /// Performs local structural validation.
    pub fn validate_structure(&self) -> Result<(), DirectiveArgumentError> {
        if let Some(name) = self.name.as_deref() {
            validate_component("argument name", name)?;
        }

        Ok(())
    }
}

/// Errors produced by [`Directive`] construction or structural validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectiveError {
    /// The directive node has the wrong AST node kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// A directive identity component is empty.
    EmptyComponent {
        /// Component name.
        component: &'static str,
    },

    /// A directive identity component contains an invalid character.
    InvalidComponentCharacter {
        /// Component name.
        component: &'static str,

        /// Invalid character.
        character: char,
    },

    /// An argument belongs to another source unit.
    ArgumentSpanSourceMismatch {
        /// Source containing the directive.
        directive_source: super::super::source::SourceId,

        /// Source containing the argument.
        argument_source: super::super::source::SourceId,
    },

    /// An argument span lies outside the directive span.
    ArgumentSpanOutsideDirective {
        /// Complete directive span.
        directive_span: Span,

        /// Invalid argument span.
        argument_span: Span,
    },

    /// A directive argument is structurally invalid.
    InvalidArgument {
        /// Error from the argument validator.
        error: DirectiveArgumentError,
    },
}

impl fmt::Display for DirectiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => write!(
                formatter,
                "invalid directive node kind: expected {}, found {}",
                Self::expected_kind_text(),
                actual
            ),
            Self::EmptyComponent { component } => {
                write!(formatter, "directive {component} must not be empty")
            }
            Self::InvalidComponentCharacter {
                component,
                character,
            } => write!(
                formatter,
                "directive {component} contains invalid character {:?}",
                character
            ),
            Self::ArgumentSpanSourceMismatch {
                directive_source,
                argument_source,
            } => write!(
                formatter,
                "directive argument belongs to source {}, but directive belongs to source {}",
                argument_source, directive_source
            ),
            Self::ArgumentSpanOutsideDirective {
                directive_span,
                argument_span,
            } => write!(
                formatter,
                "directive argument span {} is outside directive span {}",
                argument_span, directive_span
            ),
            Self::InvalidArgument { error } => {
                write!(formatter, "invalid directive argument: {error}")
            }
        }
    }
}

impl std::error::Error for DirectiveError {}

impl DirectiveError {
    fn expected_kind_text() -> &'static str {
        "zamani:directive"
    }
}

impl From<DirectiveArgumentError> for DirectiveError {
    fn from(error: DirectiveArgumentError) -> Self {
        Self::InvalidArgument { error }
    }
}

/// Errors produced while constructing or validating a directive argument.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectiveArgumentError {
    /// An argument name is empty.
    EmptyComponent {
        /// Component name.
        component: &'static str,
    },

    /// An argument name contains an invalid character.
    InvalidComponentCharacter {
        /// Component name.
        component: &'static str,

        /// Invalid character.
        character: char,
    },
}

impl fmt::Display for DirectiveArgumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(formatter, "directive {component} must not be empty")
            }
            Self::InvalidComponentCharacter {
                component,
                character,
            } => write!(
                formatter,
                "directive {component} contains invalid character {:?}",
                character
            ),
        }
    }
}

impl std::error::Error for DirectiveArgumentError {}

/// Validates an optional directive identity component.
fn validate_optional_component(
    component: &'static str,
    value: Option<&str>,
) -> Result<(), DirectiveError> {
    if let Some(value) = value {
        validate_component(component, value)?;
    }

    Ok(())
}

/// Validates a directive identity component.
///
/// This deliberately does not impose an artificial byte-length limit.
/// Compiler-wide hostile-input limits belong to configurable resource-policy
/// infrastructure.
///
/// Whitespace and control characters are rejected because the identity itself
/// must remain a lexical identifier rather than an embedded source program.
fn validate_component(
    component: &'static str,
    value: &str,
) -> Result<(), DirectiveError> {
    if value.is_empty() {
        return Err(DirectiveError::EmptyComponent { component });
    }

    for character in value.chars() {
        if character.is_control() || character.is_whitespace() {
            return Err(DirectiveError::InvalidComponentCharacter {
                component,
                character,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeId;
    use super::super::super::node_kind::{CoreNodeKind, NodeKind};
    use super::super::super::source::{SourceId, SourceOffset};

    fn span(start: u64, end: u64) -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must be structurally valid")
    }

    fn node() -> Node {
        Node::new(
            NodeId::new(1).expect("test node ID must be non-zero"),
            CoreNodeKind::Directive.into(),
            span(0, 20),
            Default::default(),
        )
    }

    #[test]
    fn expected_kind_is_directive() {
        assert_eq!(
            Directive::expected_kind(),
            NodeKind::Core(CoreNodeKind::Directive)
        );
    }

    #[test]
    fn argument_free_directive_is_valid() {
        let directive = Directive::new(
            node(),
            Some("zamani.compiler"),
            "using",
        )
        .expect("valid directive");

        assert_eq!(directive.namespace(), Some("zamani.compiler"));
        assert_eq!(directive.name(), "using");
        assert!(directive.has_no_arguments());
        assert_eq!(directive.child_count(), 0);
        assert!(directive.is_leaf());
        assert!(directive.validate_structure().is_ok());
    }

    #[test]
    fn unqualified_directive_is_supported() {
        let directive =
            Directive::new(node(), None::<String>, "using").expect("valid directive");

        assert_eq!(directive.namespace(), None);
        assert_eq!(directive.name(), "using");
        assert_eq!(directive.qualified_name(), "using");
    }

    #[test]
    fn qualified_name_is_deterministic() {
        let directive = Directive::new(
            node(),
            Some("zamani.compiler"),
            "using",
        )
        .expect("valid directive");

        assert_eq!(
            directive.qualified_name(),
            "zamani.compiler.using"
        );
        assert_eq!(
            directive.qualified_name(),
            "zamani.compiler.using"
        );
    }

    #[test]
    fn positional_argument_is_supported() {
        let argument =
            DirectiveArgument::positional("std.core", span(6, 14))
                .expect("valid argument");

        assert!(argument.is_positional());
        assert!(!argument.is_named());
        assert_eq!(argument.name(), None);
        assert_eq!(argument.value(), "std.core");
        assert!(argument.validate_structure().is_ok());
    }

    #[test]
    fn named_argument_is_supported() {
        let argument =
            DirectiveArgument::named("scope", "global", span(6, 19))
                .expect("valid argument");

        assert!(argument.is_named());
        assert_eq!(argument.name(), Some("scope"));
        assert_eq!(argument.value(), "global");
    }

    #[test]
    fn argument_order_is_preserved() {
        let arguments = vec![
            DirectiveArgument::positional("first", span(1, 6))
                .expect("valid argument"),
            DirectiveArgument::positional("second", span(7, 13))
                .expect("valid argument"),
        ];

        let directive = Directive::with_arguments(
            node(),
            None::<String>,
            "example",
            arguments,
        )
        .expect("valid directive");

        assert_eq!(directive.argument_count(), 2);
        assert_eq!(directive.arguments()[0].value(), "first");
        assert_eq!(directive.arguments()[1].value(), "second");
    }

    #[test]
    fn invalid_identity_is_rejected() {
        let result = Directive::new(
            node(),
            Some("zamani compiler"),
            "using",
        );

        assert!(matches!(
            result,
            Err(DirectiveError::InvalidComponentCharacter { .. })
        ));
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = Directive::new(
            node(),
            None::<String>,
            "",
        );

        assert!(matches!(
            result,
            Err(DirectiveError::EmptyComponent { component: "name" })
        ));
    }

    #[test]
    fn wrong_node_kind_is_reported_by_validation() {
        let wrong_node = Node::new(
            NodeId::new(2).expect("test node ID must be non-zero"),
            CoreNodeKind::Attribute.into(),
            span(0, 10),
            Default::default(),
        );

        let directive =
            Directive::new(wrong_node, None::<String>, "using")
                .expect("construction itself does not rewrite node kind");

        assert!(matches!(
            directive.validate_structure(),
            Err(DirectiveError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn_argument_span_must_be_inside_directive_span() {
        let argument =
            DirectiveArgument::positional("outside", span(21, 28))
                .expect("argument itself is structurally valid");

        let directive = Directive::with_arguments(
            node(),
            None::<String>,
            "using",
            vec![argument],
        )
        .expect("directive construction must not perform cross-node validation");

        assert!(matches!(
            directive.validate_structure(),
            Err(DirectiveError::ArgumentSpanOutsideDirective { .. })
        ));
    }

    #[test]
    fn argument_source_must_match_directive_source() {
        let argument_span = Span::new(
            SourceId::from_raw(2),
            SourceOffset::from_raw(1),
            SourceOffset::from_raw(5),
        )
        .expect("test span must be valid");

        let argument =
            DirectiveArgument::positional("other", argument_span)
                .expect("argument itself is valid");

        let directive = Directive::with_arguments(
            node(),
            None::<String>,
            "using",
            vec![argument],
        )
        .expect("directive construction succeeds");

        assert!(matches!(
            directive.validate_structure(),
            Err(DirectiveError::ArgumentSpanSourceMismatch { .. })
        ));
    }

    #[test]
    fn empty_argument_value_is_allowed() {
        let argument =
            DirectiveArgument::positional("", span(6, 6))
                .expect("empty source payload is representable");

        assert_eq!(argument.value(), "");
    }

    #[test]
    fn large_node_ids_are_supported() {
        let large_node = Node::new(
            NodeId::new(u64::MAX).expect("maximum non-zero node ID is valid"),
            CoreNodeKind::Directive.into(),
            span(0, 1),
            Default::default(),
        );

        let directive =
            Directive::new(large_node, None::<String>, "using")
                .expect("large node IDs must be structural, not machine limits");

        assert!(directive.validate_structure().is_ok());
    }

    #[test]
    fn serde_round_trip_preserves_directive() {
        let argument =
            DirectiveArgument::named("scope", "global", span(6, 18))
                .expect("valid argument");

        let directive = Directive::with_arguments(
            node(),
            Some("zamani.compiler"),
            "using",
            vec![argument],
        )
        .expect("valid directive");

        let encoded =
            serde_json::to_string(&directive).expect("serialization must succeed");

        let decoded: Directive =
            serde_json::from_str(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, directive);
    }

    #[test]
    fn ast_node_trait_exposes_common_node() {
        let directive =
            Directive::new(node(), None::<String>, "using")
                .expect("valid directive");

        assert_eq!(directive.id(), directive.node().id());
        assert_eq!(directive.kind(), directive.node().kind());
        assert_eq!(directive.span(), directive.node().span());
        assert_eq!(
            directive.metadata(),
            directive.node().metadata()
        );
    }
}