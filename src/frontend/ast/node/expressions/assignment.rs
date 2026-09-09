//! # Zamani Native AST — Assignment Expression
//!
//! Production-ready source-level representation of assignment expressions.
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
//! AssignmentExpression  ← this module
//!     │
//!     ▼
//! Structural AST validation
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
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── hybrid IR
//!     ├── HDL IR
//!     └── future domain IRs
//!     │
//!     ▼
//! Target lowering / execution
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level structural representation** of
//! assignment expressions.
//!
//! It represents:
//!
//! - the assignment target;
//! - the assigned value;
//! - whether the assignment is ordinary or compound;
//! - the source-level compound operator identity;
//! - common AST node identity;
//! - source span;
//! - source metadata.
//!
//! It deliberately does not represent:
//!
//! - resolved symbols;
//! - resolved types;
//! - storage locations;
//! - memory addresses;
//! - SSA values;
//! - registers;
//! - physical resources;
//! - quantum qubits;
//! - hardware instructions;
//! - backend state;
//! - scheduling;
//! - routing;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime state;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR.
//!
//! Those belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! Assignment is deliberately represented using source-level node references.
//!
//! Consequently the same source construct can eventually be lowered to:
//!
//! - ordinary memory assignment;
//! - SSA/dataflow construction;
//! - distributed state update;
//! - accelerator memory operation;
//! - classical control surrounding quantum execution;
//! - resource-state update;
//! - another future computational representation.
//!
//! The AST does not know which realization will be selected.
//!
//! This preserves:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! ## Critical design rule
//!
//! Assignment is **not** a hardware operation.
//!
//! The following concepts are forbidden in this file:
//!
//! ```text
//! RegisterAssignment
//! MemoryAddress
//! PhysicalQubitAssignment
//! HardwareAssignment
//! BackendAssignment
//! QIRAssignment
//! LLVMStore
//! MLIRStore
//! ```
//!
//! A source-level assignment target is represented by a `NodeId`.
//!
//! Semantic analysis determines whether that target is:
//!
//! - mutable;
//! - assignable;
//! - a variable;
//! - a property;
//! - an indexed location;
//! - a resource;
//! - a future assignable abstraction.
//!
//! ## Ordinary assignment
//!
//! Source:
//!
//! ```text
//! target = value
//! ```
//!
//! is represented structurally as:
//!
//! ```text
//! AssignmentExpression
//! ├── target: NodeId
//! └── value: NodeId
//! ```
//!
//! ## Compound assignment
//!
//! Source:
//!
//! ```text
//! target += value
//! ```
//!
//! is represented as:
//!
//! ```text
//! AssignmentExpression
//! ├── target: NodeId
//! ├── value: NodeId
//! └── operator: CompoundOperator
//! ```
//!
//! The operator is represented by a source-level semantic identity rather than
//! a lexer token. This removes the legacy dependency on `TokenType`.
//!
//! ## Why the operator is not a lexer token
//!
//! The legacy AST represents compound assignment using the lexer token type.
//!
//! That creates an undesirable dependency:
//!
//! ```text
//! AST → Lexer
//! ```
//!
//! The desired dependency direction is:
//!
//! ```text
//! Lexer
//!   ↓
//! Parser
//!   ↓
//! AST
//! ```
//!
//! The AST therefore owns a small source-level operator identity.
//!
//! The parser performs the mapping:
//!
//! ```text
//! lexer token
//!     ↓
//! CompoundOperator
//!     ↓
//! AssignmentExpression
//! ```
//!
//! Semantic analysis subsequently determines the meaning of that operator.
//!
//! ## No fixed operator universe
//!
//! The native AST must not assume that the currently known arithmetic operators
//! are the complete set of all future assignment operations.
//!
//! Built-in compound operators are represented explicitly for the current
//! language core.
//!
//! Future/domain-specific assignment syntax can use the extensible
//! `CompoundOperator::Extension` form without requiring hardware knowledge in
//! this module.
//!
//! ## Quantum compatibility
//!
//! Assignment may participate in hybrid programs.
//!
//! For example, a source program may assign:
//!
//! - a measurement result to a classical variable;
//! - a computed parameter to a generic operation;
//! - a resource handle to a variable;
//! - a future domain value to an assignable target.
//!
//! The AST does not determine whether such an assignment is legal.
//!
//! Semantic analysis determines:
//!
//! - type compatibility;
//! - mutability;
//! - ownership;
//! - resource rules;
//! - effect rules;
//! - measurement semantics;
//! - domain constraints.
//!
//! Hardware mapping remains downstream.
//!
//! ## Quantum scaling
//!
//! Nothing in this representation contains:
//!
//! - a qubit count;
//! - a register width;
//! - a physical-qubit identifier;
//! - a device identifier;
//! - a topology;
//! - a backend;
//! - a gate set.
//!
//! An assignment can therefore participate in programs containing symbolic or
//! dynamically sized resources.
//!
//! ## Child representation
//!
//! Children are represented by `NodeId` rather than recursively embedding AST
//! nodes.
//!
//! This avoids recursive ownership structures and permits the repository's
//! traversal infrastructure to perform iterative traversal.
//!
//! ## Child order
//!
//! Direct child enumeration is deterministic:
//!
//! 1. target;
//! 2. value.
//!
//! The compound operator is not a child AST node because it is a source-level
//! scalar identity rather than another expression.
//!
//! ## Structural validation boundary
//!
//! Local validation checks only intrinsic structure:
//!
//! - correct node kind;
//! - target reference is present;
//! - value reference is present;
//! - target is not the assignment node itself;
//! - value is not the assignment node itself;
//! - target and value do not alias where the AST contract forbids such aliasing.
//!
//! It does not perform:
//!
//! - name resolution;
//! - type checking;
//! - mutability checking;
//! - borrow checking;
//! - ownership checking;
//! - overload resolution;
//! - operator resolution;
//! - resource checking;
//! - quantum legality checking;
//! - hardware validation.
//!
//! Those belong downstream.
//!
//! ## Scalability
//!
//! Assignment has a constant number of direct structural children, so it does
//! not need a dynamically sized collection itself.
//!
//! This file introduces no limits on:
//!
//! - number of assignments;
//! - program size;
//! - expression count;
//! - resource count;
//! - qubit count;
//! - machine size;
//! - number of execution targets.
//!
//! A program containing arbitrarily many assignment expressions is represented
//! by the owning AST graph.
//!
//! "Infinity" means that this node introduces no artificial finite computational
//! limit. Actual compilation remains bounded by available memory, address space,
//! storage, compiler resource policy, and target capabilities.
//!
//! ## Security
//!
//! This module:
//!
//! - uses no `unsafe`;
//! - performs no I/O;
//! - executes no source program;
//! - does not dereference pointers;
//! - does not use unchecked indexing;
//! - does not recursively traverse the AST;
//! - has no global mutable state;
//! - does not perform backend lookups.
//!
//! AST input is treated as untrusted compiler input.
//!
//! ## Determinism
//!
//! The node contains only deterministic source-level state.
//!
//! There are no hash-map-backed child collections and no process-global state.
//!
//! ## Serialization
//!
//! Public structures derive Serde serialization.
//!
//! Global AST serialization/versioning remains the responsibility of the AST
//! serialization subsystem.
//!
//! This module does not serialize:
//!
//! - pointers;
//! - addresses;
//! - runtime handles;
//! - caches;
//! - backend state;
//! - synchronization primitives.
//!
//! ## Incremental compilation
//!
//! `NodeId` allows semantic information to remain outside the AST:
//!
//! ```text
//! NodeId → resolved symbol
//! NodeId → inferred type
//! NodeId → mutability result
//! NodeId → resource semantics
//! NodeId → effects
//! ```
//!
//! No semantic state is embedded in this node.
//!
//! ## Dependency contract
//!
//! This file may depend only on:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - Serde;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - lexer token types;
//! - parser implementation;
//! - semantic analysis;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - OpenQASM ASTs.
//!
//! ## Integration contract
//!
//! ### `node.rs`
//!
//! Supplies the common `Node` container.
//!
//! ### `node_id.rs`
//!
//! Supplies stable AST identity.
//!
//! ### `node_kind.rs`
//!
//! Supplies `CoreNodeKind::Assignment`.
//!
//! ### `expressions/mod.rs`
//!
//! Must expose:
//!
//! ```text
//! pub mod assignment;
//! ```
//!
//! ### Expression aggregate
//!
//! The aggregate expression representation should map both ordinary and
//! compound assignment into this node.
//!
//! Recommended canonical representation:
//!
//! ```text
//! ExpressionKind::Assignment(AssignmentExpression)
//! ```
//!
//! The `AssignmentExpression` itself records whether it is ordinary or
//! compound.
//!
//! This avoids maintaining two independent assignment node structures.
//!
//! ### Parser
//!
//! The parser maps lexer assignment tokens into this source-level representation.
//!
//! The parser must not pass `TokenType` into this file.
//!
//! ### Semantic analysis
//!
//! Semantic analysis resolves:
//!
//! - target identity;
//! - target mutability;
//! - target type;
//! - value type;
//! - operator meaning;
//! - ownership/resource semantics;
//! - effects;
//! - domain constraints.
//!
//! ### ZUIR
//!
//! The semantic layer lowers assignment into the appropriate ZUIR semantics.
//!
//! This file does not import ZUIR.
//!
//! ### Quantum compiler
//!
//! Quantum-specific meaning remains downstream.
//!
//! For example, an assignment involving a measurement result is still an
//! ordinary source-level assignment.
//!
//! ### Optimization
//!
//! Constant propagation, copy propagation, algebraic simplification, SSA
//! formation and dead-store elimination are downstream transformations.
//!
//! ### Hardware
//!
//! Hardware storage, registers, memory and resource allocation are downstream.
//!
//! ## No-reedit guarantee
//!
//! The public contract of this file is deliberately self-contained:
//!
//! - source-level node identity;
//! - source span;
//! - metadata;
//! - target child;
//! - value child;
//! - compound-operator identity;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! Changes to semantic analysis, ZUIR, quantum compilation, routing,
//! scheduling, calibration, QEC, resilience, runtime or backend implementation
//! should not require changing this file.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Independent schema version for the assignment-expression contract.
///
/// This is intentionally not the global AST schema version and not the
/// Zamani language version.
pub const ASSIGNMENT_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level diagnostic/tooling name.
pub const ASSIGNMENT_EXPRESSION_KIND_NAME: &str =
    "zamani:assignment-expression";

/// Source-level compound-assignment operator.
///
/// This type intentionally represents **source syntax**, not a semantic
/// operation or hardware instruction.
///
/// Built-in operators cover the current core language. `Extension` allows
/// future language/domain extensions without modifying this file's structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CompoundOperator {
    /// Addition assignment: `+=`.
    Add,

    /// Subtraction assignment: `-=`.
    Subtract,

    /// Multiplication assignment: `*=`.
    Multiply,

    /// Division assignment: `/=`.
    Divide,

    /// Remainder assignment: `%=`.
    Remainder,

    /// Bitwise AND assignment: `&=`.
    BitAnd,

    /// Bitwise OR assignment: `|=`.
    BitOr,

    /// Bitwise XOR assignment: `^=`.
    BitXor,

    /// Left-shift assignment: `<<=`.
    ShiftLeft,

    /// Right-shift assignment: `>>=`.
    ShiftRight,

    /// Namespaced source-language extension.
    ///
    /// The AST does not interpret the operator. Semantic analysis or the
    /// extension registry owns its meaning.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Extension-local operator name.
        name: String,
    },
}

impl CompoundOperator {
    /// Returns the canonical source spelling for a built-in operator.
    ///
    /// Extension operators return their qualified identity.
    #[must_use]
    pub fn source_name(&self) -> String {
        match self {
            Self::Add => "+=".to_owned(),
            Self::Subtract => "-=".to_owned(),
            Self::Multiply => "*=".to_owned(),
            Self::Divide => "/=".to_owned(),
            Self::Remainder => "%=".to_owned(),
            Self::BitAnd => "&=".to_owned(),
            Self::BitOr => "|=".to_owned(),
            Self::BitXor => "^=".to_owned(),
            Self::ShiftLeft => "<<=".to_owned(),
            Self::ShiftRight => ">>=".to_owned(),
            Self::Extension { namespace, name } => {
                let mut result =
                    String::with_capacity(namespace.len() + 1 + name.len());

                result.push_str(namespace);
                result.push(':');
                result.push_str(name);
                result
            }
        }
    }

    /// Returns `true` when this is an extension-defined operator.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension { .. })
    }

    /// Validates the source-level structure of the operator identity.
    ///
    /// This does not resolve the operator.
    pub fn validate(&self) -> Result<(), AssignmentValidationError> {
        if let Self::Extension { namespace, name } = self {
            if namespace.trim().is_empty() {
                return Err(
                    AssignmentValidationError::EmptyExtensionNamespace,
                );
            }

            if name.trim().is_empty() {
                return Err(AssignmentValidationError::EmptyExtensionName);
            }
        }

        Ok(())
    }
}

/// Kind of assignment represented by the node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssignmentKind {
    /// Ordinary assignment using `=`.
    Simple,

    /// Compound assignment using an operator such as `+=` or `-=`.
    Compound(CompoundOperator),
}

impl AssignmentKind {
    /// Returns `true` for ordinary assignment.
    #[must_use]
    pub const fn is_simple(&self) -> bool {
        matches!(self, Self::Simple)
    }

    /// Returns `true` for compound assignment.
    #[must_use]
    pub const fn is_compound(&self) -> bool {
        matches!(self, Self::Compound(_))
    }

    /// Returns the compound operator when present.
    #[must_use]
    pub const fn compound_operator(&self) -> Option<&CompoundOperator> {
        match self {
            Self::Simple => None,
            Self::Compound(operator) => Some(operator),
        }
    }
}

/// Canonical source-level assignment expression.
///
/// ```text
/// AssignmentExpression
/// ├── Node
/// ├── target: NodeId
/// ├── value: NodeId
/// └── kind: AssignmentKind
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssignmentExpression {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Source-level assignment target.
    target: NodeId,

    /// Source-level assigned expression.
    value: NodeId,

    /// Simple or compound assignment kind.
    kind: AssignmentKind,
}

impl AssignmentExpression {
    /// Creates an ordinary `=` assignment.
    ///
    /// This constructor establishes the canonical
    /// `CoreNodeKind::Assignment` classification.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        target: NodeId,
        value: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Assignment),
                span,
                metadata,
            ),
            target,
            value,
            kind: AssignmentKind::Simple,
        }
    }

    /// Creates an ordinary assignment using default metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        target: NodeId,
        value: NodeId,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            target,
            value,
        )
    }

    /// Creates a compound assignment.
    ///
    /// The operator remains a source-level identity. Its semantic meaning is
    /// resolved later.
    #[must_use]
    pub fn new_compound(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        target: NodeId,
        operator: CompoundOperator,
        value: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Assignment),
                span,
                metadata,
            ),
            target,
            value,
            kind: AssignmentKind::Compound(operator),
        }
    }

    /// Creates a compound assignment using default metadata.
    #[must_use]
    pub fn compound_without_metadata(
        id: NodeId,
        span: Span,
        target: NodeId,
        operator: CompoundOperator,
        value: NodeId,
    ) -> Self {
        Self::new_compound(
            id,
            span,
            NodeMetadata::default(),
            target,
            operator,
            value,
        )
    }

    /// Creates an assignment from an existing common AST node.
    ///
    /// This constructor is useful when the parser or AST builder already owns
    /// construction of the common node.
    #[must_use]
    pub fn from_node(
        node: Node,
        target: NodeId,
        value: NodeId,
        kind: AssignmentKind,
    ) -> Self {
        Self {
            node,
            target,
            value,
            kind,
        }
    }

    /// Returns the common AST node.
    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Only common metadata/span/kind infrastructure is exposed by `Node`;
    /// semantic state is never stored here.
    #[must_use]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node ID.
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the AST node kind.
    #[must_use]
    pub fn node_kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source metadata.
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source metadata.
    #[must_use]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the assignment target node ID.
    #[must_use]
    pub const fn target(&self) -> NodeId {
        self.target
    }

    /// Returns the assigned value node ID.
    #[must_use]
    pub const fn value(&self) -> NodeId {
        self.value
    }

    /// Returns the assignment kind.
    #[must_use]
    pub const fn assignment_kind(&self) -> &AssignmentKind {
        &self.kind
    }

    /// Returns `true` for ordinary `=` assignment.
    #[must_use]
    pub const fn is_simple(&self) -> bool {
        self.kind.is_simple()
    }

    /// Returns `true` for compound assignment.
    #[must_use]
    pub const fn is_compound(&self) -> bool {
        self.kind.is_compound()
    }

    /// Returns the compound operator, if any.
    #[must_use]
    pub const fn compound_operator(&self) -> Option<&CompoundOperator> {
        self.kind.compound_operator()
    }

    /// Returns the direct AST children in deterministic source order.
    ///
    /// Order:
    ///
    /// 1. target;
    /// 2. value.
    ///
    /// The returned iterator performs no recursive traversal.
    pub fn child_node_ids(
        &self,
    ) -> impl Iterator<Item = NodeId> + '_ {
        [self.target, self.value].into_iter()
    }

    /// Returns the number of direct AST children.
    ///
    /// This is always two for a structurally complete assignment expression.
    #[must_use]
    pub const fn child_count(&self) -> usize {
        2
    }

    /// Validates intrinsic local structure.
    ///
    /// This method deliberately does not inspect the owning AST graph.
    ///
    /// Existence of `target` and `value` in the graph is validated by the
    /// graph-level AST validator.
    pub fn validate_structure(
        &self,
    ) -> Result<(), AssignmentValidationError> {
        if self.node.kind()
            != &NodeKind::core(CoreNodeKind::Assignment)
        {
            return Err(
                AssignmentValidationError::InvalidNodeKind {
                    actual: self.node.kind_owned(),
                },
            );
        }

        if self.target == self.id() {
            return Err(
                AssignmentValidationError::SelfReferentialTarget,
            );
        }

        if self.value == self.id() {
            return Err(
                AssignmentValidationError::SelfReferentialValue,
            );
        }

        if self.target == self.value {
            return Err(
                AssignmentValidationError::TargetValueAlias,
            );
        }

        self.kind.validate()?;

        Ok(())
    }

    /// Validates the assignment against a caller-supplied local policy.
    ///
    /// The current policy is intentionally small because assignment has no
    /// variable-sized structural collections.
    pub fn validate_with_policy(
        &self,
        _policy: AssignmentValidationPolicy,
    ) -> Result<(), AssignmentValidationError> {
        self.validate_structure()
    }

    /// Returns the independent schema version of this node.
    #[must_use]
    pub const fn schema_version() -> u16 {
        ASSIGNMENT_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns a stable source-level node name.
    #[must_use]
    pub const fn kind_name() -> &'static str {
        ASSIGNMENT_EXPRESSION_KIND_NAME
    }
}

impl AstNode for AssignmentExpression {
    fn node(&self) -> &Node {
        &self.node
    }

    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Caller-configurable local validation policy.
///
/// Assignment itself has no dynamic child collection, so this policy currently
/// contains no mandatory limits.
///
/// It exists to provide a stable integration point for the repository-wide
/// validation system without introducing hidden language limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AssignmentValidationPolicy;

impl AssignmentValidationPolicy {
    /// Creates the default policy.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Errors detectable without semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AssignmentValidationError {
    /// The common AST node is not classified as an assignment.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The assignment refers to itself as its target.
    SelfReferentialTarget,

    /// The assignment refers to itself as its value.
    SelfReferentialValue,

    /// Target and value are the same AST node.
    TargetValueAlias,

    /// Extension operator namespace is empty.
    EmptyExtensionNamespace,

    /// Extension operator name is empty.
    EmptyExtensionName,
}

impl fmt::Display for AssignmentValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "assignment expression has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialTarget => {
                formatter.write_str(
                    "assignment expression cannot use itself as its target",
                )
            }

            Self::SelfReferentialValue => {
                formatter.write_str(
                    "assignment expression cannot use itself as its value",
                )
            }

            Self::TargetValueAlias => {
                formatter.write_str(
                    "assignment target and value must not alias at the assignment node level",
                )
            }

            Self::EmptyExtensionNamespace => {
                formatter.write_str(
                    "assignment extension operator namespace is empty",
                )
            }

            Self::EmptyExtensionName => {
                formatter.write_str(
                    "assignment extension operator name is empty",
                )
            }
        }
    }
}

impl std::error::Error for AssignmentValidationError {}

/// Stable conversion from a built-in compound operator to its source spelling.
///
/// This is intentionally parser-independent.
impl fmt::Display for CompoundOperator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.source_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn simple_assignment_has_canonical_kind() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert_eq!(
            assignment.node_kind(),
            &NodeKind::core(CoreNodeKind::Assignment)
        );
        assert!(assignment.is_simple());
        assert!(!assignment.is_compound());
    }

    #[test]
    fn compound_assignment_preserves_operator() {
        let assignment =
            AssignmentExpression::compound_without_metadata(
                node_id(1),
                span(),
                node_id(2),
                CompoundOperator::Add,
                node_id(3),
            );

        assert!(assignment.is_compound());
        assert_eq!(
            assignment.compound_operator(),
            Some(&CompoundOperator::Add)
        );
        assert_eq!(
            assignment.compound_operator()
                .expect("compound operator")
                .source_name(),
            "+="
        );
    }

    #[test]
    fn child_order_is_deterministic() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        let children: Vec<NodeId> =
            assignment.child_node_ids().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3)]
        );
    }

    #[test]
    fn self_referential_target_is_rejected() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(1),
            node_id(3),
        );

        assert_eq!(
            assignment.validate_structure(),
            Err(
                AssignmentValidationError::SelfReferentialTarget
            )
        );
    }

    #[test]
    fn self_referential_value_is_rejected() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(1),
        );

        assert_eq!(
            assignment.validate_structure(),
            Err(
                AssignmentValidationError::SelfReferentialValue
            )
        );
    }

    #[test]
    fn target_value_alias_is_rejected() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(2),
        );

        assert_eq!(
            assignment.validate_structure(),
            Err(
                AssignmentValidationError::TargetValueAlias
            )
        );
    }

    #[test]
    fn extension_operator_requires_namespace_and_name() {
        let empty_namespace = CompoundOperator::Extension {
            namespace: String::new(),
            name: "assign".to_owned(),
        };

        assert_eq!(
            empty_namespace.validate(),
            Err(
                AssignmentValidationError::EmptyExtensionNamespace
            )
        );

        let empty_name = CompoundOperator::Extension {
            namespace: "zamani.example".to_owned(),
            name: String::new(),
        };

        assert_eq!(
            empty_name.validate(),
            Err(
                AssignmentValidationError::EmptyExtensionName
            )
        );
    }

    #[test]
    fn extension_operator_is_not_backend_specific() {
        let operator = CompoundOperator::Extension {
            namespace: "example.language".to_owned(),
            name: "merge_assign".to_owned(),
        };

        assert!(operator.is_extension());
        assert_eq!(
            operator.source_name(),
            "example.language:merge_assign"
        );
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            AssignmentExpression::schema_version(),
            ASSIGNMENT_EXPRESSION_SCHEMA_VERSION
        );
    }

    #[test]
    fn clone_preserves_identity() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        let cloned = assignment.clone();

        assert_eq!(assignment.id(), cloned.id());
        assert_eq!(assignment.target(), cloned.target());
        assert_eq!(assignment.value(), cloned.value());
    }

    #[test]
    fn no_dynamic_assignment_collection_exists() {
        let assignment = AssignmentExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert_eq!(assignment.child_count(), 2);
    }
}