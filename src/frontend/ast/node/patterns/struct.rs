//! # Zamani Native AST — Struct Pattern
//!
//! Production-ready source-level representation of a struct-pattern.
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
//!     ├── patterns::literal
//!     ├── patterns::tuple
//!     ├── patterns::struct  ◄── this module
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
//!     ├── HDL IR
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
//! This module owns the source-level structural representation of a struct
//! pattern.
//!
//! Conceptually:
//!
//! ```text
//! TypeName {
//!     field_a: pattern_a,
//!     field_b: pattern_b,
//!     ...
//! }
//! ```
//!
//! The actual child nodes are represented by [`NodeId`] references.
//!
//! This module owns:
//!
//! - `StructPattern`;
//! - `StructPatternField`;
//! - field ordering;
//! - field-to-pattern relationships;
//! - local structural invariants;
//! - deterministic child enumeration;
//! - source-level construction and transformation helpers;
//! - local validation errors.
//!
//! It does NOT own:
//!
//! - struct type resolution;
//! - field existence checking;
//! - field visibility;
//! - type checking;
//! - pattern exhaustiveness;
//! - pattern reachability;
//! - symbol resolution;
//! - ownership/resource semantics;
//! - quantum semantics;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend selection;
//! - execution.
//!
//! Those responsibilities belong to later compiler phases.
//!
//! ## Existing repository compatibility
//!
//! The current native `CoreNodeKind` provides the generic `Pattern` category
//! rather than a dedicated `StructPattern` variant. Therefore this node uses:
//!
//! ```text
//! NodeKind::Core(CoreNodeKind::Pattern)
//! ```
//!
//! The concrete Rust type identifies the precise pattern form. This avoids
//! changing the global node-kind classification solely to accommodate one
//! pattern subtype.
//!
//! A future dedicated `StructPattern` node kind can be introduced as an
//! independent AST schema change without changing the structural data model
//! in this file.
//!
//! ## Child graph
//!
//! The node uses explicit graph references rather than recursively embedding
//! patterns:
//!
//! ```text
//! StructPattern
//! ├── node
//! ├── type_path: NodeId
//! └── fields: Vec<StructPatternField>
//!                 ├── field_name: NodeId
//!                 └── pattern: NodeId
//! ```
//!
//! Consequently the actual AST graph owns the child nodes.
//!
//! This is important for very large and deeply nested programs because this
//! node does not recursively allocate nested Rust values.
//!
//! ## Deterministic traversal
//!
//! Children are always enumerated in this order:
//!
//! 1. the struct/type path;
//! 2. for each field, in source order:
//!    1. field-name node;
//!    2. field-pattern node.
//!
//! No hash-map iteration is involved.
//!
//! ## Domain neutrality
//!
//! A struct pattern may eventually match:
//!
//! - an ordinary classical value;
//! - a resource aggregate;
//! - a quantum-related semantic value;
//! - a distributed value;
//! - an accelerator value;
//! - an HDL-level value;
//! - a future computational-domain value.
//!
//! None of those meanings is encoded here.
//!
//! In particular, this file must never acquire representations such as:
//!
//! ```text
//! QuantumStructPattern
//! QubitStructPattern
//! PhysicalQubitStructPattern
//! HardwareStructPattern
//! IBMStructPattern
//! SurfaceCodeStructPattern
//! ```
//!
//! ## POCO-REAF
//!
//! Nothing in this representation depends on:
//!
//! - qubit count;
//! - register width;
//! - machine size;
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - QPU architecture;
//! - topology;
//! - vendor;
//! - backend;
//! - gate set;
//! - scheduler;
//! - router;
//! - calibration;
//! - QEC implementation.
//!
//! Therefore a program containing this pattern can remain source-identical
//! while being compiled for different scales and computational technologies.
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Native AST
//!      │
//!      ▼
//! Semantic interpretation
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! Resource/capability discovery
//!      │
//!      ▼
//! Target realization
//! ```
//!
//! ## Scalability
//!
//! There is no language-level maximum for:
//!
//! - number of struct-pattern fields;
//! - number of struct patterns;
//! - number of AST nodes;
//! - pattern nesting depth;
//! - resource count;
//! - machine size;
//! - qubit count.
//!
//! `Vec` grows according to available memory and the configured compiler
//! resource policy.
//!
//! Any hostile-input limits belong to configurable compiler policy rather than
//! this AST node.
//!
//! ## Source preservation
//!
//! The struct pattern does not duplicate the spelling of its type name or
//! field names. Those are represented by child AST nodes identified by
//! `NodeId`.
//!
//! This preserves the repository's canonical identifier/path representation
//! and avoids creating a second identifier representation inside patterns.
//!
//! ## Semantic boundary
//!
//! Structural validation may determine:
//!
//! - whether the common node has the expected pattern classification;
//! - whether required child references are structurally distinct;
//! - whether field references are locally valid;
//! - whether no child is the struct-pattern node itself.
//!
//! It must NOT determine:
//!
//! - whether the named type exists;
//! - whether a field exists on that type;
//! - whether a field is visible;
//! - whether a field pattern has the correct type;
//! - whether the pattern is exhaustive;
//! - whether the pattern is reachable;
//! - whether the matched value is quantum/classical/hybrid;
//! - whether a backend can execute the resulting match.
//!
//! ## Parser integration
//!
//! The parser should:
//!
//! 1. recognize the struct-pattern syntax;
//! 2. construct the canonical type/path child node;
//! 3. construct field-name child nodes using the canonical identifier/path
//!    infrastructure;
//! 4. construct nested pattern nodes;
//! 5. allocate a distinct `NodeId` for this `StructPattern`;
//! 6. preserve source order;
//! 7. construct this node with [`StructPattern::new`];
//! 8. attach it to the enclosing pattern graph.
//!
//! The parser must not resolve the struct type or field symbols.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes:
//!
//! - `type_path()`;
//! - `fields()`;
//! - `StructPatternField::field_name()`;
//! - `StructPatternField::pattern()`;
//! - node identity;
//! - source spans.
//!
//! It then performs name resolution, field resolution, type checking,
//! exhaustiveness/reachability analysis, resource analysis, capability
//! analysis and domain analysis.
//!
//! No semantic side table is stored in this node.
//!
//! ## ZUIR integration
//!
//! This module intentionally has no ZUIR dependency.
//!
//! Semantic lowering determines whether the pattern becomes, for example:
//!
//! - a field comparison;
//! - a destructuring operation;
//! - a resource decomposition;
//! - a dispatch predicate;
//! - a control-flow predicate;
//! - another universal semantic construct.
//!
//! The AST does not decide that.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! This preserves:
//!
//! - common node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - type/path child identity;
//! - field ordering;
//! - field child identities.
//!
//! Repository-level serialization remains responsible for the global AST
//! serialization schema and graph consistency.
//!
//! This file does not create a second wire protocol.
//!
//! ## Security
//!
//! This implementation:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no source code;
//! - does not dereference `NodeId`s;
//! - does not recursively walk arbitrary graphs;
//! - does not use unchecked indexing;
//! - does not convert numeric source values;
//! - has no global mutable state.
//!
//! Validation is local and bounded by the number of direct fields in this node.
//!
//! ## Thread safety
//!
//! The type owns only ordinary Rust values and has no global mutable state.
//! Read-only instances may therefore participate in parallel compiler phases
//! whenever their constituent types satisfy `Send` and `Sync` naturally.
//!
//! No manual unsafe `Send`/`Sync` implementation is provided.
//!
//! ## No-reedit contract
//!
//! Changes to:
//!
//! - quantum IR;
//! - ZUIR;
//! - semantic analysis;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend providers;
//! - target architectures
//!
//! must not require this file to change.
//!
//! This file should change only when the source-level struct-pattern contract,
//! its canonical child representation, or its explicitly documented AST
//! invariants change.
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
//! ```text
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Independent schema version for the struct-pattern contract.
///
/// This is deliberately distinct from:
///
/// - Zamani language version;
/// - compiler version;
/// - global AST serialization version;
/// - semantic-model version;
/// - ZUIR version;
/// - quantum-IR version.
pub const STRUCT_PATTERN_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name for the concrete pattern abstraction.
pub const STRUCT_PATTERN_KIND_NAME: &str = "zamani:struct-pattern";

/// Result type for local struct-pattern operations.
pub type StructPatternResult<T> = Result<T, StructPatternError>;

/// A field inside a [`StructPattern`].
///
/// Field names and nested patterns are represented by `NodeId` rather than
/// duplicating identifier/pattern structures.
///
/// ```text
/// StructPatternField
/// ├── field_name: NodeId
/// └── pattern: NodeId
/// ```
///
/// The referenced field-name node is expected to be a source-level identifier
/// or path component. The referenced pattern node is expected to be a valid
/// pattern node.
///
/// Exact node-type validation belongs to graph-aware structural validation
/// because this type deliberately does not dereference `NodeId`s.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructPatternField {
    /// Source-level field-name node.
    field_name: NodeId,

    /// Nested pattern applied to this field.
    pattern: NodeId,
}

impl StructPatternField {
    /// Creates a struct-pattern field.
    ///
    /// The constructor does not dereference or semantically interpret either
    /// child.
    #[must_use]
    pub const fn new(field_name: NodeId, pattern: NodeId) -> Self {
        Self {
            field_name,
            pattern,
        }
    }

    /// Returns the AST node containing the field name.
    #[must_use]
    #[inline]
    pub const fn field_name(&self) -> NodeId {
        self.field_name
    }

    /// Returns the AST node containing the nested field pattern.
    #[must_use]
    #[inline]
    pub const fn pattern(&self) -> NodeId {
        self.pattern
    }

    /// Replaces the field-name node.
    ///
    /// Returns the previous node identity.
    #[inline]
    pub const fn replace_field_name(&mut self, field_name: NodeId) -> NodeId {
        core::mem::replace(&mut self.field_name, field_name)
    }

    /// Replaces the nested field pattern.
    ///
    /// Returns the previous node identity.
    #[inline]
    pub const fn replace_pattern(&mut self, pattern: NodeId) -> NodeId {
        core::mem::replace(&mut self.pattern, pattern)
    }

    /// Returns the two direct child IDs in deterministic source order.
    ///
    /// Ordering:
    ///
    /// 1. field name;
    /// 2. nested pattern.
    #[must_use]
    #[inline]
    pub fn child_node_ids(&self) -> [NodeId; 2] {
        [self.field_name, self.pattern]
    }

    /// Returns whether both child references are distinct.
    #[must_use]
    #[inline]
    pub const fn has_distinct_children(&self) -> bool {
        self.field_name != self.pattern
    }
}

/// Errors detectable without consulting the complete AST graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StructPatternError {
    /// The common node does not have the expected generic pattern kind.
    InvalidNodeKind {
        /// Actual node classification.
        actual: NodeKind,
    },

    /// The struct-pattern node refers to itself as its type/path child.
    SelfReferentialTypePath,

    /// A field-name child refers to the struct-pattern node itself.
    SelfReferentialFieldName {
        /// Zero-based source-order field position.
        field_index: usize,
    },

    /// A nested field pattern refers to the struct-pattern node itself.
    SelfReferentialFieldPattern {
        /// Zero-based source-order field position.
        field_index: usize,
    },

    /// A field-name and field-pattern child are the same node.
    FieldNamePatternAlias {
        /// Zero-based source-order field position.
        field_index: usize,
    },

    /// The type/path child aliases another direct child.
    TypePathChildAlias {
        /// Child identity that aliases the type/path.
        id: NodeId,
    },

    /// A direct child is reused in multiple structural positions.
    DuplicateChild {
        /// Duplicated child identity.
        id: NodeId,
    },
}

impl fmt::Display for StructPatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => write!(
                formatter,
                "struct pattern has invalid AST node kind: {actual}"
            ),

            Self::SelfReferentialTypePath => formatter.write_str(
                "struct pattern cannot use itself as its type/path child",
            ),

            Self::SelfReferentialFieldName { field_index } => write!(
                formatter,
                "struct pattern field {field_index} cannot use the struct pattern as its field name"
            ),

            Self::SelfReferentialFieldPattern { field_index } => write!(
                formatter,
                "struct pattern field {field_index} cannot use the struct pattern as its nested pattern"
            ),

            Self::FieldNamePatternAlias { field_index } => write!(
                formatter,
                "struct pattern field {field_index} cannot use the same node for its field name and nested pattern"
            ),

            Self::TypePathChildAlias { id } => write!(
                formatter,
                "struct pattern type/path child aliases another direct child: {id}"
            ),

            Self::DuplicateChild { id } => write!(
                formatter,
                "struct pattern contains duplicate direct child node: {id}"
            ),
        }
    }
}

impl std::error::Error for StructPatternError {}

/// Source-level struct pattern.
///
/// The node references the canonical type/path and field pattern nodes by
/// `NodeId`.
///
/// ```text
/// StructPattern
/// ├── node
/// ├── type_path
/// └── fields
///     ├── field_name → NodeId
///     └── pattern    → NodeId
/// ```
///
/// The AST itself owns the referenced nodes. This object owns only their
/// relationships and source-order structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructPattern {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Canonical path/type-name node.
    type_path: NodeId,

    /// Struct fields in source order.
    fields: Vec<StructPatternField>,
}

impl StructPattern {
    /// Creates a struct pattern from an already constructed common node.
    ///
    /// The node is not rewritten. Structural validation reports a wrong node
    /// classification rather than silently repairing it.
    #[must_use]
    pub fn from_node(
        node: Node,
        type_path: NodeId,
        fields: Vec<StructPatternField>,
    ) -> Self {
        Self {
            node,
            type_path,
            fields,
        }
    }

    /// Creates a canonical struct pattern.
    ///
    /// The common node receives the native generic `Pattern` classification
    /// used by the current AST node-kind contract.
    ///
    /// This constructor performs structural construction only.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        type_path: NodeId,
        fields: Vec<StructPatternField>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Pattern),
            span,
            metadata,
        );

        Self::from_node(node, type_path, fields)
    }

    /// Creates a canonical struct pattern with default metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        type_path: NodeId,
        fields: Vec<StructPatternField>,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            type_path,
            fields,
        )
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns this node's stable AST identity.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns this node's source span.
    #[must_use]
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the node's source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the node classification.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the canonical node kind expected by this implementation.
    #[must_use]
    #[inline]
    pub const fn expected_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::Pattern)
    }

    /// Returns the stable source-level name of this concrete pattern.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        STRUCT_PATTERN_KIND_NAME
    }

    /// Returns this node's independent schema version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        STRUCT_PATTERN_SCHEMA_VERSION
    }

    /// Returns the node identity of the canonical struct/type path.
    #[must_use]
    #[inline]
    pub const fn type_path(&self) -> NodeId {
        self.type_path
    }

    /// Replaces the type/path child.
    ///
    /// Returns the previous child identity.
    ///
    /// The caller should run structural validation after the transformation.
    #[inline]
    pub const fn replace_type_path(&mut self, type_path: NodeId) -> NodeId {
        core::mem::replace(&mut self.type_path, type_path)
    }

    /// Returns the fields in source order.
    #[must_use]
    #[inline]
    pub fn fields(&self) -> &[StructPatternField] {
        &self.fields
    }

    /// Returns mutable access to the source-ordered field list.
    ///
    /// Structural validation should be run after mutations.
    #[inline]
    pub fn fields_mut(&mut self) -> &mut Vec<StructPatternField> {
        &mut self.fields
    }

    /// Returns the number of fields.
    #[must_use]
    #[inline]
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the pattern has no explicitly listed fields.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Appends a field while preserving source order.
    ///
    /// No fixed field count is imposed here.
    pub fn push_field(&mut self, field: StructPatternField) {
        self.fields.push(field);
    }

    /// Appends a field from its two child identities.
    pub fn push_field_ids(
        &mut self,
        field_name: NodeId,
        pattern: NodeId,
    ) {
        self.fields
            .push(StructPatternField::new(field_name, pattern));
    }

    /// Removes and returns the last field.
    #[inline]
    pub fn pop_field(&mut self) -> Option<StructPatternField> {
        self.fields.pop()
    }

    /// Replaces the entire field list.
    ///
    /// Returns the previous list.
    pub fn replace_fields(
        &mut self,
        fields: Vec<StructPatternField>,
    ) -> Vec<StructPatternField> {
        core::mem::replace(&mut self.fields, fields)
    }

    /// Returns the number of direct AST child references.
    ///
    /// The count is:
    ///
    /// ```text
    /// 1 + 2 * field_count
    /// ```
    ///
    /// The calculation is checked to avoid overflow in diagnostics or tooling
    /// even if an externally constructed vector has an enormous length.
    #[must_use]
    pub fn child_count(&self) -> Option<usize> {
        self.fields
            .len()
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
    }

    /// Returns direct child node IDs in deterministic source order.
    ///
    /// Ordering:
    ///
    /// 1. type/path;
    /// 2. field-name;
    /// 3. field-pattern;
    /// 4. next field-name;
    /// 5. next field-pattern;
    /// 6. ...
    ///
    /// This method allocates a vector because the number of children is
    /// dynamic. Callers performing large-scale traversal should prefer
    /// [`Self::for_each_child`] to avoid materializing the complete child list.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::with_capacity(
            self.fields
                .len()
                .saturating_mul(2)
                .saturating_add(1),
        );

        children.push(self.type_path);

        for field in &self.fields {
            children.push(field.field_name());
            children.push(field.pattern());
        }

        children
    }

    /// Visits direct child IDs in deterministic source order without allocating
    /// a temporary child vector.
    ///
    /// This is the preferred primitive for scalable traversal.
    pub fn for_each_child<F>(&self, mut visit: F)
    where
        F: FnMut(NodeId),
    {
        visit(self.type_path);

        for field in &self.fields {
            visit(field.field_name());
            visit(field.pattern());
        }
    }

    /// Returns whether the node is structurally a leaf.
    ///
    /// Always false because a struct pattern has at least its type/path child.
    #[must_use]
    #[inline]
    pub const fn is_leaf(&self) -> bool {
        false
    }

    /// Performs local structural validation.
    ///
    /// This method intentionally does not dereference child IDs. Graph-aware
    /// validation must verify that each ID exists and has the appropriate
    /// concrete node type.
    ///
    /// Local invariants:
    ///
    /// 1. node kind is `CoreNodeKind::Pattern`;
    /// 2. the type/path child is not this node;
    /// 3. every field-name child is not this node;
    /// 4. every nested pattern child is not this node;
    /// 5. field name and nested pattern do not alias;
    /// 6. no direct child is reused by another structural position.
    pub fn validate_structure(&self) -> StructPatternResult<()> {
        let expected = Self::expected_kind();

        if !self.node.is_kind(&expected) {
            return Err(StructPatternError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        let own_id = self.id();

        if self.type_path == own_id {
            return Err(StructPatternError::SelfReferentialTypePath);
        }

        let mut seen = Vec::with_capacity(
            self.fields
                .len()
                .saturating_mul(2)
                .saturating_add(1),
        );

        seen.push(self.type_path);

        for (field_index, field) in self.fields.iter().enumerate() {
            let field_name = field.field_name();
            let field_pattern = field.pattern();

            if field_name == own_id {
                return Err(StructPatternError::SelfReferentialFieldName {
                    field_index,
                });
            }

            if field_pattern == own_id {
                return Err(StructPatternError::SelfReferentialFieldPattern {
                    field_index,
                });
            }

            if field_name == field_pattern {
                return Err(StructPatternError::FieldNamePatternAlias {
                    field_index,
                });
            }

            if seen.contains(&field_name) {
                return Err(StructPatternError::DuplicateChild { id: field_name });
            }

            seen.push(field_name);

            if seen.contains(&field_pattern) {
                return Err(StructPatternError::DuplicateChild {
                    id: field_pattern,
                });
            }

            seen.push(field_pattern);
        }

        Ok(())
    }

    /// Returns whether local structural validation succeeds.
    #[must_use]
    #[inline]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }

    /// Returns whether this node contains the supplied direct child ID.
    ///
    /// This operation does not dereference the child.
    #[must_use]
    pub fn contains_child(&self, id: NodeId) -> bool {
        if self.type_path == id {
            return true;
        }

        self.fields.iter().any(|field| {
            field.field_name() == id || field.pattern() == id
        })
    }

    /// Finds the zero-based field position containing a child node.
    ///
    /// Returns `None` when the child is not directly owned by any field.
    ///
    /// The search is deterministic and preserves source order.
    #[must_use]
    pub fn field_index_for_child(&self, id: NodeId) -> Option<usize> {
        self.fields.iter().position(|field| {
            field.field_name() == id || field.pattern() == id
        })
    }

    /// Returns the field at `index`.
    ///
    /// Uses checked indexing rather than panicking.
    #[must_use]
    #[inline]
    pub fn field(&self, index: usize) -> Option<&StructPatternField> {
        self.fields.get(index)
    }

    /// Returns mutable access to the field at `index`.
    ///
    /// Uses checked indexing rather than panicking.
    #[inline]
    pub fn field_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut StructPatternField> {
        self.fields.get_mut(index)
    }

    /// Removes a field at `index`.
    ///
    /// Returns `None` when the index is outside the current field list.
    ///
    /// Removing a field changes source structure and should therefore be
    /// followed by structural validation by the transformation framework.
    pub fn remove_field(
        &mut self,
        index: usize,
    ) -> Option<StructPatternField> {
        if index < self.fields.len() {
            Some(self.fields.remove(index))
        } else {
            None
        }
    }

    /// Returns an iterator over fields in source order.
    #[must_use]
    #[inline]
    pub fn iter_fields(
        &self,
    ) -> std::slice::Iter<'_, StructPatternField> {
        self.fields.iter()
    }

    /// Returns a mutable iterator over fields in source order.
    #[inline]
    pub fn iter_fields_mut(
        &mut self,
    ) -> std::slice::IterMut<'_, StructPatternField> {
        self.fields.iter_mut()
    }
}

impl AstNode for StructPattern {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for StructPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {{ {} field(s) }}",
            Self::kind_name(),
            self.fields.len()
        )
    }
}

impl fmt::Display for StructPatternField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "field(name={:?}, pattern={:?})",
            self.field_name,
            self.pattern
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn valid_pattern() -> StructPattern {
        StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            vec![
                StructPatternField::new(node_id(3), node_id(4)),
                StructPatternField::new(node_id(5), node_id(6)),
            ],
        )
    }

    #[test]
    fn constructor_uses_pattern_kind() {
        let pattern = valid_pattern();

        assert_eq!(
            pattern.kind(),
            &NodeKind::Core(CoreNodeKind::Pattern)
        );
    }

    #[test]
    fn constructor_preserves_type_path() {
        let pattern = valid_pattern();

        assert_eq!(pattern.type_path(), node_id(2));
    }

    #[test]
    fn fields_preserve_source_order() {
        let pattern = valid_pattern();

        assert_eq!(pattern.field_count(), 2);
        assert_eq!(
            pattern.field(0).expect("first field").field_name(),
            node_id(3)
        );
        assert_eq!(
            pattern.field(0).expect("first field").pattern(),
            node_id(4)
        );
        assert_eq!(
            pattern.field(1).expect("second field").field_name(),
            node_id(5)
        );
        assert_eq!(
            pattern.field(1).expect("second field").pattern(),
            node_id(6)
        );
    }

    #[test]
    fn child_order_is_deterministic() {
        let pattern = valid_pattern();

        assert_eq!(
            pattern.child_node_ids(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(6),
            ]
        );
    }

    #[test]
    fn child_count_matches_child_enumeration() {
        let pattern = valid_pattern();

        assert_eq!(pattern.child_count(), Some(5));
        assert_eq!(
            pattern.child_count(),
            Some(pattern.child_node_ids().len())
        );
    }

    #[test]
    fn empty_field_list_is_valid() {
        let pattern = StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            Vec::new(),
        );

        assert!(pattern.is_empty());
        assert!(pattern.validate_structure().is_ok());
    }

    #[test]
    fn local_validation_accepts_valid_pattern() {
        assert!(valid_pattern().validate_structure().is_ok());
    }

    #[test]
    fn self_referential_type_path_is_rejected() {
        let pattern = StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(1),
            Vec::new(),
        );

        assert_eq!(
            pattern.validate_structure(),
            Err(StructPatternError::SelfReferentialTypePath)
        );
    }

    #[test]
    fn self_referential_field_name_is_rejected() {
        let pattern = StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            vec![StructPatternField::new(
                node_id(1),
                node_id(3),
            )],
        );

        assert_eq!(
            pattern.validate_structure(),
            Err(StructPatternError::SelfReferentialFieldName {
                field_index: 0,
            })
        );
    }

    #[test]
    fn self_referential_field_pattern_is_rejected() {
        let pattern = StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            vec![StructPatternField::new(
                node_id(3),
                node_id(1),
            )],
        );

        assert_eq!(
            pattern.validate_structure(),
            Err(StructPatternError::SelfReferentialFieldPattern {
                field_index: 0,
            })
        );
    }

    #[test]
    fn field_name_pattern_alias_is_rejected() {
        let pattern = StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            vec![StructPatternField::new(
                node_id(3),
                node_id(3),
            )],
        );

        assert_eq!(
            pattern.validate_structure(),
            Err(StructPatternError::FieldNamePatternAlias {
                field_index: 0,
            })
        );
    }

    #[test]
    fn duplicate_direct_children_are_rejected() {
        let pattern = StructPattern::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            vec![
                StructPatternField::new(node_id(3), node_id(4)),
                StructPatternField::new(node_id(5), node_id(4)),
            ],
        );

        assert_eq!(
            pattern.validate_structure(),
            Err(StructPatternError::DuplicateChild {
                id: node_id(4),
            })
        );
    }

    #[test]
    fn contains_child_is_deterministic() {
        let pattern = valid_pattern();

        assert!(pattern.contains_child(node_id(2)));
        assert!(pattern.contains_child(node_id(3)));
        assert!(pattern.contains_child(node_id(4)));
        assert!(pattern.contains_child(node_id(5)));
        assert!(pattern.contains_child(node_id(6)));
        assert!(!pattern.contains_child(node_id(99)));
    }

    #[test]
    fn field_lookup_is_checked() {
        let pattern = valid_pattern();

        assert!(pattern.field(0).is_some());
        assert!(pattern.field(1).is_some());
        assert!(pattern.field(2).is_none());
        assert!(pattern.field_index_for_child(node_id(4)) == Some(0));
        assert!(pattern.field_index_for_child(node_id(6)) == Some(1));
        assert!(pattern.field_index_for_child(node_id(99)).is_none());
    }

    #[test]
    fn mutation_helpers_preserve_explicit_structure() {
        let mut pattern = valid_pattern();

        let previous = pattern.replace_type_path(node_id(7));
        assert_eq!(previous, node_id(2));
        assert_eq!(pattern.type_path(), node_id(7));

        pattern.push_field_ids(node_id(8), node_id(9));
        assert_eq!(pattern.field_count(), 3);

        let removed = pattern.pop_field().expect("last field");
        assert_eq!(removed.field_name(), node_id(8));
        assert_eq!(removed.pattern(), node_id(9));

        assert!(pattern.validate_structure().is_ok());
    }

    #[test]
    fn field_child_order_is_name_then_pattern() {
        let field = StructPatternField::new(node_id(10), node_id(11));

        assert_eq!(
            field.child_node_ids(),
            [node_id(10), node_id(11)]
        );
    }

    #[test]
    fn field_alias_detection_is_local() {
        let field = StructPatternField::new(node_id(10), node_id(11));

        assert!(field.has_distinct_children());

        let aliased = StructPatternField::new(node_id(10), node_id(10));

        assert!(!aliased.has_distinct_children());
    }

    #[test]
    fn ast_node_delegation_is_consistent() {
        let pattern = valid_pattern();

        assert_eq!(pattern.id(), pattern.node().id());
        assert_eq!(pattern.kind(), pattern.node().kind());
        assert_eq!(pattern.span(), pattern.node().span());
        assert_eq!(pattern.metadata(), pattern.node().metadata());
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(StructPattern::schema_version(), 1);
        assert_eq!(
            StructPattern::kind_name(),
            "zamani:struct-pattern"
        );
    }

    #[test]
    fn no_children_are_implicitly_allocated() {
        let pattern = valid_pattern();

        assert_eq!(pattern.fields().len(), 2);
        assert_eq!(pattern.child_node_ids().len(), 5);
    }
}