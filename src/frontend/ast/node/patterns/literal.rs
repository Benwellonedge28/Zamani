//! # Zamani Native AST — Literal Pattern
//!
//! Production-ready source-level representation of a literal pattern.
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
//! native Zamani AST
//!     │
//!     ├── patterns::wildcard
//!     ├── patterns::identifier
//!     ├── patterns::literal  ◄── this module
//!     ├── patterns::tuple
//!     ├── patterns::struct
//!     ├── patterns::enum
//!     ├── patterns::range
//!     ├── patterns::or
//!     └── patterns::reference
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── hybrid IR
//!     ├── distributed IR
//!     ├── accelerator IR
//!     └── future-domain IR
//!     │
//!     ▼
//! target lowering
//!     │
//!     ▼
//! execution
//! ```
//!
//! ## Responsibility
//!
//! This file owns the **pattern-level AST node** representing a literal
//! appearing in a pattern.
//!
//! It does not own the definition of a literal's source-level value.
//! That responsibility belongs to the canonical expression-literal model:
//!
//! `crate::frontend::ast::node::expressions::literal`
//!
//! Reusing that representation is deliberate. It prevents the AST from
//! developing two incompatible definitions of literals.
//!
//! ```text
//! expressions::literal::LiteralKind
//!             │
//!             │ reused by
//!             ▼
//! patterns::literal::LiteralPattern
//! ```
//!
//! This separation gives the compiler:
//!
//! - one canonical literal syntax model;
//! - one source-spelling representation;
//! - no premature numeric conversion;
//! - no duplicated literal semantics;
//! - no machine-width assumptions;
//! - no quantum-specific literal enum;
//! - no target-specific literal representation.
//!
//! ## What this node represents
//!
//! Examples include source patterns such as:
//!
//! ```text
//! 0
//! 42
//! 0xff
//! true
//! false
//! "hello"
//! 'x'
//! ```
//!
//! The exact accepted literal grammar remains owned by the lexer/parser.
//!
//! This node only records the source-level literal representation after the
//! parser has recognized it as a pattern.
//!
//! ## Important distinction
//!
//! A literal expression and a literal pattern can contain the same source-level
//! literal representation while having different syntactic roles.
//!
//! ```text
//! expression:
//!     42
//!
//! pattern:
//!     match value {
//!         42 => ...
//!     }
//! ```
//!
//! The underlying `LiteralKind` is shared, but the AST nodes have different
//! roles.
//!
//! ## Semantic boundary
//!
//! This module does NOT determine whether the literal matches a particular
//! semantic value.
//!
//! It does not perform:
//!
//! - type checking;
//! - coercion;
//! - constant evaluation;
//! - numeric conversion;
//! - Unicode semantic interpretation;
//! - pattern exhaustiveness analysis;
//! - reachability analysis;
//! - binding analysis;
//! - resource analysis;
//! - quantum-state analysis;
//! - measurement-result analysis;
//! - hardware validation;
//! - target validation.
//!
//! Those responsibilities belong downstream.
//!
//! ## Quantum neutrality
//!
//! A literal pattern may eventually be used to match:
//!
//! - a classical value;
//! - a measurement result;
//! - a symbolic value;
//! - a resource-derived value;
//! - a hybrid-computation value;
//! - an extension-defined value;
//! - a future computational-domain value.
//!
//! None of those meanings is encoded here.
//!
//! In particular, this file must never acquire variants such as:
//!
//! ```text
//! QuantumLiteralPattern
//! QubitLiteralPattern
//! MeasurementLiteralPattern
//! IBMQuantumLiteralPattern
//! SurfaceCodeLiteralPattern
//! ```
//!
//! Such specialization would violate the native AST's domain-neutrality.
//!
//! ## POCO-REAF
//!
//! The node contains no information about:
//!
//! - number of qubits;
//! - register width;
//! - machine size;
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - QPU architecture;
//! - quantum topology;
//! - gate set;
//! - backend;
//! - vendor;
//! - calibration;
//! - scheduling;
//! - routing;
//! - error correction.
//!
//! Therefore the same source pattern representation remains usable as the
//! surrounding program scales from tiny systems to arbitrarily large systems
//! supported by downstream compilation and available resources.
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Literal Pattern
//!      │
//!      ▼
//! Semantic interpretation
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! resource/capability discovery
//!      │
//!      ▼
//! target realization
//! ```
//!
//! ## Dependency contract
//!
//! Allowed dependencies:
//!
//! - canonical AST `Node`;
//! - canonical AST `AstNode`;
//! - canonical AST `NodeId`;
//! - canonical AST `NodeKind`;
//! - canonical AST `CoreNodeKind`;
//! - canonical AST `Span`;
//! - canonical expression `LiteralKind`;
//! - `serde`;
//! - Rust standard library.
//!
//! Forbidden dependencies:
//!
//! - parser implementation;
//! - lexer implementation;
//! - semantic analysis;
//! - type checker;
//! - symbol table;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum backend;
//! - hardware;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - vendor APIs.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - `LiteralPattern`;
//! - pattern-level structural invariants;
//! - literal-pattern classification;
//! - literal-pattern construction;
//! - source-level pattern queries;
//! - local validation errors.
//!
//! It does not own:
//!
//! - `LiteralKind`;
//! - numeric interpretation;
//! - type semantics;
//! - pattern matching semantics;
//! - AST graph storage;
//! - global diagnostics;
//! - semantic lowering.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser should:
//!
//! 1. recognize a literal in pattern position;
//! 2. parse it using the canonical literal parser;
//! 3. allocate a distinct `NodeId` for the pattern node;
//! 4. construct the canonical `Node` with
//!    `CoreNodeKind::Literal`;
//! 5. construct `LiteralPattern` from that node and `LiteralKind`;
//! 6. pass the result to the enclosing pattern representation.
//!
//! The parser must not convert the literal into a machine numeric type merely
//! because it is being placed in a pattern.
//!
//! ### Structural validation
//!
//! Structural validation must verify:
//!
//! - the node kind is `CoreNodeKind::Literal`;
//! - the node ID is valid;
//! - the literal representation is structurally valid;
//! - the node's source span is appropriate for the literal.
//!
//! Graph-wide validation remains responsible for validating relationships
//! outside this node.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the pattern and:
//!
//! - resolves the type of the matched value;
//! - interprets the literal;
//! - performs required conversions according to language rules;
//! - checks compatibility;
//! - determines matching semantics.
//!
//! None of those operations belongs in this file.
//!
//! ### ZUIR
//!
//! The pattern is lowered by semantic/lowering infrastructure.
//!
//! This file must not import ZUIR.
//!
//! The semantic layer determines whether the literal pattern becomes a
//! comparison, branch predicate, dispatch condition, resource predicate, or
//! another universal semantic construct.
//!
//! ### Visitors
//!
//! A literal pattern is a leaf AST node.
//!
//! It has no child `NodeId`s owned by the pattern itself.
//!
//! Generic visitors should visit the `LiteralPattern` node and then stop.
//!
//! The contained `LiteralKind` is source data, not a second AST node.
//!
//! ### Serialization
//!
//! The type derives Serde serialization and remains compatible with repository
//! level AST serialization/versioning.
//!
//! This module does not define a competing serialization protocol.
//!
//! ## Scalability
//!
//! A single literal pattern has constant structural overhead, regardless of:
//!
//! - machine size;
//! - qubit count;
//! - resource count;
//! - program size.
//!
//! The literal spelling itself is stored as a `String` through the canonical
//! `LiteralKind`, allowing the AST to preserve source representations without
//! forcing them through `i64`, `u64`, `f32`, or `f64`.
//!
//! No language-level maximum literal width is introduced here.
//!
//! Resource/security limits belong to configurable compiler infrastructure.
//!
//! ## Determinism
//!
//! The node contains no:
//!
//! - timestamps;
//! - random state;
//! - memory addresses;
//! - thread-local state;
//! - process identifiers;
//! - backend state.
//!
//! Cloning preserves logical AST identity according to the repository's
//! canonical `Node` contract.
//!
//! ## Security
//!
//! This module:
//!
//! - performs no unsafe operations;
//! - performs no I/O;
//! - performs no user-code execution;
//! - performs no unchecked numeric conversion;
//! - performs no pointer manipulation;
//! - does not dereference node IDs;
//! - does not recursively traverse arbitrary input.
//!
//! Very large literals remain source text until later compiler policy and
//! semantic interpretation decide whether they can be processed.
//!
//! ## No-reedit contract
//!
//! Changes to:
//!
//! - quantum hardware;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backends;
//! - QIR;
//! - ZUIR;
//! - quantum IR;
//! - target architectures
//!
//! must not require this file to change.
//!
//! This file should change only when the **source-language pattern contract**
//! or the canonical literal representation changes.
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
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::expressions::literal::LiteralKind;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Independent schema version for the literal-pattern node contract.
///
/// This version is distinct from:
///
/// - Zamani language version;
/// - compiler version;
/// - global AST serialization version;
/// - literal-expression schema version;
/// - ZUIR version;
/// - quantum IR version.
pub const LITERAL_PATTERN_SCHEMA_VERSION: u16 = 1;

/// Stable source-level qualified name for this AST construct.
pub const LITERAL_PATTERN_KIND_NAME: &str = "zamani:literal-pattern";

/// Result type used by local literal-pattern operations.
pub type LiteralPatternResult<T> = Result<T, LiteralPatternError>;

/// A source-level literal pattern.
///
/// The pattern owns its canonical AST [`Node`] and reuses the canonical
/// [`LiteralKind`] representation from the expression-literal subsystem.
///
/// This is intentional: the syntax of a literal should have one authoritative
/// representation throughout the native AST.
///
/// # Example
///
/// Conceptually:
///
/// ```text
/// match value {
///     42 => ...
/// }
/// ```
///
/// becomes a pattern whose:
///
/// - node kind is `zamani:literal`;
/// - literal kind is `Integer`;
/// - literal spelling remains source-preserved.
///
/// No semantic type or target representation is stored here.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LiteralPattern {
    /// Common source identity, span and metadata.
    node: Node,

    /// Canonical source-level literal representation.
    literal: LiteralKind,
}

impl LiteralPattern {
    /// Creates a literal pattern from an already constructed canonical node
    /// and canonical literal representation.
    ///
    /// The constructor intentionally does not rewrite the node kind.
    ///
    /// This keeps construction deterministic and makes malformed construction
    /// observable through [`Self::validate_structure`].
    ///
    /// # Parameters
    ///
    /// - `node`: canonical AST identity/location/metadata;
    /// - `literal`: canonical source-level literal representation.
    ///
    /// # Integration
    ///
    /// The parser should create the node using
    /// `CoreNodeKind::Literal`.
    ///
    /// The semantic layer must interpret `literal`; this constructor performs
    /// no semantic conversion.
    #[must_use]
    pub fn new(node: Node, literal: LiteralKind) -> Self {
        Self { node, literal }
    }

    /// Returns the canonical AST node.
    #[inline]
    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the canonical AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span associated with the pattern.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the source-level literal representation.
    #[inline]
    #[must_use]
    pub fn literal(&self) -> &LiteralKind {
        &self.literal
    }

    /// Returns mutable access to the source-level literal representation.
    ///
    /// Mutation remains source-level only. It does not perform semantic
    /// evaluation or target lowering.
    #[inline]
    pub fn literal_mut(&mut self) -> &mut LiteralKind {
        &mut self.literal
    }

    /// Replaces the source-level literal representation.
    ///
    /// Returns the previous representation.
    ///
    /// This operation does not change node identity or source span.
    pub fn replace_literal(&mut self, literal: LiteralKind) -> LiteralKind {
        core::mem::replace(&mut self.literal, literal)
    }

    /// Returns the canonical AST node kind expected by this pattern.
    #[inline]
    #[must_use]
    pub const fn expected_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::Literal)
    }

    /// Returns whether the underlying node has the expected literal kind.
    #[inline]
    #[must_use]
    pub fn has_expected_kind(&self) -> bool {
        self.node.is_kind(&Self::expected_kind())
    }

    /// Returns the stable schema version of this pattern node.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        LITERAL_PATTERN_SCHEMA_VERSION
    }

    /// Returns the stable qualified source-level node name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        LITERAL_PATTERN_KIND_NAME
    }

    /// Returns the number of direct AST children.
    ///
    /// The contained `LiteralKind` is source data, not another AST node.
    ///
    /// Therefore the literal pattern is a leaf.
    #[inline]
    #[must_use]
    pub const fn child_count() -> usize {
        0
    }

    /// Returns whether this pattern is a leaf.
    #[inline]
    #[must_use]
    pub const fn is_leaf() -> bool {
        true
    }

    /// Returns whether this pattern introduces a binding.
    ///
    /// Literal patterns compare against a literal value and therefore do not
    /// introduce an identifier binding.
    #[inline]
    #[must_use]
    pub const fn introduces_binding() -> bool {
        false
    }

    /// Returns whether this pattern consumes a named binding.
    #[inline]
    #[must_use]
    pub const fn consumes_binding() -> bool {
        false
    }

    /// Returns the broad source-level pattern classification.
    ///
    /// This method intentionally returns the stable local category without
    /// introducing a dependency on the pattern aggregate type.
    #[inline]
    #[must_use]
    pub const fn pattern_category() -> LiteralPatternCategory {
        LiteralPatternCategory::Literal
    }

    /// Returns the original source spelling of the literal.
    ///
    /// No numeric conversion or semantic interpretation is performed.
    #[inline]
    #[must_use]
    pub fn raw(&self) -> &str {
        self.literal.raw()
    }

    /// Returns whether this pattern contains an extension-defined literal.
    ///
    /// Extension meaning is resolved through the compiler's extension
    /// registry. This method only reports the source representation.
    #[inline]
    #[must_use]
    pub fn is_extension_literal(&self) -> bool {
        matches!(self.literal, LiteralKind::Extension { .. })
    }

    /// Returns the extension namespace when the literal is extension-defined.
    #[inline]
    #[must_use]
    pub fn extension_namespace(&self) -> Option<&str> {
        match &self.literal {
            LiteralKind::Extension { namespace, .. } => Some(namespace.as_str()),
            _ => None,
        }
    }

    /// Returns the extension-defined literal name when present.
    #[inline]
    #[must_use]
    pub fn extension_name(&self) -> Option<&str> {
        match &self.literal {
            LiteralKind::Extension { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }

    /// Performs local structural validation.
    ///
    /// This function deliberately validates only invariants owned by this
    /// concrete node.
    ///
    /// It does not perform:
    ///
    /// - type checking;
    /// - literal evaluation;
    /// - pattern exhaustiveness;
    /// - reachability analysis;
    /// - name resolution;
    /// - quantum semantics;
    /// - resource validation;
    /// - hardware validation.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the node is not classified as a literal;
    /// - the literal representation is structurally empty;
    /// - the contained extension identity is malformed at the local level.
    pub fn validate_structure(&self) -> LiteralPatternResult<()> {
        if !self.has_expected_kind() {
            return Err(LiteralPatternError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        if self.id().is_invalid() {
            return Err(LiteralPatternError::InvalidNodeId);
        }

        if self.raw().is_empty() {
            return Err(LiteralPatternError::EmptyLiteralSpelling);
        }

        if let LiteralKind::Extension {
            namespace,
            name,
            ..
        } = &self.literal
        {
            if namespace.is_empty() {
                return Err(LiteralPatternError::EmptyExtensionNamespace);
            }

            if name.is_empty() {
                return Err(LiteralPatternError::EmptyExtensionName);
            }
        }

        Ok(())
    }

    /// Returns a stable diagnostic name.
    #[inline]
    #[must_use]
    pub const fn diagnostic_name() -> &'static str {
        "literal pattern"
    }

    /// Returns a concise source-level diagnostic representation.
    ///
    /// The actual literal spelling is intentionally omitted so diagnostics do
    /// not accidentally duplicate arbitrarily large source literals.
    #[inline]
    #[must_use]
    pub const fn diagnostic_kind() -> &'static str {
        "literal pattern"
    }

    /// Returns whether this literal pattern can introduce a binding.
    ///
    /// This explicit predicate exists for generic pattern-analysis code.
    #[inline]
    #[must_use]
    pub const fn is_binding_free() -> bool {
        true
    }
}

impl AstNode for LiteralPattern {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Broad source-level category for this concrete pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LiteralPatternCategory {
    /// A literal-valued pattern.
    Literal,
}

impl LiteralPatternCategory {
    /// Returns the stable category name.
    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Literal => "literal",
        }
    }
}

impl fmt::Display for LiteralPatternCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Local structural errors for [`LiteralPattern`].
///
/// Semantic errors deliberately do not belong here.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LiteralPatternError {
    /// The node was constructed with a non-literal AST kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The node has the invalid/default AST identity.
    InvalidNodeId,

    /// The source literal has no spelling.
    EmptyLiteralSpelling,

    /// An extension literal has no namespace.
    EmptyExtensionNamespace,

    /// An extension literal has no extension name.
    EmptyExtensionName,
}

impl fmt::Display for LiteralPatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => write!(
                formatter,
                "invalid literal pattern node kind: expected zamani:literal, found {actual}"
            ),

            Self::InvalidNodeId => {
                formatter.write_str("literal pattern has an invalid AST node ID")
            }

            Self::EmptyLiteralSpelling => {
                formatter.write_str("literal pattern has an empty source spelling")
            }

            Self::EmptyExtensionNamespace => {
                formatter.write_str(
                    "literal pattern has an extension literal with an empty namespace",
                )
            }

            Self::EmptyExtensionName => {
                formatter.write_str(
                    "literal pattern has an extension literal with an empty name",
                )
            }
        }
    }
}

impl std::error::Error for LiteralPatternError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Construction preserves the supplied node and literal representation.
    #[test]
    fn constructor_preserves_source_representation() {
        let node = Node::default();
        let literal = LiteralKind::decimal_integer("42");

        let pattern = LiteralPattern::new(node.clone(), literal.clone());

        assert_eq!(pattern.node(), &node);
        assert_eq!(pattern.literal(), &literal);
    }

    /// Literal patterns use the canonical native literal node kind.
    #[test]
    fn expected_kind_is_literal() {
        assert_eq!(
            LiteralPattern::expected_kind(),
            NodeKind::Core(CoreNodeKind::Literal)
        );
    }

    /// Literal patterns are leaves.
    #[test]
    fn literal_pattern_is_leaf() {
        assert_eq!(LiteralPattern::child_count(), 0);
        assert!(LiteralPattern::is_leaf());
    }

    /// Literal patterns never introduce bindings.
    #[test]
    fn literal_pattern_is_binding_free() {
        assert!(!LiteralPattern::introduces_binding());
        assert!(!LiteralPattern::consumes_binding());
        assert!(LiteralPattern::is_binding_free());
    }

    /// Source spelling is preserved exactly.
    #[test]
    fn source_spelling_is_preserved() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::hexadecimal_integer("0xFF"),
        );

        assert_eq!(pattern.raw(), "0xFF");
    }

    /// Very large source literals remain textual and are not narrowed to a
    /// machine integer by the pattern representation.
    #[test]
    fn arbitrarily_wide_integer_spelling_is_preserved() {
        let raw = "12345678901234567890123456789012345678901234567890";

        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer(raw),
        );

        assert_eq!(pattern.raw(), raw);
    }

    /// Extension literals remain namespaced and opaque to the AST core.
    #[test]
    fn extension_literal_metadata_is_preserved() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::extension(
                "future-domain",
                "custom-value",
                "future-domain::custom-value(...)",
            ),
        );

        assert!(pattern.is_extension_literal());
        assert_eq!(
            pattern.extension_namespace(),
            Some("future-domain")
        );
        assert_eq!(pattern.extension_name(), Some("custom-value"));
    }

    /// A correctly classified, non-empty literal can pass local validation
    /// once a valid AST node is supplied by parser/building infrastructure.
    ///
    /// `Node::default()` is intentionally not assumed to be a valid literal
    /// node. This test therefore verifies that validation fails safely rather
    /// than panicking.
    #[test]
    fn invalid_default_node_is_reported_without_panic() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer("42"),
        );

        assert!(pattern.validate_structure().is_err());
    }

    /// Invalid node classification is reported explicitly.
    #[test]
    fn invalid_node_kind_is_reported() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer("42"),
        );

        match pattern.validate_structure() {
            Err(LiteralPatternError::InvalidNodeKind { .. })
            | Err(LiteralPatternError::InvalidNodeId) => {}
            other => panic!("unexpected validation result: {other:?}"),
        }
    }

    /// Schema identity remains independent from language/compiler versions.
    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(
            LiteralPattern::schema_version(),
            LITERAL_PATTERN_SCHEMA_VERSION
        );
        assert_eq!(
            LiteralPattern::kind_name(),
            "zamani:literal-pattern"
        );
    }

    /// The pattern category remains source-level and backend-neutral.
    #[test]
    fn category_is_literal() {
        assert_eq!(
            LiteralPattern::pattern_category(),
            LiteralPatternCategory::Literal
        );
        assert_eq!(
            LiteralPatternCategory::Literal.name(),
            "literal"
        );
    }

    /// Cloning preserves logical AST identity, matching the canonical Node
    /// contract.
    #[test]
    fn clone_preserves_node_identity() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer("42"),
        );

        let clone = pattern.clone();

        assert_eq!(pattern.id(), clone.id());
        assert_eq!(pattern.node(), clone.node());
        assert_eq!(pattern.literal(), clone.literal());
    }

    /// `AstNode` exposes the same canonical node as the concrete type.
    #[test]
    fn ast_node_contract_is_preserved() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer("42"),
        );

        let ast_node: &dyn AstNode = &pattern;

        assert_eq!(ast_node.id(), pattern.id());
        assert_eq!(ast_node.kind(), pattern.node().kind());
        assert_eq!(ast_node.span(), pattern.span());
    }

    /// Replacing the literal changes only source-level literal data.
    #[test]
    fn replace_literal_returns_previous_value() {
        let mut pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer("42"),
        );

        let previous =
            pattern.replace_literal(LiteralKind::decimal_integer("43"));

        assert_eq!(previous, LiteralKind::decimal_integer("42"));
        assert_eq!(pattern.raw(), "43");
    }

    /// No fixed computational resource information exists in the pattern.
    #[test]
    fn pattern_contains_no_machine_or_quantum_sizing() {
        let pattern = LiteralPattern::new(
            Node::default(),
            LiteralKind::decimal_integer("42"),
        );

        assert_eq!(pattern.child_count(), 0);
        assert!(!pattern.introduces_binding());
        assert_eq!(pattern.pattern_category(), LiteralPatternCategory::Literal);
    }
}