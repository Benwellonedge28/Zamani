//! # Zamani Frontend AST — Module
//!
//! Source-level representation of a Zamani module.
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
//!   │    └── Module
//!   │          └── Item NodeIds
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
//! `Module` represents a source-level Zamani module/namespace container.
//!
//! A module owns the *structural ordering and identity references* of its
//! source-level children. It does not own their semantic interpretation.
//!
//! This distinction is important:
//!
//! ```text
//! Module
//!   = source organization
//!
//! Semantic module
//!   = resolved namespace/symbol meaning
//!
//! ZUIR module/function/region
//!   = executable computational meaning
//! ```
//!
//! The AST module therefore contains no:
//!
//! - symbol tables;
//! - resolved types;
//! - resolved imports;
//! - backend identifiers;
//! - hardware resources;
//! - physical qubits;
//! - quantum topology;
//! - routing information;
//! - scheduling information;
//! - calibration information;
//! - QEC information;
//! - resilience information;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - runtime state.
//!
//! Those concerns belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! A module contains no assumptions about:
//!
//! - processor size;
//! - number of machines;
//! - number of modules;
//! - number of declarations;
//! - number of resources;
//! - number of qubits;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - execution environment.
//!
//! Therefore the same source-level module structure can participate in a
//! program compiled for a tiny machine, a large heterogeneous system, a
//! distributed system, a simulator, a quantum processor, or a future
//! computational platform.
//!
//! ```text
//! Same source module
//!        │
//!        ▼
//! Native AST
//!        │
//!        ▼
//! Semantic resolution
//!        │
//!        ▼
//! ZUIR
//!        │
//!        ▼
//! Target/resource discovery
//!        │
//!        ▼
//! Target-specific realization
//! ```
//!
//! ## Ownership model
//!
//! `Module` does not directly own concrete child AST objects.
//!
//! Instead, it stores their [`NodeId`] values.
//!
//! The enclosing AST/program storage is responsible for resolving those IDs
//! into concrete nodes.
//!
//! This provides an important architectural separation:
//!
//! ```text
//! Module
//!   │
//!   ├── NodeId
//!   ├── NodeId
//!   ├── NodeId
//!   └── ...
//!        │
//!        ▼
//! AST node store / arena / registry
//! ```
//!
//! The exact storage mechanism can therefore evolve without requiring this
//! module representation to change.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`super::super::node::Node`];
//! - [`super::super::node_id::NodeId`];
//! - [`super::super::node_kind::CoreNodeKind`];
//! - [`super::super::node_kind::NodeKind`];
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::source::Span`];
//! - Rust standard library;
//! - `serde` for serialization.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend providers;
//! - runtime;
//! - LLVM;
//! - MLIR;
//! - QIR;
//! - OpenQASM.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser creates a `Module`, allocates a [`NodeId`] through the AST
//! construction layer, records its source [`Span`], and then registers the
//! module's child node IDs.
//!
//! ```text
//! parser
//!   │
//!   ├── module name/path
//!   ├── module span
//!   ├── module metadata
//!   └── child NodeIds
//!          │
//!          ▼
//!       Module
//! ```
//!
//! ### Structural validation
//!
//! Validation must verify:
//!
//! - the node kind is `Module`;
//! - the module identity is structurally valid;
//! - child IDs are valid;
//! - no child ID equals the module's own ID;
//! - direct child IDs are not duplicated;
//! - source structure is otherwise valid.
//!
//! This file provides local structural validation helpers but does not resolve
//! names or types.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the module and resolves:
//!
//! - module identity;
//! - namespace membership;
//! - imports;
//! - declarations;
//! - symbols;
//! - visibility;
//! - types;
//! - generic parameters;
//! - capabilities;
//! - resources;
//! - effects.
//!
//! Semantic information must remain outside this AST node.
//!
//! ### ZUIR
//!
//! The module itself normally has no direct executable meaning. Its children
//! are lowered according to their semantic meaning.
//!
//! ```text
//! Module
//!   │
//!   └── children
//!          │
//!          ▼
//! semantic model
//!          │
//!          ▼
//!         ZUIR
//! ```
//!
//! Module namespace/organization information may be retained by compiler
//! metadata associated with ZUIR, but this AST node must not depend on ZUIR.
//!
//! ## Determinism
//!
//! Child order is preserved exactly as supplied.
//!
//! This is intentional. Source ordering can be important for:
//!
//! - diagnostics;
//! - tooling;
//! - deterministic serialization;
//! - source reconstruction;
//! - declaration ordering;
//! - incremental compilation.
//!
//! No unordered collection is used for the module's canonical child sequence.
//!
//! ## Scalability
//!
//! There is no language-level maximum number of:
//!
//! - modules;
//! - children;
//! - declarations;
//! - imports;
//! - resources;
//! - quantum resources;
//! - computational resources.
//!
//! `Vec<NodeId>` grows according to available memory and the host/compiler's
//! configured resource policy.
//!
//! Operational safety limits belong to configurable compiler infrastructure,
//! not this source-language representation.
//!
//! ## Security
//!
//! This type is safe to construct from untrusted parser output.
//!
//! It performs checked structural operations and does not use:
//!
//! - `unsafe`;
//! - pointer identity;
//! - raw memory access;
//! - global mutable state;
//! - process-global allocation counters.
//!
//! ## Serialization
//!
//! The structure derives `Serialize` and `Deserialize` so the AST serialization
//! layer can provide a deterministic, versioned representation.
//!
//! Serialization format versioning belongs to the AST serialization layer,
//! not this module.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! No unstable language features are required.
//!
//! ## Migration note
//!
//! This file intentionally does not define a competing `Item` type.
//!
//! Child nodes are represented by [`NodeId`] until the canonical `item.rs`
//! abstraction is finalized. This avoids duplicate AST hierarchies during
//! migration and allows `item.rs` to become the single authoritative item
//! representation.
//!
//! Once the AST store exists, it should resolve the IDs held here.
//!
//! ## Non-goals
//!
//! This module must never become responsible for:
//!
//! - importing external languages;
//! - lowering OpenQASM;
//! - selecting quantum gates;
//! - selecting hardware;
//! - resource allocation;
//! - quantum routing;
//! - quantum scheduling;
//! - error correction;
//! - resilience;
//! - pulse generation;
//! - calibration;
//! - backend execution.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the source-level module node.
///
/// This version is independent from:
///
/// - Zamani language version;
/// - AST serialization version;
/// - extension versions;
/// - compiler version.
///
/// A serialization layer may use this value as part of a larger schema
/// compatibility mechanism.
pub const MODULE_AST_SCHEMA_VERSION: u16 = 1;

/// Source-level identity of a Zamani module.
///
/// `Module` is deliberately a structural AST container. It does not resolve
/// the module into a semantic namespace.
///
/// # Representation
///
/// A module contains:
///
/// - common AST node metadata;
/// - an optional source-level module name;
/// - an ordered list of child node IDs.
///
/// # Why `Option<String>` for the name?
///
/// The AST must be capable of representing:
///
/// - named modules;
/// - anonymous/root modules;
/// - parser recovery states;
/// - programmatically constructed module fragments.
///
/// Semantic analysis determines whether a missing name is legal in the
/// particular source context.
///
/// # Why `NodeId` children?
///
/// A module must not depend on the concrete storage representation of child
/// nodes. The AST store can therefore use:
///
/// - an arena;
/// - indexed storage;
/// - an immutable graph;
/// - an incremental store;
/// - another safe representation.
///
/// without changing this public module contract.
///
/// # Invariants
///
/// A structurally valid module satisfies:
///
/// 1. `node.kind()` is `CoreNodeKind::Module`.
/// 2. Every direct child has a non-zero [`NodeId`].
/// 3. The module does not directly contain itself.
/// 4. No direct child ID occurs more than once.
/// 5. Child ordering is preserved.
/// 6. The module contains no semantic/backend state.
///
/// Cross-node validation, including verifying that each child ID resolves to
/// an appropriate AST node, belongs to the AST validation/store layer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Optional source-level module name.
    ///
    /// This is syntax, not a resolved semantic symbol.
    name: Option<String>,

    /// Ordered direct children of the module.
    ///
    /// The IDs refer to nodes owned by the enclosing AST store.
    items: Vec<NodeId>,
}

impl Module {
    /// Creates a new named or anonymous module.
    ///
    /// This constructor performs only local construction. It does not resolve
    /// names and does not inspect child nodes.
    ///
    /// The supplied `node` must have `CoreNodeKind::Module` as its kind.
    /// Call [`Module::try_new`] when construction-time validation is required.
    #[must_use]
    pub fn new(
        node: Node,
        name: Option<String>,
        items: Vec<NodeId>,
    ) -> Self {
        Self {
            node,
            name,
            items,
        }
    }

    /// Creates a module while validating its local invariants.
    ///
    /// This is the preferred constructor for parser and builder code that wants
    /// construction-time structural guarantees.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError`] when:
    ///
    /// - the supplied node is not a module node;
    /// - the module directly contains itself;
    /// - a direct child ID occurs more than once.
    pub fn try_new(
        node: Node,
        name: Option<String>,
        items: Vec<NodeId>,
    ) -> Result<Self, ModuleError> {
        let module = Self {
            node,
            name,
            items,
        };

        module.validate_local()?;

        Ok(module)
    }

    /// Creates a module from the supplied node with no name and no children.
    ///
    /// This is useful for parser recovery and incremental construction.
    ///
    /// The caller must still provide a node whose kind is `Module`.
    pub fn empty(node: Node) -> Result<Self, ModuleError> {
        Self::try_new(node, None, Vec::new())
    }

    /// Creates an empty named module.
    pub fn named(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
    ) -> Result<Self, ModuleError> {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Module),
            span,
            metadata,
        );

        Self::try_new(node, Some(name.into()), Vec::new())
    }

    /// Returns the underlying common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Mutation is restricted to the common node API, primarily metadata and
    /// controlled source transformations.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST identity of the module.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the module node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span of the module.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns module metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable module metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the module's source-level name.
    ///
    /// This is not a resolved semantic symbol.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns mutable access to the source-level module name.
    ///
    /// Parser recovery and AST transformation code may use this method.
    ///
    /// Semantic analysis must not use the AST as a mutable symbol table.
    #[inline]
    pub fn name_mut(&mut self) -> &mut Option<String> {
        &mut self.name
    }

    /// Replaces the source-level module name.
    ///
    /// Returns the previous name.
    pub fn replace_name(
        &mut self,
        name: Option<String>,
    ) -> Option<String> {
        std::mem::replace(&mut self.name, name)
    }

    /// Returns whether this module has a source-level name.
    #[inline]
    #[must_use]
    pub fn is_named(&self) -> bool {
        self.name.is_some()
    }

    /// Returns the ordered direct child node IDs.
    ///
    /// The returned sequence is deterministic and preserves source insertion
    /// order.
    #[inline]
    #[must_use]
    pub fn items(&self) -> &[NodeId] {
        &self.items
    }

    /// Returns mutable access to the ordered child sequence.
    ///
    /// Direct arbitrary mutation can violate local invariants. Prefer
    /// [`Module::push_item`], [`Module::insert_item`], [`Module::remove_item`],
    /// and [`Module::clear_items`] when possible.
    #[inline]
    pub fn items_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.items
    }

    /// Returns the number of direct child nodes.
    #[inline]
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the module contains no direct children.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the first child ID, if one exists.
    #[inline]
    #[must_use]
    pub fn first_item(&self) -> Option<NodeId> {
        self.items.first().copied()
    }

    /// Returns the last child ID, if one exists.
    #[inline]
    #[must_use]
    pub fn last_item(&self) -> Option<NodeId> {
        self.items.last().copied()
    }

    /// Returns the child ID at `index`.
    #[inline]
    #[must_use]
    pub fn item(&self, index: usize) -> Option<NodeId> {
        self.items.get(index).copied()
    }

    /// Returns an iterator over direct child IDs.
    #[inline]
    pub fn iter_items(&self) -> std::slice::Iter<'_, NodeId> {
        self.items.iter()
    }

    /// Adds one direct child to the end of the module.
    ///
    /// The operation rejects:
    ///
    /// - the module's own ID;
    /// - an ID already directly contained by the module.
    ///
    /// This method does not verify that the ID resolves to a particular node
    /// because resolution requires the enclosing AST store.
    pub fn push_item(
        &mut self,
        item: NodeId,
    ) -> Result<(), ModuleError> {
        self.validate_child_id(item)?;

        self.items.push(item);

        Ok(())
    }

    /// Adds multiple direct children in order.
    ///
    /// The operation is transactional: if any child would violate a local
    /// invariant, no children are added.
    pub fn extend_items<I>(
        &mut self,
        items: I,
    ) -> Result<(), ModuleError>
    where
        I: IntoIterator<Item = NodeId>,
    {
        let additions: Vec<NodeId> = items.into_iter().collect();

        self.validate_additions(&additions)?;

        self.items.extend(additions);

        Ok(())
    }

    /// Inserts a child at `index`.
    ///
    /// Existing children at or after the index are shifted to preserve order.
    pub fn insert_item(
        &mut self,
        index: usize,
        item: NodeId,
    ) -> Result<(), ModuleError> {
        if index > self.items.len() {
            return Err(ModuleError::IndexOutOfBounds {
                index,
                len: self.items.len(),
            });
        }

        self.validate_child_id(item)?;

        self.items.insert(index, item);

        Ok(())
    }

    /// Removes and returns the child at `index`.
    pub fn remove_item(
        &mut self,
        index: usize,
    ) -> Result<NodeId, ModuleError> {
        if index >= self.items.len() {
            return Err(ModuleError::IndexOutOfBounds {
                index,
                len: self.items.len(),
            });
        }

        Ok(self.items.remove(index))
    }

    /// Removes all direct children while preserving module identity and
    /// metadata.
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    /// Replaces all direct children.
    ///
    /// The operation is transactional.
    pub fn replace_items(
        &mut self,
        items: Vec<NodeId>,
    ) -> Result<Vec<NodeId>, ModuleError> {
        self.validate_additions(&items)?;

        Ok(std::mem::replace(&mut self.items, items))
    }

    /// Returns whether the module directly contains `item`.
    #[inline]
    #[must_use]
    pub fn contains_item(&self, item: NodeId) -> bool {
        self.items.contains(&item)
    }

    /// Returns the first index of `item`.
    #[inline]
    #[must_use]
    pub fn position_of(&self, item: NodeId) -> Option<usize> {
        self.items.iter().position(|candidate| *candidate == item)
    }

    /// Validates invariants that can be checked without access to the
    /// enclosing AST store.
    ///
    /// This intentionally does not verify that child IDs resolve to actual
    /// nodes. That is the responsibility of the AST graph/store validator.
    pub fn validate_local(&self) -> Result<(), ModuleError> {
        match self.node.kind().as_core() {
            Some(CoreNodeKind::Module) => {}
            _ => {
                return Err(ModuleError::InvalidNodeKind {
                    expected: CoreNodeKind::Module,
                    actual: self.node.kind().clone(),
                });
            }
        }

        for (index, item) in self.items.iter().copied().enumerate() {
            if item == self.id() {
                return Err(ModuleError::SelfReference {
                    module_id: self.id(),
                    index,
                });
            }

            if self.items[..index].contains(&item) {
                return Err(ModuleError::DuplicateChild {
                    child_id: item,
                    first_index: self
                        .items
                        .iter()
                        .position(|candidate| *candidate == item)
                        .unwrap_or(index),
                    duplicate_index: index,
                });
            }
        }

        Ok(())
    }

    /// Returns a compact source-level diagnostic summary.
    ///
    /// Child IDs are deliberately excluded so diagnostics cannot accidentally
    /// materialize a potentially enormous module.
    #[must_use]
    pub fn diagnostic_summary(&self) -> ModuleDiagnosticSummary {
        ModuleDiagnosticSummary {
            id: self.id(),
            name: self.name.clone(),
            span: self.span().clone(),
            item_count: self.items.len(),
        }
    }

    /// Returns the module AST schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        MODULE_AST_SCHEMA_VERSION
    }

    fn validate_child_id(
        &self,
        item: NodeId,
    ) -> Result<(), ModuleError> {
        if item == self.id() {
            return Err(ModuleError::SelfReference {
                module_id: self.id(),
                index: self.items.len(),
            });
        }

        if let Some(index) = self.position_of(item) {
            return Err(ModuleError::DuplicateChild {
                child_id: item,
                first_index: index,
                duplicate_index: self.items.len(),
            });
        }

        Ok(())
    }

    fn validate_additions(
        &self,
        additions: &[NodeId],
    ) -> Result<(), ModuleError> {
        let mut seen: Vec<NodeId> = Vec::with_capacity(additions.len());

        for (offset, item) in additions.iter().copied().enumerate() {
            if item == self.id() {
                return Err(ModuleError::SelfReference {
                    module_id: self.id(),
                    index: self.items.len() + offset,
                });
            }

            if self.items.contains(&item) {
                return Err(ModuleError::DuplicateChild {
                    child_id: item,
                    first_index: self
                        .items
                        .iter()
                        .position(|candidate| *candidate == item)
                        .unwrap_or_default(),
                    duplicate_index: self.items.len() + offset,
                });
            }

            if let Some(first_offset) = seen.iter().position(|candidate| *candidate == item) {
                return Err(ModuleError::DuplicateChild {
                    child_id: item,
                    first_index: self.items.len() + first_offset,
                    duplicate_index: self.items.len() + offset,
                });
            }

            seen.push(item);
        }

        Ok(())
    }
}

/// Lightweight diagnostic representation of a [`Module`].
///
/// This deliberately excludes the complete child-ID list and metadata payload.
/// Diagnostics should remain bounded with respect to module size and metadata
/// size.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleDiagnosticSummary {
    /// Stable module AST identity.
    pub id: NodeId,

    /// Source-level module name.
    pub name: Option<String>,

    /// Module source span.
    pub span: Span,

    /// Number of direct children.
    pub item_count: usize,
}

impl fmt::Display for ModuleDiagnosticSummary {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.name {
            Some(name) => write!(
                formatter,
                "module `{name}` with {} item(s) at {}",
                self.item_count,
                self.span
            ),
            None => write!(
                formatter,
                "anonymous module with {} item(s) at {}",
                self.item_count,
                self.span
            ),
        }
    }
}

/// Errors produced by local [`Module`] construction and mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModuleError {
    /// The supplied common node does not have the `Module` node kind.
    InvalidNodeKind {
        /// The required core node kind.
        expected: CoreNodeKind,

        /// The actual supplied node kind.
        actual: NodeKind,
    },

    /// A module cannot directly contain itself.
    SelfReference {
        /// Identity of the module.
        module_id: NodeId,

        /// Position at which the invalid reference occurred.
        index: usize,
    },

    /// The same child node was added more than once.
    DuplicateChild {
        /// Duplicated child identity.
        child_id: NodeId,

        /// First position where the child occurred.
        first_index: usize,

        /// Position of the duplicate occurrence.
        duplicate_index: usize,
    },

    /// An insertion/removal index is outside the valid sequence.
    IndexOutOfBounds {
        /// Requested index.
        index: usize,

        /// Current number of children.
        len: usize,
    },
}

impl fmt::Display for ModuleError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "invalid module node kind: expected `{}`, found `{actual}`",
                    expected.name()
                )
            }

            Self::SelfReference { module_id, index } => {
                write!(
                    formatter,
                    "module {module_id} cannot contain itself at child index {index}"
                )
            }

            Self::DuplicateChild {
                child_id,
                first_index,
                duplicate_index,
            } => {
                write!(
                    formatter,
                    "module child {child_id} is duplicated: first at index \
                     {first_index}, again at index {duplicate_index}"
                )
            }

            Self::IndexOutOfBounds { index, len } => {
                write!(
                    formatter,
                    "module child index {index} is out of bounds for {len} item(s)"
                )
            }
        }
    }
}

impl std::error::Error for ModuleError {}

/// Provides the common AST-node integration surface.
///
/// This implementation intentionally delegates to the embedded canonical
/// [`Node`].
impl super::super::node::AstNode for Module {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeId;

    fn test_node(id: u64) -> Node {
        Node::new(
            NodeId::new(id).expect("test IDs are non-zero"),
            NodeKind::core(CoreNodeKind::Module),
            Span::default(),
            NodeMetadata::default(),
        )
    }

    fn test_child(id: u64) -> NodeId {
        NodeId::new(id).expect("test IDs are non-zero")
    }

    #[test]
    fn creates_empty_module() {
        let module = Module::empty(test_node(1))
            .expect("module should be valid");

        assert_eq!(module.id().get(), 1);
        assert_eq!(module.kind().as_core(), Some(CoreNodeKind::Module));
        assert!(module.is_empty());
        assert_eq!(module.item_count(), 0);
    }

    #[test]
    fn creates_named_module() {
        let module = Module::named(
            test_child(1),
            Span::default(),
            NodeMetadata::default(),
            "example",
        )
        .expect("named module should be valid");

        assert_eq!(module.name(), Some("example"));
        assert!(module.is_named());
    }

    #[test]
    fn preserves_child_order() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        module.push_item(test_child(2)).expect("child 2");
        module.push_item(test_child(3)).expect("child 3");
        module.push_item(test_child(4)).expect("child 4");

        assert_eq!(
            module.items(),
            &[
                test_child(2),
                test_child(3),
                test_child(4),
            ]
        );
    }

    #[test]
    fn rejects_self_reference() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        let error = module
            .push_item(test_child(1))
            .expect_err("self reference must fail");

        assert!(matches!(
            error,
            ModuleError::SelfReference { .. }
        ));
    }

    #[test]
    fn rejects_duplicate_children() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        module.push_item(test_child(2))
            .expect("first child should succeed");

        let error = module
            .push_item(test_child(2))
            .expect_err("duplicate must fail");

        assert!(matches!(
            error,
            ModuleError::DuplicateChild { .. }
        ));

        assert_eq!(module.item_count(), 1);
    }

    #[test]
    fn transactional_extend_rejects_invalid_batch() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        module.push_item(test_child(2))
            .expect("initial child should succeed");

        let error = module.extend_items([
            test_child(3),
            test_child(2),
            test_child(4),
        ]);

        assert!(matches!(
            error,
            Err(ModuleError::DuplicateChild { .. })
        ));

        assert_eq!(
            module.items(),
            &[test_child(2)]
        );
    }

    #[test]
    fn transactional_replace_rejects_invalid_batch() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        module.push_item(test_child(2))
            .expect("initial child should succeed");

        let original = module.items().to_vec();

        let error = module.replace_items(vec![
            test_child(3),
            test_child(3),
        ]);

        assert!(matches!(
            error,
            Err(ModuleError::DuplicateChild { .. })
        ));

        assert_eq!(module.items(), original.as_slice());
    }

    #[test]
    fn insert_preserves_order() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        module.push_item(test_child(2)).expect("child 2");
        module.push_item(test_child(4)).expect("child 4");

        module
            .insert_item(1, test_child(3))
            .expect("insertion should succeed");

        assert_eq!(
            module.items(),
            &[
                test_child(2),
                test_child(3),
                test_child(4),
            ]
        );
    }

    #[test]
    fn remove_returns_original_child() {
        let mut module = Module::empty(test_node(1))
            .expect("module should be valid");

        module.push_item(test_child(2)).expect("child 2");
        module.push_item(test_child(3)).expect("child 3");

        let removed = module
            .remove_item(0)
            .expect("removal should succeed");

        assert_eq!(removed, test_child(2));
        assert_eq!(module.items(), &[test_child(3)]);
    }

    #[test]
    fn local_validation_succeeds_for_valid_module() {
        let module = Module::new(
            test_node(1),
            Some(String::from("valid")),
            vec![
                test_child(2),
                test_child(3),
                test_child(4),
            ],
        );

        module
            .validate_local()
            .expect("module should satisfy local invariants");
    }

    #[test]
    fn rejects_non_module_node() {
        let node = Node::new(
            test_child(1),
            NodeKind::core(CoreNodeKind::Function),
            Span::default(),
            NodeMetadata::default(),
        );

        let error = Module::try_new(
            node,
            Some(String::from("invalid")),
            Vec::new(),
        )
        .expect_err("function node cannot construct a module");

        assert!(matches!(
            error,
            ModuleError::InvalidNodeKind { .. }
        ));
    }

    #[test]
    fn diagnostic_summary_is_compact() {
        let module = Module::named(
            test_child(1),
            Span::default(),
            NodeMetadata::default(),
            "example",
        )
        .expect("module should be valid");

        let summary = module.diagnostic_summary();

        assert_eq!(summary.id, test_child(1));
        assert_eq!(summary.name.as_deref(), Some("example"));
        assert_eq!(summary.item_count, 0);
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(Module::schema_version(), 1);
    }
}