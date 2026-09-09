//! # Zamani Native AST — Binary Expression
//!
//! Canonical source-level representation of a binary/infix expression.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! Native Zamani AST
//!     │
//!     └── BinaryExpression  ← this module
//!             │
//!             ▼
//!     structural validation
//!             │
//!             ▼
//!     semantic analysis
//!             │
//!             ▼
//!     semantic model
//!             │
//!             ▼
//!           ZUIR
//!             │
//!       ┌─────┼─────┐
//!       ▼     ▼     ▼
//!    classical quantum future
//!       IR      IR    domains
//! ```
//!
//! ## Purpose
//!
//! `BinaryExpression` represents a source-level binary/infix expression:
//!
//! ```text
//! left <operator> right
//! ```
//!
//! It describes source structure only.
//!
//! It does NOT determine:
//!
//! - the operand types;
//! - overload resolution;
//! - whether the operator is arithmetic;
//! - whether the operator is logical;
//! - whether the operator is quantum;
//! - whether the operator acts on resources;
//! - hardware instructions;
//! - quantum gates;
//! - qubit mappings;
//! - physical topology;
//! - scheduling;
//! - routing;
//! - calibration;
//! - error correction;
//! - resilience;
//! - backend selection;
//! - execution strategy.
//!
//! Those concerns belong to later compiler phases.
//!
//! ## POCO-REAF
//!
//! Binary expressions contain no machine-size or hardware assumptions.
//!
//! Consequently, the same source representation can participate in programs
//! compiled for different:
//!
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - QPUs;
//! - simulators;
//! - distributed systems;
//! - heterogeneous systems;
//! - future computational systems.
//!
//! No qubit count, register width, machine size, topology, vendor, backend,
//! instruction set, or device identifier is represented here.
//!
//! ## Child representation
//!
//! Children are represented by [`NodeId`].
//!
//! The binary expression therefore does not recursively own its operands.
//!
//! ```text
//! BinaryExpression
//! ├── Node
//! ├── left: NodeId
//! ├── operator: OperatorRef
//! └── right: NodeId
//! ```
//!
//! The AST graph owns the actual child nodes.
//!
//! This avoids recursive Rust ownership structures and permits the global AST
//! traversal layer to choose an iterative traversal strategy for arbitrarily
//! deep source programs.
//!
//! ## Operator representation
//!
//! Operators are represented by [`OperatorRef`] rather than lexer token types.
//!
//! This is essential because the AST must not depend on the lexer.
//!
//! An operator is identified by a namespace and name:
//!
//! ```text
//! namespace:name
//! ```
//!
//! Examples of source-level identities could include:
//!
//! ```text
//! zamani:add
//! zamani:subtract
//! zamani:multiply
//! zamani:equal
//! zamani:logical-and
//! ```
//!
//! The binary-expression node does not assign semantic meaning to these names.
//!
//! A semantic analyzer decides what an operator means in a particular context.
//!
//! This also permits future extensions without changing this structure.
//!
//! ## Quantum neutrality
//!
//! The node deliberately does NOT contain:
//!
//! ```text
//! enum QuantumBinaryOperator {
//!     CNOT,
//!     CZ,
//!     ...
//! }
//! ```
//!
//! A quantum operation requiring binary source structure can instead be
//! represented through generic language operators or a namespaced extension.
//!
//! Quantum-specific interpretation remains downstream.
//!
//! ## Legacy migration
//!
//! The legacy AST represented infix expressions using lexer-level
//! `TokenType` values and recursively embedded `Box<Expression>` operands.
//!
//! That representation is intentionally not reproduced.
//!
//! The canonical design is:
//!
//! ```text
//! legacy:
//! Infix(Span, Box<Expression>, TokenType, Box<Expression>)
//!
//! canonical:
//! BinaryExpression {
//!     node,
//!     left,
//!     operator,
//!     right,
//! }
//! ```
//!
//! The parser is responsible for translating lexical tokens into the
//! source-level [`OperatorRef`] representation.
//!
//! ## Structural validation boundary
//!
//! This module validates only local structural invariants.
//!
//! It does not perform:
//!
//! - type checking;
//! - name resolution;
//! - operator overload resolution;
//! - constant folding;
//! - algebraic simplification;
//! - borrow checking;
//! - ownership checking;
//! - capability checking;
//! - resource allocation;
//! - quantum legality checking;
//! - hardware compatibility;
//! - target selection.
//!
//! ## Traversal
//!
//! `child_node_ids()` returns the two direct child references.
//!
//! It never recursively visits children.
//!
//! Global traversal infrastructure is responsible for recursively or
//! iteratively walking the AST graph.
//!
//! ## Scalability
//!
//! This module introduces no artificial limits on:
//!
//! - number of binary expressions;
//! - number of AST nodes;
//! - program size;
//! - nesting depth;
//! - resource count;
//! - qubit count;
//! - register size;
//! - machine size.
//!
//! `NodeId` is the only child reference representation.
//!
//! Any resource limits required for hostile input protection belong to the
//! compiler's configurable validation policy.
//!
//! ## Determinism
//!
//! The structure contains only deterministic values.
//!
//! Child ordering is always:
//!
//! ```text
//! left, right
//! ```
//!
//! No hash-map iteration is used.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization.
//!
//! Serialization schema versioning belongs to the AST serialization subsystem.
//! This module does not invent a competing global serialization protocol.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe` code;
//! - performs no I/O;
//! - executes no user code;
//! - dereferences no raw pointers;
//! - performs no unchecked indexing;
//! - performs no recursive traversal;
//! - performs no backend calls;
//! - performs no global mutable-state access.
//!
//! Empty or invalid operator identities are rejected during construction.
//! Child existence is verified by the AST graph validator because this node
//! intentionally stores only `NodeId` references.
//!
//! ## Thread safety
//!
//! `BinaryExpression` contains no global mutable state.
//!
//! Immutable instances can therefore be shared across compiler phases when
//! their constituent types are `Send + Sync`.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! ## `node.rs`
//!
//! `Node` owns:
//!
//! - `NodeId`;
//! - `NodeKind`;
//! - `Span`;
//! - `NodeMetadata`.
//!
//! `BinaryExpression` embeds a `Node` and therefore does not duplicate those
//! fields.
//!
//! ## `node_id.rs`
//!
//! `NodeId` identifies the left and right operand nodes.
//!
//! A `NodeId` is not a symbol ID, resource ID, qubit ID, hardware ID, or
//! semantic value.
//!
//! ## `node_kind.rs`
//!
//! The canonical node kind is:
//!
//! ```text
//! CoreNodeKind::BinaryExpression
//! ```
//!
//! This module does not define a second binary-expression taxonomy.
//!
//! ## `source`
//!
//! `Node` owns the source span.
//!
//! The binary expression's span should normally cover the complete source
//! expression:
//!
//! ```text
//! left <operator> right
//! ^------------------^
//! ```
//!
//! ## Parser
//!
//! The parser must:
//!
//! 1. parse the left operand;
//! 2. parse the operator token;
//! 3. convert the token to a source-level operator identity;
//! 4. parse the right operand;
//! 5. allocate the binary expression's `NodeId`;
//! 6. construct this node.
//!
//! The parser must not perform semantic operator resolution.
//!
//! ## Structural validation
//!
//! The AST validation layer must verify that:
//!
//! - the node kind is `BinaryExpression`;
//! - the node ID is valid according to the AST graph policy;
//! - the left child exists;
//! - the right child exists;
//! - the child references are structurally valid;
//! - the operator identity is structurally valid;
//! - source relationships satisfy the global AST span policy.
//!
//! ## Semantic analysis
//!
//! Semantic analysis resolves:
//!
//! ```text
//! operator namespace/name
//!          │
//!          ▼
//! semantic operator identity
//!          │
//!          ▼
//! overload resolution
//!          │
//!          ▼
//! operand/result types
//!          │
//!          ▼
//! effects/capabilities/resources
//! ```
//!
//! This module must not perform any of those operations.
//!
//! ## ZUIR
//!
//! ZUIR lowering consumes the semantic meaning of the expression.
//!
//! `binary.rs` must not import ZUIR.
//!
//! The semantic lowering layer decides whether a binary expression becomes:
//!
//! - classical computation;
//! - resource computation;
//! - quantum-domain semantics;
//! - hybrid computation;
//! - another registered computational domain.
//!
//! ## Quantum integration
//!
//! Quantum frontends may use this node for source expressions involving
//! quantum-related values when the language grammar defines such syntax.
//!
//! No quantum-specific type or operation is stored here.
//!
//! Quantum external formats such as OpenQASM retain their own ASTs and lower
//! independently into Zamani's semantic representation.
//!
//! ## Visitors
//!
//! Visitors should treat the binary expression as a node with exactly two
//! direct children:
//!
//! ```text
//! left
//! right
//! ```
//!
//! Visitor implementations should obtain these references through
//! `child_node_ids()` rather than depending on private storage.
//!
//! ## No-re-edit integration guarantee
//!
//! This file's public contract deliberately depends only on foundational AST
//! types and its own operator identity.
//!
//! Changes to:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - backends;
//! - target architectures
//!
//! must not require modification of this file.
//!
//! A new semantic interpretation of an existing operator is therefore a
//! downstream change, not an AST change.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

// ============================================================================
// Schema
// ============================================================================

/// Schema version for the binary-expression contract.
///
/// This is deliberately independent from:
//!
//! - Zamani language version;
//! - complete AST schema version;
//! - serialization format version;
//! - compiler version.
//!
//! The global AST serialization layer owns compatibility policy.
pub const BINARY_EXPRESSION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Errors
// ============================================================================

/// Structural errors produced by binary-expression construction or validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinaryExpressionError {
    /// The supplied common node does not have the binary-expression kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The left operand reference is invalid.
    InvalidLeftOperand {
        /// Supplied node ID.
        id: NodeId,
    },

    /// The right operand reference is invalid.
    InvalidRightOperand {
        /// Supplied node ID.
        id: NodeId,
    },

    /// The left and right operands refer to the same AST node.
    ///
    /// This is structurally invalid for a normal binary-expression AST edge.
    /// If a language needs aliasing or self-reference semantics, those belong
    /// to the AST graph/semantic model rather than being silently inferred here.
    SameOperand {
        /// Shared operand ID.
        id: NodeId,
    },

    /// Operator namespace is empty.
    EmptyOperatorNamespace,

    /// Operator name is empty.
    EmptyOperatorName,

    /// Operator namespace contains an invalid Unicode/NUL character.
    InvalidOperatorNamespace,

    /// Operator name contains an invalid Unicode/NUL character.
    InvalidOperatorName,
}

impl fmt::Display for BinaryExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "binary expression has invalid node kind: {actual}"
                )
            }

            Self::InvalidLeftOperand { id } => {
                write!(
                    formatter,
                    "binary expression has invalid left operand node id: {id:?}"
                )
            }

            Self::InvalidRightOperand { id } => {
                write!(
                    formatter,
                    "binary expression has invalid right operand node id: {id:?}"
                )
            }

            Self::SameOperand { id } => {
                write!(
                    formatter,
                    "binary expression uses the same operand node for both sides: {id:?}"
                )
            }

            Self::EmptyOperatorNamespace => {
                formatter.write_str("binary expression operator namespace is empty")
            }

            Self::EmptyOperatorName => {
                formatter.write_str("binary expression operator name is empty")
            }

            Self::InvalidOperatorNamespace => {
                formatter.write_str(
                    "binary expression operator namespace contains a NUL character",
                )
            }

            Self::InvalidOperatorName => {
                formatter.write_str(
                    "binary expression operator name contains a NUL character",
                )
            }
        }
    }
}

impl std::error::Error for BinaryExpressionError {}

/// Result type for binary-expression operations.
pub type BinaryExpressionResult<T> = Result<T, BinaryExpressionError>;

// ============================================================================
// Operator identity
// ============================================================================

/// Source-level operator identity.
///
/// `OperatorRef` intentionally represents an operator as a namespaced source
/// identity rather than as a closed enum.
///
/// This prevents the native AST from requiring modification whenever a new
/// language operator or computational-domain operator is introduced.
///
/// # Important
///
/// This is NOT:
///
/// - a lexer token;
/// - a semantic operator implementation;
/// - an overload;
/// - a function pointer;
/// - a backend instruction;
/// - a quantum gate;
/// - a machine instruction.
///
/// It is simply the source-level identity written between two expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OperatorRef {
    /// Operator namespace.
///
/// The empty namespace may be used for operators belonging to the language's
/// default operator namespace.
    namespace: String,

    /// Source-level operator name.
    name: String,
}

impl OperatorRef {
    /// Creates a namespaced operator identity.
    ///
    /// The constructor rejects only structurally impossible identities.
    /// Semantic legality belongs downstream.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> BinaryExpressionResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        Self::validate_component(
            &namespace,
            true,
            BinaryExpressionError::EmptyOperatorNamespace,
            BinaryExpressionError::InvalidOperatorNamespace,
        )?;

        Self::validate_component(
            &name,
            false,
            BinaryExpressionError::EmptyOperatorName,
            BinaryExpressionError::InvalidOperatorName,
        )?;

        Ok(Self { namespace, name })
    }

    /// Creates an operator in the default language namespace.
    ///
    /// The empty namespace is intentional and means that semantic resolution
    /// should use the language's configured default operator namespace.
    pub fn unqualified(
        name: impl Into<String>,
    ) -> BinaryExpressionResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(BinaryExpressionError::EmptyOperatorName);
        }

        if name.contains('\0') {
            return Err(BinaryExpressionError::InvalidOperatorName);
        }

        Ok(Self {
            namespace: String::new(),
            name,
        })
    }

    /// Returns the operator namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the operator name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether this operator uses the default namespace.
    #[inline]
    #[must_use]
    pub fn is_unqualified(&self) -> bool {
        self.namespace.is_empty()
    }

    /// Returns the fully qualified source-level identity.
    ///
    /// The result is deterministic and contains no target information.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        if self.namespace.is_empty() {
            return self.name.clone();
        }

        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }

    /// Consumes the identity and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (String, String) {
        (self.namespace, self.name)
    }

    fn validate_component(
        value: &str,
        namespace: bool,
        empty_error: BinaryExpressionError,
        invalid_error: BinaryExpressionError,
    ) -> BinaryExpressionResult<()> {
        if value.is_empty() {
            return Err(empty_error);
        }

        // NUL cannot represent a normal source-level textual identifier and
        // must not be silently accepted into a compiler identity.
        if value.contains('\0') {
            return Err(invalid_error);
        }

        // `namespace` is deliberately otherwise opaque. The AST does not
        // impose a finite grammar on extension namespaces.
        let _ = namespace;

        Ok(())
    }
}

impl fmt::Display for OperatorRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.namespace.is_empty() {
            formatter.write_str(&self.name)
        } else {
            write!(
                formatter,
                "{}:{}",
                self.namespace,
                self.name
            )
        }
    }
}

// ============================================================================
// Binary expression
// ============================================================================

/// Canonical source-level binary/infix expression.
///
/// # Representation
///
/// ```text
/// BinaryExpression
/// ├── node
/// │   ├── NodeId
/// │   ├── NodeKind::BinaryExpression
/// │   ├── Span
/// │   └── metadata
/// ├── left: NodeId
/// ├── operator: OperatorRef
/// └── right: NodeId
/// ```
///
/// The operands are references into the AST graph rather than recursively
/// embedded Rust values.
///
/// # Invariants
///
/// A structurally valid binary expression satisfies:
///
/// 1. `node.kind()` is `CoreNodeKind::BinaryExpression`.
/// 2. `left` is a valid AST child reference.
/// 3. `right` is a valid AST child reference.
/// 4. `left != right`.
/// 5. The operator has a non-empty name.
/// 6. The operator contains no NUL characters.
/// 7. The node contains no target-specific information.
///
/// Child existence is a graph-level invariant and must be verified by the
/// structural AST validator.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BinaryExpression {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Left-hand operand node.
    left: NodeId,

    /// Source-level binary operator identity.
    operator: OperatorRef,

    /// Right-hand operand node.
    right: NodeId,
}

impl BinaryExpression {
    /// Creates a binary expression with an explicitly constructed `Node`.
    ///
    /// This is the primary low-level constructor.
    ///
    /// The constructor verifies local invariants but cannot verify that the
    /// child node IDs actually exist because the AST graph is external to this
    /// structure.
    pub fn new(
        node: Node,
        left: NodeId,
        operator: OperatorRef,
        right: NodeId,
    ) -> BinaryExpressionResult<Self> {
        Self::validate_node_kind(&node)?;

        Self::validate_operands(left, right)?;

        Ok(Self {
            node,
            left,
            operator,
            right,
        })
    }

    /// Creates a binary expression from its foundational AST components.
    ///
    /// This convenience constructor keeps the common `Node` construction
    /// policy centralized in `Node::new`.
    pub fn from_parts(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        left: NodeId,
        operator: OperatorRef,
        right: NodeId,
    ) -> BinaryExpressionResult<Self> {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::BinaryExpression),
            span,
            metadata,
        );

        Self::new(node, left, operator, right)
    }

    /// Creates a binary expression without explicit metadata.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        left: NodeId,
        operator: OperatorRef,
        right: NodeId,
    ) -> BinaryExpressionResult<Self> {
        let node = Node::without_metadata(
            id,
            NodeKind::core(CoreNodeKind::BinaryExpression),
            span,
        );

        Self::new(node, left, operator, right)
    }

    /// Returns the embedded common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded common AST node.
    ///
    /// Structural transformations should generally be performed by the
    /// dedicated AST transformation infrastructure.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node ID.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the node metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the node kind.
    #[inline]
    #[must_use]
    pub fn node_kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the left operand node ID.
    #[inline]
    #[must_use]
    pub fn left(&self) -> NodeId {
        self.left
    }

    /// Returns the right operand node ID.
    #[inline]
    #[must_use]
    pub fn right(&self) -> NodeId {
        self.right
    }

    /// Returns the source-level operator.
    #[inline]
    #[must_use]
    pub fn operator(&self) -> &OperatorRef {
        &self.operator
    }

    /// Returns mutable access to the operator identity.
    ///
    /// Changing the operator's semantic meaning remains a semantic-analysis
    /// concern. This method exists for AST transformation infrastructure.
    #[inline]
    pub fn operator_mut(&mut self) -> &mut OperatorRef {
        &mut self.operator
    }

    /// Returns the direct child node IDs in source/evaluation order.
    ///
    /// The returned ordering is always:
    ///
    /// ```text
    /// [left, right]
    /// ```
    ///
    /// This method does not recursively traverse the AST.
    #[must_use]
    pub fn child_node_ids(&self) -> [NodeId; 2] {
        [self.left, self.right]
    }

    /// Returns the number of direct children.
    ///
    /// Binary expressions always have exactly two structural operand edges.
    #[inline]
    #[must_use]
    pub const fn child_count(&self) -> usize {
        2
    }

    /// Returns whether this node is structurally a leaf.
    ///
    /// Always `false`, because a binary expression owns two child references.
    #[inline]
    #[must_use]
    pub const fn is_leaf(&self) -> bool {
        false
    }

    /// Returns the schema version of this local node contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        BINARY_EXPRESSION_SCHEMA_VERSION
    }

    /// Performs local structural validation.
    ///
    /// This method intentionally does not inspect the external AST graph.
    /// Child existence must be checked by the aggregate AST validator.
    pub fn validate_structure(&self) -> BinaryExpressionResult<()> {
        Self::validate_node_kind(&self.node)?;
        Self::validate_operands(self.left, self.right)?;

        if self.operator.name().is_empty() {
            return Err(BinaryExpressionError::EmptyOperatorName);
        }

        if self.operator.namespace().contains('\0') {
            return Err(BinaryExpressionError::InvalidOperatorNamespace);
        }

        if self.operator.name().contains('\0') {
            return Err(BinaryExpressionError::InvalidOperatorName);
        }

        Ok(())
    }

    /// Validates that the node has the correct canonical node kind.
    pub fn validate_node_kind(node: &Node) -> BinaryExpressionResult<()> {
        let expected =
            NodeKind::core(CoreNodeKind::BinaryExpression);

        if node.kind() != &expected {
            return Err(BinaryExpressionError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        Ok(())
    }

    /// Validates the local operand-reference invariants.
    pub fn validate_operands(
        left: NodeId,
        right: NodeId,
    ) -> BinaryExpressionResult<()> {
        if left.is_invalid() {
            return Err(BinaryExpressionError::InvalidLeftOperand {
                id: left,
            });
        }

        if right.is_invalid() {
            return Err(BinaryExpressionError::InvalidRightOperand {
                id: right,
            });
        }

        if left == right {
            return Err(BinaryExpressionError::SameOperand {
                id: left,
            });
        }

        Ok(())
    }

    /// Returns whether this binary expression references a particular child.
    #[inline]
    #[must_use]
    pub fn references(&self, id: NodeId) -> bool {
        self.left == id || self.right == id
    }

    /// Returns whether the supplied node ID is the left operand.
    #[inline]
    #[must_use]
    pub fn is_left_operand(&self, id: NodeId) -> bool {
        self.left == id
    }

    /// Returns whether the supplied node ID is the right operand.
    #[inline]
    #[must_use]
    pub fn is_right_operand(&self, id: NodeId) -> bool {
        self.right == id
    }

    /// Returns a deterministic source-level textual representation of the
    /// operator identity.
    #[inline]
    #[must_use]
    pub fn operator_name(&self) -> String {
        self.operator.qualified_name()
    }

    /// Replaces the operator identity.
    ///
    /// Returns the previous operator.
    ///
    /// This operation changes source-level AST structure and therefore should
    /// normally be used by parser recovery or AST transformation code.
    pub fn replace_operator(
        &mut self,
        operator: OperatorRef,
    ) -> OperatorRef {
        core::mem::replace(&mut self.operator, operator)
    }

    /// Replaces the left operand reference.
    ///
    /// The supplied ID must be valid and must not equal the right operand.
    pub fn replace_left(
        &mut self,
        left: NodeId,
    ) -> BinaryExpressionResult<NodeId> {
        Self::validate_operands(left, self.right)?;

        Ok(core::mem::replace(&mut self.left, left))
    }

    /// Replaces the right operand reference.
    ///
    /// The supplied ID must be valid and must not equal the left operand.
    pub fn replace_right(
        &mut self,
        right: NodeId,
    ) -> BinaryExpressionResult<NodeId> {
        Self::validate_operands(self.left, right)?;

        Ok(core::mem::replace(&mut self.right, right))
    }

    /// Returns a lightweight structural summary useful to diagnostics and
    /// tooling.
    #[must_use]
    pub fn diagnostic_summary(&self) -> BinaryExpressionSummary {
        BinaryExpressionSummary {
            id: self.id(),
            span: self.span().clone(),
            left: self.left,
            operator: self.operator.qualified_name(),
            right: self.right,
        }
    }
}

/// Lightweight diagnostic representation of a binary expression.
///
/// It intentionally does not include metadata, because metadata can contain
/// arbitrarily large extension payloads.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BinaryExpressionSummary {
    /// Binary-expression AST node identity.
    pub id: NodeId,

    /// Source span of the complete binary expression.
    pub span: Span,

    /// Left operand node ID.
    pub left: NodeId,

    /// Operator source identity.
    pub operator: String,

    /// Right operand node ID.
    pub right: NodeId,
}

impl fmt::Display for BinaryExpressionSummary {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "binary expression {} {} {} at {}",
            self.left,
            self.operator,
            self.right,
            self.span
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn span() -> Span {
        Span::new(
            crate::frontend::ast::source::SourceId::new(1),
            10,
            15,
        )
    }

    fn add_operator() -> OperatorRef {
        OperatorRef::new("zamani", "add")
            .expect("valid operator")
    }

    #[test]
    fn operator_identity_is_namespaced_and_deterministic() {
        let operator =
            OperatorRef::new("zamani", "add")
                .expect("valid operator");

        assert_eq!(operator.namespace(), "zamani");
        assert_eq!(operator.name(), "add");
        assert_eq!(operator.qualified_name(), "zamani:add");
        assert!(!operator.is_unqualified());
    }

    #[test]
    fn unqualified_operator_is_supported() {
        let operator =
            OperatorRef::unqualified("+")
                .expect("valid operator");

        assert!(operator.is_unqualified());
        assert_eq!(operator.namespace(), "");
        assert_eq!(operator.name(), "+");
        assert_eq!(operator.qualified_name(), "+");
    }

    #[test]
    fn empty_operator_name_is_rejected() {
        let result = OperatorRef::new("zamani", "");

        assert_eq!(
            result,
            Err(BinaryExpressionError::EmptyOperatorName)
        );
    }

    #[test]
    fn empty_unqualified_operator_is_rejected() {
        let result = OperatorRef::unqualified("");

        assert_eq!(
            result,
            Err(BinaryExpressionError::EmptyOperatorName)
        );
    }

    #[test]
    fn nul_in_operator_name_is_rejected() {
        let result = OperatorRef::new("zamani", "ad\0d");

        assert_eq!(
            result,
            Err(BinaryExpressionError::InvalidOperatorName)
        );
    }

    #[test]
    fn invalid_left_operand_is_rejected() {
        let result = BinaryExpression::without_metadata(
            node_id(1),
            span(),
            NodeId::INVALID,
            add_operator(),
            node_id(3),
        );

        assert_eq!(
            result,
            Err(BinaryExpressionError::InvalidLeftOperand {
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn invalid_right_operand_is_rejected() {
        let result = BinaryExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            add_operator(),
            NodeId::INVALID,
        );

        assert_eq!(
            result,
            Err(BinaryExpressionError::InvalidRightOperand {
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn same_operand_is_rejected() {
        let result = BinaryExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            add_operator(),
            node_id(2),
        );

        assert_eq!(
            result,
            Err(BinaryExpressionError::SameOperand {
                id: node_id(2)
            })
        );
    }

    #[test]
    fn valid_binary_expression_is_constructed() {
        let expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.left(), node_id(2));
        assert_eq!(expression.right(), node_id(3));
        assert_eq!(
            expression.operator().qualified_name(),
            "zamani:add"
        );
        assert_eq!(
            expression.node_kind(),
            &NodeKind::core(CoreNodeKind::BinaryExpression)
        );
        assert_eq!(expression.child_count(), 2);
        assert_eq!(
            expression.child_node_ids(),
            [node_id(2), node_id(3)]
        );
    }

    #[test]
    fn source_span_is_preserved() {
        let expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        assert_eq!(expression.span(), &span());
    }

    #[test]
    fn metadata_is_preserved() {
        let mut metadata = NodeMetadata::default();

        metadata.insert(
            "documentation",
            super::super::super::metadata::MetadataValue::String(
                "example".to_owned(),
            ),
        );

        let expression =
            BinaryExpression::from_parts(
                node_id(1),
                span(),
                metadata.clone(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        assert_eq!(expression.metadata(), &metadata);
    }

    #[test]
    fn references_detects_both_operands() {
        let expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        assert!(expression.references(node_id(2)));
        assert!(expression.references(node_id(3)));
        assert!(!expression.references(node_id(4)));
    }

    #[test]
    fn operand_replacement_preserves_invariants() {
        let mut expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        let previous =
            expression
                .replace_left(node_id(4))
                .expect("valid replacement");

        assert_eq!(previous, node_id(2));
        assert_eq!(expression.left(), node_id(4));

        let previous =
            expression
                .replace_right(node_id(5))
                .expect("valid replacement");

        assert_eq!(previous, node_id(3));
        assert_eq!(expression.right(), node_id(5));
    }

    #[test]
    fn invalid_operand_replacement_is_rejected_without_mutation() {
        let mut expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        let result =
            expression.replace_left(node_id(3));

        assert_eq!(
            result,
            Err(BinaryExpressionError::SameOperand {
                id: node_id(3)
            })
        );

        assert_eq!(expression.left(), node_id(2));
        assert_eq!(expression.right(), node_id(3));
    }

    #[test]
    fn validation_succeeds_for_valid_expression() {
        let expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        assert_eq!(expression.validate_structure(), Ok(()));
    }

    #[test]
    fn diagnostic_summary_is_small_and_deterministic() {
        let expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        let summary = expression.diagnostic_summary();

        assert_eq!(summary.id, node_id(1));
        assert_eq!(summary.left, node_id(2));
        assert_eq!(summary.right, node_id(3));
        assert_eq!(summary.operator, "zamani:add");
        assert_eq!(summary.span, span());
    }

    #[test]
    fn serde_round_trip_preserves_binary_expression() {
        let expression =
            BinaryExpression::without_metadata(
                node_id(1),
                span(),
                node_id(2),
                add_operator(),
                node_id(3),
            )
            .expect("valid binary expression");

        let encoded =
            serde_json::to_string(&expression)
                .expect("serialization succeeds");

        let decoded: BinaryExpression =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(decoded, expression);
    }

    #[test]
    fn operator_ordering_is_deterministic() {
        let a =
            OperatorRef::new("zamani", "add")
                .expect("valid operator");

        let b =
            OperatorRef::new("zamani", "subtract")
                .expect("valid operator");

        assert!(a < b);
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(
            BinaryExpression::schema_version(),
            BINARY_EXPRESSION_SCHEMA_VERSION
        );
        assert_eq!(BINARY_EXPRESSION_SCHEMA_VERSION, 1);
    }
}