//! # Zamani Native AST — Cast Expression
//!
//! Production-ready source-level representation of an explicit type cast.
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
//! CastExpression  ← this module
//!     │
//!     ▼
//! structural AST validation
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
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level structural representation** of an
//! explicit cast expression.
//!
//! A cast has exactly two structural children:
//!
//! 1. the expression being converted;
//! 2. the source-level type expression describing the requested target type.
//!
//! This module deliberately does not determine whether the conversion is
//! legal, lossless, lossy, implicit/explicit, runtime-checked, compile-time
//! evaluable, resource-sensitive, or supported by a particular target.
//!
//! Those decisions belong to semantic analysis and later compilation stages.
//!
//! ## POCO-REAF
//!
//! A cast is represented using generic AST node identities rather than
//! machine-specific representations.
//!
//! Consequently this node contains no assumptions about:
//!
//! - CPU width;
//! - GPU width;
//! - FPGA width;
//! - QPU architecture;
//! - qubit count;
//! - register size;
//! - pointer width;
//! - vendor;
//! - backend;
//! - instruction set;
//! - quantum technology;
//! - hardware topology;
//! - execution target.
//!
//! A cast such as:
//!
//! ```text
//! value as TargetType
//! ```
//!
//! describes source-level intent.
//!
//! The semantic/compiler pipeline determines what that intent means for the
//! selected compilation target.
//!
//! ## Critical AST boundary
//!
//! This type is an AST node, not an IR node.
//!
//! It must never contain:
//!
//! - resolved source type;
//! - resolved target type;
//! - symbol identity;
//! - runtime type information;
//! - SSA values;
//! - memory addresses;
//! - physical resource identifiers;
//! - quantum qubit identifiers;
//! - QIR values;
//! - LLVM types;
//! - LLVM values;
//! - MLIR operations;
//! - backend conversion instructions;
//! - hardware conversion rules;
//! - optimization decisions.
//!
//! Those belong to later compiler layers.
//!
//! ## Child representation
//!
//! Children are represented by [`NodeId`] rather than recursively embedding
//! AST values.
//!
//! ```text
//! CastExpression
//! ├── Node
//! ├── expression: NodeId
//! └── target_type: NodeId
//! ```
//!
//! This matches the canonical Zamani expression architecture.
//!
//! It also prevents recursive Rust ownership structures and permits the
//! repository's traversal subsystem to perform iterative traversal for very
//! deeply nested programs.
//!
//! ## Why `target_type` is a `NodeId`
//!
//! The target is deliberately represented by a type AST node rather than by a
//! Rust `Type`, primitive enum, string, or resolved semantic type.
//!
//! This preserves source-level information and allows the target type to be:
//!
//! - primitive;
//! - named;
//! - generic;
//! - parameterized;
//! - qualified;
//! - reference-like;
//! - resource-like;
//! - extension-defined;
//! - future language-defined;
//! - otherwise representable by Zamani's type AST.
//!
//! Semantic analysis determines the actual meaning.
//!
//! ## No finite type list
//!
//! This module must never introduce a closed representation such as:
//!
//! ```text
//! enum CastTarget {
//!     Int32,
//!     Int64,
//!     Float32,
//!     Float64,
//!     Qubit,
//!     ...
//! }
//! ```
//!
//! Such a representation would make the cast AST depend on today's type and
//! hardware vocabulary.
//!
//! The canonical `NodeId` reference keeps the representation extensible.
//!
//! ## Explicit cast vs type ascription
//!
//! A cast expresses a conversion request.
//!
//! A type ascription expresses a type assertion/annotation without necessarily
//! requesting conversion.
//!
//! These must remain distinct AST concepts.
//!
//! `CastExpression` therefore must not be reused to represent type ascription.
//!
//! The repository already has a separate `CoreNodeKind::TypeAscription`
//! classification.
//!
//! ## Semantic boundary
//!
//! Semantic analysis is responsible for determining, for example:
//!
//! - whether the source expression is convertible;
//! - whether the target type is well formed;
//! - whether conversion is implicit or explicit according to language rules;
//! - whether conversion can lose information;
//! - whether conversion requires runtime checks;
//! - whether conversion is permitted for a resource type;
//! - whether conversion affects ownership;
//! - whether conversion affects effects;
//! - whether conversion affects capabilities;
//! - whether conversion is valid for a quantum/classical boundary;
//! - whether a target supports the required realization.
//!
//! None of those properties are stored here.
//!
//! ## Quantum compatibility
//!
//! A generic cast can participate in quantum/classical/hybrid programs without
//! making the AST quantum-specific.
//!
//! For example, a source language may eventually permit conversions involving
//! generic resource or value types. Whether a particular conversion is legal
//! is determined downstream.
//!
//! This module must never contain special cases such as:
//!
//! ```text
//! QuantumCast
//! QubitCast
//! PhysicalQubitCast
//! MeasurementCast
//! QIRCast
//! ```
//!
//! If a genuinely new source-level quantum construct is required, it belongs
//! in an appropriate language extension rather than contaminating this core
//! AST node.
//!
//! ## Hardware boundary
//!
//! A cast must not select or encode:
//!
//! - a hardware conversion instruction;
//! - a vendor instruction;
//! - a device-specific representation;
//! - a physical resource;
//! - a register width;
//! - a machine ABI.
//!
//! Target lowering is downstream.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native AST infrastructure:
//!
//! - [`AstNode`];
//! - [`Node`];
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
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - external quantum-language ASTs.
//!
//! ## Structural validation boundary
//!
//! [`CastExpression::validate_structure`] performs only local structural
//! validation.
//!
//! It checks:
//!
//! - the embedded node has `CoreNodeKind::CastExpression`;
//! - the expression child does not reference this cast node itself;
//! - the target-type child does not reference this cast node itself;
//! - the two child roles do not alias one another.
//!
//! It does **not** check whether the child IDs actually exist in the complete
//! AST graph. That belongs to graph-level AST validation.
//!
//! It also does not check:
//!
//! - whether the source type is castable;
//! - whether the target type exists;
//! - whether the conversion is safe;
//! - whether the conversion is lossy;
//! - whether the conversion is constant-evaluable;
//! - whether a quantum conversion is legal;
//! - whether a target supports the conversion.
//!
//! ## Source spans
//!
//! The common [`Node`] stores the complete source span of the cast expression.
//!
//! Child source spans remain attached to the referenced child AST nodes.
//!
//! This avoids duplicating source-coordinate information.
//!
//! ## Traversal
//!
//! [`CastExpression::child_node_ids`] returns children in deterministic
//! source-semantic order:
//!
//! 1. expression;
//! 2. target type.
//!
//! The method is deliberately non-recursive.
//!
//! ## Scalability
//!
//! The node has exactly two direct child references because a cast has exactly
//! two structural operands.
//!
//! This constant structural arity is not a machine-size limit.
//!
//! There is no limit here on:
//!
//! - number of casts;
//! - AST size;
//! - nesting depth;
//! - type complexity;
//! - program size;
//! - number of resources;
//! - number of qubits;
//! - number of machines.
//!
//! Deep nesting is handled by the repository's traversal and validation
//! infrastructure, which may impose configurable operational safety policies.
//!
//! ## Determinism
//!
//! This type contains no hash-map-backed child collection.
//!
//! Child enumeration is always:
//!
//! ```text
//! expression → target_type
//! ```
//!
//! Equality and hashing are structural and deterministic.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! Global AST serialization versioning belongs to the repository's
//! serialization subsystem and must not be duplicated here.
//!
//! The serialized representation must preserve:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - expression child ID;
//! - target-type child ID.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no source program;
//! - does not dereference pointers;
//! - performs no unchecked indexing;
//! - does not recursively traverse children;
//! - does not use global mutable state;
//! - does not impose machine-specific limits.
//!
//! ## Parser integration
//!
//! The parser is responsible for:
//!
//! 1. parsing the source-level cast syntax;
//! 2. allocating a `NodeId` for the cast;
//! 3. determining the complete source `Span`;
//! 4. constructing the expression child;
//! 5. constructing the target-type child;
//! 6. constructing this node.
//!
//! The parser must not resolve the source or target type.
//!
//! ## Expression aggregate integration
//!
//! The existing canonical expression aggregate already recognizes a cast
//! through:
//!
//! ```text
//! ExpressionKind::Cast {
//!     expression,
//!     target_type,
//! }
//! ```
//!
//! Therefore this file must remain structurally compatible with those two
//! `NodeId` fields.
//!
//! The aggregate should delegate the cast-specific structural representation
//! to this type during the AST decomposition/migration.
//!
//! The aggregate must continue mapping the cast to:
//!
//! ```text
//! CoreNodeKind::CastExpression
//! ```
//!
//! rather than introducing another node kind.
//!
//! ## `expressions/mod.rs` integration
//!
//! The expressions module should expose:
//!
//! ```text
//! pub mod cast;
//! ```
//!
//! and, where the repository's public API convention permits:
//!
//! ```text
//! pub use cast::CastExpression;
//! ```
//!
//! No additional cast classification is required.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes this node and resolves:
//!
//! ```text
//! expression: NodeId
//! target_type: NodeId
//! ```
//!
//! against the semantic environment.
//!
//! The semantic layer may produce a representation conceptually equivalent to:
//!
//! ```text
//! Cast {
//!     source_value: resolved value,
//!     source_type: resolved type,
//!     target_type: resolved type,
//!     conversion: resolved conversion semantics,
//! }
//! ```
//!
//! That semantic representation must not be stored back into this AST node.
//!
//! ## ZUIR integration
//!
//! ZUIR lowering must consume the resolved semantic cast rather than requiring
//! this AST node to understand ZUIR.
//!
//! Conceptually:
//!
//! ```text
//! CastExpression
//!       │
//!       ▼
//! SemanticModel::Cast
//!       │
//!       ▼
//! ZUIR conversion/value operation
//! ```
//!
//! The exact ZUIR operation is determined by the semantic and IR contracts.
//!
//! This file must not import ZUIR.
//!
//! ## Optimization
//!
//! Constant folding, redundant-cast elimination, conversion fusion, range
//! analysis and target-specific conversion optimization belong downstream.
//!
//! This AST node must preserve what the programmer wrote.
//!
//! ## No-re-edit contract
//!
//! This file's stable contract is:
//!
//! - one common AST node;
//! - one source expression child;
//! - one target-type child;
//! - deterministic child enumeration;
//! - local structural validation;
//! - source span preservation;
//! - metadata preservation;
//! - no semantic resolution;
//! - no target knowledge.
//!
//! Changes to:
//!
//! - ZUIR;
//! - quantum IR;
//! - QEC;
//! - routing;
//! - scheduling;
//! - hardware;
//! - calibration;
//! - resilience;
//! - runtime;
//! - backend providers
//!
//! must not require modifying this file.
//!
//! A modification is required only when the actual **source-language cast
//! semantics or AST contract** changes.
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

/// Schema version of the cast-expression contract.
///
/// This version is independent from:
///
/// - the Zamani language version;
/// - the complete AST serialization version;
/// - the compiler version;
/// - extension versions.
pub const CAST_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name of the cast-expression node.
pub const CAST_EXPRESSION_KIND_NAME: &str = "zamani:cast-expression";

/// A source-level explicit type-cast expression.
///
/// The node contains only source structure.
///
/// ```text
/// CastExpression
/// ├── node
/// ├── expression
/// └── target_type
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CastExpression {
    /// Common source-level identity, classification, span and metadata.
    node: Node,

    /// AST node containing the value/expression being converted.
    expression: NodeId,

    /// AST node containing the requested source-level target type.
    target_type: NodeId,
}

/// Stable short alias for code that prefers `Cast`.
pub type Cast = CastExpression;

/// Errors detectable using only the local structure of a cast expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CastValidationError {
    /// The embedded node has a classification other than
    /// `CoreNodeKind::CastExpression`.
    InvalidNodeKind {
        /// The actual node classification.
        actual: NodeKind,
    },

    /// The source expression refers to the cast node itself.
    SelfReferentialExpression,

    /// The target type refers to the cast node itself.
    SelfReferentialTargetType,

    /// The source expression and target type refer to the same AST node.
    ExpressionAndTargetTypeAlias,
}

impl fmt::Display for CastValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "cast expression has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialExpression => {
                formatter.write_str(
                    "cast expression cannot use itself as its source expression",
                )
            }

            Self::SelfReferentialTargetType => {
                formatter.write_str(
                    "cast expression cannot use itself as its target type",
                )
            }

            Self::ExpressionAndTargetTypeAlias => {
                formatter.write_str(
                    "cast expression source and target type must not alias",
                )
            }
        }
    }
}

impl std::error::Error for CastValidationError {}

impl CastExpression {
    /// Creates a cast expression from an already constructed [`Node`].
    ///
    /// This constructor preserves the supplied node unchanged.
    ///
    /// Use [`Self::new`] when constructing a canonical cast node from scratch.
    #[must_use]
    pub fn from_node(
        node: Node,
        expression: NodeId,
        target_type: NodeId,
    ) -> Self {
        Self {
            node,
            expression,
            target_type,
        }
    }

    /// Creates a canonical cast expression.
    ///
    /// The constructor establishes:
    ///
    /// ```text
    /// NodeKind::Core(CoreNodeKind::CastExpression)
    /// ```
    ///
    /// It does not perform semantic validation.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        expression: NodeId,
        target_type: NodeId,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::CastExpression),
            span,
            metadata,
        );

        Self::from_node(node, expression, target_type)
    }

    /// Creates a canonical cast expression with default metadata.
    ///
    /// The caller still supplies the node identity and source span explicitly.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        expression: NodeId,
        target_type: NodeId,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            expression,
            target_type,
        )
    }

    /// Returns the embedded common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded common AST node.
    ///
    /// Mutation is intentionally limited to the common node API.
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

    /// Returns the node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
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

    /// Returns mutable node metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the AST node ID of the expression being converted.
    #[inline]
    #[must_use]
    pub const fn expression(&self) -> NodeId {
        self.expression
    }

    /// Returns the AST node ID of the target type.
    #[inline]
    #[must_use]
    pub const fn target_type(&self) -> NodeId {
        self.target_type
    }

    /// Replaces the source expression child.
    ///
    /// This changes AST structure only. It does not resolve or validate the
    /// resulting conversion.
    #[inline]
    pub fn replace_expression(&mut self, expression: NodeId) -> NodeId {
        core::mem::replace(&mut self.expression, expression)
    }

    /// Replaces the target type child.
    ///
    /// This changes AST structure only. It does not perform type checking.
    #[inline]
    pub fn replace_target_type(&mut self, target_type: NodeId) -> NodeId {
        core::mem::replace(&mut self.target_type, target_type)
    }

    /// Returns the direct children of this cast in deterministic order.
    ///
    /// Ordering:
    ///
    /// 1. source expression;
    /// 2. target type.
    ///
    /// This method does not recursively walk the children.
    #[must_use]
    pub fn child_node_ids(&self) -> [NodeId; 2] {
        [self.expression, self.target_type]
    }

    /// Validates the local structural invariants of this cast.
    ///
    /// This method intentionally does not require access to the complete AST
    /// graph.
    ///
    /// Graph-level child existence checks belong to the AST validation layer.
    pub fn validate_structure(
        &self,
    ) -> Result<(), CastValidationError> {
        match self.node.kind() {
            NodeKind::Core(CoreNodeKind::CastExpression) => {}
            actual => {
                return Err(CastValidationError::InvalidNodeKind {
                    actual: actual.clone(),
                });
            }
        }

        let id = self.node.id();

        if self.expression == id {
            return Err(CastValidationError::SelfReferentialExpression);
        }

        if self.target_type == id {
            return Err(CastValidationError::SelfReferentialTargetType);
        }

        if self.expression == self.target_type {
            return Err(
                CastValidationError::ExpressionAndTargetTypeAlias,
            );
        }

        Ok(())
    }

    /// Returns the schema version of this node contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        CAST_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        CAST_EXPRESSION_KIND_NAME
    }

    /// Returns a compact diagnostic summary.
    ///
    /// The summary intentionally does not include arbitrary metadata.
    ///
    /// This prevents diagnostics from accidentally materializing or logging
    /// potentially large metadata payloads.
    #[must_use]
    pub fn diagnostic_summary(&self) -> CastDiagnosticSummary {
        CastDiagnosticSummary {
            id: self.id(),
            span: self.span().clone(),
        }
    }
}

impl AstNode for CastExpression {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Compact diagnostic representation of a cast expression.
///
/// Child IDs are intentionally omitted because diagnostics normally identify
/// the cast itself and can inspect its children separately through the AST
/// graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CastDiagnosticSummary {
    /// Stable AST identity.
    pub id: NodeId,

    /// Complete source span of the cast.
    pub span: Span,
}

impl fmt::Display for CastDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cast expression at {}", self.span)
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
    fn new_creates_canonical_cast_kind() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert_eq!(
            cast.kind(),
            &NodeKind::core(CoreNodeKind::CastExpression)
        );
    }

    #[test]
    fn stores_source_expression_and_target_type() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert_eq!(cast.expression(), node_id(2));
        assert_eq!(cast.target_type(), node_id(3));
    }

    #[test]
    fn child_order_is_deterministic() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert_eq!(
            cast.child_node_ids(),
            [node_id(2), node_id(3)]
        );
    }

    #[test]
    fn valid_cast_passes_structural_validation() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert!(cast.validate_structure().is_ok());
    }

    #[test]
    fn self_referential_expression_is_rejected() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(1),
            node_id(3),
        );

        assert_eq!(
            cast.validate_structure(),
            Err(CastValidationError::SelfReferentialExpression)
        );
    }

    #[test]
    fn self_referential_target_type_is_rejected() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(1),
        );

        assert_eq!(
            cast.validate_structure(),
            Err(CastValidationError::SelfReferentialTargetType)
        );
    }

    #[test]
    fn aliased_children_are_rejected() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(2),
        );

        assert_eq!(
            cast.validate_structure(),
            Err(CastValidationError::ExpressionAndTargetTypeAlias)
        );
    }

    #[test]
    fn wrong_node_kind_is_rejected() {
        let node = Node::new(
            node_id(1),
            NodeKind::core(CoreNodeKind::Identifier),
            span(),
            NodeMetadata::default(),
        );

        let cast = CastExpression::from_node(
            node,
            node_id(2),
            node_id(3),
        );

        assert!(matches!(
            cast.validate_structure(),
            Err(CastValidationError::InvalidNodeKind {
                actual: NodeKind::Core(CoreNodeKind::Identifier)
            })
        ));
    }

    #[test]
    fn replacement_methods_return_previous_values() {
        let mut cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        assert_eq!(
            cast.replace_expression(node_id(4)),
            node_id(2)
        );

        assert_eq!(
            cast.replace_target_type(node_id(5)),
            node_id(3)
        );

        assert_eq!(cast.expression(), node_id(4));
        assert_eq!(cast.target_type(), node_id(5));
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            CastExpression::schema_version(),
            CAST_EXPRESSION_SCHEMA_VERSION
        );

        assert_eq!(
            CastExpression::kind_name(),
            CAST_EXPRESSION_KIND_NAME
        );
    }

    #[test]
    fn clone_preserves_node_identity() {
        let cast = CastExpression::without_metadata(
            node_id(1),
            span(),
            node_id(2),
            node_id(3),
        );

        let cloned = cast.clone();

        assert_eq!(cloned.id(), cast.id());
        assert_eq!(cloned, cast);
    }
}