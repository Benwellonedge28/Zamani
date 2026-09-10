//! # Zamani Native AST — Reference Pattern
//!
//! Canonical source-level representation of a reference/borrow pattern in the
//! Zamani frontend AST.
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
//! ReferencePattern
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ├── name resolution
//!     ├── pattern typing
//!     ├── ownership/borrowing analysis
//!     ├── lifetime analysis
//!     ├── resource analysis
//!     └── domain analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ## Purpose
//!
//! `ReferencePattern` represents the source-level pattern operation that
//! matches a value/resource through a reference rather than consuming the
//! matched value directly.
//!
//! The node is intentionally generic. A referenced value may ultimately be:
//!
//! - a classical value;
//! - a user-defined value;
//! - a collection;
//! - a resource;
//! - a quantum resource;
//! - a logical quantum resource;
//! - a distributed resource;
//! - an accelerator resource;
//! - another future computational abstraction.
//!
//! The AST does not decide which of those meanings applies.
//!
//! Ownership, borrowing, aliasing, mutability, lifetime, resource legality,
//! type compatibility, and domain-specific meaning are semantic concerns.
//!
//! ## Native-AST boundary
//!
//! This node represents source-language structure only.
//!
//! It must not contain:
//!
//! - physical addresses;
//! - memory addresses;
//! - backend handles;
//! - hardware identifiers;
//! - physical qubit identifiers;
//! - quantum topology;
//! - routing information;
//! - scheduling information;
//! - calibration information;
//! - QEC implementation;
//! - resilience implementation;
//! - runtime state;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - vendor-specific state.
//!
//! Those concepts belong to downstream compiler layers.
//!
//! ## POCO-REAF
//!
//! The representation contains no machine-size, processor, qubit-count,
//! register-width, topology, vendor, backend, or instruction-set assumptions.
//!
//! Therefore the same source representation can participate in compilation
//! toward different computational scales and technologies:
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Native Zamani AST
//!      │
//!      ▼
//! Semantic analysis
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! Capability/resource discovery
//!      │
//!      ▼
//! Mapping / optimization / scheduling / execution
//! ```
//!
//! ## Pattern shape
//!
//! A reference pattern consists of:
//!
//! ```text
//! reference-pattern
//!     └── referenced pattern
//! ```
//!
//! The referenced pattern is represented by `NodeId`, rather than embedding a
//! second AST object.
//!
//! This follows the native AST's canonical identity model and keeps ownership
//! of the complete AST graph in the surrounding AST container.
//!
//! The child may itself be another pattern, allowing arbitrary nesting subject
//! only to configurable compiler resource limits.
//!
//! For example, conceptually:
//!
//! ```text
//! &x
//! &&x
//! &Some(x)
//! &mut x
//! ```
//!
//! The exact lexical spelling and supported modifier syntax are parser/grammar
//! responsibilities. This node stores the resulting source structure.
//!
//! ## Important semantic boundary
//!
//! This node does **not** decide whether a reference is:
//!
//! - shared;
//! - exclusive;
//! - mutable;
//! - immutable;
//! - borrowed;
//! - moved;
//! - copied;
//! - aliased;
//! - short-lived;
//! - long-lived;
//! - legal for a quantum resource.
//!
//! If Zamani's grammar distinguishes those forms, the distinction should be
//! represented by source-level fields or dedicated source-level pattern nodes.
//! Semantic analysis must determine their actual legality.
//!
//! ## Child identity
//!
//! `pattern` is a `NodeId`, not a Rust reference or pointer.
//!
//! This provides:
//!
//! - stable identity;
//! - deterministic serialization;
//! - source-map integration;
//! - side-table compatibility;
//! - semantic-analysis lookup;
//! - iterative traversal;
//! - large-AST scalability;
//! - absence of pointer-address semantics.
//!
//! The node does not own the child node.
//!
//! ## Dependency contract
//!
//! This module may depend only on:
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
//! - ownership checker implementation;
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
//! Local validation is intentionally limited to structural properties:
//!
//! 1. the node has the native reference-pattern classification;
//! 2. its child `NodeId` is structurally valid;
//! 3. the referenced child is present in the surrounding AST;
//! 4. the referenced child is a pattern;
//! 5. the child relationship is not otherwise malformed.
//!
//! This file must **not** perform:
//!
//! - name resolution;
//! - type checking;
//! - borrow checking;
//! - lifetime checking;
//! - ownership checking;
//! - resource-capability checking;
//! - quantum legality checking;
//! - target capability checking.
//!
//! Those belong to later phases.
//!
//! ## Validation ownership
//!
//! Because the child is represented by `NodeId`, complete graph validation
//! necessarily requires access to the surrounding AST storage.
//!
//! Consequently, this node provides local accessors and invariants, while the
//! central AST validation subsystem validates the cross-node relationship.
//!
//! No local API invents a second AST-storage mechanism.
//!
//! ## Traversal
//!
//! This node has exactly one child:
//!
//! ```text
//! ReferencePattern
//!      │
//!      ▼
//! referenced pattern
//! ```
//!
//! Generic visitors/traversals should obtain that child through
//! [`Self::child_node_id`].
//!
//! The node does not recursively traverse itself. This is deliberate: traversal
//! depth must be controlled by the AST traversal subsystem rather than hidden
//! inside individual nodes.
//!
//! ## Scalability
//!
//! There is no language-level maximum for:
//!
//! - reference nesting;
//! - number of reference patterns;
//! - number of AST nodes;
//! - number of bindings;
//! - number of quantum resources;
//! - number of machines;
//! - resource cardinality.
//!
//! The representation contains exactly one child identity and therefore grows
//! with the logical structure of the program.
//!
//! Any hostile-input protection such as maximum AST depth must be supplied by a
//! configurable compiler policy.
//!
//! No fixed value such as `MAX_REFERENCE_DEPTH` belongs in this file.
//!
//! ## Determinism
//!
//! The node contains no:
//!
//! - memory address;
//! - timestamp;
//! - random state;
//! - process identifier;
//! - thread-local state;
//! - backend state.
//!
//! Equality, hashing, and serialization therefore operate on logical AST state.
//!
//! ## Serialization
//!
//! Serde serialization preserves:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - child `NodeId`.
//!
//! Global AST serialization/versioning remains owned by the AST serialization
//! subsystem.
//!
//! This file does not introduce a competing wire format.
//!
//! ## Source mapping
//!
//! The node's `Span` belongs to the complete source construct, including the
//! reference syntax and referenced pattern according to parser policy.
//!
//! The child node owns its own span.
//!
//! This permits diagnostics to distinguish:
//!
//! ```text
//! entire reference pattern
//!         │
//!         └── nested pattern
//! ```
//!
//! without reconstructing source locations from strings.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes:
//!
//! ```text
//! ReferencePattern::pattern()
//! ```
//!
//! and resolves the referenced child through the AST's `NodeId` storage.
//!
//! The semantic phase may then construct side-table information describing:
//!
//! - resolved type;
//! - binding;
//! - ownership;
//! - borrow mode;
//! - lifetime;
//! - aliasing;
//! - resource capability;
//! - domain;
//! - quantum-resource semantics.
//!
//! None of that information is stored here.
//!
//! ## ZUIR integration
//!
//! This module deliberately imports no ZUIR type.
//!
//! A reference pattern does not itself become a hardware instruction.
//!
//! Semantic lowering determines whether the reference corresponds to:
//!
//! - a value binding;
//! - a borrowed resource;
//! - a view;
//! - an alias;
//! - another semantic construct.
//!
//! The resulting semantic representation is then lowered to ZUIR.
//!
//! ## Quantum integration
//!
//! A reference pattern may eventually refer to a quantum resource if the
//! surrounding semantic context permits it.
//!
//! For example, conceptually:
//!
//! ```text
//! match quantum_resource {
//!     &q => ...
//! }
//! ```
//!
//! The AST does not assume that `q` is a physical qubit, logical qubit,
//! superconducting qubit, ion, photon, neutral atom, or any other technology.
//!
//! Quantum-resource semantics remain downstream.
//!
//! ## Parser integration contract
//!
//! The parser must construct `ReferencePattern` only when the grammar recognizes
//! a reference-pattern production.
//!
//! The parser is responsible for:
//!
//! - lexical recognition;
//! - source spans;
//! - node-ID allocation;
//! - constructing the referenced pattern;
//! - attaching metadata;
//! - syntactic recovery.
//!
//! The parser must not perform semantic ownership, lifetime, type, resource,
//! quantum, hardware, or backend analysis.
//!
//! ## Pattern integration
//!
//! The referenced child must be a pattern node.
//!
//! The parser must not use this type as a generic wrapper around arbitrary
//! expressions or statements.
//!
//! If the language later introduces reference expressions, those belong to the
//! expression subsystem and must not reuse this pattern node.
//!
//! ## Error behavior
//!
//! Constructors do not panic on malformed `NodeId` relationships.
//!
//! Structural errors are reported by the AST validation layer when the node is
//! validated against the complete AST graph.
//!
//! This separation is important because a standalone node cannot determine
//! whether its child ID exists without access to the containing AST.
//!
//! ## Thread safety
//!
//! The type owns no global mutable state and contains no execution state.
//!
//! Read-only AST consumers can use it from parallel compiler phases whenever
//! the surrounding AST storage and constituent types satisfy the relevant
//! `Send`/`Sync` requirements.
//!
//! ## Security
//!
//! The node performs no unchecked indexing, pointer arithmetic, allocation based
//! on attacker-controlled multiplication, or recursive traversal.
//!
//! Malformed child IDs remain ordinary data until central structural validation
//! checks them against AST storage.
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
//! ```text
//! src/frontend/ast/node/patterns/mod.rs
//!     └── pub mod reference;
//!
//! src/frontend/ast/node/mod.rs
//!     └── expose patterns module according to canonical AST module layout
//!
//! parser
//!     └── reference-pattern grammar
//!         └── construct ReferencePattern
//!
//! structural validation
//!     └── verify referenced NodeId exists
//!         └── verify referenced node is a pattern
//!
//! visitors
//!     └── visit ReferencePattern
//!         └── visit referenced pattern
//!
//! traversal
//!     └── expose exactly one child NodeId
//!
//! semantic analysis
//!     └── resolve reference/ownership/lifetime semantics
//!
//! ZUIR lowering
//!     └── consume resolved semantic representation
//! ```
//!
//! Adding a quantum backend, quantum technology, hardware topology,
//! computational domain, machine size, or target architecture must not require
//! changing this file.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Stable contract version for [`ReferencePattern`].
///
/// This version is independent of the Zamani language version and the global
/// AST serialization format version.
pub const REFERENCE_PATTERN_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for this AST construct.
pub const REFERENCE_PATTERN_KIND_NAME: &str = "zamani:reference-pattern";

/// Canonical source-level reference pattern.
///
/// A reference pattern wraps exactly one nested pattern identified by
/// [`NodeId`].
///
/// The nested pattern is owned by the surrounding AST storage, not by this
/// structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReferencePattern {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Node ID of the nested pattern.
    pattern: NodeId,
}

impl ReferencePattern {
    /// Creates a reference pattern from an existing common AST node.
    ///
    /// The supplied node is preserved exactly. In particular, this constructor
    /// does not silently rewrite an incorrect node kind.
    ///
    /// Structural validation must therefore verify that the node has the
    /// expected reference-pattern classification.
    #[must_use]
    pub fn from_node(node: Node, pattern: NodeId) -> Self {
        Self { node, pattern }
    }

    /// Creates a canonical reference pattern.
    ///
    /// No semantic validation is performed.
    ///
    /// The nested `NodeId` is intentionally accepted without dereferencing it:
    /// the complete AST graph is the authority for cross-node validity.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        pattern: NodeId,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::ReferencePattern),
            span,
            metadata,
        );

        Self::from_node(node, pattern)
    }

    /// Creates a canonical reference pattern with default metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        pattern: NodeId,
    ) -> Self {
        Self::new(id, span, NodeMetadata::default(), pattern)
    }

    /// Returns the nested pattern's stable AST identity.
    #[must_use]
    #[inline]
    pub fn pattern(&self) -> NodeId {
        self.pattern
    }

    /// Returns the nested pattern's stable AST identity.
    ///
    /// This alias makes traversal-oriented code explicit without exposing the
    /// internal field.
    #[must_use]
    #[inline]
    pub fn child_node_id(&self) -> NodeId {
        self.pattern
    }

    /// Returns the number of direct AST children.
    ///
    /// A reference pattern always contains exactly one nested pattern.
    #[must_use]
    #[inline]
    pub const fn child_count(&self) -> usize {
        1
    }

    /// Returns the number of direct AST children.
    ///
    /// This is a named predicate useful to generic traversal code.
    #[must_use]
    #[inline]
    pub const fn has_child(&self) -> bool {
        true
    }

    /// Returns whether this node is structurally a wrapper around another
    /// pattern.
    #[must_use]
    #[inline]
    pub const fn is_wrapper(&self) -> bool {
        true
    }

    /// Returns whether this node contains a nested pattern.
    #[must_use]
    #[inline]
    pub const fn has_nested_pattern(&self) -> bool {
        true
    }

    /// Replaces the nested pattern ID.
    ///
    /// The replacement is not dereferenced here. The complete AST validator
    /// remains responsible for checking that the resulting ID exists and
    /// identifies a valid pattern.
    ///
    /// The previous ID is returned so transformations can remain explicit and
    /// deterministic.
    #[inline]
    pub fn replace_pattern(&mut self, pattern: NodeId) -> NodeId {
        std::mem::replace(&mut self.pattern, pattern)
    }

    /// Sets the nested pattern ID.
    ///
    /// This is equivalent to [`Self::replace_pattern`] when the previous value
    /// is not needed.
    #[inline]
    pub fn set_pattern(&mut self, pattern: NodeId) {
        self.pattern = pattern;
    }

    /// Returns the stable schema version of this node contract.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        REFERENCE_PATTERN_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind identifier.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        REFERENCE_PATTERN_KIND_NAME
    }

    /// Returns the direct child as a single-item iterator.
    ///
    /// This avoids allocating a temporary `Vec` merely to expose one child.
    ///
    /// The iterator is backed by a stack-local array and therefore does not
    /// modify the AST or allocate according to this method's implementation.
    ///
    /// For hot traversal paths, [`Self::child_node_id`] is preferable because
    /// it returns the ID directly.
    #[must_use]
    pub fn child_node_ids(&self) -> std::array::IntoIter<NodeId, 1> {
        [self.pattern].into_iter()
    }

    /// Performs validation that is possible without access to the surrounding
    /// AST storage.
    ///
    /// This checks only the node's local classification. Cross-node validation
    /// must be performed by the central AST validation subsystem.
    ///
    /// The method returns `Ok(())` when the embedded node has the canonical
    /// reference-pattern classification.
    pub fn validate_local(&self) -> Result<(), ReferencePatternError> {
        let expected = NodeKind::core(CoreNodeKind::ReferencePattern);

        if self.node.kind() != &expected {
            return Err(ReferencePatternError::InvalidNodeKind {
                expected,
                actual: self.node.kind_owned(),
            });
        }

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
    /// Only common metadata/span/kind operations exposed by `Node` are
    /// available through this object. Semantic state cannot be attached here.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl AstNode for ReferencePattern {
    /// Returns the common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Structural errors local to a [`ReferencePattern`].
///
/// Cross-node graph errors are intentionally represented by the central AST
/// validator because this node alone cannot determine whether `pattern` exists
/// or whether it points to a pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReferencePatternError {
    /// The common node has a classification other than the canonical
    /// reference-pattern classification.
    InvalidNodeKind {
        /// Required classification.
        expected: NodeKind,

        /// Actual classification.
        actual: NodeKind,
    },
}

impl std::fmt::Display for ReferencePatternError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNodeKind { expected, actual } => write!(
                formatter,
                "invalid reference pattern node kind: expected {expected}, found {actual}"
            ),
        }
    }
}

impl std::error::Error for ReferencePatternError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::from(value)
    }

    #[test]
    fn constructor_preserves_child_identity() {
        let id = node_id(1);
        let child = node_id(2);

        let pattern = ReferencePattern::without_metadata(
            id,
            Span::default(),
            child,
        );

        assert_eq!(pattern.id(), id);
        assert_eq!(pattern.pattern(), child);
        assert_eq!(pattern.child_node_id(), child);
        assert_eq!(pattern.child_count(), 1);
        assert!(pattern.has_child());
        assert!(pattern.has_nested_pattern());
        assert!(pattern.is_wrapper());
    }

    #[test]
    fn constructor_uses_reference_pattern_kind() {
        let pattern = ReferencePattern::without_metadata(
            node_id(1),
            Span::default(),
            node_id(2),
        );

        assert_eq!(
            pattern.kind(),
            &NodeKind::core(CoreNodeKind::ReferencePattern)
        );
    }

    #[test]
    fn local_validation_accepts_canonical_node() {
        let pattern = ReferencePattern::without_metadata(
            node_id(1),
            Span::default(),
            node_id(2),
        );

        assert!(pattern.validate_local().is_ok());
    }

    #[test]
    fn local_validation_rejects_wrong_node_kind() {
        let node = Node::without_metadata(
            node_id(1),
            NodeKind::core(CoreNodeKind::Pattern),
            Span::default(),
        );

        let pattern = ReferencePattern::from_node(node, node_id(2));

        let error = pattern
            .validate_local()
            .expect_err("wrong node kind must be rejected");

        assert!(matches!(
            error,
            ReferencePatternError::InvalidNodeKind { .. }
        ));
    }

    #[test]
    fn child_iterator_contains_exactly_one_id() {
        let child = node_id(42);

        let pattern = ReferencePattern::without_metadata(
            node_id(1),
            Span::default(),
            child,
        );

        let children: Vec<_> = pattern.child_node_ids().collect();

        assert_eq!(children, vec![child]);
    }

    #[test]
    fn replacement_returns_previous_child() {
        let first = node_id(2);
        let second = node_id(3);

        let mut pattern = ReferencePattern::without_metadata(
            node_id(1),
            Span::default(),
            first,
        );

        assert_eq!(pattern.replace_pattern(second), first);
        assert_eq!(pattern.pattern(), second);
    }

    #[test]
    fn serialization_round_trip_preserves_logical_state() {
        let pattern = ReferencePattern::without_metadata(
            node_id(1),
            Span::default(),
            node_id(2),
        );

        let encoded =
            serde_json::to_string(&pattern).expect("serialization must succeed");

        let decoded: ReferencePattern =
            serde_json::from_str(&encoded).expect("deserialization must succeed");

        assert_eq!(pattern, decoded);
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(ReferencePattern::schema_version(), 1);
        assert_eq!(
            ReferencePattern::kind_name(),
            "zamani:reference-pattern"
        );
    }

    #[test]
    fn nested_reference_patterns_do_not_require_schema_changes() {
        let innermost = node_id(3);
        let middle = node_id(2);
        let outer = node_id(1);

        let first = ReferencePattern::without_metadata(
            middle,
            Span::default(),
            innermost,
        );

        let second = ReferencePattern::without_metadata(
            outer,
            Span::default(),
            first.id(),
        );

        assert_eq!(second.child_count(), 1);
        assert_eq!(second.pattern(), first.id());
    }

    #[test]
    fn unicode_and_large_source_ids_are_not_interpreted_as_limits() {
        let pattern = ReferencePattern::without_metadata(
            node_id(u64::MAX),
            Span::default(),
            node_id(u64::MAX - 1),
        );

        assert_eq!(pattern.id(), node_id(u64::MAX));
        assert_eq!(pattern.pattern(), node_id(u64::MAX - 1));
    }
}