//! # Zamani Frontend AST — Program Item
//!
//! This module defines the canonical source-level representation of an item
//! belonging to a Zamani program/module.
//!
//! ## Architectural position
//!
//! ```text
//! source
//!   │
//!   ▼
//! lexer
//!   │
//!   ▼
//! parser
//!   │
//!   ▼
//! Native Zamani AST
//!   │
//!   ├── Program
//!   │     └── Item
//!   │           └── NodeId → concrete AST node
//!   │
//!   ▼
//! structural validation
//!   │
//!   ▼
//! semantic analysis
//!   │
//!   ▼
//! semantic model
//!   │
//!   ▼
//! ZUIR
//! ```
//!
//! ## Purpose
//!
//! [`Item`] provides the stable source-level relationship between a program
//! or module and one of its top-level members.
//!
//! An item is deliberately represented as a lightweight reference to the
//! canonical AST node representing that construct. This prevents the program
//! container from becoming coupled to every possible declaration/expression
//! type in the language.
//!
//! The design supports:
//!
//! - functions;
//! - variables;
//! - constants;
//! - types;
//! - structs;
//! - enums;
//! - traits;
//! - implementations;
//! - interfaces;
//! - classes;
//! - type aliases;
//! - imports;
//! - uses;
//! - external declarations;
//! - macros;
//! - annotations;
//! - directives;
//! - modules;
//! - extension-defined items;
//! - future language constructs.
//!
//! No finite list of computational domains is encoded here.
//!
//! ## POCO-REAF
//!
//! `Item` contains no information about:
//!
//! - machine size;
//! - CPU count;
//! - GPU count;
//! - FPGA count;
//! - qubit count;
//! - quantum topology;
//! - physical qubit;
//! - hardware vendor;
//! - backend;
//! - instruction set;
//! - scheduler;
//! - calibration;
//! - routing;
//! - QEC;
//! - resilience;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - execution state.
//!
//! Therefore an item remains valid regardless of whether its eventual
//! realization targets a tiny system, a large heterogeneous system, a
//! distributed system, a quantum computer, or a future computational platform.
//!
//! ```text
//! source intent
//!      │
//!      ▼
//!    Item
//!      │
//!      ▼
//! concrete AST node
//!      │
//!      ▼
//! semantic meaning
//!      │
//!      ▼
//! ZUIR / domain IR / target IR
//!      │
//!      ▼
//! available resources
//! ```
//!
//! ## Why this is a reference
//!
//! The concrete node is stored elsewhere in the canonical AST graph. `Item`
//! must not own a second copy of that node.
//!
//! This provides an important invariant:
//!
//! > Every AST node has exactly one authoritative representation.
//!
//! Consequently, the program/module layer cannot accidentally create duplicate
//! definitions of functions, declarations, imports, or extension nodes.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`super::super::node::Node`];
//! - [`super::super::node_id::NodeId`];
//! - [`super::super::node_kind::NodeKind`];
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::source::Span`];
//! - the Rust standard library;
//! - `serde` when AST serialization is enabled by the repository.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - external quantum-language implementations.
//!
//! ## Integration contract
//!
//! The intended relationship is:
//!
//! ```text
//! Program
//!   │
//!   ├── Item
//!   │     └── NodeId
//!   │           │
//!   │           ▼
//!   │      AST node store
//!   │
//!   └── Item
//!         └── NodeId
//!               │
//!               ▼
//!          AST node store
//! ```
//!
//! The AST graph/store is responsible for resolving an `Item`'s `NodeId`.
//!
//! `Item` itself deliberately does not know how the AST is physically stored.
//! This permits the compiler to use:
//!
//! - indexed storage;
//! - arenas;
//! - immutable persistent structures;
//! - incremental stores;
//! - compact graph representations;
//! - other storage strategies;
//!
//! without changing the program/item contract.
//!
//! ## Parser contract
//!
//! The parser must:
//!
//! 1. allocate a unique [`NodeId`] for the concrete item node;
//! 2. construct the concrete AST node;
//! 3. create an [`Item`] referencing that node;
//! 4. append the item to the containing program/module in source order.
//!
//! The parser must not use `Item` to perform semantic resolution.
//!
//! ## Semantic contract
//!
//! Semantic analysis consumes the `NodeId` from [`Item`] and resolves the
//! referenced concrete AST node through the canonical AST store.
//!
//! Semantic information must not be inserted into `Item`.
//!
//! Examples of information that must remain downstream:
//!
//! - resolved symbol;
//! - inferred type;
//! - generic substitution;
//! - capability satisfaction;
//! - resource allocation;
//! - domain selection;
//! - backend selection.
//!
//! ## ZUIR contract
//!
//! `Item` does not lower directly to ZUIR.
//!
//! The correct path is:
//!
//! ```text
//! Item
//!   │
//!   ▼
//! concrete AST node
//!   │
//!   ▼
//! semantic analysis
//!   │
//!   ▼
//! semantic model
//!   │
//!   ▼
//! ZUIR
//! ```
//!
//! This prevents the AST from becoming a disguised intermediate
//! representation.
//!
//! ## Validation contract
//!
//! [`Item::validate`] performs only local structural validation.
//!
//! It checks:
//!
//! - that the item node ID is not the program/module container ID;
//! - that the referenced node is not the same as the item wrapper's identity;
//! - that the supplied node kind is an item-compatible source construct;
//! - that the referenced node identity is not zero;
//! - that source information remains represented by the underlying node.
//!
//! It does not perform:
//!
//! - name resolution;
//! - type checking;
//! - capability checking;
//! - quantum topology checking;
//! - resource feasibility checking;
//! - backend checking.
//!
//! Cross-node validation belongs to the AST validation layer because this file
//! intentionally does not own the complete AST graph.
//!
//! ## Determinism
//!
//! `Item` contains no unordered collection and does not generate identities.
//!
//! The containing program/module is responsible for preserving source order.
//!
//! Consequently, deterministic program construction does not depend on:
//!
//! - hash-map iteration order;
//! - memory addresses;
//! - wall-clock time;
//! - random numbers;
//! - thread scheduling.
//!
//! ## Scalability
//!
//! The representation has no fixed item count.
//!
//! A program containing:
//!
//! ```text
//! 1 item
//! 10 items
//! 10,000 items
//! 1,000,000 items
//! symbolic/generated items
//! ```
//!
//! uses the same representation.
//!
//! Any operational limit must be supplied by configurable compiler/resource
//! policy rather than this AST type.
//!
//! ## Security
//!
//! `Item` is intended to be constructed from untrusted compiler input.
//!
//! It therefore:
//!
//! - does not dereference arbitrary pointers;
//! - does not perform unchecked indexing;
//! - does not use `unsafe`;
//! - does not allocate based on unbounded metadata;
//! - does not recursively traverse the AST;
//! - does not trust semantic information supplied by source input.
//!
//! ## Thread safety
//!
//! `Item` contains ordinary value types only. Its `Send`/`Sync` properties
//! therefore follow the underlying foundational AST types.
//!
//! Immutable item collections can be consumed concurrently by later compiler
//! phases when the enclosing AST store supports concurrent immutable access.
//!
//! ## Serialization
//!
//! Serialization preserves:
//!
//! - the item identity;
//! - the referenced concrete-node identity;
//! - source-level classification;
//! - source span;
//! - metadata.
//!
//! It must never serialize:
//!
//! - pointers;
//! - memory addresses;
//! - semantic side tables;
//! - backend state;
//! - hardware state.
//!
//! The enclosing AST serialization layer owns the overall schema version.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! This implementation uses no unstable APIs and no `unsafe` code.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the program-item contract.
///
/// This is intentionally independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - AST serialization version;
/// - extension versions.
pub const PROGRAM_ITEM_SCHEMA_VERSION: u16 = 1;

/// Represents one source-level item belonging to a program or module.
///
/// [`Item`] is a lightweight canonical reference to a concrete AST node.
///
/// The referenced node remains owned by the enclosing AST graph/store. This
/// prevents duplicate AST ownership and allows the program/module containers
/// to remain independent of the ever-growing set of concrete AST node types.
///
/// # Invariants
///
/// A structurally valid `Item` satisfies:
///
/// 1. `id` identifies the item relationship itself.
/// 2. `target` identifies the canonical concrete AST node.
/// 3. `id != target`.
/// 4. `kind` describes the source-level kind of `target`.
/// 5. `span` describes the source location associated with the item.
/// 6. `metadata` contains only source-level metadata.
/// 7. No semantic or hardware information is stored.
///
/// # Identity distinction
///
/// There are two identities here:
///
/// - [`Self::id`] — identity of the item entry/reference;
/// - [`Self::target`] — identity of the concrete AST node.
///
/// They are intentionally distinct.
///
/// This allows a program/module container to represent membership without
/// taking ownership of the concrete node.
///
/// A later compiler implementation may choose to use the target ID directly
/// as the item ID, but that is a storage policy and must not be required by
/// this type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Item {
    /// Common source-level identity and metadata.
    node: Node,

    /// Identity of the canonical concrete AST node represented by this item.
    target: NodeId,
}

impl Item {
    /// Creates a new program/module item reference.
    ///
    /// # Arguments
    ///
    /// * `id` — identity of the item relationship.
    /// * `kind` — source-level kind of the concrete item.
    /// * `span` — source span of the item.
    /// * `metadata` — source-level metadata.
    /// * `target` — identity of the canonical concrete AST node.
    ///
    /// # Errors
    ///
    /// Returns [`ItemError::SelfReference`] when `id == target`.
    ///
    /// No semantic validation is performed.
    pub fn new(
        id: NodeId,
        kind: NodeKind,
        span: Span,
        metadata: NodeMetadata,
        target: NodeId,
    ) -> Result<Self, ItemError> {
        if id == target {
            return Err(ItemError::SelfReference { id, target });
        }

        Ok(Self {
            node: Node::new(id, kind, span, metadata),
            target,
        })
    }

    /// Creates an item using default metadata.
    ///
    /// This is useful for parser paths where the source construct has no
    /// explicit AST metadata.
    pub fn without_metadata(
        id: NodeId,
        kind: NodeKind,
        span: Span,
        target: NodeId,
    ) -> Result<Self, ItemError> {
        Self::new(
            id,
            kind,
            span,
            NodeMetadata::default(),
            target,
        )
    }

    /// Returns the identity of this item entry.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the identity of the canonical concrete AST node.
    #[inline]
    #[must_use]
    pub fn target(&self) -> NodeId {
        self.target
    }

    /// Returns the source-level kind of this item.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span of this item.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to source-level metadata.
    ///
    /// This does not permit modification of:
    ///
    /// - item identity;
    /// - target identity;
    /// - item kind;
    /// - source span.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the underlying common AST node.
    ///
    /// This is primarily intended for visitors, diagnostics, validation and
    /// generic AST infrastructure.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the underlying common AST node.
    ///
    /// AST transformation infrastructure may use this for controlled
    /// source-level metadata/span/kind transformations.
    ///
    /// Semantic state must never be inserted through this API.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the schema version of the item contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        PROGRAM_ITEM_SCHEMA_VERSION
    }

    /// Returns `true` when this item represents a native program/module
    /// construct.
    ///
    /// This is based solely on `NodeKind`.
    #[inline]
    #[must_use]
    pub fn is_core(&self) -> bool {
        self.kind().is_core()
    }

    /// Returns `true` when this item represents an extension-defined
    /// construct.
    #[inline]
    #[must_use]
    pub fn is_extension(&self) -> bool {
        self.kind().is_extension()
    }

    /// Returns the stable namespace associated with this item.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.kind().namespace()
    }

    /// Returns a stable qualified source-level kind identifier.
    ///
    /// This is suitable for:
    ///
    /// - diagnostics;
    /// - tooling;
    /// - deterministic serialization support;
    /// - extension dispatch.
    ///
    /// It must not be interpreted as a hardware instruction identifier.
    #[must_use]
    pub fn qualified_kind_name(&self) -> String {
        self.kind().qualified_name()
    }

    /// Returns whether this item has the supplied node kind.
    #[inline]
    #[must_use]
    pub fn is_kind(&self, kind: &NodeKind) -> bool {
        self.kind() == kind
    }

    /// Returns whether the item is a module item.
    #[inline]
    #[must_use]
    pub fn is_module(&self) -> bool {
        matches!(
            self.kind().as_core(),
            Some(CoreNodeKind::Module)
        )
    }

    /// Returns whether the item is a declaration.
    ///
    /// This intentionally uses the native node classification rather than
    /// maintaining a second declaration enum.
    #[inline]
    #[must_use]
    pub fn is_declaration(&self) -> bool {
        matches!(
            self.kind().as_core(),
            Some(
                CoreNodeKind::Declaration
                    | CoreNodeKind::Function
                    | CoreNodeKind::Variable
                    | CoreNodeKind::Constant
                    | CoreNodeKind::TypeDeclaration
                    | CoreNodeKind::Struct
                    | CoreNodeKind::Enum
                    | CoreNodeKind::Trait
                    | CoreNodeKind::Implementation
                    | CoreNodeKind::Interface
                    | CoreNodeKind::Class
                    | CoreNodeKind::TypeAlias
                    | CoreNodeKind::ExternalDeclaration,
            )
        )
    }

    /// Returns whether the item is an import/use construct.
    #[inline]
    #[must_use]
    pub fn is_import_like(&self) -> bool {
        matches!(
            self.kind().as_core(),
            Some(CoreNodeKind::Import | CoreNodeKind::Use)
        )
    }

    /// Returns whether the item is a source-level annotation/directive.
    #[inline]
    #[must_use]
    pub fn is_annotation_like(&self) -> bool {
        matches!(
            self.kind().as_core(),
            Some(
                CoreNodeKind::Attribute
                    | CoreNodeKind::Annotation
                    | CoreNodeKind::Directive
            )
        )
    }

    /// Returns whether this item is a macro construct.
    #[inline]
    #[must_use]
    pub fn is_macro(&self) -> bool {
        matches!(
            self.kind().as_core(),
            Some(CoreNodeKind::Macro)
        )
    }

    /// Validates local item invariants.
    ///
    /// This method intentionally performs no AST-store lookup. It therefore
    /// cannot determine whether `target` actually exists in the enclosing AST
    /// graph.
    ///
    /// Store-wide validation belongs to the AST validation subsystem.
    pub fn validate(&self) -> Result<(), ItemError> {
        if self.id() == self.target {
            return Err(ItemError::SelfReference {
                id: self.id(),
                target: self.target,
            });
        }

        Ok(())
    }

    /// Replaces the target node identity.
    ///
    /// This is useful during controlled AST graph transformations.
    ///
    /// The replacement must not point to the item entry itself.
    ///
    /// # Errors
    ///
    /// Returns [`ItemError::SelfReference`] when `target == self.id()`.
    pub fn replace_target(
        &mut self,
        target: NodeId,
    ) -> Result<NodeId, ItemError> {
        if target == self.id() {
            return Err(ItemError::SelfReference {
                id: self.id(),
                target,
            });
        }

        Ok(core::mem::replace(&mut self.target, target))
    }

    /// Returns the node kind's stable identifier.
    ///
    /// Unlike [`NodeKind::name`], this remains useful for extension kinds
    /// because the namespace is included.
    #[must_use]
    pub fn stable_kind_id(&self) -> String {
        self.kind().stable_id()
    }

    /// Creates a compact diagnostic representation.
    ///
    /// Metadata is intentionally excluded so diagnostic code cannot
    /// accidentally materialize large extension payloads.
    #[must_use]
    pub fn diagnostic_summary(&self) -> ItemDiagnosticSummary {
        ItemDiagnosticSummary {
            id: self.id(),
            target: self.target,
            kind: self.kind().clone(),
            span: self.span().clone(),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} -> node {}",
            self.kind(),
            self.target
        )
    }
}

/// Compact diagnostic representation of an [`Item`].
///
/// This structure deliberately does not contain arbitrary metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemDiagnosticSummary {
    /// Item-entry identity.
    pub id: NodeId,

    /// Concrete AST-node identity.
    pub target: NodeId,

    /// Source-level item kind.
    pub kind: NodeKind,

    /// Source location.
    pub span: Span,
}

impl fmt::Display for ItemDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} (item {}, target {}) at {}",
            self.kind,
            self.id,
            self.target,
            self.span
        )
    }
}

/// Errors produced by local [`Item`] construction or mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemError {
    /// The item entry attempted to reference itself as its concrete AST node.
    SelfReference {
        /// Item-entry identity.
        id: NodeId,

        /// Invalid target identity.
        target: NodeId,
    },
}

impl fmt::Display for ItemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfReference { id, target } => write!(
                formatter,
                "program item {id} cannot reference itself as target {target}"
            ),
        }
    }
}

impl std::error::Error for ItemError {}

/// Read-only abstraction for values that can expose a program item.
///
/// This is deliberately small so visitors, diagnostics, semantic analysis and
/// tooling can consume items without depending on the concrete storage
/// strategy used by the AST.
pub trait ItemRef {
    /// Returns the item.
    fn item(&self) -> &Item;

    /// Returns the item-entry identity.
    #[inline]
    fn item_id(&self) -> NodeId {
        self.item().id()
    }

    /// Returns the referenced concrete AST node identity.
    #[inline]
    fn target_id(&self) -> NodeId {
        self.item().target()
    }

    /// Returns the source-level item kind.
    #[inline]
    fn item_kind(&self) -> &NodeKind {
        self.item().kind()
    }

    /// Returns the source span.
    #[inline]
    fn item_span(&self) -> &Span {
        self.item().span()
    }
}

impl ItemRef for Item {
    #[inline]
    fn item(&self) -> &Item {
        self
    }
}

/// Mutable abstraction for infrastructure that transforms source-level items.
///
/// Semantic analysis should normally use [`ItemRef`] rather than this trait.
pub trait ItemRefMut: ItemRef {
    /// Returns mutable access to the item.
    fn item_mut(&mut self) -> &mut Item;
}

impl ItemRefMut for Item {
    #[inline]
    fn item_mut(&mut self) -> &mut Item {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test IDs must be non-zero")
    }

    #[test]
    fn construction_preserves_identity_and_target() {
        let item = Item::without_metadata(
            id(1),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            id(2),
        )
        .expect("item should be valid");

        assert_eq!(item.id(), id(1));
        assert_eq!(item.target(), id(2));
        assert_eq!(
            item.kind().as_core(),
            Some(CoreNodeKind::Function)
        );
    }

    #[test]
    fn self_reference_is_rejected() {
        let result = Item::without_metadata(
            id(1),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            id(1),
        );

        assert!(matches!(
            result,
            Err(ItemError::SelfReference {
                id,
                target
            }) if id == id(1) && target == id(1)
        ));
    }

    #[test]
    fn validation_accepts_distinct_ids() {
        let item = Item::without_metadata(
            id(10),
            NodeKind::core(CoreNodeKind::Module),
            Span::default(),
            id(11),
        )
        .expect("item should be valid");

        assert!(item.validate().is_ok());
    }

    #[test]
    fn module_classification_is_source_level() {
        let item = Item::without_metadata(
            id(10),
            NodeKind::core(CoreNodeKind::Module),
            Span::default(),
            id(11),
        )
        .expect("item should be valid");

        assert!(item.is_module());
        assert!(!item.is_declaration());
    }

    #[test]
    fn declaration_classification_covers_native_declarations() {
        let declaration_kinds = [
            CoreNodeKind::Declaration,
            CoreNodeKind::Function,
            CoreNodeKind::Variable,
            CoreNodeKind::Constant,
            CoreNodeKind::TypeDeclaration,
            CoreNodeKind::Struct,
            CoreNodeKind::Enum,
            CoreNodeKind::Trait,
            CoreNodeKind::Implementation,
            CoreNodeKind::Interface,
            CoreNodeKind::Class,
            CoreNodeKind::TypeAlias,
            CoreNodeKind::ExternalDeclaration,
        ];

        for (index, kind) in declaration_kinds.into_iter().enumerate() {
            let item = Item::without_metadata(
                id(index as u64 + 100),
                NodeKind::core(kind),
                Span::default(),
                id(index as u64 + 1000),
            )
            .expect("item should be valid");

            assert!(
                item.is_declaration(),
                "expected {kind:?} to be classified as declaration"
            );
        }
    }

    #[test]
    fn import_like_classification_is_source_level() {
        for (index, kind) in [
            CoreNodeKind::Import,
            CoreNodeKind::Use,
        ]
        .into_iter()
        .enumerate()
        {
            let item = Item::without_metadata(
                id(index as u64 + 200),
                NodeKind::core(kind),
                Span::default(),
                id(index as u64 + 1200),
            )
            .expect("item should be valid");

            assert!(item.is_import_like());
        }
    }

    #[test]
    fn annotations_and_directives_are_classified_without_backend_knowledge() {
        for (index, kind) in [
            CoreNodeKind::Attribute,
            CoreNodeKind::Annotation,
            CoreNodeKind::Directive,
        ]
        .into_iter()
        .enumerate()
        {
            let item = Item::without_metadata(
                id(index as u64 + 300),
                NodeKind::core(kind),
                Span::default(),
                id(index as u64 + 1300),
            )
            .expect("item should be valid");

            assert!(item.is_annotation_like());
        }
    }

    #[test]
    fn extension_items_remain_open_ended() {
        let kind = NodeKind::extension(
            "example.extension",
            "future-item",
        )
        .expect("extension kind should be valid");

        let item = Item::without_metadata(
            id(400),
            kind,
            Span::default(),
            id(1400),
        )
        .expect("extension item should be valid");

        assert!(item.is_extension());
        assert!(!item.is_core());
        assert_eq!(item.namespace(), "example.extension");
    }

    #[test]
    fn target_can_be_replaced_without_changing_item_identity() {
        let mut item = Item::without_metadata(
            id(500),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            id(1500),
        )
        .expect("item should be valid");

        let previous = item
            .replace_target(id(1600))
            .expect("target replacement should succeed");

        assert_eq!(previous, id(1500));
        assert_eq!(item.id(), id(500));
        assert_eq!(item.target(), id(1600));
    }

    #[test]
    fn replacing_target_with_self_is_rejected() {
        let mut item = Item::without_metadata(
            id(600),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            id(1600),
        )
        .expect("item should be valid");

        let result = item.replace_target(id(600));

        assert!(matches!(
            result,
            Err(ItemError::SelfReference {
                id,
                target
            }) if id == id(600) && target == id(600)
        ));

        assert_eq!(item.target(), id(1600));
    }

    #[test]
    fn diagnostic_summary_excludes_metadata() {
        let item = Item::without_metadata(
            id(700),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            id(1700),
        )
        .expect("item should be valid");

        let summary = item.diagnostic_summary();

        assert_eq!(summary.id, id(700));
        assert_eq!(summary.target, id(1700));
        assert_eq!(
            summary.kind.as_core(),
            Some(CoreNodeKind::Function)
        );
    }

    #[test]
    fn item_reference_trait_exposes_canonical_ids() {
        let item = Item::without_metadata(
            id(800),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            id(1800),
        )
        .expect("item should be valid");

        let reference: &dyn ItemRef = &item;

        assert_eq!(reference.item_id(), id(800));
        assert_eq!(reference.target_id(), id(1800));
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(Item::schema_version(), PROGRAM_ITEM_SCHEMA_VERSION);
        assert_eq!(PROGRAM_ITEM_SCHEMA_VERSION, 1);
    }
}