//! # Zamani Frontend AST — Or Pattern
//!
//! Production-ready source-level representation of the native Zamani
//! `orPattern` construct.
//!
//! ## Grammar
//!
//! The current Zamani grammar defines:
//!
//! ```text
//! orPattern: pattern '|' pattern ;
//! ```
//!
//! Therefore an `OrPattern` owns exactly two **references** to child pattern
//! nodes:
//!
//! ```text
//!             OrPattern
//!             /        \
//!        left NodeId   right NodeId
//! ```
//!
//! The referenced child nodes are owned by the enclosing AST graph, not by this
//! node.
//!
//! This representation preserves the grammar's binary structure while allowing
//! arbitrarily large OR expressions through normal AST composition:
//!
//! ```text
//! a | b
//!
//! a | (b | c)
//!
//! a | (b | (c | d))
//! ```
//!
//! No fixed number of alternatives is encoded here.
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
//!     └── patterns::or::OrPattern
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
//! `OrPattern` is therefore a source-language AST node only.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - [`OrPattern`];
//! - its two child-pattern references;
//! - local structural invariants;
//! - local construction APIs;
//! - local structural validation errors;
//! - source-level queries over the OR relationship.
//!
//! This file does **not** own:
//!
//! - the AST graph;
//! - the child pattern nodes;
//! - pattern type checking;
//! - symbol resolution;
//! - binding resolution;
//! - exhaustiveness analysis;
//! - reachability analysis;
//! - constant evaluation;
//! - quantum semantics;
//! - resource allocation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - calibration;
//! - hardware mapping;
//! - backend execution;
//! - ZUIR.
//!
//! ## Why child nodes are `NodeId`s
//!
//! Child patterns are references into the canonical AST graph rather than
//! recursively embedded Rust values.
//!
//! This provides:
//!
//! - one authoritative node for every AST element;
//! - stable node identity;
//! - source-span preservation;
//! - efficient sharing by compiler infrastructure;
//! - no duplicated child nodes;
//! - no recursive ownership explosion;
//! - compatibility with side tables;
//! - scalable graph traversal;
//! - compatibility with incremental compilation.
//!
//! It also means this file does not need to know the concrete type of either
//! child pattern.
//!
//! A child may eventually be a:
//!
//! - wildcard;
//! - identifier;
//! - literal;
//! - tuple;
//! - struct;
//! - enum;
//! - range;
//! - reference;
//! - another OR pattern;
//! - another source-level pattern extension.
//!
//! ## Domain neutrality
//!
//! An OR pattern is a generic language construct.
//!
//! It must not contain:
//!
//! - qubit IDs;
//! - qubit counts;
//! - quantum gates;
//! - quantum topology;
//! - backend IDs;
//! - vendor IDs;
//! - hardware instructions;
//! - physical-device information;
//! - QIR types;
//! - LLVM values;
//! - MLIR operations;
//! - scheduling information;
//! - calibration information;
//! - QEC information.
//!
//! The same source-level OR pattern can participate in classical, quantum,
//! hybrid, distributed, accelerator, or future-domain programs.
//!
//! ## POCO-REAF
//!
//! The node contains no machine-size assumption and no target-specific
//! information.
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! source-level OrPattern
//!      │
//!      ▼
//! semantic interpretation
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! capability/resource analysis
//!      │
//!      ▼
//! target realization
//! ```
//!
//! The number of available machines, processors, QPUs, resources, qubits,
//! registers, cores, accelerators, or other execution resources does not alter
//! this AST representation.
//!
//! ## Scalability
//!
//! This node has exactly two child references because the grammar defines an OR
//! expression as a binary operation.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_ALTERNATIVES
//! MAX_PATTERNS
//! MAX_OR_DEPTH
//! MAX_QUANTUM_ALTERNATIVES
//! ```
//!
//! A long OR expression is represented by composing ordinary `OrPattern`
//! nodes. Its practical size is therefore determined by available memory and
//! by configurable compiler resource-safety policies, not by a language-level
//! machine-size constant.
//!
//! This distinction is important:
//!
//! - **language semantics:** no artificial finite OR capacity;
//! - **compiler safety:** configurable resource limits may exist elsewhere.
//!
//! This file introduces no such safety limit.
//!
//! ## Semantic boundary
//!
//! Structural AST construction does not decide what an OR pattern means for a
//! particular scrutinee.
//!
//! Semantic analysis is responsible for:
//!
//! - resolving child pattern types;
//! - checking compatibility of alternatives;
//! - determining bindings;
//! - checking binding consistency according to Zamani's pattern rules;
//! - exhaustiveness;
//! - reachability;
//! - contextual validity;
//! - lowering the pattern to semantic predicates.
//!
//! None of those operations belongs here.
//!
//! ## Quantum boundary
//!
//! A quantum program may eventually use pattern matching over values derived
//! from classical computation, measurement results, symbolic values, or other
//! language-level values.
//!
//! This node does not determine whether the matched value is quantum.
//!
//! It therefore must never acquire APIs such as:
//!
//! ```text
//! quantum_alternatives()
//! qubit_pattern()
//! measurement_pattern()
//! physical_qubit_pattern()
//! ```
//!
//! Quantum interpretation belongs to the semantic/domain layers.
//!
//! ## Parser integration
//!
//! The parser should:
//!
//! 1. parse the left `pattern`;
//! 2. consume `|`;
//! 3. parse the right `pattern`;
//! 4. allocate a fresh `NodeId` for the OR node;
//! 5. calculate a source span covering the complete OR construct;
//! 6. construct a canonical `Node` classified as `CoreNodeKind::Pattern`;
//! 7. construct [`OrPattern`];
//! 8. insert the node into the AST graph;
//! 9. attach the resulting node to the enclosing pattern representation.
//!
//! The parser must not:
//!
//! - type-check alternatives;
//! - resolve bindings;
//! - flatten alternatives semantically;
//! - select hardware;
//! - perform quantum routing;
//! - lower to ZUIR.
//!
//! ## Associativity and source preservation
//!
//! The grammar is binary:
//!
//! ```text
//! orPattern: pattern '|' pattern ;
//! ```
//!
//! Consequently this node does not flatten an OR chain into a `Vec<NodeId>`.
//!
//! For example, depending on parser associativity, a source expression can
//! remain structurally represented as:
//!
//! ```text
//! Or(a, Or(b, c))
//! ```
//!
//! rather than being silently rewritten into:
//!
//! ```text
//! Or([a, b, c])
//! ```
//!
//! This preserves the parser's source structure and leaves semantic
//! normalization to a later phase if Zamani eventually defines such a
//! normalization.
//!
//! ## Validation boundary
//!
//! Local structural validation checks only:
//!
//! - node classification;
//! - presence of two distinct child positions.
//!
//! It does not require the child `NodeId`s to resolve because resolving IDs is
//! an AST-graph responsibility.
//!
//! In particular, this module does not dereference IDs.
//!
//! Graph validation must separately verify:
//!
//! - each child ID exists;
//! - each referenced node is a valid pattern node;
//! - graph relationships are valid;
//! - cycles are prohibited where the AST graph requires an acyclic structure.
//!
//! ## Duplicate child IDs
//!
//! The two child references are allowed to contain the same `NodeId`.
//!
//! This is intentional.
//!
//! Whether a source construct that ultimately produces identical pattern
//! references is semantically meaningful is not a structural concern of this
//! node.
//!
//! Rejecting equal IDs here could incorrectly impose semantic assumptions on
//! AST transformations, macro expansion, or future source constructs.
//!
//! ## Determinism
//!
//! The node preserves:
//!
//! - left/right ordering;
//! - supplied node identity;
//! - supplied source span;
//! - supplied metadata.
//!
//! It does not:
//!
//! - sort alternatives;
//! - deduplicate alternatives;
//! - canonicalize expressions;
//! - perform semantic simplification;
//! - use randomness;
//! - use timestamps;
//! - use process IDs;
//! - inspect hardware.
//!
//! Therefore construction is deterministic with respect to its inputs.
//!
//! ## Serialization
//!
//! Serialization is derived through Serde and uses the repository's canonical
//! AST serialization infrastructure.
//!
//! This file does not introduce a competing serialization format.
//!
//! The schema version is local to this node and is independent from:
//!
//! - Zamani language version;
//! - compiler version;
//! - global AST serialization version;
//! - semantic model version;
//! - ZUIR version;
//! - quantum IR version;
//! - backend version.
//!
//! ## Visitor integration
//!
//! `OrPattern` has exactly two direct AST children.
//!
//! Generic AST traversal should visit:
//!
//! ```text
//! OrPattern
//! ├── left
//! └── right
//! ```
//!
//! in source order.
//!
//! This module deliberately does not depend on the visitor implementation.
//! The visitor/traversal subsystem should consume [`Self::child_node_ids`].
//!
//! ## Semantic lowering contract
//!
//! The semantic layer consumes this node and its child nodes.
//!
//! Conceptually:
//!
//! ```text
//! OrPattern(left, right)
//!        │
//!        ├── lower(left)
//!        └── lower(right)
//!                 │
//!                 ▼
//!       semantic OR/pattern predicate
//!                 │
//!                 ▼
//!                ZUIR
//! ```
//!
//! The exact ZUIR representation is intentionally not selected here.
//!
//! This avoids coupling the source AST to one implementation strategy.
//!
//! ## Error model
//!
//! [`OrPatternError`] contains only local structural failures.
//!
//! It intentionally does not depend on the compiler diagnostics subsystem.
//!
//! The compiler can translate the local error into its canonical diagnostic
//! representation at the validation boundary.
//!
//! ## Security
//!
//! This implementation:
//!
//! - contains no `unsafe` code;
//! - performs no I/O;
//! - executes no user code;
//! - dereferences no arbitrary pointer;
//! - resolves no node IDs;
//! - performs no recursive traversal;
//! - performs no unbounded computation;
//! - performs no target-specific work.
//!
//! Child references are treated as opaque IDs.
//!
//! This is important for hostile or malformed AST input: local validation
//! cannot accidentally walk an attacker-controlled graph.
//!
//! ## Rust compatibility
//!
//! Supported compiler versions:
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

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Independent schema version for [`OrPattern`].
///
/// This version is local to this AST construct and is deliberately separate
/// from the language, compiler, global AST serialization, semantic-model,
/// ZUIR, and backend versions.
pub const OR_PATTERN_SCHEMA_VERSION: u16 = 1;

/// Stable source-level qualified name for an OR pattern.
pub const OR_PATTERN_KIND_NAME: &str = "zamani:or-pattern";

/// The canonical node classification used by the native AST.
///
/// The current AST architecture classifies pattern forms under the generic
/// `CoreNodeKind::Pattern` category. This keeps the native classification
/// extensible instead of requiring a new core enum variant for every pattern
/// shape.
pub const OR_PATTERN_NODE_KIND: NodeKind = NodeKind::Core(CoreNodeKind::Pattern);

/// Result type for local OR-pattern operations.
pub type OrPatternResult<T> = Result<T, OrPatternError>;

/// Source-level binary OR pattern.
///
/// The two child references correspond directly to the two `pattern`
/// productions in:
///
/// ```text
/// orPattern: pattern '|' pattern ;
/// ```
///
/// The referenced nodes are owned by the enclosing AST graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrPattern {
    /// Canonical AST identity, classification, source span, and metadata.
    node: Node,

    /// Left-hand pattern reference.
    left: NodeId,

    /// Right-hand pattern reference.
    right: NodeId,
}

impl OrPattern {
    /// Returns the schema version for this AST node.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        OR_PATTERN_SCHEMA_VERSION
    }

    /// Returns the stable source-level qualified name.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        OR_PATTERN_KIND_NAME
    }

    /// Returns the canonical node classification expected by this type.
    #[must_use]
    #[inline]
    pub const fn expected_kind() -> NodeKind {
        OR_PATTERN_NODE_KIND
    }

    /// Constructs an OR pattern from its complete common AST components.
    ///
    /// The two child nodes are represented by stable `NodeId` references and
    /// are not copied into this structure.
    ///
    /// # Complexity
    ///
    /// Construction is `O(1)` with respect to the child graph because only two
    /// identifiers are stored.
    ///
    /// # Errors
    ///
    /// This constructor does not perform graph resolution and therefore cannot
    /// determine whether the child IDs currently resolve to valid pattern
    /// nodes. That check belongs to graph-level validation.
    ///
    /// It can only reject invalid common-node classification when validation is
    /// explicitly requested through [`Self::from_node`].
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        left: NodeId,
        right: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                OR_PATTERN_NODE_KIND,
                span,
                metadata,
            ),
            left,
            right,
        }
    }

    /// Constructs an OR pattern using default node metadata.
    ///
    /// This is the convenience constructor intended for parser/builders that
    /// have no additional source metadata to attach.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        left: NodeId,
        right: NodeId,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            left,
            right,
        )
    }

    /// Constructs an OR pattern from an already-created canonical node.
    ///
    /// The node is preserved exactly; its kind is never silently rewritten.
    ///
    /// # Errors
    ///
    /// Returns [`OrPatternError::InvalidNodeKind`] if the supplied node is not
    /// classified as a generic native pattern.
    #[must_use]
    pub fn from_node(
        node: Node,
        left: NodeId,
        right: NodeId,
    ) -> OrPatternResult<Self> {
        if node.kind() != &OR_PATTERN_NODE_KIND {
            return Err(OrPatternError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        Ok(Self {
            node,
            left,
            right,
        })
    }

    /// Returns the canonical AST node.
    #[must_use]
    #[inline]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the canonical AST node.
    ///
    /// This exposes only the common AST node infrastructure. It does not expose
    /// child graph storage.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node identifier.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span covering the complete OR pattern.
    #[must_use]
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the node classification.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns common AST metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to common AST metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the left-hand child-pattern ID.
    #[must_use]
    #[inline]
    pub const fn left(&self) -> NodeId {
        self.left
    }

    /// Returns the right-hand child-pattern ID.
    #[must_use]
    #[inline]
    pub const fn right(&self) -> NodeId {
        self.right
    }

    /// Replaces the left-hand child reference.
    ///
    /// Returns the previous child ID.
    ///
    /// This is a structural AST transformation only. It does not resolve the
    /// new ID or validate its graph membership.
    #[must_use]
    #[inline]
    pub fn replace_left(&mut self, left: NodeId) -> NodeId {
        core::mem::replace(&mut self.left, left)
    }

    /// Replaces the right-hand child reference.
    ///
    /// Returns the previous child ID.
    ///
    /// This is a structural AST transformation only. It does not resolve the
    /// new ID or validate its graph membership.
    #[must_use]
    #[inline]
    pub fn replace_right(&mut self, right: NodeId) -> NodeId {
        core::mem::replace(&mut self.right, right)
    }

    /// Returns the two direct child IDs in source order.
    ///
    /// The result is an array because this node has exactly two syntactic child
    /// positions according to the grammar.
    ///
    /// This does not impose a limit on the size of an OR expression: arbitrary
    /// OR chains are represented by composing multiple `OrPattern` nodes.
    #[must_use]
    #[inline]
    pub const fn child_node_ids(&self) -> [NodeId; 2] {
        [self.left, self.right]
    }

    /// Returns the number of direct AST children.
    #[must_use]
    #[inline]
    pub const fn child_count() -> usize {
        2
    }

    /// Returns whether this node has no direct AST children.
    ///
    /// Always `false` for a valid `OrPattern`.
    #[must_use]
    #[inline]
    pub const fn is_leaf() -> bool {
        false
    }

    /// Returns whether this node is a binary pattern composition.
    #[must_use]
    #[inline]
    pub const fn is_binary() -> bool {
        true
    }

    /// Returns whether this node has the canonical pattern classification.
    ///
    /// This non-panicking predicate is suitable for validation and diagnostics.
    #[must_use]
    #[inline]
    pub fn has_expected_kind(&self) -> bool {
        self.node.is_kind(&Self::expected_kind())
    }

    /// Returns the source-level spelling of the binary operator.
    ///
    /// This is syntax, not a target instruction.
    #[must_use]
    #[inline]
    pub const fn operator_spelling() -> &'static str {
        "|"
    }

    /// Returns the source-level diagnostic name.
    #[must_use]
    #[inline]
    pub const fn diagnostic_name() -> &'static str {
        "or pattern"
    }

    /// Performs local structural validation.
    ///
    /// This method deliberately does not resolve the two child IDs.
    ///
    /// Graph-wide validation must verify that the IDs refer to existing
    /// pattern nodes.
    ///
    /// # Errors
    ///
    /// Returns [`OrPatternError::InvalidNodeKind`] when the common node is not
    /// classified as a native pattern.
    pub fn validate_structure(&self) -> OrPatternResult<()> {
        if !self.has_expected_kind() {
            return Err(OrPatternError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        Ok(())
    }

    /// Returns whether both child references are identical.
    ///
    /// This is a structural observation only. It does not classify the pattern
    /// as invalid because semantic equivalence/deduplication is outside this
    /// node's responsibility.
    #[must_use]
    #[inline]
    pub fn has_identical_children(&self) -> bool {
        self.left == self.right
    }

    /// Returns whether the two child references are structurally distinct.
    ///
    /// This is the inverse of [`Self::has_identical_children`].
    #[must_use]
    #[inline]
    pub fn has_distinct_children(&self) -> bool {
        self.left != self.right
    }
}

impl AstNode for OrPattern {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Local structural errors for [`OrPattern`].
///
/// These errors intentionally do not depend on semantic-analysis or diagnostic
/// infrastructure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrPatternError {
    /// The common AST node has a classification other than
    /// `CoreNodeKind::Pattern`.
    InvalidNodeKind {
        /// The actual classification supplied by the caller.
        actual: NodeKind,
    },
}

impl core::fmt::Display for OrPatternError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => write!(
                formatter,
                "invalid node kind for or pattern: expected {expected}, found {actual}",
                expected = OR_PATTERN_NODE_KIND,
            ),
        }
    }
}

impl std::error::Error for OrPatternError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(OrPattern::schema_version(), 1);
    }

    #[test]
    fn kind_name_is_stable() {
        assert_eq!(
            OrPattern::kind_name(),
            "zamani:or-pattern"
        );
    }

    #[test]
    fn expected_kind_is_generic_pattern_kind() {
        assert_eq!(
            OrPattern::expected_kind(),
            NodeKind::Core(CoreNodeKind::Pattern)
        );
    }

    #[test]
    fn child_count_is_exactly_two() {
        assert_eq!(OrPattern::child_count(), 2);
    }

    #[test]
    fn operator_spelling_is_pipe() {
        assert_eq!(
            OrPattern::operator_spelling(),
            "|"
        );
    }

    #[test]
    fn binary_pattern_is_not_leaf() {
        assert!(OrPattern::is_binary());
        assert!(!OrPattern::is_leaf());
    }

    #[test]
    fn constructor_preserves_child_order() {
        let left = NodeId::new(1);
        let right = NodeId::new(2);

        let pattern = OrPattern::without_metadata(
            NodeId::new(3),
            Span::default(),
            left,
            right,
        );

        assert_eq!(pattern.left(), left);
        assert_eq!(pattern.right(), right);
        assert_eq!(
            pattern.child_node_ids(),
            [left, right]
        );
    }

    #[test]
    fn constructor_preserves_node_identity() {
        let id = NodeId::new(42);

        let pattern = OrPattern::without_metadata(
            id,
            Span::default(),
            NodeId::new(1),
            NodeId::new(2),
        );

        assert_eq!(pattern.id(), id);
        assert_eq!(pattern.node().id(), id);
    }

    #[test]
    fn identical_child_ids_are_structurally_allowed() {
        let child = NodeId::new(7);

        let pattern = OrPattern::without_metadata(
            NodeId::new(8),
            Span::default(),
            child,
            child,
        );

        assert!(pattern.has_identical_children());
        assert!(!pattern.has_distinct_children());
        assert_eq!(
            pattern.child_node_ids(),
            [child, child]
        );
    }

    #[test]
    fn distinct_child_ids_are_reported() {
        let left = NodeId::new(10);
        let right = NodeId::new(11);

        let pattern = OrPattern::without_metadata(
            NodeId::new(12),
            Span::default(),
            left,
            right,
        );

        assert!(!pattern.has_identical_children());
        assert!(pattern.has_distinct_children());
    }

    #[test]
    fn expected_kind_predicate_is_true_for_constructed_node() {
        let pattern = OrPattern::without_metadata(
            NodeId::new(1),
            Span::default(),
            NodeId::new(2),
            NodeId::new(3),
        );

        assert!(pattern.has_expected_kind());
    }

    #[test]
    fn structural_validation_accepts_canonical_node() {
        let pattern = OrPattern::without_metadata(
            NodeId::new(1),
            Span::default(),
            NodeId::new(2),
            NodeId::new(3),
        );

        assert!(pattern.validate_structure().is_ok());
    }

    #[test]
    fn replace_left_returns_previous_id() {
        let old_left = NodeId::new(1);
        let new_left = NodeId::new(2);

        let mut pattern = OrPattern::without_metadata(
            NodeId::new(3),
            Span::default(),
            old_left,
            NodeId::new(4),
        );

        assert_eq!(
            pattern.replace_left(new_left),
            old_left
        );
        assert_eq!(pattern.left(), new_left);
    }

    #[test]
    fn replace_right_returns_previous_id() {
        let old_right = NodeId::new(1);
        let new_right = NodeId::new(2);

        let mut pattern = OrPattern::without_metadata(
            NodeId::new(3),
            Span::default(),
            NodeId::new(4),
            old_right,
        );

        assert_eq!(
            pattern.replace_right(new_right),
            old_right
        );
        assert_eq!(pattern.right(), new_right);
    }

    #[test]
    fn constructor_does_not_resolve_child_ids() {
        // Child IDs need not exist in a graph at construction time.
        //
        // Graph validation owns resolution. This is important because AST
        // construction and graph assembly can legitimately occur in separate
        // phases.
        let pattern = OrPattern::without_metadata(
            NodeId::new(100),
            Span::default(),
            NodeId::new(1_000_000),
            NodeId::new(2_000_000),
        );

        assert_eq!(
            pattern.child_node_ids(),
            [NodeId::new(1_000_000), NodeId::new(2_000_000)]
        );
    }

    #[test]
    fn no_machine_specific_information_is_stored() {
        let pattern = OrPattern::without_metadata(
            NodeId::new(1),
            Span::default(),
            NodeId::new(2),
            NodeId::new(3),
        );

        // The only state specific to this node is source AST structure:
        // common node metadata and two child references.
        assert_eq!(pattern.child_count(), 2);
        assert!(pattern.has_expected_kind());
    }

    #[test]
    fn ast_node_trait_exposes_common_node() {
        let pattern = OrPattern::without_metadata(
            NodeId::new(1),
            Span::default(),
            NodeId::new(2),
            NodeId::new(3),
        );

        let ast_node: &dyn AstNode = &pattern;

        assert_eq!(
            ast_node.node().id(),
            pattern.id()
        );
    }
}