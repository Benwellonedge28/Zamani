//! # Zamani Frontend AST — Expression Builder
//!
//! `src/frontend/ast/node/builders/expression_builder.rs`
//!
//! Production construction API for canonical native Zamani expressions.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! ExpressionBuilder  ← this file
//!     │
//!     ▼
//! Expression
//!     │
//!     ▼
//! AST graph storage
//!     │
//!     ▼
//! Structural validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── Classical IR
//!     ├── Quantum IR
//!     ├── HDL IR
//!     └── Future domain IRs
//! ```
//!
//! ## Purpose
//!
//! This file provides a single construction boundary for native Zamani
//! [`Expression`] nodes.
//!
//! The builder:
//!
//! - obtains deterministic [`NodeId`] values from a caller-owned allocator;
//! - preserves the caller-supplied [`Span`];
//! - preserves caller-supplied [`NodeMetadata`];
//! - constructs the canonical [`ExpressionKind`];
//! - invokes the canonical expression structural validator;
//! - exposes convenience constructors for the existing expression variants;
//! - provides a generic construction method for future `ExpressionKind`
//!   variants;
//! - introduces no hardware, backend, quantum-technology, or machine-size
//!   assumptions.
//!
//! ## Critical architectural rule
//!
//! This builder MUST NOT define a second expression hierarchy.
//!
//! The authoritative representations are:
//!
//! - [`Node`] for common AST identity;
//! - [`NodeId`] for AST identity;
//! - [`Expression`] for an expression node;
//! - [`ExpressionKind`] for expression structure;
//! - [`ExpressionError`] for expression-local structural errors.
//!
//! The builder only orchestrates their construction.
//!
//! ## POCO-REAF
//!
//! The builder deliberately contains no:
//!
//! - qubit count;
//! - register count;
//! - machine size;
//! - CPU count;
//! - GPU count;
//! - FPGA count;
//! - topology;
//! - gate set;
//! - vendor;
//! - backend;
//! - scheduler;
//! - router;
//! - calibration;
//! - QEC implementation;
//! - runtime state;
//! - physical resource mapping.
//!
//! Therefore expression construction remains independent of the eventual
//! computational realization.
//!
//! ```text
//! Program once
//!      │
//!      ▼
//! Native Expression AST
//!      │
//!      ▼
//! Semantic analysis
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! resource discovery / mapping / optimization / scheduling
//!      │
//!      ▼
//! target realization
//! ```
//!
//! ## Graph-oriented AST
//!
//! The canonical Zamani AST represents child relationships using [`NodeId`]
//! references rather than recursively embedding AST nodes.
//!
//! This builder therefore accepts child IDs instead of owning child nodes.
//!
//! For example:
//!
//! ```text
//! BinaryExpression
//! ├── Node
//! ├── left: NodeId
//! ├── operator: OperatorRef
//! └── right: NodeId
//! ```
//!
//! The builder does not own the AST graph store.
//!
//! A caller is responsible for inserting the resulting [`Expression`] into
//! the canonical AST storage structure.
//!
//! ## Validation boundary
//!
//! This builder invokes [`Expression::validate_structure`] after construction.
//!
//! Local expression validation may detect structural problems such as:
//!
//! - incompatible node kinds;
//! - invalid expression structure;
//! - invalid operator identity;
//! - invalid extension identity;
//! - duplicate structural child references where the expression contract
//!   prohibits them;
//! - configured expression-level structural violations exposed by the
//!   canonical expression implementation.
//!
//! It does NOT perform:
//!
//! - name resolution;
//! - type checking;
//! - overload resolution;
//! - generic substitution;
//! - borrow checking;
//! - ownership checking;
//! - capability resolution;
//! - resource allocation;
//! - quantum legality checking;
//! - topology validation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backend selection.
//!
//! Those belong to later compiler stages.
//!
//! ## Parser contract
//!
//! The parser should:
//!
//! 1. recognize source syntax;
//! 2. obtain source span;
//! 3. supply source metadata;
//! 4. construct child nodes first when necessary;
//! 5. retain their `NodeId`s;
//! 6. call the appropriate builder method;
//! 7. insert the returned expression into canonical AST storage.
//!
//! The parser must not use this builder to perform semantic analysis.
//!
//! ## Semantic-analysis contract
//!
//! Semantic analysis consumes the resulting expression graph and resolves:
//!
//! - identifiers;
//! - types;
//! - operators;
//! - calls;
//! - generic arguments;
//! - resources;
//! - effects;
//! - capabilities;
//! - domains.
//!
//! This builder does not import semantic-analysis code.
//!
//! ## ZUIR contract
//!
//! The builder does not import ZUIR.
//!
//! Later lowering consumes the validated AST/semantic model and produces ZUIR.
//!
//! This preserves the required separation:
//!
//! ```text
//! AST            = source structure
//! Semantic Model = resolved meaning
//! ZUIR           = universal computational semantics
//! Domain IR      = domain implementation
//! Target IR      = target realization
//! ```
//!
//! ## Extensibility
//!
//! [`Expression::build`] is intentionally generic over [`ExpressionKind`].
//!
//! The builder does NOT require a modification whenever a new `ExpressionKind`
//! variant is added to the canonical expression implementation.
//!
//! This is important for long-term extensibility.
//!
//! Domain-specific expressions should normally use the existing
//! `ExpressionKind::Extension` mechanism rather than adding technology-specific
//! variants to the native AST.
//!
//! ## Scalability
//!
//! There are no:
//!
//! - `MAX_EXPRESSIONS`;
//! - `MAX_CHILDREN`;
//! - `MAX_ARGUMENTS`;
//! - `MAX_QUBITS`;
//! - `MAX_REGISTERS`;
//! - `MAX_DEPTH`;
//! - machine-size constants.
//!
//! Collection sizes are controlled by the canonical `ExpressionKind`
//! representation and ordinary Rust allocation.
//!
//! Any denial-of-service or compiler operational limits must be supplied by
//! the configurable compiler policy/validation layer, not this builder.
//!
//! "Infinity" here means that this builder introduces no artificial finite
//! language or machine-size ceiling. Actual execution remains bounded by
//! available computational resources and representation limits.
//!
//! ## Determinism
//!
//! Determinism is delegated to the caller-owned [`NodeIdAllocator`] and the
//! explicitly supplied source data.
//!
//! This builder:
//!
//! - has no global state;
//! - has no random state;
//! - has no timestamps;
//! - has no memory-address identity;
//! - does not inspect hardware;
//! - does not use unordered collections;
//! - does not reorder child IDs.
//!
//! ## Error handling
//!
//! Errors are returned through [`ExpressionBuilderError`].
//!
//! Ordinary malformed construction must not panic.
//!
//! Allocation exhaustion is reported rather than wrapping or reusing an ID.
//!
//! ## Ownership
//!
//! The builder borrows the caller's [`NodeIdAllocator`] mutably.
//!
//! It does not own the allocator.
//!
//! This allows:
//!
//! - parser sessions to retain allocator ownership;
//! - multiple builders to be used sequentially;
//! - deterministic compilation;
//! - independent AST fragments;
//! - explicit allocator lifetime;
//! - no global identity service.
//!
//! ## Thread safety
//!
//! The builder contains only a mutable reference to its allocator and ordinary
//! source metadata.
//!
//! It does not create shared mutable global state.
//!
//! Concurrent construction should use independently owned allocators or other
//! explicit synchronization supplied by the caller.
//!
//! ## Serialization
//!
//! The builder itself is construction infrastructure and is not part of the
//! serialized AST.
//!
//! The resulting [`Expression`] owns the serializable AST representation.
//!
//! Builder configuration must never become hidden serialized program state.
//!
//! ## Security
//!
//! This implementation:
//!
//! - contains no `unsafe` code;
//! - performs no I/O;
//! - executes no user code;
//! - accesses no hardware;
//! - performs no network operations;
//! - performs no unchecked indexing;
//! - does not dereference raw pointers;
//! - does not use global mutable state.
//!
//! Untrusted input limits belong to the compiler's configurable resource-policy
//! layer.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use super::super::expressions::expression::{
    Expression,
    ExpressionError,
    ExpressionKind,
    ExpressionResult,
    OperatorRef,
};
use super::super::metadata::NodeMetadata;
use super::super::node_id::{
    NodeId,
    NodeIdAllocationError,
    NodeIdAllocator,
};
use super::super::source::Span;

/// Errors produced while constructing a native Zamani expression.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpressionBuilderError {
    /// The node allocator could not provide another identity.
    Allocation(NodeIdAllocationError),

    /// The canonical expression rejected the constructed structure.
    Validation(ExpressionError),
}

impl core::fmt::Display for ExpressionBuilderError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Allocation(error) => {
                write!(formatter, "failed to allocate expression node ID: {error}")
            }

            Self::Validation(error) => {
                write!(formatter, "invalid expression structure: {error}")
            }
        }
    }
}

impl std::error::Error for ExpressionBuilderError {}

impl From<NodeIdAllocationError> for ExpressionBuilderError {
    #[inline]
    fn from(error: NodeIdAllocationError) -> Self {
        Self::Allocation(error)
    }
}

impl From<ExpressionError> for ExpressionBuilderError {
    #[inline]
    fn from(error: ExpressionError) -> Self {
        Self::Validation(error)
    }
}

/// Result type for expression construction.
pub type ExpressionBuilderResult<T> = Result<T, ExpressionBuilderError>;

/// Builder for canonical native Zamani [`Expression`] nodes.
///
/// The builder is intentionally scoped to one source location/metadata
/// context. Each call to a construction method creates one new AST expression
/// identity.
///
/// # Example
///
/// ```ignore
/// let mut allocator = NodeIdAllocator::new();
///
/// let builder = ExpressionBuilder::new(
///     &mut allocator,
///     span,
///     NodeMetadata::default(),
/// );
///
/// let expression = builder.identifier(identifier_id)?;
/// ```
///
/// The actual AST graph storage remains outside this builder.
pub struct ExpressionBuilder<'a> {
    allocator: &'a mut NodeIdAllocator,
    span: Span,
    metadata: NodeMetadata,
}

impl<'a> ExpressionBuilder<'a> {
    /// Creates a builder using explicit source information.
    ///
    /// Source identity is deliberately explicit. The builder never invents
    /// source locations or node identities.
    #[must_use]
    pub fn new(
        allocator: &'a mut NodeIdAllocator,
        span: Span,
        metadata: NodeMetadata,
    ) -> Self {
        Self {
            allocator,
            span,
            metadata,
        }
    }

    /// Creates a builder with default metadata.
    ///
    /// The caller still supplies the source span explicitly.
    #[must_use]
    pub fn without_metadata(
        allocator: &'a mut NodeIdAllocator,
        span: Span,
    ) -> Self {
        Self::new(
            allocator,
            span,
            NodeMetadata::default(),
        )
    }

    /// Returns the source span currently attached to constructed expressions.
    #[must_use]
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Returns the metadata currently attached to constructed expressions.
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        &self.metadata
    }

    /// Replaces the source span used by subsequent constructions.
    ///
    /// Existing expressions are unaffected.
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    /// Replaces metadata used by subsequent constructions.
    ///
    /// Existing expressions are unaffected.
    pub fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = metadata;
    }

    /// Returns the next node ID without consuming it.
    ///
    /// `None` indicates that the allocator is exhausted.
    #[must_use]
    pub fn peek_next_id(&self) -> Option<NodeId> {
        self.allocator.peek()
    }

    /// Allocates a fresh node ID.
    ///
    /// This method is primarily useful when the parser needs to construct
    /// related graph nodes explicitly. It does not create an AST node.
    ///
    /// Prefer the expression construction methods when a complete expression
    /// is being created.
    pub fn allocate_node_id(&mut self) -> ExpressionBuilderResult<NodeId> {
        self.allocator.allocate().map_err(Into::into)
    }

    /// Constructs an expression from the canonical [`ExpressionKind`].
    ///
    /// This is the fundamental construction method.
    ///
    /// All convenience methods in this builder delegate to this method.
    ///
    /// The generic method is deliberately exposed so that the builder does not
    /// need to be modified merely because a new canonical expression variant
    /// is introduced.
    pub fn build(
        &mut self,
        kind: ExpressionKind,
    ) -> ExpressionBuilderResult<Expression> {
        let id = self.allocator.allocate()?;

        let expression = Expression::new(
            id,
            kind,
            self.span.clone(),
            self.metadata.clone(),
        );

        expression.validate_structure()?;

        Ok(expression)
    }

    /// Constructs an expression with explicitly supplied identity.
    ///
    /// This method exists for AST transformation/import infrastructure that has
    /// already established the identity of a node.
    ///
    /// Normal parser construction should prefer [`Self::build`], which obtains
    /// identities from the caller-owned allocator.
    ///
    /// This method does not modify allocator state.
    pub fn build_with_id(
        &self,
        id: NodeId,
        kind: ExpressionKind,
    ) -> ExpressionBuilderResult<Expression> {
        let expression = Expression::new(
            id,
            kind,
            self.span.clone(),
            self.metadata.clone(),
        );

        expression.validate_structure()?;

        Ok(expression)
    }

    // =========================================================================
    // Core leaf expressions
    // =========================================================================

    /// Constructs an identifier expression.
    pub fn identifier(
        &mut self,
        identifier: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Identifier { identifier })
    }

    /// Constructs a literal expression.
    pub fn literal(
        &mut self,
        literal: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Literal { literal })
    }

    // =========================================================================
    // Operators
    // =========================================================================

    /// Constructs a unary expression.
    pub fn unary(
        &mut self,
        operator: OperatorRef,
        operand: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Unary {
            operator,
            operand,
        })
    }

    /// Constructs a binary expression.
    pub fn binary(
        &mut self,
        left: NodeId,
        operator: OperatorRef,
        right: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Binary {
            left,
            operator,
            right,
        })
    }

    /// Constructs an assignment expression.
    pub fn assignment(
        &mut self,
        target: NodeId,
        value: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Assignment {
            target,
            value,
        })
    }

    /// Constructs a compound-assignment expression.
    pub fn compound_assignment(
        &mut self,
        target: NodeId,
        operator: OperatorRef,
        value: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::CompoundAssignment {
            target,
            operator,
            value,
        })
    }

    // =========================================================================
    // Control-flow expressions
    // =========================================================================

    /// Constructs a conditional expression.
    pub fn conditional(
        &mut self,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Conditional {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Constructs a block expression.
    ///
    /// `statements` are retained exactly in caller-provided source order.
    pub fn block(
        &mut self,
        statements: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Block { statements })
    }

    /// Constructs a match expression.
    ///
    /// Match arms are retained exactly in source order.
    pub fn match_expression(
        &mut self,
        scrutinee: NodeId,
        arms: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Match {
            scrutinee,
            arms,
        })
    }

    /// Constructs a loop expression.
    pub fn loop_expression(
        &mut self,
        body: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Loop { body })
    }

    // =========================================================================
    // Callable expressions
    // =========================================================================

    /// Constructs a lambda expression.
    ///
    /// Parameters remain in source order.
    pub fn lambda(
        &mut self,
        parameters: Vec<NodeId>,
        body: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Lambda {
            parameters,
            body,
        })
    }

    /// Constructs a general callable expression.
    ///
    /// This remains generic enough for functions, closures, operations,
    /// resource operations, domain extensions and future callable constructs.
    pub fn call(
        &mut self,
        callee: NodeId,
        arguments: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Call {
            callee,
            arguments,
        })
    }

    // =========================================================================
    // Access expressions
    // =========================================================================

    /// Constructs member/property access.
    pub fn member_access(
        &mut self,
        object: NodeId,
        member: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::MemberAccess {
            object,
            member,
        })
    }

    /// Constructs indexing.
    pub fn index(
        &mut self,
        object: NodeId,
        index: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Index {
            object,
            index,
        })
    }

    /// Constructs a range expression.
    pub fn range(
        &mut self,
        start: Option<NodeId>,
        end: Option<NodeId>,
        inclusive: bool,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Range {
            start,
            end,
            inclusive,
        })
    }

    // =========================================================================
    // Aggregate expressions
    // =========================================================================

    /// Constructs an array/list expression.
    ///
    /// Elements are retained exactly in source order.
    pub fn array(
        &mut self,
        elements: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Array { elements })
    }

    /// Constructs a tuple expression.
    ///
    /// Elements are retained exactly in source order.
    pub fn tuple(
        &mut self,
        elements: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Tuple { elements })
    }

    /// Constructs a struct/object literal expression.
    ///
    /// Fields are retained exactly in source order.
    pub fn struct_expression(
        &mut self,
        type_name: NodeId,
        fields: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Struct {
            type_name,
            fields,
        })
    }

    /// Constructs an object/constructor expression.
    ///
    /// Arguments are retained exactly in source order.
    pub fn construction(
        &mut self,
        type_name: NodeId,
        arguments: Vec<NodeId>,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Construction {
            type_name,
            arguments,
        })
    }

    // =========================================================================
    // Type-related expressions
    // =========================================================================

    /// Constructs a cast expression.
    pub fn cast(
        &mut self,
        expression: NodeId,
        target_type: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Cast {
            expression,
            target_type,
        })
    }

    /// Constructs a type-ascription expression.
    pub fn type_ascription(
        &mut self,
        expression: NodeId,
        type_node: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::TypeAscription {
            expression,
            type_node,
        })
    }

    // =========================================================================
    // Error/concurrency expressions
    // =========================================================================

    /// Constructs a try/error-propagation expression.
    pub fn try_expression(
        &mut self,
        expression: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Try { expression })
    }

    /// Constructs an await expression.
    pub fn await_expression(
        &mut self,
        expression: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Await { expression })
    }

    /// Constructs an async expression.
    pub fn async_expression(
        &mut self,
        body: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Async { body })
    }

    /// Constructs a spawn/concurrency expression.
    pub fn spawn(
        &mut self,
        expression: NodeId,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Spawn { expression })
    }

    // =========================================================================
    // Extensibility
    // =========================================================================

    /// Constructs a namespaced expression extension.
    ///
    /// Domain-specific source constructs should normally use this boundary
    /// rather than adding hardware/vendor-specific concepts to the native AST.
    ///
    /// Examples of possible extension namespaces include future language
    /// domains, provided they remain source-level and are independently
    /// validated/registered downstream.
    pub fn extension(
        &mut self,
        extension: super::super::expressions::expression::ExpressionExtension,
    ) -> ExpressionBuilderResult<Expression> {
        self.build(ExpressionKind::Extension(extension))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> Span {
        Span::default()
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    #[test]
    fn allocates_deterministic_expression_ids() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder =
            ExpressionBuilder::new(&mut allocator, span(), metadata());

        let first = builder
            .identifier(NodeId::new(100).expect("non-zero"))
            .expect("valid identifier");

        let second = builder
            .identifier(NodeId::new(101).expect("non-zero"))
            .expect("valid identifier");

        assert_ne!(first.id(), second.id());
        assert_eq!(first.id().get(), 1);
        assert_eq!(second.id().get(), 2);
    }

    #[test]
    fn preserves_source_span_and_metadata() {
        let mut allocator = NodeIdAllocator::new();

        let source_span = Span::default();
        let source_metadata = NodeMetadata::default();

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                source_span.clone(),
                source_metadata.clone(),
            );

        let expression = builder
            .identifier(NodeId::new(10).expect("non-zero"))
            .expect("valid identifier");

        assert_eq!(expression.span(), &source_span);
        assert_eq!(expression.metadata(), &source_metadata);
    }

    #[test]
    fn preserves_child_order_for_binary_expression() {
        let mut allocator = NodeIdAllocator::new();

        let left = NodeId::new(10).expect("non-zero");
        let right = NodeId::new(20).expect("non-zero");
        let operator =
            OperatorRef::native("add").expect("valid operator");

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                span(),
                metadata(),
            );

        let expression = builder
            .binary(left, operator.clone(), right)
            .expect("valid binary expression");

        match expression.kind() {
            ExpressionKind::Binary {
                left: actual_left,
                operator: actual_operator,
                right: actual_right,
            } => {
                assert_eq!(*actual_left, left);
                assert_eq!(actual_operator, &operator);
                assert_eq!(*actual_right, right);
            }

            other => {
                panic!("unexpected expression kind: {other:?}");
            }
        }
    }

    #[test]
    fn supports_empty_dynamic_collections() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                span(),
                metadata(),
            );

        let array = builder
            .array(Vec::new())
            .expect("empty arrays are structurally representable");

        let tuple = builder
            .tuple(Vec::new())
            .expect("empty tuples are structurally representable");

        assert!(matches!(
            array.kind(),
            ExpressionKind::Array { elements } if elements.is_empty()
        ));

        assert!(matches!(
            tuple.kind(),
            ExpressionKind::Tuple { elements } if elements.is_empty()
        ));
    }

    #[test]
    fn generic_build_supports_the_canonical_kind_surface() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                span(),
                metadata(),
            );

        let identifier =
            NodeId::new(42).expect("non-zero");

        let expression = builder
            .build(ExpressionKind::Identifier { identifier })
            .expect("valid expression");

        assert_eq!(expression.kind(), &ExpressionKind::Identifier {
            identifier,
        });
    }

    #[test]
    fn explicit_id_construction_does_not_consume_allocator_state() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                span(),
                metadata(),
            );

        let explicit_id =
            NodeId::new(900).expect("non-zero");

        let expression = builder
            .build_with_id(
                explicit_id,
                ExpressionKind::Identifier {
                    identifier: NodeId::new(901).expect("non-zero"),
                },
            )
            .expect("valid expression");

        assert_eq!(expression.id(), explicit_id);
        assert_eq!(
            builder.peek_next_id().expect("allocator not exhausted").get(),
            1
        );
    }

    #[test]
    fn invalid_operator_is_returned_without_panic() {
        let mut allocator = NodeIdAllocator::new();

        let invalid_operator =
            OperatorRef::new("", "").expect_err("empty operator must fail");

        assert_eq!(
            invalid_operator,
            ExpressionError::EmptyOperatorName
        );

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                span(),
                metadata(),
            );

        let result = builder.unary(
            invalid_operator.into(),
            NodeId::new(2).expect("non-zero"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn allocation_is_checked() {
        let mut allocator =
            NodeIdAllocator::starting_at(
                NodeId::new(u64::MAX).expect("non-zero"),
            );

        let mut builder =
            ExpressionBuilder::new(
                &mut allocator,
                span(),
                metadata(),
            );

        let first = builder
            .identifier(NodeId::new(2).expect("non-zero"))
            .expect("last ID is representable");

        assert_eq!(first.id().get(), u64::MAX);

        let second = builder.identifier(
            NodeId::new(3).expect("non-zero"),
        );

        assert!(matches!(
            second,
            Err(ExpressionBuilderError::Allocation(
                NodeIdAllocationError::Exhausted
            ))
        ));
    }
}