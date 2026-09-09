//! Unary expression AST node.
//!
//! # Contract
//!
//! This module owns the source-level representation of a unary expression.
//!
//! A unary expression consists of:
//!
//! - a common [`Node`] identity/source header;
//! - an operator spelling;
//! - one operand expression.
//!
//! This is intentionally a native Zamani AST representation.
//!
//! ## Architectural boundaries
//!
//! This type MUST NOT:
//!
//! - resolve names;
//! - resolve types;
//! - resolve symbols;
//! - select a backend;
//! - select hardware;
//! - select a quantum gate;
//! - perform quantum lowering;
//! - perform optimization;
//! - perform scheduling;
//! - perform routing;
//! - perform resource allocation;
//! - contain physical qubit IDs;
//! - contain classical/quantum hardware identifiers;
//! - depend on QIR, LLVM, MLIR, OpenQASM, or target IRs.
//!
//! Operator semantics are resolved after parsing by semantic analysis.
//!
//! ## Extensibility
//!
//! The operator is represented by its source spelling rather than a closed
//! enum such as `UnaryOperator::Not`, `UnaryOperator::Negate`, etc.
//!
//! This permits the language and extension system to introduce additional
//! unary operators without requiring this AST node to be redesigned.
//!
//! ## Source preservation
//!
//! `operator` contains the parser-provided source spelling exactly as it was
//! represented by the lexer/parser. No normalization, Unicode folding, symbol
//! resolution, or semantic interpretation is performed here.
//!
//! ## Scalability
//!
//! There is no hardware-dependent or language-semantic maximum for:
//!
//! - operand depth;
//! - operator spelling length;
//! - number of unary expressions.
//!
//! Resource limits, if required to protect the compiler from hostile input,
//! belong to configurable parser/validation policies rather than this node.
//!
//! ## Dependency direction
//!
//! ```text
//! lexer/parser
//!      |
//!      v
//! native AST
//!      |
//!      v
//! structural validation
//!      |
//!      v
//! semantic analysis
//!      |
//!      v
//! ZUIR
//! ```
//!
//! This module may depend on other native AST modules, but must never depend
//! on semantic analysis, ZUIR, quantum IR, hardware, runtime, or backend code.
//!
//! ## Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1, edition 2021.
//!
//! No `unsafe` code is used.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::frontend::ast::node::{
    metadata::NodeMetadata,
    node::Node,
    node_id::NodeId,
    node_kind::NodeKind,
};
use crate::frontend::ast::source::Span;

use super::expression::Expression;

/// Error produced when constructing an invalid unary expression.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UnaryExpressionError {
    /// The operator spelling cannot be empty.
    #[error("unary operator cannot be empty")]
    EmptyOperator,
}

/// A source-level unary expression.
///
/// The node represents the syntax:
///
/// ```text
/// operator operand
/// ```
///
/// Examples include source constructs such as:
///
/// ```text
/// -x
/// !x
/// ~x
/// ++x
/// --x
/// ```
///
/// The AST does not decide what an operator means. For example, `-` may
/// represent numeric negation for one type and another language-defined
/// operation for another type. That interpretation belongs to semantic
/// analysis.
///
/// Likewise, an operator spelling does not imply a quantum, classical,
/// hardware, or backend operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnaryExpression {
    /// Common AST identity and source information.
    node: Node,

    /// Exact source-level operator spelling.
    ///
    /// The spelling is intentionally open-ended rather than represented by a
    /// closed enum. This prevents this AST node from becoming a bottleneck
    /// when new language or extension operators are introduced.
    operator: String,

    /// Operand of the unary expression.
    operand: Box<Expression>,
}

impl UnaryExpression {
    /// Construct a unary expression.
    ///
    /// # Errors
    ///
    /// Returns [`UnaryExpressionError::EmptyOperator`] when `operator` is
    /// empty.
    ///
    /// The constructor does not attempt to validate whether the operator is
    /// semantically valid. Operator validity belongs to parser/extension
    /// validation and semantic analysis.
    pub fn try_new(
        id: NodeId,
        span: Span,
        operator: impl Into<String>,
        operand: Expression,
    ) -> Result<Self, UnaryExpressionError> {
        let operator = operator.into();

        if operator.is_empty() {
            return Err(UnaryExpressionError::EmptyOperator);
        }

        Ok(Self {
            node: Node::new(id, span),
            operator,
            operand: Box::new(operand),
        })
    }

    /// Construct a unary expression with metadata.
    ///
    /// This is equivalent to [`Self::try_new`] followed by attaching the
    /// supplied metadata.
    pub fn try_with_metadata(
        id: NodeId,
        span: Span,
        operator: impl Into<String>,
        operand: Expression,
        metadata: NodeMetadata,
    ) -> Result<Self, UnaryExpressionError> {
        let mut expression = Self::try_new(id, span, operator, operand)?;
        expression.node.set_metadata(metadata);
        Ok(expression)
    }

    /// Return the common AST node header.
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Return mutable access to the common AST node header.
    ///
    /// Mutation is limited to AST-level node information such as metadata.
    /// Semantic information must remain outside this node.
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Return this node's stable AST identity.
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Return the complete source span of the unary expression.
    ///
    /// The span should normally cover the operator and operand as emitted by
    /// the parser.
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Return the node metadata, if present.
    pub fn metadata(&self) -> Option<&NodeMetadata> {
        self.node.metadata()
    }

    /// Attach metadata to this node.
    pub fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.node.set_metadata(metadata);
    }

    /// Remove and return the node metadata.
    pub fn take_metadata(&mut self) -> Option<NodeMetadata> {
        self.node.take_metadata()
    }

    /// Return the source-level operator spelling.
    pub fn operator(&self) -> &str {
        &self.operator
    }

    /// Return the operand.
    pub fn operand(&self) -> &Expression {
        self.operand.as_ref()
    }

    /// Return mutable access to the operand.
    ///
    /// This is intended for AST transformations and syntax-preserving
    /// compiler tooling. Semantic resolution must remain downstream.
    pub fn operand_mut(&mut self) -> &mut Expression {
        self.operand.as_mut()
    }

    /// Consume this node and return its operand.
    pub fn into_operand(self) -> Expression {
        *self.operand
    }

    /// Consume this node and return its operator spelling.
    pub fn into_operator(self) -> String {
        self.operator
    }

    /// Consume this node and return `(operator, operand)`.
    pub fn into_parts(self) -> (String, Expression) {
        (self.operator, *self.operand)
    }

    /// Consume this node and return its common AST node header.
    pub fn into_node(self) -> Node {
        self.node
    }

    /// Return the native AST classification for this node.
    ///
    /// `NodeKind` intentionally remains coarse-grained. It must not grow a
    /// separate variant for every expression/operator.
    pub const fn node_kind(&self) -> NodeKind {
        NodeKind::Expression
    }

    /// Unary expressions contain exactly one child expression.
    pub const fn child_count(&self) -> usize {
        1
    }

    /// Return the sole child expression.
    ///
    /// This method is useful to generic AST traversal code without requiring
    /// this node to know about a concrete visitor implementation.
    pub fn child(&self) -> &Expression {
        self.operand()
    }

    /// Return whether this node has a child expression.
    ///
    /// This is always true for a structurally valid unary expression.
    pub const fn has_child(&self) -> bool {
        true
    }

    /// Return whether the operator spelling is syntactically non-empty.
    ///
    /// Valid instances always return `true`. The method is provided for
    /// generic validation/tooling code.
    pub fn has_operator(&self) -> bool {
        !self.operator.is_empty()
    }

    /// Validate intrinsic structural invariants.
    ///
    /// This method intentionally performs only validation that belongs to the
    /// node itself. It does not:
    ///
    /// - validate operator semantics;
    /// - perform type checking;
    /// - resolve names;
    /// - validate hardware capabilities;
    /// - validate quantum operations.
    ///
    /// Recursive validation of the operand belongs to aggregate AST
    /// validation/traversal infrastructure.
    pub fn validate(&self) -> Result<(), UnaryExpressionError> {
        if self.operator.is_empty() {
            return Err(UnaryExpressionError::EmptyOperator);
        }

        Ok(())
    }

    /// Return the node's source span and operator spelling.
    ///
    /// This is useful for diagnostics without exposing implementation details.
    pub fn source_operator(&self) -> (Span, &str) {
        (self.span(), self.operator())
    }
}

impl std::fmt::Display for UnaryExpression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}{}", self.operator, self.operand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::source::source_id::SourceId;

    fn test_span() -> Span {
        Span::new(SourceId::new(1), 0, 2)
    }

    fn identifier_expression() -> Expression {
        // This test intentionally assumes the native expression module
        // provides the canonical identifier constructor.
        //
        // If `Expression` is implemented as an enum rather than a wrapper,
        // this conversion should be supplied by `expressions/expression.rs`
        // instead of duplicating identifier construction here.
        Expression::identifier_for_test("x", test_span())
    }

    #[test]
    fn constructs_valid_unary_expression() {
        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "-",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.operator(), "-");
        assert_eq!(expression.child_count(), 1);
        assert!(expression.has_child());
        assert!(expression.has_operator());
        assert_eq!(expression.node_kind(), NodeKind::Expression);
    }

    #[test]
    fn rejects_empty_operator() {
        let result = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "",
            identifier_expression(),
        );

        assert_eq!(
            result,
            Err(UnaryExpressionError::EmptyOperator)
        );
    }

    #[test]
    fn preserves_node_identity() {
        let expression = UnaryExpression::try_new(
            NodeId::new(42),
            test_span(),
            "!",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.id(), NodeId::new(42));
    }

    #[test]
    fn preserves_source_span() {
        let span = Span::new(SourceId::new(7), 10, 13);

        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            span,
            "-",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.span(), span);
    }

    #[test]
    fn preserves_operator_spelling() {
        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "custom_unary_operator",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.operator(), "custom_unary_operator");
    }

    #[test]
    fn supports_unicode_operator_spelling() {
        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "⊕",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.operator(), "⊕");
    }

    #[test]
    fn metadata_can_be_attached() {
        let mut metadata = NodeMetadata::new();
        metadata.insert(
            "test.key",
            crate::frontend::ast::node::metadata::MetadataValue::Boolean(true),
        );

        let expression = UnaryExpression::try_with_metadata(
            NodeId::new(1),
            test_span(),
            "-",
            identifier_expression(),
            metadata.clone(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.metadata(), Some(&metadata));
    }

    #[test]
    fn operand_can_be_retrieved() {
        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "-",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert!(matches!(expression.operand(), Expression::Identifier(_)));
    }

    #[test]
    fn validation_succeeds_for_valid_node() {
        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "-",
            identifier_expression(),
        )
        .expect("valid unary expression");

        assert_eq!(expression.validate(), Ok(()));
    }

    #[test]
    fn serde_round_trip_preserves_node() {
        let expression = UnaryExpression::try_new(
            NodeId::new(1),
            test_span(),
            "-",
            identifier_expression(),
        )
        .expect("valid unary expression");

        let encoded =
            serde_json::to_string(&expression).expect("serialize unary expression");

        let decoded: UnaryExpression =
            serde_json::from_str(&encoded).expect("deserialize unary expression");

        assert_eq!(expression, decoded);
    }
}