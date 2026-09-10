//! # Zamani Frontend AST — Statement Builder
//!
//! `src/frontend/ast/node/builders/statement_builder.rs`
//!
//! Production construction API for the canonical source-level
//! [`Statement`](super::super::statements::statement::Statement) AST node.
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
//! StatementBuilder
//!      │
//!      ▼
//! Native Zamani AST
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
//!      ├──────────────┬──────────────┬──────────────┐
//!      ▼              ▼              ▼              ▼
//! classical        quantum          HDL          future
//! domain IR        domain IR      domain IR      domains
//! ```
//!
//! ## Purpose
//!
//! `StatementBuilder` owns construction of one canonical source-level
//! [`Statement`].
//!
//! It provides:
//!
//! - deterministic statement-node ID allocation;
//! - explicit source-span preservation;
//! - explicit metadata preservation;
//! - canonical `StatementKind` construction;
//! - structural validation through the existing `Statement` implementation;
//! - convenient constructors for every currently supported statement kind;
//! - generic extension-statement construction;
//! - configurable structural validation;
//! - no machine-specific assumptions;
//! - no quantum-specific assumptions;
//! - no backend assumptions.
//!
//! ## What this file does NOT own
//!
//! This builder does not own:
//!
//! - AST graph storage;
//! - declarations;
//! - expressions;
//! - patterns;
//! - types;
//! - semantic analysis;
//! - symbol resolution;
//! - resource allocation;
//! - capability resolution;
//! - domain resolution;
//! - ZUIR;
//! - quantum IR;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - hardware;
//! - backend selection;
//! - runtime execution.
//!
//! ## Canonical representation
//!
//! There is exactly one statement representation:
//!
//! ```text
//! Statement
//! ├── Node
//! │   ├── NodeId
//! │   ├── NodeKind
//! │   ├── Span
//! │   └── NodeMetadata
//! │
//! └── StatementKind
//!     └── NodeId references
//! ```
//!
//! This builder does not introduce a parallel statement representation.
//!
//! ## POCO-REAF
//!
//! The builder has no knowledge of:
//!
//! - number of qubits;
//! - number of registers;
//! - number of CPUs;
//! - number of GPUs;
//! - number of QPUs;
//! - machine topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - gate set;
//! - simulator;
//! - physical resource assignment.
//!
//! Consequently, a statement can participate in a program that is eventually
//! compiled for any supported computational realization.
//!
//! ```text
//! Program_Once
//!      │
//!      ▼
//! source-level Statement
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! target/resource discovery
//!      │
//!      ▼
//! target realization
//! ```
//!
//! ## Scalability
//!
//! No language-level maximum is imposed here.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_STATEMENTS
//! MAX_CHILDREN
//! MAX_QUBITS
//! MAX_REGISTERS
//! MAX_OPERANDS
//! MAX_MACHINE_SIZE
//! ```
//!
//! Dynamically sized statement collections are represented by the canonical
//! `StatementKind` using `Vec<NodeId>` where required.
//!
//! Operational limits for hostile or resource-constrained compilation belong
//! to explicit compiler/session policies.
//!
//! ## Determinism
//!
//! Determinism is provided by the caller-owned [`NodeIdAllocator`].
//!
//! This builder does not use:
//!
//! - global mutable counters;
//! - random identifiers;
//! - timestamps;
//! - memory addresses;
//! - thread IDs;
//! - hash-map iteration order.
//!
//! The builder preserves all supplied child IDs in the exact order defined by
//! the corresponding `StatementKind`.
//!
//! ## Ownership
//!
//! The builder temporarily borrows the caller's allocator:
//!
//! ```text
//! parser/compiler session
//!        │
//!        ▼
//! NodeIdAllocator
//!        │
//!        ▼
//! StatementBuilder
//!        │
//!        ▼
//! Statement
//! ```
//!
//! Child nodes are NOT owned by this builder. Their `NodeId`s refer to nodes
//! stored by the enclosing AST graph.
//!
//! ## Important allocator rule
//!
//! While a `StatementBuilder` exists, the same allocator cannot be mutably
//! borrowed elsewhere. This is intentional Rust ownership behavior and prevents
//! concurrent mutation of the deterministic allocation state.
//!
//! Callers that need child IDs should therefore:
//!
//! 1. construct/allocate child nodes before creating the builder; or
//! 2. construct the complete child node through another builder/session API;
//! 3. then pass its already allocated `NodeId` to this builder.
//!
//! The builder never manufactures fake child nodes.
//!
//! ## Validation boundary
//!
//! Construction validates local statement invariants through the canonical
//! [`Statement`] API.
//!
//! Graph-wide validation remains outside this file.
//!
//! ```text
//! StatementBuilder
//!       │
//!       ▼
//! Statement local validation
//!       │
//!       ▼
//! AST graph validation
//!       │
//!       ▼
//! semantic validation
//! ```
//!
//! The builder does not resolve whether a `NodeId` actually exists in the
//! enclosing AST graph.
//!
//! ## Semantic boundary
//!
//! No semantic information is resolved here.
//!
//! In particular this builder does not determine:
//!
//! - whether an expression has a particular type;
//! - whether a binding is legal;
//! - whether a loop is terminating;
//! - whether a quantum resource is available;
//! - whether a capability exists;
//! - whether an operation is supported by a target.
//!
//! Those decisions belong to semantic and downstream compilation phases.
//!
//! ## ZUIR boundary
//!
//! There is intentionally no ZUIR dependency.
//!
//! The pipeline remains:
//!
//! ```text
//! StatementBuilder
//!      ▼
//! Statement AST
//!      ▼
//! structural validation
//!      ▼
//! semantic model
//!      ▼
//! ZUIR
//!      ▼
//! domain IR
//!      ▼
//! target IR
//! ```
//!
//! ## Quantum boundary
//!
//! Quantum computation uses the same statement construction infrastructure as
//! other computational domains.
//!
//! The builder does not contain variants such as:
//!
//! ```text
//! Hadamard
//! CNOT
//! MeasureQubit
//! ResetQubit
//! ```
//!
//! Quantum operations are represented by the appropriate expression/resource/
//! extension structures and referenced by ordinary source-level statements.
//!
//! Adding a new quantum operation, technology, backend, topology, QEC strategy,
//! or execution model therefore does not require changing this file.
//!
//! ## Extension statements
//!
//! `StatementKind::Extension` is the canonical escape point for genuinely
//! source-level extension constructs.
//!
//! The builder preserves:
//!
//! - extension namespace;
//! - extension name;
//! - ordered child references.
//!
//! It does not interpret the extension payload.
//!
//! ## Serialization
//!
//! The builder itself is not serialized.
//!
//! The constructed [`Statement`] owns the canonical serializable representation
//! through the existing Serde implementation.
//!
//! No second serialization format is introduced here.
//!
//! ## Visitor/traversal integration
//!
//! This builder does not implement traversal.
//!
//! The constructed `Statement` exposes its child `NodeId`s through the existing
//! `Statement::child_node_ids` API, while canonical AST traversal remains owned
//! by the AST traversal subsystem.
//!
//! ## Parser integration
//!
//! The parser may use this builder after syntactic recognition:
//!
//! ```text
//! lexer
//!   │
//!   ▼
//! parser
//!   │
//!   ├── allocate/reference child NodeIds
//!   │
//!   ▼
//! StatementBuilder
//!   │
//!   ▼
//! Statement
//! ```
//!
//! The parser must not use this builder to perform semantic or target analysis.
//!
//! ## File completion contract
//!
//! This file is complete when:
//!
//! - every existing `StatementKind` has a construction path;
//! - statement IDs are allocated deterministically;
//! - source spans are preserved;
//! - metadata is preserved;
//! - extension identities are validated;
//! - local statement invariants are validated;
//! - no fixed machine/resource limits exist;
//! - no unsafe code exists;
//! - no downstream compiler dependency exists;
//! - tests cover all construction families;
//! - the resulting `Statement` is the canonical repository representation.
//!
//! Adding a new statement variant requires an intentional change to the
//! canonical `StatementKind` itself. Adding a new quantum backend or hardware
//! target must not require changing this builder.
//!
//! ## Allowed dependencies
//!
//! ```text
//! super::super::node
//! super::super::node_id
//! super::super::node_kind
//! super::super::statements::statement
//! standard library
//! ```
//!
//! Forbidden dependencies include:
//!
//! ```text
//! semantic
//! compiler driver
//! ZUIR
//! quantum::ir
//! hardware
//! optimizer
//! router
//! scheduler
//! QEC
//! resilience
//! runtime
//! backend providers
//! QIR
//! LLVM
//! MLIR
//! ```
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

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::super::node::Node;
use super::super::node_id::{NodeId, NodeIdAllocationError, NodeIdAllocator};
use super::super::node_kind::{NodeKind, NodeKindError};
use super::super::statements::statement::{
    Statement,
    StatementError,
    StatementKind,
    StatementResult,
    StatementValidationPolicy,
};

/// Result type for statement-builder operations.
pub type StatementBuilderResult<T> = Result<T, StatementBuilderError>;

/// Errors produced by [`StatementBuilder`].
///
/// Errors retain their original structured representation instead of being
/// converted into strings.
#[derive(Debug)]
#[non_exhaustive]
pub enum StatementBuilderError {
    /// The shared deterministic AST node-ID allocator could not allocate an
    /// identity.
    Allocation(NodeIdAllocationError),

    /// The requested node kind could not be constructed.
    NodeKind(NodeKindError),

    /// The resulting statement violated its local structural contract.
    Statement(StatementError),
}

impl fmt::Display for StatementBuilderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation(error) => {
                write!(
                    formatter,
                    "statement builder node-ID allocation failed: {error}"
                )
            }

            Self::NodeKind(error) => {
                write!(
                    formatter,
                    "statement builder node-kind construction failed: {error}"
                )
            }

            Self::Statement(error) => {
                write!(
                    formatter,
                    "statement construction failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for StatementBuilderError {}

impl From<NodeIdAllocationError> for StatementBuilderError {
    #[inline]
    fn from(error: NodeIdAllocationError) -> Self {
        Self::Allocation(error)
    }
}

impl From<NodeKindError> for StatementBuilderError {
    #[inline]
    fn from(error: NodeKindError) -> Self {
        Self::NodeKind(error)
    }
}

impl From<StatementError> for StatementBuilderError {
    #[inline]
    fn from(error: StatementError) -> Self {
        Self::Statement(error)
    }
}

/// Production builder for the canonical source-level [`Statement`].
///
/// The builder owns no child AST nodes. It only allocates the identity of the
/// statement being constructed and assembles the already-existing child
/// identities supplied by the caller.
///
/// # Invariant
///
/// A successfully returned statement has:
///
/// - a valid allocated `NodeId`;
/// - a caller-supplied source span;
/// - caller-supplied metadata;
/// - a `NodeKind` consistent with `StatementKind`;
/// - locally valid child references.
pub struct StatementBuilder<'a> {
    allocator: &'a mut NodeIdAllocator,
}

impl<'a> StatementBuilder<'a> {
    /// Creates a statement builder backed by the caller-owned allocator.
    ///
    /// The allocator remains owned by the caller after this builder is dropped
    /// or consumed.
    #[inline]
    pub fn new(allocator: &'a mut NodeIdAllocator) -> Self {
        Self { allocator }
    }

    /// Allocates the identity for a new statement.
    ///
    /// This method exists for integrations that need to construct a concrete
    /// statement using the exact same allocation stream while retaining the
    /// construction logic in this builder.
    ///
    /// The returned ID does not itself create an AST node.
    #[inline]
    pub fn allocate_id(&mut self) -> StatementBuilderResult<NodeId> {
        Ok(self.allocator.allocate()?)
    }

    /// Constructs a statement from a caller-supplied [`StatementKind`].
    ///
    /// The builder derives the corresponding native `NodeKind` from the
    /// statement kind. This prevents the caller from accidentally supplying a
    /// mismatching classification.
    ///
    /// The statement ID is allocated from the shared deterministic allocator.
    pub fn build(
        &mut self,
        kind: StatementKind,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        let id = self.allocate_id()?;

        self.build_with_id(id, kind, span, metadata)
    }

    /// Constructs a statement using an already allocated statement ID.
    ///
    /// This is useful when a parser or AST graph builder must allocate the ID
    /// before entering another construction phase.
    ///
    /// The supplied ID is never reallocated.
    pub fn build_with_id(
        &self,
        id: NodeId,
        kind: StatementKind,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        let node_kind = if kind.is_extension() {
            let (namespace, name) = match &kind {
                StatementKind::Extension {
                    namespace,
                    name,
                    ..
                } => (namespace, name),

                _ => unreachable!(
                    "extension classification must correspond to Extension"
                ),
            };

            NodeKind::extension(namespace.clone(), name.clone())?
        } else {
            NodeKind::core(kind.core_node_kind())
        };

        let node = Node::new(
            id,
            node_kind,
            span,
            metadata,
        );

        Ok(Statement::from_node(node, kind)?)
    }

    /// Constructs an expression statement.
    pub fn expression(
        &mut self,
        expression: NodeId,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Expression { expression },
            span,
            metadata,
        )
    }

    /// Constructs a `let` statement.
    pub fn let_binding(
        &mut self,
        pattern: NodeId,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Let {
                pattern,
                type_annotation,
                initializer,
            },
            span,
            metadata,
        )
    }

    /// Constructs a `const` statement.
    pub fn const_binding(
        &mut self,
        pattern: NodeId,
        type_annotation: Option<NodeId>,
        initializer: NodeId,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Const {
                pattern,
                type_annotation,
                initializer,
            },
            span,
            metadata,
        )
    }

    /// Constructs a `return` statement.
    pub fn return_statement(
        &mut self,
        value: Option<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Return { value },
            span,
            metadata,
        )
    }

    /// Constructs a `break` statement.
    pub fn break_statement(
        &mut self,
        label: Option<NodeId>,
        value: Option<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Break {
                label,
                value,
            },
            span,
            metadata,
        )
    }

    /// Constructs a `continue` statement.
    pub fn continue_statement(
        &mut self,
        label: Option<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Continue { label },
            span,
            metadata,
        )
    }

    /// Constructs an `if` statement.
    pub fn if_statement(
        &mut self,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::If {
                condition,
                then_branch,
                else_branch,
            },
            span,
            metadata,
        )
    }

    /// Constructs a `while` statement.
    pub fn while_statement(
        &mut self,
        condition: NodeId,
        body: NodeId,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::While {
                condition,
                body,
            },
            span,
            metadata,
        )
    }

    /// Constructs a `for` statement.
    pub fn for_statement(
        &mut self,
        pattern: NodeId,
        iterable: NodeId,
        body: NodeId,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::For {
                pattern,
                iterable,
                body,
            },
            span,
            metadata,
        )
    }

    /// Constructs a statement block.
    ///
    /// The supplied `statements` are retained in exactly the supplied order.
    ///
    /// No fixed block size is imposed.
    pub fn block(
        &mut self,
        statements: Vec<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Block { statements },
            span,
            metadata,
        )
    }

    /// Constructs a match statement.
    ///
    /// Match-arm ordering is preserved exactly.
    pub fn match_statement(
        &mut self,
        scrutinee: NodeId,
        arms: Vec<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Match {
                scrutinee,
                arms,
            },
            span,
            metadata,
        )
    }

    /// Constructs a declaration-in-statement-position.
    pub fn declaration(
        &mut self,
        declaration: NodeId,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Declaration { declaration },
            span,
            metadata,
        )
    }

    /// Constructs a parser-recovery/error statement.
    ///
    /// This representation preserves recovery structure but does not assign
    /// semantic meaning to malformed source.
    pub fn error(
        &mut self,
        recovered: Option<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Error { recovered },
            span,
            metadata,
        )
    }

    /// Constructs an extensible source-level statement.
    ///
    /// The namespace and name are validated by the canonical `NodeKind`
    /// extension infrastructure and by `StatementKind` structural validation.
    ///
    /// Child order is preserved.
    pub fn extension(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
        children: Vec<NodeId>,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
    ) -> StatementBuilderResult<Statement> {
        self.build(
            StatementKind::Extension {
                namespace: namespace.into(),
                name: name.into(),
                children,
            },
            span,
            metadata,
        )
    }

    /// Constructs a statement and validates it using an explicit policy.
    ///
    /// This is the preferred entry point when compiler infrastructure has
    /// already established a resource/security policy.
    pub fn build_with_policy(
        &mut self,
        kind: StatementKind,
        span: super::super::source::Span,
        metadata: super::super::metadata::NodeMetadata,
        policy: StatementValidationPolicy,
    ) -> StatementBuilderResult<Statement> {
        let statement = self.build(
            kind,
            span,
            metadata,
        )?;

        statement.validate_structure(policy)?;

        Ok(statement)
    }

    /// Returns the allocator's next candidate node ID without allocating it.
    ///
    /// `None` indicates that the ID space is exhausted.
    #[inline]
    #[must_use]
    pub fn next_node_id(&self) -> Option<NodeId> {
        self.allocator.peek()
    }

    /// Returns the number of IDs allocated from an allocator that started at
    /// the canonical first ID.
    ///
    /// This is informational and does not impose a semantic program limit.
    #[inline]
    #[must_use]
    pub fn allocated_node_count(&self) -> u64 {
        self.allocator.allocated_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> super::super::source::Span {
        super::super::source::Span::default()
    }

    fn metadata() -> super::super::metadata::NodeMetadata {
        super::super::metadata::NodeMetadata::default()
    }

    fn child_ids(
        allocator: &mut NodeIdAllocator,
        count: usize,
    ) -> Vec<NodeId> {
        (0..count)
            .map(|_| allocator.allocate().expect("child ID allocation"))
            .collect()
    }

    #[test]
    fn builds_expression_statement() {
        let mut allocator = NodeIdAllocator::new();

        let expression = allocator
            .allocate()
            .expect("expression ID");

        let mut builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .expression(
                expression,
                span(),
                metadata(),
            )
            .expect("expression statement");

        assert_eq!(statement.id().get(), 2);
        assert_eq!(
            statement.kind(),
            &StatementKind::Expression { expression }
        );
    }

    #[test]
    fn builds_all_native_statement_families() {
        let mut allocator = NodeIdAllocator::new();

        let ids = child_ids(&mut allocator, 8);

        let mut builder = StatementBuilder::new(&mut allocator);

        let _ = builder
            .expression(ids[0], span(), metadata())
            .expect("expression");

        let _ = builder
            .let_binding(
                ids[0],
                Some(ids[1]),
                Some(ids[2]),
                span(),
                metadata(),
            )
            .expect("let");

        let _ = builder
            .const_binding(
                ids[0],
                Some(ids[1]),
                ids[2],
                span(),
                metadata(),
            )
            .expect("const");

        let _ = builder
            .return_statement(
                Some(ids[0]),
                span(),
                metadata(),
            )
            .expect("return");

        let _ = builder
            .break_statement(
                Some(ids[0]),
                Some(ids[1]),
                span(),
                metadata(),
            )
            .expect("break");

        let _ = builder
            .continue_statement(
                Some(ids[0]),
                span(),
                metadata(),
            )
            .expect("continue");

        let _ = builder
            .if_statement(
                ids[0],
                ids[1],
                Some(ids[2]),
                span(),
                metadata(),
            )
            .expect("if");

        let _ = builder
            .while_statement(
                ids[0],
                ids[1],
                span(),
                metadata(),
            )
            .expect("while");

        let _ = builder
            .for_statement(
                ids[0],
                ids[1],
                ids[2],
                span(),
                metadata(),
            )
            .expect("for");

        let _ = builder
            .block(
                vec![ids[0], ids[1], ids[2]],
                span(),
                metadata(),
            )
            .expect("block");

        let _ = builder
            .match_statement(
                ids[0],
                vec![ids[1], ids[2]],
                span(),
                metadata(),
            )
            .expect("match");

        let _ = builder
            .declaration(
                ids[0],
                span(),
                metadata(),
            )
            .expect("declaration");

        let _ = builder
            .error(
                Some(ids[0]),
                span(),
                metadata(),
            )
            .expect("error");

        let _ = builder
            .extension(
                "example",
                "custom_statement",
                vec![ids[0], ids[1]],
                span(),
                metadata(),
            )
            .expect("extension");
    }

    #[test]
    fn allocates_deterministically() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = StatementBuilder::new(&mut allocator);

        let first = builder
            .return_statement(
                None,
                span(),
                metadata(),
            )
            .expect("first statement");

        let second = builder
            .return_statement(
                None,
                span(),
                metadata(),
            )
            .expect("second statement");

        assert_eq!(first.id().get(), 1);
        assert_eq!(second.id().get(), 2);
    }

    #[test]
    fn preserves_dynamic_child_order() {
        let mut allocator = NodeIdAllocator::new();

        let ids = child_ids(&mut allocator, 5);

        let mut builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .block(
                vec![ids[4], ids[1], ids[3], ids[0], ids[2]],
                span(),
                metadata(),
            )
            .expect("block");

        assert_eq!(
            statement.child_node_ids(),
            vec![ids[4], ids[1], ids[3], ids[0], ids[2]]
        );
    }

    #[test]
    fn supports_empty_blocks() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .block(
                Vec::new(),
                span(),
                metadata(),
            )
            .expect("empty block");

        assert!(statement.child_node_ids().is_empty());
        assert_eq!(statement.child_count(), 0);
    }

    #[test]
    fn supports_empty_optional_control_values() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = StatementBuilder::new(&mut allocator);

        let return_statement = builder
            .return_statement(
                None,
                span(),
                metadata(),
            )
            .expect("return");

        let break_statement = builder
            .break_statement(
                None,
                None,
                span(),
                metadata(),
            )
            .expect("break");

        let continue_statement = builder
            .continue_statement(
                None,
                span(),
                metadata(),
            )
            .expect("continue");

        assert_eq!(return_statement.child_count(), 0);
        assert_eq!(break_statement.child_count(), 0);
        assert_eq!(continue_statement.child_count(), 0);
    }

    #[test]
    fn supports_extension_statements_without_core_changes() {
        let mut allocator = NodeIdAllocator::new();

        let child = allocator
            .allocate()
            .expect("child ID");

        let mut builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .extension(
                "zamani.example",
                "future_operation",
                vec![child],
                span(),
                metadata(),
            )
            .expect("extension statement");

        assert!(statement.is_extension());
        assert_eq!(
            statement.child_node_ids(),
            vec![child]
        );
    }

    #[test]
    fn rejects_empty_extension_namespace() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = StatementBuilder::new(&mut allocator);

        let result = builder.extension(
            "",
            "operation",
            Vec::new(),
            span(),
            metadata(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_extension_name() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = StatementBuilder::new(&mut allocator);

        let result = builder.extension(
            "zamani.example",
            "",
            Vec::new(),
            span(),
            metadata(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn explicit_id_does_not_consume_allocator() {
        let mut allocator = NodeIdAllocator::new();

        let supplied_id = allocator
            .allocate()
            .expect("supplied ID");

        let builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .build_with_id(
                supplied_id,
                StatementKind::Return { value: None },
                span(),
                metadata(),
            )
            .expect("statement");

        assert_eq!(statement.id(), supplied_id);
        assert_eq!(
            builder.next_node_id().map(NodeId::get),
            Some(2)
        );
    }

    #[test]
    fn build_with_policy_uses_caller_policy() {
        let mut allocator = NodeIdAllocator::new();

        let ids = child_ids(&mut allocator, 2);

        let mut builder = StatementBuilder::new(&mut allocator);

        let result = builder.build_with_policy(
            StatementKind::Block {
                statements: vec![ids[0], ids[1]],
            },
            span(),
            metadata(),
            StatementValidationPolicy {
                max_children: Some(1),
                ..StatementValidationPolicy::unrestricted()
            },
        );

        assert!(matches!(
            result,
            Err(StatementBuilderError::Statement(
                StatementError::LimitExceeded {
                    limit: "max_children",
                    actual: 2,
                    maximum: 1,
                }
            ))
        );
    }

    #[test]
    fn unrestricted_policy_does_not_impose_statement_size() {
        let mut allocator = NodeIdAllocator::new();

        let ids = child_ids(&mut allocator, 1024);

        let mut builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .build_with_policy(
                StatementKind::Block {
                    statements: ids.clone(),
                },
                span(),
                metadata(),
                StatementValidationPolicy::unrestricted(),
            )
            .expect("large block");

        assert_eq!(statement.child_count(), ids.len());
    }

    #[test]
    fn allocator_remains_available_after_builder_scope() {
        let mut allocator = NodeIdAllocator::new();

        {
            let mut builder = StatementBuilder::new(&mut allocator);

            builder
                .return_statement(
                    None,
                    span(),
                    metadata(),
                )
                .expect("statement");
        }

        let next = allocator
            .allocate()
            .expect("allocator remains usable");

        assert_eq!(next.get(), 2);
    }

    #[test]
    fn explicit_child_ids_are_not_reallocated() {
        let mut allocator = NodeIdAllocator::new();

        let child = allocator
            .allocate()
            .expect("child ID");

        let mut builder = StatementBuilder::new(&mut allocator);

        let statement = builder
            .expression(
                child,
                span(),
                metadata(),
            )
            .expect("statement");

        assert_eq!(
            statement.child_node_ids(),
            vec![child]
        );

        // Root statement gets the next ID; the child ID is never silently
        // replaced with a new identity.
        assert_eq!(statement.id().get(), 3);
    }
}