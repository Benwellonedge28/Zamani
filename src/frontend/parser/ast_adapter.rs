
#![forbid(unsafe_code)]

//! AST construction adapter for the Zamani frontend parser.
//!
//! # Purpose
//!
//! This module is the construction boundary between parser productions and
//! the canonical AST in `crate::frontend::ast::node`.
//!
//! It reuses the canonical:
//! - `Node`, `NodeId`, `NodeIdAllocator`, `NodeKind`, and `CoreNodeKind`;
//! - `Span`, `SourceId`, and `SourceOffset`;
//! - `NodeMetadata`;
//! - `Expression` and `ExpressionKind`;
//! - `Statement` and `StatementKind`.
//!
//! It does not define a competing AST, tokenize source, resolve names, perform
//! type checking, evaluate contracts, negotiate resources, or lower syntax to
//! a target-specific IR.
//!
//! # Source coordinates
//!
//! Canonical AST spans use UTF-8 byte offsets and half-open ranges `[start, end)`.
//! Lexer spans are converted using their byte offsets; line and column values
//! remain presentation information owned by the lexer/source-map layer.
//!
//! # Scalability and safety
//!
//! This module imposes no machine-capacity, qubit-count, register-width,
//! tensor-rank, device-count, or computational-domain ceilings. Node identity
//! allocation is checked and never silently wraps. Any operational allocation
//! or source-size budget belongs to the parser session/configuration layer.
//!
//! Each adapter owns its allocator. Use one adapter per AST construction unit,
//! or provide a deliberately allocated ID range when constructing independent
//! fragments. Do not share one mutable adapter between concurrent parse tasks.
//!
//! # Integration contract
//!
//! Parser grammar modules supply:
//! - a canonical `Span` or lexer span;
//! - a `CoreNodeKind`, `NodeKind`, `ExpressionKind`, or `StatementKind`;
//! - metadata already collected from source syntax;
//! - child `NodeId`s for graph-shaped expression and statement kinds.
//!
//! This adapter allocates identities and constructs canonical nodes. It does
//! not manufacture missing child nodes or silently discard unrepresented
//! syntax. Parser recovery nodes and extension nodes must be explicit.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::{
    validate_node, CoreNodeKind, Node, NodeId, NodeIdAllocationError,
    NodeIdAllocator, NodeKind, NodeKindError, NodeMetadata, SourceId,
    SourceOffset, Span, SpanError,
};
use crate::frontend::ast::node::expressions::expression::{
    Expression, ExpressionError, ExpressionKind, ExpressionValidationPolicy,
};
use crate::frontend::ast::node::statements::statement::{
    Statement, StatementError, StatementKind,
};

/// Errors that can occur while adapting parser output into the canonical AST.
#[derive(Debug)]
#[non_exhaustive]
pub enum AstAdapterError {
    /// The AST node identity allocator has no remaining representable IDs.
    NodeIdAllocation(NodeIdAllocationError),

    /// A source range could not be represented as a valid canonical span.
    InvalidSpan(SpanError),

    /// An extension node identifier is invalid.
    InvalidNodeKind(NodeKindError),

    /// A canonical node failed its local structural validation.
    InvalidNode(String),

    /// An expression failed its local structural validation.
    InvalidExpression(ExpressionError),

    /// A statement failed construction or structural validation.
    InvalidStatement(StatementError),
}

impl fmt::Display for AstAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeIdAllocation(error) => {
                write!(formatter, "AST node identity allocation failed: {error}")
            }
            Self::InvalidSpan(error) => {
                write!(formatter, "invalid AST source span: {error}")
            }
            Self::InvalidNodeKind(error) => {
                write!(formatter, "invalid AST node kind: {error}")
            }
            Self::InvalidNode(message) => {
                write!(formatter, "invalid AST node: {message}")
            }
            Self::InvalidExpression(error) => {
                write!(formatter, "invalid AST expression: {error}")
            }
            Self::InvalidStatement(error) => {
                write!(formatter, "invalid AST statement: {error}")
            }
        }
    }
}

impl Error for AstAdapterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NodeIdAllocation(error) => Some(error),
            Self::InvalidSpan(error) => Some(error),
            Self::InvalidNodeKind(error) => Some(error),
            Self::InvalidExpression(error) => Some(error),
            Self::InvalidStatement(error) => Some(error),
            Self::InvalidNode(_) => None,
        }
    }
}

impl From<NodeIdAllocationError> for AstAdapterError {
    fn from(error: NodeIdAllocationError) -> Self {
        Self::NodeIdAllocation(error)
    }
}

impl From<SpanError> for AstAdapterError {
    fn from(error: SpanError) -> Self {
        Self::InvalidSpan(error)
    }
}

impl From<NodeKindError> for AstAdapterError {
    fn from(error: NodeKindError) -> Self {
        Self::InvalidNodeKind(error)
    }
}

impl From<ExpressionError> for AstAdapterError {
    fn from(error: ExpressionError) -> Self {
        Self::InvalidExpression(error)
    }
}

impl From<StatementError> for AstAdapterError {
    fn from(error: StatementError) -> Self {
        Self::InvalidStatement(error)
    }
}

/// Constructs canonical AST nodes for one source unit or AST fragment.
///
/// `AstAdapter` intentionally owns no source text, token stream, diagnostics
/// collection, extension registry, semantic environment, or target state.
/// Those are owned by the source-map, parser-session, diagnostics, registry,
/// and downstream compiler layers respectively.
#[derive(Debug)]
pub struct AstAdapter {
    source_id: SourceId,
    ids: NodeIdAllocator,
}

impl AstAdapter {
    /// Creates an adapter with a fresh deterministic node-ID allocator.
    ///
    /// `source_id` must identify the source unit in the surrounding source-map
    /// contract. This constructor does not verify source-map registration.
    #[must_use]
    pub const fn new(source_id: SourceId) -> Self {
        Self {
            source_id,
            ids: NodeIdAllocator::new(),
        }
    }

    /// Creates an adapter using a caller-supplied allocator.
    ///
    /// Use this when the parser/session owns ID allocation policy or when
    /// independent AST fragments have been assigned distinct ID ranges.
    #[must_use]
    pub const fn with_allocator(
        source_id: SourceId,
        ids: NodeIdAllocator,
    ) -> Self {
        Self { source_id, ids }
    }

    /// Returns the source identity used by this adapter.
    #[must_use]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    /// Returns the next node identity that would be allocated, if available.
    #[must_use]
    pub fn next_node_id(&self) -> Option<NodeId> {
        self.ids.peek()
    }

    /// Allocates a deterministic, non-reused node identity.
    ///
    /// # Errors
    ///
    /// Returns `NodeIdAllocation` if the representable identity space is
    /// exhausted. Identity exhaustion is an error, never a wraparound.
    pub fn allocate_node_id(&mut self) -> Result<NodeId, AstAdapterError> {
        self.ids.allocate().map_err(AstAdapterError::from)
    }

    /// Converts a byte range into the canonical AST span for this source.
    ///
    /// Offsets are UTF-8 byte offsets, not character counts, UTF-16 code units,
    /// display columns, or hardware-dependent sizes.
    ///
    /// # Errors
    ///
    /// Returns an error if the offsets cannot be represented or if `end < start`.
    pub fn span_from_range(
        &self,
        start: usize,
        end: usize,
    ) -> Result<Span, AstAdapterError> {
        Span::from_usize(self.source_id, start, end)
            .map_err(AstAdapterError::from)
    }

    /// Creates a zero-width canonical span, typically for an insertion point.
    pub fn point_span(
        &self,
        offset: usize,
    ) -> Result<Span, AstAdapterError> {
        Span::point_usize(self.source_id, offset)
            .map_err(AstAdapterError::from)
    }

    /// Converts the existing lexer span to the canonical AST span.
    ///
    /// The lexer span stores source positions; the AST span stores a source ID
    /// and byte offsets. The source ID is supplied by this adapter because the
    /// lexer span itself is source-relative.
    pub fn span_from_lexer(
        &self,
        span: &crate::frontend::lexer::span::Span,
    ) -> Result<Span, AstAdapterError> {
        self.span_from_range(span.start_byte(), span.end_byte())
    }

    /// Creates a validated canonical node with the supplied node kind.
    ///
    /// This is the common construction path for parser productions that use
    /// graph nodes directly. Typed expression and statement productions should
    /// normally use `build_expression` and `build_statement`.
    pub fn build_node(
        &mut self,
        kind: NodeKind,
        span: Span,
        metadata: NodeMetadata,
    ) -> Result<Node, AstAdapterError> {
        self.ensure_span_source(span)?;

        let id = self.allocate_node_id()?;
        let node = Node::new(id, kind, span, metadata);

        validate_node(&node).map_err(|error| {
            AstAdapterError::InvalidNode(error.to_string())
        })?;

        Ok(node)
    }

    /// Creates a native node using the canonical core node classification.
    pub fn build_core_node(
        &mut self,
        kind: CoreNodeKind,
        span: Span,
        metadata: NodeMetadata,
    ) -> Result<Node, AstAdapterError> {
        self.build_node(NodeKind::core(kind), span, metadata)
    }

    /// Creates a node for syntax owned by a registered extension.
    ///
    /// The namespace and name identify source-level syntax, not a vendor
    /// device, physical operation, runtime handle, or backend instruction.
    /// Registration and extension-version checks belong to the parser registry.
    pub fn build_extension_node(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
        span: Span,
        metadata: NodeMetadata,
    ) -> Result<Node, AstAdapterError> {
        let kind = NodeKind::extension(namespace, name)?;
        self.build_node(kind, span, metadata)
    }

    /// Creates a canonical expression using the existing expression model.
    ///
    /// Child references in `kind` must refer to nodes created or managed by the
    /// same AST construction unit. This method validates local structure; it
    /// does not resolve those references against a complete AST graph.
    pub fn build_expression(
        &mut self,
        kind: ExpressionKind,
        span: Span,
        metadata: NodeMetadata,
    ) -> Result<Expression, AstAdapterError> {
        self.ensure_span_source(span)?;

        let id = self.allocate_node_id()?;
        let expression = Expression::new(id, kind, span, metadata);

        expression
            .validate_structure(ExpressionValidationPolicy::unrestricted())
            .map_err(AstAdapterError::from)?;

        validate_node(expression.node()).map_err(|error| {
            AstAdapterError::InvalidNode(error.to_string())
        })?;

        Ok(expression)
    }

    /// Creates a canonical statement using the existing statement model.
    ///
    /// Native statements receive their canonical core node kind. Extension
    /// statements receive a namespaced extension node kind so extension
    /// identity is preserved rather than reduced to a generic statement.
    pub fn build_statement(
        &mut self,
        kind: StatementKind,
        span: Span,
        metadata: NodeMetadata,
    ) -> Result<Statement, AstAdapterError> {
        self.ensure_span_source(span)?;

        let node_kind = match &kind {
            StatementKind::Extension {
                namespace, name, ..
            } => NodeKind::extension(namespace.clone(), name.clone())?,
            _ => NodeKind::core(kind.core_node_kind()),
        };

        let id = self.allocate_node_id()?;
        let node = Node::new(id, node_kind, span, metadata);

        validate_node(&node).map_err(|error| {
            AstAdapterError::InvalidNode(error.to_string())
        })?;

        Statement::from_node(node, kind).map_err(AstAdapterError::from)
    }

    /// Validates that a span belongs to this adapter's source unit.
    ///
    /// Mixing spans from different source units would corrupt diagnostics,
    /// navigation, and source-to-AST traceability. Cross-source generated
    /// constructs must be handled explicitly by the source-origin/provenance
    /// layer rather than silently relabeled here.
    fn ensure_span_source(&self, span: Span) -> Result<(), AstAdapterError> {
        if span.source() != self.source_id {
            return Err(AstAdapterError::InvalidNode(format!(
                "span source {} does not match adapter source {}",
                span.source(),
                self.source_id,
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::CoreNodeKind;

    fn adapter() -> AstAdapter {
        AstAdapter::new(SourceId::from_raw(7))
    }

    #[test]
    fn span_uses_the_adapter_source_and_half_open_offsets() {
        let adapter = adapter();
        let span = adapter.span_from_range(3, 9).expect("valid span");

        assert_eq!(span.source(), SourceId::from_raw(7));
        assert_eq!(span.start_raw(), 3);
        assert_eq!(span.end_raw(), 9);
        assert_eq!(span.len(), 6);
    }

    #[test]
    fn reversed_span_is_rejected() {
        let adapter = adapter();

        assert!(matches!(
            adapter.span_from_range(9, 3),
            Err(AstAdapterError::InvalidSpan(_))
        ));
    }

    #[test]
    fn empty_span_is_supported_for_eof_and_insertions() {
        let adapter = adapter();
        let span = adapter.point_span(12).expect("valid point span");

        assert!(span.is_empty());
        assert_eq!(span.start_raw(), 12);
        assert_eq!(span.end_raw(), 12);
    }

    #[test]
    fn native_nodes_receive_unique_ids_and_keep_their_spans() {
        let mut adapter = adapter();
        let span = adapter.span_from_range(0, 4).expect("valid span");

        let first = adapter
            .build_core_node(
                CoreNodeKind::Identifier,
                span,
                NodeMetadata::new(),
            )
            .expect("first node");

        let second = adapter
            .build_core_node(
                CoreNodeKind::Literal,
                span,
                NodeMetadata::new(),
            )
            .expect("second node");

        assert_ne!(first.id(), second.id());
        assert_eq!(first.span(), &span);
        assert_eq!(second.span(), &span);
        assert_eq!(first.kind().as_core(), Some(CoreNodeKind::Identifier));
        assert_eq!(second.kind().as_core(), Some(CoreNodeKind::Literal));
    }

    #[test]
    fn extension_node_identity_is_preserved() {
        let mut adapter = adapter();
        let span = adapter.span_from_range(0, 8).expect("valid span");

        let node = adapter
            .build_extension_node(
                "example.domain",
                "custom_construct",
                span,
                NodeMetadata::new(),
            )
            .expect("valid extension node");

        let extension = node.kind().as_extension().expect("extension kind");
        assert_eq!(extension.namespace(), "example.domain");
        assert_eq!(extension.name(), "custom_construct");
    }

    #[test]
    fn spans_from_another_source_are_rejected() {
        let mut adapter = adapter();
        let foreign_span = Span::from_usize(
            SourceId::from_raw(99),
            0,
            1,
        )
        .expect("valid foreign span");

        let result = adapter.build_core_node(
            CoreNodeKind::Identifier,
            foreign_span,
            NodeMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(AstAdapterError::InvalidNode(_))
        ));
    }

    #[test]
    fn allocator_exposes_the_next_identity_without_consuming_it() {
        let mut adapter = adapter();
        let before = adapter.next_node_id().expect("initial identity");
        let allocated = adapter
            .allocate_node_id()
            .expect("allocate identity");

        assert_eq!(before, allocated);
        assert_ne!(adapter.next_node_id(), Some(allocated));
    }
}
