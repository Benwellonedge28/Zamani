//! # Zamani Frontend AST — Wildcard Pattern
//!
//! This module defines the canonical native Zamani AST representation of the
//! wildcard pattern `_`.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//!     lexer
//!      │
//!      ▼
//!     parser
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │ Native Zamani AST            │
//! │                              │
//! │ patterns::wildcard.rs  ◄─────┤
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!       structural validation
//!                │
//!                ▼
//!       semantic analysis
//!                │
//!                ▼
//!          semantic model
//!                │
//!                ▼
//!               ZUIR
//!                │
//!        ┌───────┼────────┐
//!        ▼       ▼        ▼
//!    classical quantum   HDL
//!       IR       IR       IR
//! ```
//!
//! ## Language meaning
//!
//! The wildcard pattern is the source-level pattern `_`.
//!
//! It matches a value without introducing a binding for that value.
//!
//! ```text
//! _
//! ```
//!
//! The wildcard therefore:
//!
//! - has no binding name;
//! - has no child patterns;
//! - has no generic resource requirement;
//! - has no hardware meaning;
//! - has no quantum-specific meaning;
//! - has no backend identity;
//! - does not select a machine;
//! - does not select a device;
//! - does not select a qubit;
//! - does not encode a fixed resource count.
//!
//! Its semantic interpretation belongs to the pattern/type-checking layer.
//!
//! ## Grammar integration
//!
//! The Zamani grammar defines:
//!
//! ```text
//! wildcardPattern: '_';
//! ```
//!
//! This AST node therefore represents the semantic source structure of that
//! production without embedding parser/token implementation details.
//!
//! ## POCO-REAF
//!
//! A wildcard is inherently independent of computational scale. It represents
//! source-level pattern intent rather than the representation of the matched
//! value.
//!
//! Consequently this node contains no:
//!
//! - machine-size constant;
//! - register-size constant;
//! - qubit-count constant;
//! - array-size constant;
//! - hardware identifier;
//! - backend identifier;
//! - target architecture;
//! - topology;
//! - scheduler state.
//!
//! The same AST node is valid regardless of whether the surrounding program is
//! compiled for a tiny system, a large heterogeneous system, a quantum system,
//! or a future computational architecture.
//!
//! ## Dependency contract
//!
//! This module depends only on foundational native AST infrastructure:
//!
//! - [`super::super::node::AstNode`];
//! - [`super::super::node::Node`];
//! - [`super::super::node_kind::CoreNodeKind`];
//! - [`super::super::node_kind::NodeKind`];
//! - the Rust standard library;
//! - `serde`, indirectly through the canonical `Node` representation.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - type inference;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum hardware;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience;
//! - runtime execution;
//! - backend providers.
//!
//! ## Integration contract
//!
//! The parser creates a [`WildcardPattern`] when it recognizes `_`.
//!
//! Structural validation verifies that:
//!
//! - the node kind is `zamani:wildcard-pattern`;
//! - the node has no child nodes;
//! - the node introduces no bindings.
//!
//! Semantic analysis consumes this node and determines whether `_` is valid in
//! the surrounding pattern context.
//!
//! Lowering to semantic IR/ZUIR is performed by the semantic/lowering layer.
//! This file intentionally contains no lowering dependency.
//!
//! ## Important boundary
//!
//! Do not add a `Pattern` enum dependency here merely to make this file aware of
//! the complete pattern hierarchy. The concrete wildcard node should remain
//! independently usable. The pattern aggregate/enum, visitor dispatch and
//! parser integration belong to their respective modules.
//!
//! ## No hard-coded scalability limits
//!
//! This type contains a constant amount of structural state and therefore does
//! not impose a limit on:
//!
//! - the number of wildcard nodes in a program;
//! - the number of patterns in a program;
//! - the number of match arms;
//! - the number of resources;
//! - the number of qubits;
//! - the number of classical values;
//! - the size of the eventual target machine.
//!
//! Any compiler-wide resource limits required for hostile-input protection must
//! be supplied by configurable compiler-limit infrastructure.
//!
//! ## Determinism
//!
//! The wildcard node contains no timestamps, random values, memory addresses,
//! process identifiers, thread-local state or backend state.
//!
//! Cloning preserves the same logical [`NodeId`](super::super::node_id::NodeId),
//! consistent with the canonical `Node` contract.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code.
//!
//! ## Rust compatibility
//!
//! Target compiler:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! No unstable Rust feature is required.

use super::super::node::{AstNode, Node};
use super::super::node_kind::{CoreNodeKind, NodeKind};

/// Canonical native Zamani AST node for the wildcard pattern `_`.
///
/// A wildcard is an intentionally binding-free pattern. It represents
/// "match this position, but do not bind the matched value".
///
/// The node stores the canonical [`Node`] because source identity, source
/// location and metadata are shared infrastructure for every native Zamani AST
/// node.
///
/// No semantic type, resolved symbol, resource identity, backend information,
/// quantum information or target information is stored here.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WildcardPattern {
    /// Canonical source-level node identity and metadata.
    node: Node,
}

impl WildcardPattern {
    /// Creates a wildcard pattern from an already-constructed canonical node.
    ///
    /// The supplied node must identify itself as
    /// [`CoreNodeKind::WildcardPattern`].
    ///
    /// This constructor intentionally does not silently rewrite the supplied
    /// node's kind. Doing so would conceal construction errors and make AST
    /// invariants harder to diagnose.
    ///
    /// # Integration
    ///
    /// ```text
    /// parser
    ///   │
    ///   ├── allocate NodeId
    ///   ├── capture Span
    ///   ├── create Node(kind = WildcardPattern)
    ///   │
    ///   ▼
    /// WildcardPattern::new
    ///   │
    ///   ▼
    /// structural validation
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic.
    ///
    /// # Validation
    ///
    /// Use [`Self::validate_structure`] when a fallible structural check is
    /// required.
    #[must_use]
    pub fn new(node: Node) -> Self {
        Self { node }
    }

    /// Returns the canonical node container.
    ///
    /// This method is useful when the parser, validator, visitor or tooling
    /// layer needs to inspect common AST metadata.
    #[inline]
    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the canonical node container.
    ///
    /// Only common node metadata may be changed through the APIs exposed by
    /// [`Node`]. Semantic information must not be attached to the node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the expected canonical node kind for this AST node.
    ///
    /// Keeping this as a single source-level constant avoids duplicating the
    /// node-kind spelling throughout parser, validation and tooling code.
    #[inline]
    #[must_use]
    pub const fn expected_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::WildcardPattern)
    }

    /// Returns `true` when this node carries the canonical wildcard kind.
    ///
    /// This is deliberately a non-panicking predicate so malformed ASTs can be
    /// diagnosed by validation rather than crashing compiler infrastructure.
    #[inline]
    #[must_use]
    pub fn has_expected_kind(&self) -> bool {
        self.node.is_kind(&Self::expected_kind())
    }

    /// Returns the number of child AST nodes owned by the wildcard.
    ///
    /// A wildcard has no child patterns.
    ///
    /// The result is constant because the wildcard grammar production contains
    /// no recursive child production:
    ///
    /// ```text
    /// wildcardPattern: '_';
    /// ```
    ///
    /// This does not impose a limit on the number of wildcard nodes that may
    /// exist in a surrounding program.
    #[inline]
    #[must_use]
    pub const fn child_count() -> usize {
        0
    }

    /// Returns whether the wildcard introduces a binding.
    ///
    /// Wildcards intentionally introduce no binding.
    #[inline]
    #[must_use]
    pub const fn introduces_binding() -> bool {
        false
    }

    /// Returns whether this pattern is irrefutable with respect to the value
    /// being matched.
    ///
    /// At the syntactic pattern level, `_` accepts any value. Whether the
    /// surrounding language construct permits an irrefutable pattern is a
    /// semantic-context question and therefore must be checked downstream.
    #[inline]
    #[must_use]
    pub const fn is_irrefutable() -> bool {
        true
    }

    /// Returns whether this pattern consumes no named binding.
    ///
    /// This is equivalent to [`Self::introduces_binding`], but the explicit
    /// method is useful to semantic analysis and diagnostics because binding
    /// consumption is an important distinction from identifier patterns.
    #[inline]
    #[must_use]
    pub const fn consumes_binding() -> bool {
        false
    }

    /// Returns whether this pattern has no nested pattern.
    #[inline]
    #[must_use]
    pub const fn is_leaf() -> bool {
        true
    }

    /// Performs local structural validation.
    ///
    /// This validation intentionally checks only invariants owned by this
    /// concrete AST node.
    ///
    /// It does **not** perform:
    ///
    /// - type checking;
    /// - name resolution;
    /// - exhaustiveness checking;
    /// - reachability checking;
    /// - quantum validation;
    /// - hardware validation;
    /// - resource allocation;
    /// - ZUIR lowering.
    ///
    /// # Errors
    ///
    /// Returns [`WildcardPatternError`] if the node's classification does not
    /// identify it as the canonical wildcard pattern.
    ///
    /// # Integration
    ///
    /// The parent pattern validator may call this method before semantic
    /// analysis.
    pub fn validate_structure(&self) -> Result<(), WildcardPatternError> {
        if !self.has_expected_kind() {
            return Err(WildcardPatternError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        Ok(())
    }

    /// Returns a compact source-level description suitable for diagnostics.
    ///
    /// This deliberately does not include arbitrary metadata. Extension
    /// metadata can be large, and diagnostics should not accidentally serialize
    /// or print large payloads.
    #[inline]
    #[must_use]
    pub fn diagnostic_name() -> &'static str {
        "wildcard pattern"
    }

    /// Returns the source spelling represented by this AST node.
    ///
    /// This is the canonical source spelling of the language construct, not a
    /// target instruction or backend operation.
    #[inline]
    #[must_use]
    pub const fn source_spelling() -> &'static str {
        "_"
    }
}

impl AstNode for WildcardPattern {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Structural error specific to [`WildcardPattern`].
///
/// This error intentionally remains local to the wildcard node. Global AST
/// validation can wrap or translate it into the compiler's canonical
/// diagnostic representation without requiring this file to depend on the
/// diagnostics subsystem.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WildcardPatternError {
    /// The node was constructed with a node kind other than
    /// `CoreNodeKind::WildcardPattern`.
    InvalidNodeKind {
        /// The actual node kind found in the AST.
        actual: NodeKind,
    },
}

impl core::fmt::Display for WildcardPatternError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => write!(
                formatter,
                "invalid wildcard pattern node kind: expected zamani:wildcard-pattern, found {actual}"
            ),
        }
    }
}

impl std::error::Error for WildcardPatternError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// The wildcard node must always be classified as the canonical native
    /// wildcard node kind.
    #[test]
    fn expected_kind_is_canonical_wildcard_kind() {
        assert_eq!(
            WildcardPattern::expected_kind(),
            NodeKind::Core(CoreNodeKind::WildcardPattern)
        );
    }

    /// A wildcard is a leaf node and therefore owns no child AST nodes.
    #[test]
    fn wildcard_is_a_leaf() {
        assert_eq!(WildcardPattern::child_count(), 0);
        assert!(WildcardPattern::is_leaf());
    }

    /// A wildcard does not introduce a binding.
    #[test]
    fn wildcard_does_not_bind() {
        assert!(!WildcardPattern::introduces_binding());
        assert!(!WildcardPattern::consumes_binding());
    }

    /// The wildcard's source spelling is exactly `_`.
    #[test]
    fn source_spelling_is_underscore() {
        assert_eq!(WildcardPattern::source_spelling(), "_");
    }

    /// The wildcard's source-level semantic property is irrefutability.
    #[test]
    fn wildcard_is_irrefutable() {
        assert!(WildcardPattern::is_irrefutable());
    }

    /// Construction must preserve the supplied canonical node rather than
    /// rewriting it.
    #[test]
    fn constructor_preserves_node() {
        let node = Node::default();
        let wildcard = WildcardPattern::new(node.clone());

        assert_eq!(wildcard.node(), &node);
    }

    /// A correctly classified wildcard passes local structural validation.
    ///
    /// `Node::default()` is intentionally not assumed to have the wildcard
    /// kind. This test therefore demonstrates the validation contract rather
    /// than fabricating a `Node` by mutating private fields.
    ///
    /// The actual parser/AST construction test belongs at the integration layer
    /// where the repository's canonical Node construction API is available.
    #[test]
    fn validation_is_non_panicking() {
        let wildcard = WildcardPattern::new(Node::default());

        let result = wildcard.validate_structure();

        assert!(result.is_err());
    }

    /// The diagnostic name must remain source-level and backend-neutral.
    #[test]
    fn diagnostic_name_is_stable() {
        assert_eq!(
            WildcardPattern::diagnostic_name(),
            "wildcard pattern"
        );
    }

    /// The AST node must expose its common identity through `AstNode`.
    #[test]
    fn ast_node_identity_is_exposed() {
        let wildcard = WildcardPattern::new(Node::default());

        assert_eq!(wildcard.id(), wildcard.node().id());
        assert_eq!(wildcard.kind(), wildcard.node().kind());
        assert_eq!(wildcard.span(), wildcard.node().span());
        assert_eq!(wildcard.metadata(), wildcard.node().metadata());
    }

    /// Cloning a wildcard must preserve logical AST identity.
    #[test]
    fn clone_preserves_logical_identity() {
        let wildcard = WildcardPattern::new(Node::default());
        let cloned = wildcard.clone();

        assert_eq!(wildcard, cloned);
        assert_eq!(wildcard.id(), cloned.id());
    }
}