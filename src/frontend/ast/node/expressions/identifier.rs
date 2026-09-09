//! # Zamani Native AST — Identifier Expression
//!
//! This module defines the canonical source-level identifier expression used
//! by the Zamani frontend AST.
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
//! IdentifierExpression  ← this module
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ├── name resolution
//!     ├── scope resolution
//!     ├── type resolution
//!     ├── capability resolution
//!     ├── resource resolution
//!     └── domain resolution
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ## Core responsibility
//!
//! `IdentifierExpression` represents the spelling of an identifier appearing
//! in expression position.
//!
//! It records only source-level information:
//!
//! - AST identity;
//! - source node classification;
//! - source span;
//! - source-level metadata;
//! - identifier spelling.
//!
//! It deliberately does **not** record:
//!
//! - resolved symbols;
//! - lexical scopes;
//! - declaration IDs;
//! - type information;
//! - generic substitutions;
//! - resource identities;
//! - physical qubit identifiers;
//! - machine registers;
//! - hardware addresses;
//! - backend identifiers;
//! - QIR values;
//! - LLVM values;
//! - MLIR values;
//! - runtime handles;
//! - memory addresses;
//! - optimization state;
//! - scheduling state.
//!
//! Those concepts belong to later compiler stages.
//!
//! ## POCO-REAF
//!
//! An identifier is intentionally independent of the eventual computational
//! realization of the program.
//!
//! The same source-level identifier can ultimately refer to:
//!
//! - a classical value;
//! - a quantum resource;
//! - a resource collection;
//! - a function;
//! - an operation;
//! - an object;
//! - a module member;
//! - an accelerator abstraction;
//! - a distributed value;
//! - a future computational abstraction.
//!
//! Determining that meaning is semantic analysis, not parsing.
//!
//! Consequently, this file contains no:
//!
//! - qubit count;
//! - machine-size assumption;
//! - topology;
//! - vendor;
//! - backend;
//! - gate set;
//! - target architecture.
//!
//! This is necessary for Program Once, Compile Once, Run Everywhere/Anywhere/
//! Forever (POCO-REAF).
//!
//! ## Relationship with paths
//!
//! An identifier is one source-level name.
//!
//! A qualified name such as:
//!
//! ```text
//! math.linear.transform
//! ```
//!
//! is not represented by adding path semantics to this type.
//!
//! Path structure belongs to the canonical path AST under the paths subsystem.
//!
//! Conceptually:
//!
//! ```text
//! IdentifierExpression
//!     │
//!     └── name = "transform"
//!
//! PathExpression / Path
//!     │
//!     ├── "math"
//!     ├── "linear"
//!     └── "transform"
//! ```
//!
//! This separation prevents identifiers and paths from becoming competing
//! representations of the same concept.
//!
//! ## Relationship with declarations
//!
//! Declaration nodes may contain an identifier spelling as their declared name.
//!
//! Expression identifiers are references appearing in expression position.
//!
//! Neither representation resolves the name.
//!
//! ```text
//! declaration name ──────┐
//!                         │
//!                         ▼
//!                    semantic resolver
//!                         ▲
//!                         │
//! expression identifier ─┘
//! ```
//!
//! The semantic layer establishes the relationship between the declaration
//! and reference.
//!
//! ## Relationship with quantum computing
//!
//! Quantum-specific names must not be hard-coded here.
//!
//! For example, this module must not contain:
//!
//! ```text
//! QubitIdentifier
//! QuantumRegisterIdentifier
//! PhysicalQubitIdentifier
//! LogicalQubitIdentifier
//! GateIdentifier
//! ```
//!
//! Those would prematurely couple the native AST to one computational domain.
//!
//! Instead, a source expression such as:
//!
//! ```text
//! state
//! ```
//!
//! remains an ordinary identifier.
//!
//! If `state` is semantically a quantum resource, that meaning is established
//! downstream.
//!
//! Similarly:
//!
//! ```text
//! algorithm.qft
//! ```
//!
//! is represented using the normal identifier/path mechanisms rather than a
//! special QFT AST node.
//!
//! ## Relationship with the legacy AST
//!
//! The legacy AST represented identifiers as:
//!
//! ```text
//! Identifier(String, Span)
//! ```
//!
//! The new AST separates common node identity/source information into `Node`
//! and keeps the identifier spelling as the expression-specific payload.
//!
//! Conceptually:
//!
//! ```text
//! legacy:
//!
//! Identifier(name, span)
//!
//! new:
//!
//! IdentifierExpression
//! ├── Node
//! │   ├── NodeId
//! │   ├── NodeKind::Identifier
//! │   ├── Span
//! │   └── metadata
//! └── name
//! ```
//!
//! This gives identifiers the same identity, source mapping, metadata,
//! traversal, validation, and serialization contract as every other native
//! AST node.
//!
//! ## Child representation
//!
//! An identifier expression has no AST children.
//!
//! The identifier spelling is terminal source data.
//!
//! Therefore its canonical child sequence is:
//!
//! ```text
//! IdentifierExpression
//! └── no children
//! ```
//!
//! This is important for generic visitors and iterative traversal.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native-AST infrastructure:
//!
//! - [`Node`];
//! - [`AstNode`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - Serde;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - symbol tables;
//! - type checking;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - backend providers;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - execution;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - external quantum-language ASTs.
//!
//! ## Structural validation boundary
//!
//! This module validates only local structural properties.
//!
//! It does not validate whether an identifier:
//!
//! - exists;
//! - is declared;
//! - is visible;
//! - has a particular type;
//! - refers to a resource;
//! - is usable as a quantum operand;
//! - satisfies a capability;
//! - is supported by a target.
//!
//! Those checks belong downstream.
//!
//! ## Lexical validation boundary
//!
//! Whether a spelling is a valid Zamani identifier according to the language's
//! lexical grammar belongs to the lexer/parser.
//!
//! This type must not duplicate the complete lexical grammar.
//!
//! `validate_structure` therefore checks only the invariant that the AST
//! contains an actual identifier spelling rather than an empty source name.
//!
//! This avoids coupling the AST to a particular lexer implementation and makes
//! future lexical evolution possible without rewriting the AST.
//!
//! ## Unicode
//!
//! The identifier is stored as a Rust `String` and therefore preserves valid
//! UTF-8 source text.
//!
//! No ASCII-only assumption is made here.
//!
//! Whether a particular Unicode spelling is lexically legal is determined by
//! the lexer/parser.
//!
//! ## Scalability
//!
//! This type contains no fixed-size buffers and imposes no language-level limit
//! on:
//!
//! - identifier length;
//! - number of identifiers;
//! - number of expressions;
//! - number of resources;
//! - number of qubits;
//! - number of machines;
//! - number of operations.
//!
//! Any hostile-input or compiler-resource limits belong to configurable
//! compiler/validation policy rather than this AST node.
//!
//! ## Determinism
//!
//! Equality, hashing, and serialization depend only on the logical AST data.
//!
//! No:
//!
//! - memory addresses;
//! - process IDs;
//! - timestamps;
//! - random values;
//! - thread-local state;
//! - backend state
//!
//! are included.
//!
//! ## Serialization
//!
//! Serde serialization preserves:
//!
//! - node identity;
//! - node classification;
//! - source span;
//! - metadata;
//! - identifier spelling.
//!
//! AST-wide schema versioning belongs to the AST serialization subsystem.
//! This file must not introduce a competing serialization-version mechanism.
//!
//! ## Visitor/traversal integration
//!
//! `child_node_ids()` returns an empty iterator because identifiers are leaf
//! nodes.
//!
//! Generic traversal infrastructure can therefore process an identifier
//! without knowing anything about its semantic meaning.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes this node and resolves its spelling using the
//! current lexical/module/symbol environment.
//!
//! The semantic layer may associate the node's [`NodeId`] with a resolved
//! symbol through a side table.
//!
//! The AST itself remains unchanged.
//!
//! ## ZUIR integration
//!
//! This file must not import ZUIR.
//!
//! A resolved identifier may eventually become a ZUIR value, resource reference,
//! callable reference, constant, or another semantic entity depending on what
//! the identifier denotes.
//!
//! That transformation is owned by semantic analysis/lowering.
//!
//! ## Error behavior
//!
//! Construction does not panic for ordinary malformed source spelling.
//!
//! Structural invalidity is reported through [`IdentifierValidationError`].
//!
//! This is preferable to embedding parser policy or panic-based validation in
//! the AST.
//!
//! ## Thread safety
//!
//! The type contains owned immutable source data and foundational AST values.
//!
//! It contains no global mutable state and no synchronization primitives.
//!
//! Read-only instances can therefore participate in parallel compiler phases
//! whenever the containing AST and its source infrastructure satisfy the
//! corresponding `Send`/`Sync` requirements.
//!
//! ## Safety
//!
//! This implementation contains no `unsafe` code.
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
//! ## Integration checklist
//!
//! The surrounding AST integration should provide:
//!
//! ```text
//! src/frontend/ast/node/expressions/mod.rs
//!     └── pub mod identifier;
//!
//! src/frontend/ast/node/mod.rs
//!     └── expressions module exposure;
//!
//! parser
//!     └── creates IdentifierExpression
//!
//! structural validation
//!     └── calls validate_structure
//!
//! visitors/traversal
//!     └── treats IdentifierExpression as a leaf
//!
//! semantic analysis
//!     └── resolves IdentifierExpression::name()
//!
//! ZUIR lowering
//!     └── consumes the semantic resolution, not this AST node directly
//! ```
//!
//! No change to this file is required merely because a new quantum backend,
//! quantum technology, computational domain, or target architecture is added.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Stable schema identifier for the identifier-expression contract.
///
/// This is deliberately separate from the global AST serialization schema
/// version. It identifies the logical contract of this node type for
/// documentation, tooling, and compatibility checks.
pub const IDENTIFIER_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level kind identifier for identifier expressions.
pub const IDENTIFIER_EXPRESSION_KIND_NAME: &str = "zamani:identifier";

/// Source-level identifier expression.
///
/// This is a terminal native-AST node containing the spelling of a source
/// identifier. It has no semantic resolution state.
///
/// # Invariants
///
/// A structurally valid identifier expression:
///
/// 1. contains a valid common [`Node`];
/// 2. uses [`CoreNodeKind::Identifier`] as its node classification;
/// 3. contains a non-empty identifier spelling;
/// 4. contains no semantic/backend information;
/// 5. has no child AST nodes.
///
/// # Example
///
/// ```text
/// answer
/// ```
///
/// is represented conceptually as:
///
/// ```text
/// IdentifierExpression
/// ├── node = Node(...)
/// └── name = "answer"
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentifierExpression {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Original source spelling of the identifier.
    ///
    /// This is intentionally not a resolved symbol.
    name: String,
}

/// Short canonical alias for callers that prefer the expression-oriented name.
///
/// `IdentifierExpression` remains the canonical public type name.
pub type Identifier = IdentifierExpression;

impl IdentifierExpression {
    /// Creates an identifier expression from an already constructed AST node.
    ///
    /// The supplied node is expected to have
    /// `CoreNodeKind::Identifier`.
    ///
    /// This constructor does not rewrite the supplied node's classification.
    /// That makes accidental misuse observable to structural validation rather
    /// than silently changing AST identity.
    #[must_use]
    pub fn from_node(node: Node, name: impl Into<String>) -> Self {
        Self {
            node,
            name: name.into(),
        }
    }

    /// Creates a new identifier expression.
    ///
    /// The node is classified as the native `Identifier` node kind.
    ///
    /// The constructor does not perform semantic or lexical validation.
    /// Structural validation is available through [`Self::validate_structure`].
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Identifier),
            span,
            metadata,
        );

        Self::from_node(node, name)
    }

    /// Creates a new identifier expression with empty metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
    ) -> Self {
        Self::new(id, span, NodeMetadata::default(), name)
    }

    /// Returns the identifier's source spelling.
    ///
    /// No semantic resolution is performed.
    #[must_use]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of UTF-8 bytes in the source spelling.
    ///
    /// This is a source-storage property, not a language-level identifier
    /// limit.
    #[must_use]
    #[inline]
    pub fn name_byte_len(&self) -> usize {
        self.name.len()
    }

    /// Returns `true` when the identifier spelling is empty.
    ///
    /// An empty spelling is structurally invalid for an identifier node.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Replaces the identifier spelling.
    ///
    /// Lexical validity remains the responsibility of the lexer/parser.
    /// Structural validation can subsequently be run with
    /// [`Self::validate_structure`].
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), IdentifierValidationError> {
        let name = name.into();

        if name.is_empty() {
            return Err(IdentifierValidationError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Replaces the identifier spelling without performing structural
    /// validation.
    ///
    /// This method exists for controlled AST transformation infrastructure.
    /// Callers should validate the resulting AST before handing it to semantic
    /// analysis.
    pub fn replace_name(&mut self, name: impl Into<String>) -> String {
        std::mem::replace(&mut self.name, name.into())
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// This is required by [`AstNode`] and controlled AST transformation
    /// infrastructure.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node ID.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span.
    #[must_use]
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the canonical node kind.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the stable source-level kind identifier.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        IDENTIFIER_EXPRESSION_KIND_NAME
    }

    /// Returns this node's schema version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        IDENTIFIER_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns whether this identifier expression has any AST children.
    ///
    /// Identifiers are terminal nodes, so this always returns `false`.
    #[must_use]
    #[inline]
    pub const fn is_leaf() -> bool {
        true
    }

    /// Returns the number of AST children.
    ///
    /// This always returns zero because an identifier is a terminal AST node.
    #[must_use]
    #[inline]
    pub const fn child_count(&self) -> usize {
        0
    }

    /// Returns an iterator over child node IDs.
    ///
    /// The iterator is empty because an identifier expression has no children.
    ///
    /// The return type intentionally avoids allocating a `Vec<NodeId>` for a
    /// leaf node.
    #[must_use]
    #[inline]
    pub fn child_node_ids(
        &self,
    ) -> std::iter::Empty<NodeId> {
        std::iter::empty()
    }

    /// Validates the local structural invariants of this identifier.
    ///
    /// This method intentionally does not perform lexical, semantic, type,
    /// scope, capability, resource, or target validation.
    ///
    /// # Errors
    ///
    /// Returns [`IdentifierValidationError`] when:
    ///
    /// - the common node is not classified as an identifier;
    /// - the identifier spelling is empty.
    pub fn validate_structure(
        &self,
    ) -> Result<(), IdentifierValidationError> {
        if self.node.kind()
            != &NodeKind::core(CoreNodeKind::Identifier)
        {
            return Err(IdentifierValidationError::InvalidNodeKind {
                actual: self.node.kind().clone(),
            });
        }

        if self.name.is_empty() {
            return Err(IdentifierValidationError::EmptyName);
        }

        Ok(())
    }

    /// Returns a compact diagnostic representation.
    ///
    /// The complete metadata payload is intentionally excluded so diagnostic
    /// formatting cannot accidentally materialize a large extension payload.
    #[must_use]
    pub fn diagnostic_summary(&self) -> IdentifierDiagnosticSummary {
        IdentifierDiagnosticSummary {
            id: self.id(),
            span: self.span().clone(),
            name: self.name.clone(),
        }
    }
}

impl AstNode for IdentifierExpression {
    /// Returns the embedded common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded common AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Structural errors specific to an identifier expression.
///
/// These errors deliberately describe only local AST invariants.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdentifierValidationError {
    /// The identifier has no source spelling.
    EmptyName,

    /// The embedded node has a classification other than `Identifier`.
    InvalidNodeKind {
        /// Actual node classification found in the AST.
        actual: NodeKind,
    },
}

impl core::fmt::Display for IdentifierValidationError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "identifier expression has an empty name",
                )
            }

            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "identifier expression has invalid node kind: {actual}"
                )
            }
        }
    }
}

impl std::error::Error for IdentifierValidationError {}

/// Lightweight diagnostic representation of an identifier expression.
///
/// The summary contains enough information for diagnostics without requiring
/// consumers to expose the entire AST node or arbitrary metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentifierDiagnosticSummary {
    /// Stable AST identity.
    pub id: NodeId,

    /// Source span of the identifier.
    pub span: Span,

    /// Original source spelling.
    pub name: String,
}

impl core::fmt::Display for IdentifierDiagnosticSummary {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(
            formatter,
            "{} at {}",
            self.name,
            self.span
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_id() -> NodeId {
        NodeId::new(1)
    }

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn constructs_identifier() {
        let identifier = IdentifierExpression::without_metadata(
            test_id(),
            test_span(),
            "answer",
        );

        assert_eq!(identifier.name(), "answer");
        assert_eq!(identifier.id(), test_id());
        assert_eq!(
            identifier.kind(),
            &NodeKind::core(CoreNodeKind::Identifier)
        );
        assert_eq!(identifier.child_count(), 0);
        assert!(identifier.is_leaf());
    }

    #[test]
    fn preserves_unicode_identifier_spelling() {
        let identifier = IdentifierExpression::without_metadata(
            test_id(),
            test_span(),
            "résultat",
        );

        assert_eq!(identifier.name(), "résultat");
        assert!(!identifier.is_empty());
    }

    #[test]
    fn rejects_empty_name_during_validation() {
        let identifier = IdentifierExpression::without_metadata(
            test_id(),
            test_span(),
            "",
        );

        assert_eq!(
            identifier.validate_structure(),
            Err(IdentifierValidationError::EmptyName)
        );
    }

    #[test]
    fn set_name_rejects_empty_name() {
        let mut identifier =
            IdentifierExpression::without_metadata(
                test_id(),
                test_span(),
                "value",
            );

        let result = identifier.set_name("");

        assert_eq!(
            result,
            Err(IdentifierValidationError::EmptyName)
        );
        assert_eq!(identifier.name(), "value");
    }

    #[test]
    fn set_name_accepts_non_empty_name() {
        let mut identifier =
            IdentifierExpression::without_metadata(
                test_id(),
                test_span(),
                "value",
            );

        identifier
            .set_name("other")
            .expect("non-empty name must be accepted");

        assert_eq!(identifier.name(), "other");
    }

    #[test]
    fn validates_correct_node_kind() {
        let identifier =
            IdentifierExpression::without_metadata(
                test_id(),
                test_span(),
                "value",
            );

        assert!(identifier.validate_structure().is_ok());
    }

    #[test]
    fn child_iteration_is_empty() {
        let identifier =
            IdentifierExpression::without_metadata(
                test_id(),
                test_span(),
                "value",
            );

        assert_eq!(
            identifier.child_node_ids().count(),
            0
        );
    }

    #[test]
    fn diagnostic_summary_is_compact() {
        let identifier =
            IdentifierExpression::without_metadata(
                test_id(),
                test_span(),
                "value",
            );

        let summary = identifier.diagnostic_summary();

        assert_eq!(summary.id, test_id());
        assert_eq!(summary.name, "value");
    }

    #[test]
    fn clone_preserves_identity() {
        let identifier =
            IdentifierExpression::without_metadata(
                test_id(),
                test_span(),
                "value",
            );

        let cloned = identifier.clone();

        assert_eq!(identifier, cloned);
        assert_eq!(identifier.id(), cloned.id());
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(
            IdentifierExpression::schema_version(),
            IDENTIFIER_EXPRESSION_SCHEMA_VERSION
        );

        assert_eq!(
            IdentifierExpression::kind_name(),
            IDENTIFIER_EXPRESSION_KIND_NAME
        );
    }
}