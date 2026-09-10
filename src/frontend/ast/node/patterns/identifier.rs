//! # Zamani Native AST — Identifier Pattern
//!
//! This module defines the canonical source-level identifier pattern used by
//! the Zamani frontend AST.
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
//! IdentifierPattern
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ├── name/scope resolution
//!     ├── binding analysis
//!     ├── type analysis
//!     ├── ownership/resource analysis
//!     ├── capability analysis
//!     └── domain analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ## Scope
//!
//! `IdentifierPattern` represents the plain identifier alternative of the
//! Zamani pattern grammar:
//!
//! ```text
//! IDENTIFIER
//! ```
//!
//! It does **not** represent:
//!
//! ```text
//! IDENTIFIER '@' pattern
//! ```
//!
//! The latter is a binding/at-pattern and must have its own AST node.
//!
//! Likewise, this type does not represent:
//!
//! - qualified paths;
//! - field patterns;
//! - enum patterns;
//! - struct patterns;
//! - wildcard patterns;
//! - reference patterns;
//! - range patterns;
//! - OR patterns;
//! - literal patterns.
//!
//! Those are separate source constructs.
//!
//! ## Core architectural rule
//!
//! This node records source-language structure, not semantic meaning.
//!
//! The identifier may eventually denote:
//!
//! - a local binding;
//! - a parameter;
//! - a value;
//! - a resource;
//! - a quantum resource;
//! - a collection of quantum resources;
//! - a classical value;
//! - a distributed resource;
//! - another future computational abstraction.
//!
//! Determining what the identifier denotes belongs to semantic analysis.
//!
//! ## POCO-REAF
//!
//! This type deliberately contains no information about:
//!
//! - machine size;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - gate set;
//! - processor architecture;
//! - physical qubit;
//! - logical qubit;
//! - scheduler;
//! - routing;
//! - calibration;
//! - QEC implementation;
//! - resilience implementation;
//! - runtime;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target instructions.
//!
//! Therefore an identifier pattern remains unchanged when the same Zamani
//! program is compiled for different computational scales or technologies.
//!
//! This is required for:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## Relationship with `IdentifierExpression`
//!
//! An identifier expression and an identifier pattern have similar source
//! spelling but different grammatical roles.
//!
//! ```text
//! expression:
//!     identifier
//!
//! pattern:
//!     identifier
//! ```
//!
//! They must not be represented by one type merely because their textual
//! spelling is identical.
//!
//! Their semantic roles differ:
//!
//! ```text
//! IdentifierExpression
//!     └── reference/use position
//!
//! IdentifierPattern
//!     └── pattern/binding position
//! ```
//!
//! Semantic analysis determines the actual binding semantics.
//!
//! ## Relationship with binding patterns
//!
//! The grammar separately supports:
//!
//! ```text
//! IDENTIFIER '@' pattern
//! ```
//!
//! That construct has two semantic components:
//!
//! ```text
//! binding-name @ nested-pattern
//! ```
//!
//! It must therefore not be encoded by adding optional child state to this
//! leaf node. Keeping `IdentifierPattern` a true leaf makes generic traversal
//! deterministic and prevents unrelated pattern variants from accumulating
//! inside one structure.
//!
//! ## Relationship with paths
//!
//! A plain identifier is one name.
//!
//! A qualified name belongs to the canonical path subsystem.
//!
//! This type must not grow path-specific fields merely because another pattern
//! may eventually contain a path.
//!
//! ## Relationship with quantum computation
//!
//! Consider:
//!
//! ```text
//! match resource {
//!     q => ...
//! }
//! ```
//!
//! `q` is represented here as an ordinary identifier pattern.
//!
//! If semantic analysis determines that `q` binds a quantum resource, that
//! information belongs to the semantic model.
//!
//! The AST must not become quantum-specific merely because the identifier is
//! eventually bound to a qubit, logical resource, register, or distributed
//! quantum resource.
//!
//! ## Dependency boundary
//!
//! This module may depend only on foundational native-AST infrastructure:
//!
//! - `Node`;
//! - `AstNode`;
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
//! - semantic analysis;
//! - symbol tables;
//! - type checking;
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
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - external quantum-language ASTs.
//!
//! ## Structural validation
//!
//! This file validates only local invariants:
//!
//! 1. the common node is classified as `CoreNodeKind::Identifier`;
//! 2. the identifier spelling is not empty;
//! 3. the node is a leaf.
//!
//! It does not validate whether the identifier:
//!
//! - exists;
//! - is declared;
//! - is visible;
//! - is shadowed;
//! - has a type;
//! - is a legal binding in a particular context;
//! - represents a quantum resource;
//! - satisfies ownership rules;
//! - satisfies a target capability.
//!
//! Those checks belong to later phases.
//!
//! ## Lexical validation
//!
//! The lexer/parser owns the language's lexical identifier grammar.
//!
//! This node intentionally does not duplicate that grammar.
//!
//! Consequently, this file does not impose an ASCII-only policy or invent a
//! second identifier grammar.
//!
//! Unicode source text is preserved through Rust's UTF-8 `String`.
//!
//! ## Scalability
//!
//! There is no language-level maximum for:
//!
//! - identifier length;
//! - number of identifier patterns;
//! - number of patterns;
//! - number of AST nodes;
//! - number of bindings;
//! - number of quantum resources;
//! - number of machines.
//!
//! Compiler resource limits, when necessary for hostile-input protection,
//! belong to configurable compiler policy.
//!
//! No fixed machine-size constant exists in this module.
//!
//! ## Determinism
//!
//! Equality, hashing and serialization depend only on logical AST state.
//!
//! This type contains no:
//!
//! - memory address;
//! - timestamp;
//! - process ID;
//! - random state;
//! - thread-local state;
//! - backend state.
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
//! Global AST serialization/versioning remains owned by the AST serialization
//! subsystem. This file does not create a competing serialization protocol.
//!
//! ## Traversal
//!
//! `IdentifierPattern` is a leaf node.
//!
//! It therefore has zero AST children.
//!
//! This is important for:
//!
//! - visitors;
//! - iterative traversal;
//! - deterministic traversal;
//! - large ASTs;
//! - deep pattern structures.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes:
//!
//! ```text
//! IdentifierPattern::name()
//! ```
//!
//! together with the surrounding pattern context.
//!
//! The semantic phase may associate the node's `NodeId` with a resolved symbol
//! or binding through a side table.
//!
//! No semantic state is stored in this node.
//!
//! ## ZUIR integration
//!
//! This file does not import ZUIR.
//!
//! An identifier pattern may eventually lower to a semantic binding, resource
//! reference, value binding, or another semantic construct. That decision is
//! made after name and type resolution.
//!
//! ## Parser integration contract
//!
//! The parser should construct this node for the plain:
//!
//! ```text
//! IDENTIFIER
//! ```
//!
//! pattern alternative.
//!
//! The parser must not use this node for:
//!
//! ```text
//! IDENTIFIER '@' pattern
//! ```
//!
//! or any qualified/compound pattern.
//!
//! ## Error behavior
//!
//! Construction does not panic for an empty spelling. An empty spelling can be
//! created by programmatic AST construction and is rejected by structural
//! validation.
//!
//! `set_name` rejects an empty replacement so an already-valid node cannot be
//! silently changed into an invalid identifier pattern.
//!
//! ## Thread safety
//!
//! The type owns its source spelling and contains no global mutable state.
//!
//! Read-only instances can participate in parallel compiler phases whenever the
//! containing AST infrastructure satisfies the corresponding `Send`/`Sync`
//! requirements.
//!
//! ## Safety
//!
//! This implementation uses no `unsafe` code.
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
//! ```text
//! src/frontend/ast/node/patterns/mod.rs
//!     └── pub mod identifier;
//!
//! src/frontend/ast/node/mod.rs
//!     └── patterns module exposure
//!
//! parser
//!     └── IDENTIFIER → IdentifierPattern
//!
//! structural validation
//!     └── IdentifierPattern::validate_structure()
//!
//! visitors
//!     └── visit identifier pattern as leaf
//!
//! traversal
//!     └── zero child node IDs
//!
//! semantic analysis
//!     └── resolve name/binding semantics
//!
//! ZUIR lowering
//!     └── consume resolved semantic representation
//! ```
//!
//! Adding a new quantum backend, hardware technology, machine size,
//! computational domain, or target architecture must not require changing
//! this file.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Stable contract version for `IdentifierPattern`.
///
/// This identifies the logical node contract. It is intentionally independent
/// of the global AST serialization schema version and the Zamani language
/// version.
pub const IDENTIFIER_PATTERN_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for this AST node kind.
pub const IDENTIFIER_PATTERN_KIND_NAME: &str = "zamani:identifier-pattern";

/// Source-level identifier pattern.
///
/// This represents the plain `IDENTIFIER` pattern alternative.
///
/// It is deliberately a leaf node. Binding, type, symbol, ownership, resource,
/// capability and domain semantics are resolved later.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentifierPattern {
    /// Common source-level AST identity, kind, span and metadata.
    node: Node,

    /// Original source spelling.
    ///
    /// This is not a resolved symbol and must remain source-level data.
    name: String,
}

impl IdentifierPattern {
    /// Creates an identifier pattern from an existing AST node.
    ///
    /// The supplied node is not rewritten. If it does not have
    /// `CoreNodeKind::Identifier`, structural validation will report the
    /// mismatch.
    ///
    /// This behavior is intentional: constructors must not silently repair
    /// malformed AST structure.
    #[must_use]
    pub fn from_node(node: Node, name: impl Into<String>) -> Self {
        Self {
            node,
            name: name.into(),
        }
    }

    /// Creates a canonical identifier pattern.
    ///
    /// The node is assigned the native `Identifier` classification.
    ///
    /// This constructor performs no lexical or semantic validation.
    /// Call [`Self::validate_structure`] when structural validity is required.
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

    /// Creates a canonical identifier pattern with empty metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
    ) -> Self {
        Self::new(id, span, NodeMetadata::default(), name)
    }

    /// Returns the source spelling.
    ///
    /// No semantic lookup is performed.
    #[must_use]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of UTF-8 bytes in the source spelling.
    ///
    /// This is informational only and is not a language-level identifier
    /// length limit.
    #[must_use]
    #[inline]
    pub fn name_byte_len(&self) -> usize {
        self.name.len()
    }

    /// Returns the number of Unicode scalar values in the source spelling.
    ///
    /// This does not impose or imply a lexical limit.
    #[must_use]
    #[inline]
    pub fn name_char_len(&self) -> usize {
        self.name.chars().count()
    }

    /// Returns whether the spelling is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Returns whether this pattern is a leaf.
    ///
    /// Always `true` because the identifier spelling is terminal source data.
    #[must_use]
    #[inline]
    pub const fn is_leaf(&self) -> bool {
        true
    }

    /// Returns the canonical number of AST children.
    ///
    /// Identifier patterns contain no child AST nodes.
    #[must_use]
    #[inline]
    pub const fn child_count(&self) -> usize {
        0
    }

    /// Returns an empty child-node iterator.
    ///
    /// The explicit empty slice keeps traversal allocation-free.
    #[must_use]
    #[inline]
    pub fn child_node_ids(&self) -> std::slice::Iter<'_, NodeId> {
        static CHILDREN: [NodeId; 0] = [];
        CHILDREN.iter()
    }

    /// Returns whether this pattern introduces a binding.
    ///
    /// The AST does not decide the complete binding semantics. The answer is
    /// nevertheless `true` for the plain identifier-pattern form because the
    /// identifier is the source-level binding name.
    ///
    /// Semantic analysis still determines the binding's type, scope, ownership,
    /// lifetime and domain.
    #[must_use]
    #[inline]
    pub const fn introduces_binding(&self) -> bool {
        true
    }

    /// Returns whether this pattern contains a nested pattern.
    ///
    /// Plain identifier patterns do not.
    #[must_use]
    #[inline]
    pub const fn has_nested_pattern(&self) -> bool {
        false
    }

    /// Replaces the source spelling.
    ///
    /// Empty names are rejected because they violate the local structural
    /// invariant of an identifier pattern.
    ///
    /// Lexical legality of non-empty names remains the responsibility of the
    /// lexer/parser.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), IdentifierPatternValidationError> {
        let name = name.into();

        if name.is_empty() {
            return Err(IdentifierPatternValidationError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Structural validation must be rerun after arbitrary metadata/span/kind
    /// transformations.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the expected native node kind.
    #[must_use]
    #[inline]
    pub const fn expected_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::Identifier)
    }

    /// Returns whether the embedded node has the expected classification.
    #[must_use]
    #[inline]
    pub fn has_expected_kind(&self) -> bool {
        self.node().kind() == &Self::expected_kind()
    }

    /// Validates local structural invariants.
    ///
    /// This does not perform:
    ///
    /// - name resolution;
    /// - type checking;
    /// - scope checking;
    /// - ownership analysis;
    /// - resource analysis;
    /// - capability analysis;
    /// - quantum analysis;
    /// - target validation.
    pub fn validate_structure(
        &self,
    ) -> Result<(), IdentifierPatternValidationError> {
        if !self.has_expected_kind() {
            return Err(
                IdentifierPatternValidationError::InvalidNodeKind {
                    actual: self.node().kind().clone(),
                },
            );
        }

        if self.name.is_empty() {
            return Err(IdentifierPatternValidationError::EmptyName);
        }

        Ok(())
    }

    /// Returns the canonical source-level spelling contract.
    ///
    /// This does not return the actual identifier name. It returns the
    /// grammatical category's human-readable name.
    #[must_use]
    #[inline]
    pub const fn diagnostic_name() -> &'static str {
        "identifier pattern"
    }

    /// Returns the actual source spelling.
    ///
    /// This is equivalent to [`Self::name`] and is useful for source-oriented
    /// tooling APIs.
    #[must_use]
    #[inline]
    pub fn source_spelling(&self) -> &str {
        self.name()
    }

    /// Returns the stable node-kind identifier.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        IDENTIFIER_PATTERN_KIND_NAME
    }

    /// Returns this node's schema contract version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        IDENTIFIER_PATTERN_SCHEMA_VERSION
    }
}

impl AstNode for IdentifierPattern {
    /// Returns the common node container.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common node container.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Structural validation errors for [`IdentifierPattern`].
///
/// These errors intentionally describe only local AST structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentifierPatternValidationError {
    /// The identifier spelling is empty.
    EmptyName,

    /// The embedded common node is classified as something other than the
    /// native identifier kind.
    InvalidNodeKind {
        /// Actual classification found in the node.
        actual: NodeKind,
    },
}

impl std::fmt::Display for IdentifierPatternValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "identifier pattern must contain a non-empty name",
                )
            }
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "identifier pattern has invalid AST node kind: {actual}"
                )
            }
        }
    }
}

impl std::error::Error for IdentifierPatternValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id() -> NodeId {
        NodeId::default()
    }

    fn span() -> Span {
        Span::default()
    }

    fn identifier(name: &str) -> IdentifierPattern {
        IdentifierPattern::without_metadata(node_id(), span(), name)
    }

    #[test]
    fn new_creates_identifier_kind() {
        let pattern = identifier("value");

        assert_eq!(
            pattern.node().kind(),
            &NodeKind::Core(CoreNodeKind::Identifier)
        );
    }

    #[test]
    fn name_is_preserved() {
        let pattern = identifier("value");

        assert_eq!(pattern.name(), "value");
        assert_eq!(pattern.source_spelling(), "value");
    }

    #[test]
    fn unicode_name_is_preserved() {
        let pattern = identifier("量子状態");

        assert_eq!(pattern.name(), "量子状態");
        assert!(!pattern.is_empty());
        assert!(pattern.name_byte_len() > 0);
        assert!(pattern.name_char_len() > 0);
    }

    #[test]
    fn identifier_pattern_is_a_leaf() {
        let pattern = identifier("value");

        assert!(pattern.is_leaf());
        assert_eq!(pattern.child_count(), 0);
        assert!(pattern.has_nested_pattern());
        assert!(!pattern.has_nested_pattern());
        assert_eq!(pattern.child_node_ids().count(), 0);
    }

    #[test]
    fn identifier_pattern_introduces_binding() {
        let pattern = identifier("value");

        assert!(pattern.introduces_binding());
    }

    #[test]
    fn valid_pattern_passes_structural_validation() {
        let pattern = identifier("value");

        assert_eq!(pattern.validate_structure(), Ok(()));
    }

    #[test]
    fn empty_name_fails_structural_validation() {
        let pattern = IdentifierPattern::without_metadata(
            node_id(),
            span(),
            "",
        );

        assert_eq!(
            pattern.validate_structure(),
            Err(IdentifierPatternValidationError::EmptyName)
        );
    }

    #[test]
    fn set_name_rejects_empty_name() {
        let mut pattern = identifier("value");

        let result = pattern.set_name("");

        assert_eq!(
            result,
            Err(IdentifierPatternValidationError::EmptyName)
        );
        assert_eq!(pattern.name(), "value");
    }

    #[test]
    fn set_name_replaces_valid_name() {
        let mut pattern = identifier("value");

        pattern.set_name("result").expect("valid name");

        assert_eq!(pattern.name(), "result");
        assert_eq!(pattern.validate_structure(), Ok(()));
    }

    #[test]
    fn from_node_does_not_silently_rewrite_kind() {
        let node = Node::new(
            node_id(),
            NodeKind::Core(CoreNodeKind::WildcardPattern),
            span(),
            NodeMetadata::default(),
        );

        let pattern = IdentifierPattern::from_node(node, "value");

        assert_eq!(
            pattern.validate_structure(),
            Err(
                IdentifierPatternValidationError::InvalidNodeKind {
                    actual: NodeKind::Core(CoreNodeKind::WildcardPattern),
                }
            )
        );
    }

    #[test]
    fn expected_kind_is_identifier() {
        assert_eq!(
            IdentifierPattern::expected_kind(),
            NodeKind::Core(CoreNodeKind::Identifier)
        );
    }

    #[test]
    fn kind_contract_is_stable() {
        assert_eq!(
            IdentifierPattern::kind_name(),
            "zamani:identifier-pattern"
        );
        assert_eq!(
            IdentifierPattern::schema_version(),
            IDENTIFIER_PATTERN_SCHEMA_VERSION
        );
    }

    #[test]
    fn diagnostic_name_is_stable() {
        assert_eq!(
            IdentifierPattern::diagnostic_name(),
            "identifier pattern"
        );
    }

    #[test]
    fn ast_node_identity_is_exposed() {
        let pattern = identifier("value");

        assert_eq!(pattern.id(), pattern.node().id());
        assert_eq!(pattern.kind(), pattern.node().kind());
        assert_eq!(pattern.span(), pattern.node().span());
        assert_eq!(pattern.metadata(), pattern.node().metadata());
    }

    #[test]
    fn clone_preserves_logical_identity() {
        let pattern = identifier("value");
        let cloned = pattern.clone();

        assert_eq!(pattern, cloned);
        assert_eq!(pattern.id(), cloned.id());
    }

    #[test]
    fn different_names_are_not_equal() {
        let first = identifier("first");
        let second = identifier("second");

        assert_ne!(first, second);
    }

    #[test]
    fn empty_child_sequence_is_deterministic() {
        let pattern = identifier("value");

        let first: Vec<NodeId> = pattern.child_node_ids().copied().collect();
        let second: Vec<NodeId> = pattern.child_node_ids().copied().collect();

        assert_eq!(first, second);
        assert!(first.is_empty());
    }
}