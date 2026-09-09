//! # Zamani Frontend AST — Macro Expression
//!
//! Source-level representation of a macro invocation.
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
//!     ├── MacroExpression  ← this module
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! macro resolution / expansion
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level structure of a macro invocation**.
//!
//! A macro invocation consists of:
//!
//! - a source-level macro name;
//! - zero or more argument expressions;
//! - the common AST node containing identity, source span and metadata.
//!
//! This module deliberately does **not** perform macro expansion.
//!
//! Macro expansion belongs to the compiler/macro-expansion layer.
//!
//! ## Source-level example
//!
//! A source construct such as:
//!
//! ```text
//! compute!(a, b, c)
//! ```
//!
//! is represented conceptually as:
//!
//! ```text
//! MacroExpression
//! ├── name: "compute"
//! ├── argument[0]: NodeId
//! ├── argument[1]: NodeId
//! └── argument[2]: NodeId
//! ```
//!
//! The argument expressions are represented by `NodeId` rather than recursively
//! embedding concrete AST nodes. The canonical AST graph owns those nodes.
//!
//! ## Important boundary
//!
//! This node represents:
//!
//! ```text
//! "invoke this source-level macro"
//! ```
//!
//! It does not represent:
//!
//! ```text
//! "expand this macro now"
//! "execute this generated code"
//! "select this backend"
//! "select this machine"
//! "select this quantum device"
//! ```
//!
//! Those decisions belong downstream.
//!
//! ## POCO-REAF
//!
//! Macro syntax must not encode a particular machine or computational domain.
//!
//! A macro can eventually generate:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - HDL;
//! - accelerator code;
//! - distributed computation;
//! - future computational constructs.
//!
//! The native AST does not need to know which realization will eventually be
//! selected.
//!
//! ## Quantum compatibility
//!
//! A macro invocation may eventually expand into quantum source constructs.
//! That does not justify putting quantum-specific fields into this structure.
//!
//! For example:
//!
//! ```text
//! quantum_prepare!(register, angle)
//! ```
//!
//! remains a generic macro invocation at the native AST layer.
//!
//! The macro implementation and later semantic analysis determine what the
//! invocation means.
//!
//! The AST must not contain fields such as:
//!
//! - `qubit_count`;
//! - `physical_qubits`;
//! - `backend`;
//! - `topology`;
//! - `gate_set`;
//! - `device`;
//! - `scheduler`;
//! - `qec_code`.
//!
//! ## No hard-coded operation vocabulary
//!
//! Macro names are represented as source-level strings rather than a closed
//! enumeration such as:
//!
//! ```text
//! enum MacroKind {
//!     QuantumMacro,
//!     CpuMacro,
//!     GpuMacro,
//!     ...
//! }
//! ```
//!
//! New macros therefore do not require modification of this AST node.
//!
//! ## Macro declarations
//!
//! Macro invocation and macro declaration are separate source constructs.
//!
//! This file represents the invocation expression only.
//!
//! Macro definitions belong in the declarations/program side of the AST and
//! macro expansion belongs to the compiler's macro-expansion infrastructure.
//!
//! ## External macro systems
//!
//! This node must not depend on:
//!
//! - Rust procedural macros;
//! - Rust `macro_rules!` semantics;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - OpenQASM;
//! - vendor macro systems;
//! - hardware instruction macros.
//!
//! Zamani macro semantics are owned by Zamani's language/compiler layers.
//!
//! ## Dependency policy
//!
//! Allowed dependencies:
//!
//! - [`AstNode`];
//! - [`Node`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - Serde;
//! - Rust standard-library facilities.
//!
//! Forbidden dependencies:
//!
//! - semantic analysis;
//! - semantic types;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
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
//! - OpenQASM;
//! - compiler execution services.
//!
//! ## Structural validation boundary
//!
//! Local validation checks only information available inside this node.
//!
//! It validates:
//!
//! - the node has `CoreNodeKind::Macro`;
//! - the macro name is not empty;
//! - the macro name does not contain NUL;
//! - this node does not directly reference itself as an argument.
//!
//! It does **not** validate:
//!
//! - whether the macro exists;
//! - whether the macro is visible;
//! - whether arguments match parameters;
//! - whether argument types are correct;
//! - whether expansion is legal;
//! - whether expansion terminates;
//! - whether generated code is valid;
//! - whether a macro is permitted in a particular semantic context.
//!
//! Those checks require compiler-wide or semantic information.
//!
//! ## Duplicate arguments
//!
//! Duplicate `NodeId`s among arguments are legal.
//!
//! For example:
//!
//! ```text
//! duplicate!(x, x)
//! ```
//!
//! may be perfectly valid source syntax.
//!
//! Therefore this node deliberately does **not** reject repeated argument
//! references.
//!
//! ## Cycles
//!
//! A direct self-reference is rejected:
//!
//! ```text
//! MacroExpression(id = N)
//! ├── name = "m"
//! └── argument = N
//! ```
//!
//! Longer graph cycles are intentionally not checked here. Global AST graph
//! validation owns graph-wide cycle detection.
//!
//! ## Traversal
//!
//! Direct children are yielded in source order:
//!
//! ```text
//! argument[0]
//! argument[1]
//! ...
//! argument[n]
//! ```
//!
//! Traversal is non-recursive and allocation-free.
//!
//! The complete AST traversal subsystem is responsible for recursively or
//! iteratively walking the graph.
//!
//! ## Scalability
//!
//! There is no machine-size limit, argument-count limit, macro-count limit,
//! quantum-resource limit or computational-domain limit in this node.
//!
//! The argument collection is dynamically sized.
//!
//! Practical limits, when required for compiler security, belong to explicit
//! configurable compiler resource policies.
//!
//! The theoretical POCO-REAF target is therefore:
//!
//! ```text
//! tiny program
//!      │
//!      ▼
//! same AST schema
//!      │
//!      ▼
//! arbitrarily large program
//! ```
//!
//! subject only to available resources and explicitly configured compiler
//! policies.
//!
//! ## Determinism
//!
//! Arguments are stored in `Vec<NodeId>` because source order is significant.
//!
//! No hash-map iteration is used.
//!
//! Child traversal therefore has deterministic ordering.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization/deserialization.
//!
//! The repository-wide AST serialization layer owns the overall serialization
//! format and version compatibility policy.
//!
//! The local schema version exposed by this module documents this node's
//! structural contract and is not a replacement for the global AST schema
//! version.
//!
//! ## Source preservation
//!
//! The complete invocation span is stored in the common [`Node`].
//!
//! Individual argument nodes retain their own spans in their respective AST
//! nodes.
//!
//! This allows diagnostics to identify both the invocation and individual
//! arguments without duplicating source-location state.
//!
//! ## Metadata
//!
//! Source-level metadata is stored in [`NodeMetadata`].
//!
//! Metadata must not be used to store:
//!
//! - resolved macro definitions;
//! - expansion state;
//! - compiler runtime state;
//! - backend information;
//! - hardware state.
//!
//! ## Parser integration
//!
//! The grammar currently defines macro invocation syntax in the form:
//!
//! ```text
//! macroCall: IDENTIFIER '(' argumentList? ')';
//! ```
//!
//! The parser should:
//!
//! 1. parse the macro identifier;
//! 2. parse zero or more argument expressions;
//! 3. allocate a fresh `NodeId`;
//! 4. calculate the complete invocation span;
//! 5. attach source metadata;
//! 6. construct [`MacroExpression`];
//! 7. insert it into the canonical AST graph.
//!
//! The parser must not:
//!
//! - resolve the macro definition;
//! - expand the macro;
//! - execute generated code;
//! - select hardware;
//! - select a quantum backend.
//!
//! The repository grammar explicitly contains `macroCall` with an identifier
//! and optional argument list. The legacy AST also represented macros as a
//! name plus expression vector. This node preserves that source-level shape
//! while replacing recursively embedded expressions with canonical `NodeId`
//! references.
//!
//! ## Expression aggregate integration
//!
//! The central expression representation should contain one canonical variant
//! corresponding to this structure, conceptually:
//!
//! ```text
//! ExpressionKind::Macro {
//!     name: String,
//!     arguments: Vec<NodeId>,
//! }
//! ```
//!
//! Do not create a second competing macro representation in the expression
//! aggregate.
//!
//! If the aggregate is migrated to hold concrete expression-node structures,
//! this type remains the authoritative source-level contract for macro
//! invocation structure.
//!
//! ## Node-kind integration
//!
//! The repository already provides:
//!
//! ```text
//! CoreNodeKind::Macro
//! ```
//!
//! This file uses that existing node classification rather than introducing a
//! second macro node-kind enumeration.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes this node and resolves:
//!
//! - macro identity;
//! - lexical/module scope;
//! - visibility;
//! - parameter binding;
//! - argument correspondence;
//! - macro constraints;
//! - expansion context;
//! - resulting semantic constructs.
//!
//! Macro expansion must produce compiler-owned AST/semantic structures through
//! the macro-expansion subsystem rather than mutating this invocation into a
//! backend-specific representation.
//!
//! ## Expansion boundary
//!
//! The AST must remain a representation of the source invocation until the
//! expansion phase decides otherwise.
//!
//! Expansion may eventually generate arbitrary valid Zamani constructs.
//!
//! It must not require this file to know what the expansion contains.
//!
//! This is important for extensibility and POCO-REAF.
//!
//! ## ZUIR integration
//!
//! `MacroExpression` does not lower directly to ZUIR as a macro execution
//! primitive.
//!
//! The intended flow is:
//!
//! ```text
//! MacroExpression
//!       │
//!       ▼
//! macro resolution
//!       │
//!       ▼
//! macro expansion / semantic processing
//!       │
//!       ▼
//! resulting semantic model
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! A macro that expands into quantum computation therefore eventually reaches
//! the quantum domain lowering pipeline without requiring quantum-specific
//! fields in this AST node.
//!
//! ## Security
//!
//! This module:
//!
//! - forbids unsafe Rust;
//! - performs no I/O;
//! - executes no macro;
//! - performs no dynamic code loading;
//! - does not dereference raw pointers;
//! - does not recursively traverse children;
//! - does not use global mutable state;
//! - does not communicate with hardware;
//! - does not contact backend providers.
//!
//! Macro expansion is a security-sensitive compiler phase and must enforce its
//! own configurable expansion/resource policies.
//!
//! In particular, this AST node must not introduce hidden limits such as:
//!
//! ```text
//! MAX_MACRO_ARGUMENTS
//! MAX_MACROS
//! MAX_QUANTUM_MACROS
//! MAX_EXPANSION_DEPTH
//! ```
//!
//! Expansion-depth and expansion-size limits belong to explicit compiler
//! resource-policy infrastructure.
//!
//! ## Thread safety
//!
//! The node has no global mutable state.
//!
//! Immutable instances can be consumed by parallel read-only compiler phases
//! when their constituent types satisfy the relevant `Send`/`Sync` bounds.
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
//! # File contract
//!
//! **Inputs**
//!
//! - stable `NodeId`;
//! - source `Span`;
//! - `NodeMetadata`;
//! - source-level macro name;
//! - zero or more argument `NodeId`s.
//!
//! **Outputs**
//!
//! - canonical `MacroExpression`;
//! - deterministic argument traversal;
//! - local structural validation.
//!
//! **Invariants**
//!
//! - node kind is `CoreNodeKind::Macro` for nodes created with `new`;
//! - macro name is non-empty;
//! - macro name contains no NUL character;
//! - no direct argument references this node itself;
//! - argument order is preserved;
//! - repeated argument references remain legal.
//!
//! **Error model**
//!
//! Only local structural errors are reported here.
//!
//! Name resolution, parameter binding, expansion validity and expansion
//! termination are not local errors.
//!
//! **Serialization**
//!
//! Serde-compatible and deterministic with respect to stored field ordering and
//! vector ordering.
//!
//! **Visitor integration**
//!
//! Visitors should visit the macro invocation and then its arguments in source
//! order.
//!
//! **Traversal integration**
//!
//! `child_node_ids()` exposes direct argument references without recursion.
//!
//! **Parser integration**
//!
//! Parser creates the node after parsing the identifier and argument list.
//!
//! **Semantic integration**
//!
//! Semantic analysis resolves the macro and its arguments.
//!
//! **ZUIR integration**
//!
//! Macro invocation is resolved/expanded before the resulting computation is
//! lowered to ZUIR.
//!
//! **Scalability**
//!
//! No fixed number of arguments, macros, resources, machines or computational
//! domains is encoded in this type.
//!
//! **Determinism**
//!
//! Argument order is deterministic source order.
//!
//! **Forbidden dependencies**
//!
//! No semantic, IR, hardware, backend or runtime dependencies.
//!
//! **Migration**
//!
//! Legacy:
//!
//! ```text
//! Macro(Span, String, Vec<Expression>)
//! ```
//!
//! maps to:
//!
//! ```text
//! MacroExpression {
//!     node: Node,
//!     name: String,
//!     arguments: Vec<NodeId>,
//! }
//! ```
//!
//! The migration removes recursive ownership from the legacy AST while
//! preserving source-level macro intent.
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

/// Local structural schema version for macro expressions.
///
/// This is deliberately independent of the Zamani language version and the
/// global serialized AST schema version.
pub const MACRO_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for this AST construct.
pub const MACRO_EXPRESSION_KIND_NAME: &str = "zamani:macro-expression";

/// Result type for local macro-expression validation.
pub type MacroExpressionResult<T> = Result<T, MacroExpressionError>;

/// Errors detectable from the local structure of a macro invocation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MacroExpressionError {
    /// The common node has the wrong node kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// A macro invocation has no source-level name.
    EmptyName,

    /// The source-level name contains an embedded NUL character.
    InvalidName,

    /// An argument directly references the macro invocation itself.
    SelfReferentialArgument,
}

impl fmt::Display for MacroExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "macro expression has invalid AST node kind: {actual}"
                )
            }

            Self::EmptyName => {
                formatter.write_str(
                    "macro expression must have a non-empty name",
                )
            }

            Self::InvalidName => {
                formatter.write_str(
                    "macro expression name contains an invalid NUL character",
                )
            }

            Self::SelfReferentialArgument => {
                formatter.write_str(
                    "macro expression cannot directly reference itself as an argument",
                )
            }
        }
    }
}

impl std::error::Error for MacroExpressionError {}

/// A source-level Zamani macro invocation.
///
/// The node contains only source-level information:
///
/// - common AST identity and metadata;
/// - macro name;
/// - argument node references.
///
/// It does not contain macro definitions, expansion state, execution state,
/// semantic types, backend information or hardware information.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MacroExpression {
    /// Common AST identity, kind, source span and metadata.
    node: Node,

    /// Source-level macro name.
///
/// The string is intentionally preserved rather than converted into a semantic
/// symbol identity. Name resolution belongs to semantic analysis.
    name: String,

    /// Source-order argument expression node IDs.
    arguments: Vec<NodeId>,
}

/// Short alias for [`MacroExpression`].
pub type Macro = MacroExpression;

impl MacroExpression {
    /// Creates a new macro invocation.
    ///
    /// The constructor establishes the canonical `CoreNodeKind::Macro`.
    ///
    /// It intentionally does not perform semantic validation.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        arguments: Vec<NodeId>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Macro),
            span,
            metadata,
        );

        Self {
            node,
            name: name.into(),
            arguments,
        }
    }

    /// Creates a macro invocation from an existing common [`Node`].
    ///
    /// The supplied node is preserved exactly. Its node kind is validated by
    /// [`Self::validate_structure`].
    #[must_use]
    pub fn from_node(
        node: Node,
        name: impl Into<String>,
        arguments: Vec<NodeId>,
    ) -> Self {
        Self {
            node,
            name: name.into(),
            arguments,
        }
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns this macro invocation's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns this macro invocation's metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to this macro invocation's metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Replaces this invocation's metadata.
    ///
    /// Returns the previous metadata.
    #[inline]
    pub fn replace_metadata(
        &mut self,
        metadata: NodeMetadata,
    ) -> NodeMetadata {
        self.node.replace_metadata(metadata)
    }

    /// Returns the source-level macro name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source-level macro name.
    ///
    /// This does not perform name resolution.
    ///
    /// Returns the previous name.
    pub fn replace_name(
        &mut self,
        name: impl Into<String>,
    ) -> String {
        std::mem::replace(&mut self.name, name.into())
    }

    /// Sets the source-level macro name.
    #[inline]
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Returns the source-order macro arguments.
    #[inline]
    #[must_use]
    pub fn arguments(&self) -> &[NodeId] {
        &self.arguments
    }

    /// Returns mutable access to the argument list.
    ///
    /// Structural validation should be run after mutating the argument list.
    #[inline]
    pub fn arguments_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.arguments
    }

    /// Replaces the complete argument list.
    ///
    /// The previous list is returned.
    #[inline]
    pub fn replace_arguments(
        &mut self,
        arguments: Vec<NodeId>,
    ) -> Vec<NodeId> {
        std::mem::replace(&mut self.arguments, arguments)
    }

    /// Returns the number of direct argument children.
    #[inline]
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns whether the invocation has no arguments.
    #[inline]
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns the direct child at `index`.
    ///
    /// This is a checked access operation.
    #[inline]
    #[must_use]
    pub fn argument(&self, index: usize) -> Option<NodeId> {
        self.arguments.get(index).copied()
    }

    /// Returns the direct AST children in deterministic source order.
    ///
    /// No allocation or recursive traversal is performed.
    #[inline]
    pub fn child_node_ids(
        &self,
    ) -> impl Iterator<Item = NodeId> + '_ {
        self.arguments.iter().copied()
    }

    /// Alias for [`Self::child_node_ids`].
    #[inline]
    pub fn children(
        &self,
    ) -> impl Iterator<Item = NodeId> + '_ {
        self.child_node_ids()
    }

    /// Returns the number of direct AST children.
    #[inline]
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns the canonical node kind expected by this structure.
    #[inline]
    #[must_use]
    pub const fn expected_node_kind() -> NodeKind {
        NodeKind::core(CoreNodeKind::Macro)
    }

    /// Returns the local structural schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        MACRO_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns the stable source-level construct name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        MACRO_EXPRESSION_KIND_NAME
    }

    /// Validates invariants that can be checked without access to the complete
    /// AST graph or semantic model.
    ///
    /// This method intentionally does not verify that argument IDs exist in
    /// the global AST.
    pub fn validate_structure(
        &self,
    ) -> MacroExpressionResult<()> {
        let expected = Self::expected_node_kind();

        if self.node.kind() != &expected {
            return Err(MacroExpressionError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        if self.name.is_empty() {
            return Err(MacroExpressionError::EmptyName);
        }

        if self.name.contains('\0') {
            return Err(MacroExpressionError::InvalidName);
        }

        for argument in &self.arguments {
            if *argument == self.id() {
                return Err(
                    MacroExpressionError::SelfReferentialArgument,
                );
            }
        }

        Ok(())
    }

    /// Returns whether the node satisfies all local structural invariants.
    #[inline]
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }

    /// Consumes the node and returns its common AST node.
    #[must_use]
    pub fn into_node(self) -> Node {
        self.node
    }

    /// Consumes the node and returns its macro name.
    #[must_use]
    pub fn into_name(self) -> String {
        self.name
    }

    /// Consumes the node and returns its argument list.
    #[must_use]
    pub fn into_arguments(self) -> Vec<NodeId> {
        self.arguments
    }

    /// Consumes the node and returns `(Node, name, arguments)`.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (Node, String, Vec<NodeId>) {
        (self.node, self.name, self.arguments)
    }
}

impl AstNode for MacroExpression {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for MacroExpression {
    /// Formats the structural representation of the invocation.
    ///
    /// Because child AST nodes are represented by `NodeId`, this method does
    /// not attempt to reconstruct complete source text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}!(<{} argument{}>)",
            self.name,
            self.arguments.len(),
            if self.arguments.len() == 1 {
                ""
            } else {
                "s"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::source::source_id::SourceId;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
            .expect("test NodeId must be non-zero")
    }

    fn span() -> Span {
        Span::new(SourceId::new(1), 0, 16)
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn macro_expression(
        id: u64,
        name: &str,
        arguments: &[u64],
    ) -> MacroExpression {
        MacroExpression::new(
            node_id(id),
            span(),
            metadata(),
            name,
            arguments
                .iter()
                .copied()
                .map(node_id)
                .collect(),
        )
    }

    #[test]
    fn constructs_macro_expression() {
        let expression =
            macro_expression(1, "compute", &[2, 3]);

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.name(), "compute");
        assert_eq!(expression.argument_count(), 2);
    }

    #[test]
    fn uses_canonical_macro_node_kind() {
        let expression =
            macro_expression(1, "compute", &[2, 3]);

        assert_eq!(
            expression.kind(),
            &NodeKind::core(CoreNodeKind::Macro)
        );
    }

    #[test]
    fn expected_node_kind_is_canonical() {
        assert_eq!(
            MacroExpression::expected_node_kind(),
            NodeKind::core(CoreNodeKind::Macro)
        );
    }

    #[test]
    fn preserves_argument_order() {
        let expression =
            macro_expression(1, "compute", &[2, 3, 4]);

        let children: Vec<NodeId> =
            expression.child_node_ids().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3), node_id(4)]
        );
    }

    #[test]
    fn child_count_matches_argument_count() {
        let expression =
            macro_expression(1, "compute", &[2, 3, 4]);

        assert_eq!(
            expression.child_count(),
            expression.argument_count()
        );
    }

    #[test]
    fn zero_argument_macro_is_valid() {
        let expression =
            macro_expression(1, "initialize", &[]);

        assert!(expression.has_no_arguments());
        assert_eq!(expression.validate_structure(), Ok(()));
    }

    #[test]
    fn repeated_argument_references_are_valid() {
        let expression =
            macro_expression(1, "duplicate", &[2, 2]);

        assert_eq!(expression.argument_count(), 2);
        assert!(expression.is_structurally_valid());
    }

    #[test]
    fn empty_name_is_rejected() {
        let expression =
            macro_expression(1, "", &[]);

        assert_eq!(
            expression.validate_structure(),
            Err(MacroExpressionError::EmptyName)
        );
    }

    #[test]
    fn nul_name_is_rejected() {
        let expression =
            macro_expression(1, "compute\0macro", &[]);

        assert_eq!(
            expression.validate_structure(),
            Err(MacroExpressionError::InvalidName)
        );
    }

    #[test]
    fn direct_self_reference_is_rejected() {
        let expression = MacroExpression::new(
            node_id(1),
            span(),
            metadata(),
            "recursive",
            vec![node_id(1)],
        );

        assert_eq!(
            expression.validate_structure(),
            Err(
                MacroExpressionError::SelfReferentialArgument
            )
        );
    }

    #[test]
    fn valid_structure_passes_validation() {
        let expression =
            macro_expression(1, "compute", &[2, 3]);

        assert_eq!(expression.validate_structure(), Ok(()));
        assert!(expression.is_structurally_valid());
    }

    #[test]
    fn checked_argument_access_is_safe() {
        let expression =
            macro_expression(1, "compute", &[2, 3]);

        assert_eq!(
            expression.argument(0),
            Some(node_id(2))
        );

        assert_eq!(
            expression.argument(1),
            Some(node_id(3))
        );

        assert_eq!(expression.argument(2), None);
    }

    #[test]
    fn empty_argument_list_is_allocation_free_for_traversal() {
        let expression =
            macro_expression(1, "initialize", &[]);

        assert_eq!(
            expression.child_node_ids().count(),
            0
        );
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(
            MacroExpression::schema_version(),
            MACRO_EXPRESSION_SCHEMA_VERSION
        );

        assert_eq!(
            MacroExpression::kind_name(),
            "zamani:macro-expression"
        );
    }

    #[test]
    fn node_id_maximum_representable_value_is_supported() {
        let id = node_id(u64::MAX);
        let argument = node_id(u64::MAX - 1);

        let expression = MacroExpression::new(
            id,
            span(),
            metadata(),
            "large",
            vec![argument],
        );

        assert_eq!(expression.id(), id);
        assert_eq!(
            expression.argument(0),
            Some(argument)
        );
        assert!(expression.is_structurally_valid());
    }

    #[test]
    fn display_does_not_require_recursive_ast_traversal() {
        let expression =
            macro_expression(1, "compute", &[2, 3]);

        assert_eq!(
            expression.to_string(),
            "compute!(<2 arguments>)"
        );
    }
}