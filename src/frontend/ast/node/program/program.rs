//! # Zamani Frontend AST — Program Root
//!
//! Canonical source-level representation of a complete Zamani program.
//!
//! ## Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                           Lexer
//!                              │
//!                              ▼
//!                           Parser
//!                              │
//!                              ▼
//!                  ┌──────────────────────┐
//!                  │ Native Zamani AST    │
//!                  │                      │
//!                  │ Program              │
//!                  │   │                  │
//!                  │   ├── NodeId ─────┐  │
//!                  │   ├── NodeId ─────┤  │
//!                  │   └── NodeId ─────┤  │
//!                  └───────────────────┼──┘
//!                                      │
//!                                      ▼
//!                              AST node storage
//!                                      │
//!                                      ▼
//!                           Structural validation
//!                                      │
//!                                      ▼
//!                            Semantic analysis
//!                                      │
//!                                      ▼
//!                              Semantic Model
//!                                      │
//!                                      ▼
//!                                    ZUIR
//!                                      │
//!                 ┌────────────────────┼────────────────────┐
//!                 ▼                    ▼                    ▼
//!             Classical             Quantum                HDL
//!                IR                    IR                    IR
//!                 │                    │                    │
//!                 └────────────────────┼────────────────────┘
//!                                      ▼
//!                              Target realization
//!                                      │
//!                                      ▼
//!                        CPU / GPU / FPGA / QPU /
//!                        distributed / future hardware
//! ```
//!
//! ## POCO-REAF
//!
//! The program root contains source-level program structure only.
//!
//! It deliberately does **not** contain:
//!
//! - a fixed number of qubits;
//! - a fixed number of classical resources;
//! - a machine size;
//! - a processor topology;
//! - a hardware vendor;
//! - a backend identifier;
//! - a gate set;
//! - physical qubit assignments;
//! - routing information;
//! - scheduling information;
//! - calibration information;
//! - error-correction implementation;
//! - resilience implementation;
//! - execution jobs;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations.
//!
//! Consequently, the same `Program` representation can be used as the source
//! root for a tiny program, a very large program, a quantum program, a hybrid
//! program, a distributed program, or a future computational domain.
//!
//! The eventual amount of computation is determined downstream from the
//! program's semantics and available resources.
//!
//! ## Why child `NodeId`s are used
//!
//! The new native AST is being decomposed into independently maintainable
//! node families. At the time this file is introduced, those child families
//! are not all present yet.
//!
//! `Program` therefore stores the identity of each top-level child rather than
//! inventing a competing `Statement`, `Declaration`, `Expression`, or generic
//! enum here.
//!
//! This provides an important architectural property:
//!
//! ```text
//! Program
//!   │
//!   ├── owns ordering
//!   ├── owns root identity
//!   ├── owns root source span
//!   └── references children by stable NodeId
//!
//! Child node implementation
//!   │
//!   ├── owns its own structure
//!   ├── owns its own source span
//!   └── is resolved by AST storage
//! ```
//!
//! This prevents `program.rs` from becoming a second monolithic AST.
//!
//! ## Dependency boundary
//!
//! This file may depend only on foundational AST infrastructure:
//!
//! - `super::super::node::AstNode`;
//! - `super::super::node::Node`;
//! - `super::super::node_id::NodeId`;
//! - `super::super::node_kind::{CoreNodeKind, NodeKind}`;
//! - `super::super::metadata::NodeMetadata`;
//! - `super::super::source::Span`;
//! - Rust standard library;
//! - `serde` for serialization.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - quantum optimization;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum error correction;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - OpenQASM;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! ## Integration contract
//!
//! ```text
//! NodeId / NodeKind / Span / Metadata
//!                 │
//!                 ▼
//!              Program
//!                 │
//!                 ├────────► AST storage
//!                 │              │
//!                 │              ▼
//!                 │       child AST nodes
//!                 │
//!                 ▼
//!        structural validation
//!                 │
//!                 ▼
//!          semantic analysis
//!                 │
//!                 ▼
//!             SemanticModel
//!                 │
//!                 ▼
//!                ZUIR
//! ```
//!
//! `Program` does not perform semantic lowering itself.
//!
//! ## Scalability
//!
//! There is no program-size constant in this file.
//!
//! The `Vec<NodeId>` grows according to the available address space and
//! allocator capacity. Compiler-level operational limits, if required for
//! untrusted input, must be supplied by the configurable compiler-limit layer.
//!
//! Such limits are policies, not language semantics.
//!
//! ## Determinism
//!
//! Program child ordering is explicit and deterministic.
//!
//! `Program` does not use:
//!
//! - global mutable state;
//! - timestamps;
//! - random IDs;
//! - memory addresses;
//! - hash iteration order;
//! - hardware information.
//!
//! ## Thread safety
//!
//! `Program` contains ordinary value types only. Immutable programs can be
//! shared between read-only compiler phases when their constituent types are
//! `Send + Sync`.
//!
//! Mutation is explicit and occurs through `&mut self`.
//!
//! ## Safety
//!
//! This file contains no `unsafe` code.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! Only stable language/library facilities are used.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the native Zamani program-root representation.
///
/// This is intentionally independent from:
///
/// - the Zamani language version;
/// - the compiler version;
/// - the serialized AST format version;
/// - extension versions.
///
/// Changing one version space must not implicitly change another.
pub const PROGRAM_AST_SCHEMA_VERSION: u16 = 1;

/// Canonical source-level program root.
///
/// `Program` owns the ordered top-level structure of a source unit while
/// delegating individual node definitions to their respective AST modules.
///
/// # Representation
///
/// A program consists of:
///
/// - one canonical [`Node`] carrying root identity, kind, source span and
///   metadata;
/// - an ordered vector of child [`NodeId`] values.
///
/// The child IDs refer to nodes owned by the surrounding AST storage layer.
///
/// # Important
///
/// `Program` deliberately does not store concrete declaration/statement types.
/// Doing so here would recreate the monolithic AST that the new frontend
/// architecture is explicitly intended to eliminate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    /// Common source-level identity and metadata.
    node: Node,

    /// Ordered top-level child node identities.
    ///
    /// Ordering is semantically relevant at the source-structure level because
    /// it preserves source program order.
    ///
    /// The vector contains no hardware or machine assumptions.
    children: Vec<NodeId>,
}

impl Program {
    /// Creates a new canonical program root.
    ///
    /// # Arguments
    ///
    /// * `node` - Root node. Its kind must be `CoreNodeKind::Program`.
    /// * `children` - Ordered IDs of direct top-level AST children.
    ///
    /// # Errors
    ///
    /// Returns [`ProgramConstructionError`] if the supplied node is not a
    /// native program node or if the root ID occurs among its own children.
    ///
    /// Duplicate child IDs are also rejected because two positions referring
    /// to the same direct AST node would make the source tree ambiguous.
    ///
    /// # Integration
    ///
    /// Parser:
    ///
    /// ```text
    /// parser
    ///   │
    ///   ├── allocate root NodeId
    ///   ├── create root Node
    ///   ├── parse top-level constructs
    ///   ├── allocate child NodeIds
    ///   └── Program::try_new(...)
    /// ```
    ///
    /// AST storage subsequently owns the concrete nodes identified by
    /// `children`.
    pub fn try_new(
        node: Node,
        children: Vec<NodeId>,
    ) -> Result<Self, ProgramConstructionError> {
        Self::validate_root_node(&node)?;
        Self::validate_child_ids(node.id(), &children)?;

        Ok(Self { node, children })
    }

    /// Creates an empty program with an explicitly supplied root node.
    ///
    /// This is useful when the parser has recognized a source unit before
    /// parsing its first top-level item.
    ///
    /// The supplied node must have `CoreNodeKind::Program`.
    pub fn empty(node: Node) -> Result<Self, ProgramConstructionError> {
        Self::try_new(node, Vec::new())
    }

    /// Creates an empty program using the supplied identity, source span and
    /// metadata.
    ///
    /// This is the primary convenience constructor for parser/builder code.
    ///
    /// The constructor does not manufacture a node ID. The caller remains
    /// responsible for deterministic allocation through `NodeIdAllocator`.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Program),
            span,
            metadata,
        );

        Self {
            node,
            children: Vec::new(),
        }
    }

    /// Creates an empty program with default metadata.
    ///
    /// The root identity and source span remain explicit because silently
    /// manufacturing them would weaken deterministic source construction.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
    ) -> Self {
        Self::new(id, span, NodeMetadata::default())
    }

    /// Returns the number of direct top-level children.
    ///
    /// This is a structural count only. It does not imply the number of:
    ///
    /// - statements;
    /// - functions;
    /// - quantum operations;
    /// - qubits;
    /// - hardware resources.
    #[inline]
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns `true` when the program contains no direct top-level children.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the ordered child IDs.
    ///
    /// The returned slice does not expose the program's internal allocation.
    #[inline]
    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    /// Returns the child ID at `index`.
    ///
    /// Returns `None` when the index is outside the program.
    #[inline]
    pub fn child(&self, index: usize) -> Option<NodeId> {
        self.children.get(index).copied()
    }

    /// Returns the first child ID, if one exists.
    #[inline]
    pub fn first_child(&self) -> Option<NodeId> {
        self.children.first().copied()
    }

    /// Returns the last child ID, if one exists.
    #[inline]
    pub fn last_child(&self) -> Option<NodeId> {
        self.children.last().copied()
    }

    /// Returns an iterator over direct children in source order.
    ///
    /// This is intentionally allocation-free.
    #[inline]
    pub fn iter_children(&self) -> std::slice::Iter<'_, NodeId> {
        self.children.iter()
    }

    /// Returns an exact double-ended iterator over direct children.
    ///
    /// Consumers can traverse the program from either end without copying.
    #[inline]
    pub fn iter_children_reversed(
        &self,
    ) -> std::iter::Rev<std::slice::Iter<'_, NodeId>> {
        self.children.iter().rev()
    }

    /// Returns whether a direct child with `id` exists.
    ///
    /// This performs a linear scan and does not allocate.
    ///
    /// It is intended for occasional queries. Bulk validation uses a
    /// `HashSet` to avoid quadratic duplicate detection.
    #[inline]
    pub fn contains_child(&self, id: NodeId) -> bool {
        self.children.contains(&id)
    }

    /// Appends a direct child to the program.
    ///
    /// The operation preserves source order.
    ///
    /// # Errors
    ///
    /// Fails if:
    ///
    /// - `id` is the program's own identity;
    /// - `id` already exists as a direct child.
    ///
    /// No machine-specific validation is performed here.
    pub fn push_child(
        &mut self,
        id: NodeId,
    ) -> Result<(), ProgramMutationError> {
        if id == self.id() {
            return Err(ProgramMutationError::SelfReference { id });
        }

        if self.children.contains(&id) {
            return Err(ProgramMutationError::DuplicateChild { id });
        }

        self.children.push(id);
        Ok(())
    }

    /// Inserts a child at `index`.
    ///
    /// Existing children at or after `index` are shifted while preserving
    /// their relative order.
    ///
    /// This method deliberately uses the standard `Vec` insertion semantics;
    /// callers constructing very large programs should prefer append-oriented
    /// construction when source order permits.
    pub fn insert_child(
        &mut self,
        index: usize,
        id: NodeId,
    ) -> Result<(), ProgramMutationError> {
        if id == self.id() {
            return Err(ProgramMutationError::SelfReference { id });
        }

        if self.children.contains(&id) {
            return Err(ProgramMutationError::DuplicateChild { id });
        }

        if index > self.children.len() {
            return Err(ProgramMutationError::ChildIndexOutOfBounds {
                index,
                len: self.children.len(),
            });
        }

        self.children.insert(index, id);
        Ok(())
    }

    /// Removes the child at `index`.
    ///
    /// Returns the removed node ID when the index exists.
    #[inline]
    pub fn remove_child(&mut self, index: usize) -> Option<NodeId> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /// Removes a direct child by identity.
    ///
    /// Returns `true` when the child was present.
    pub fn remove_child_by_id(&mut self, id: NodeId) -> bool {
        if let Some(index) = self.children.iter().position(|child| *child == id) {
            self.children.remove(index);
            true
        } else {
            false
        }
    }

    /// Clears all direct children while retaining the program root.
    ///
    /// The root identity, source span and metadata are unchanged.
    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    /// Reserves capacity for additional children.
    ///
    /// This method does not impose a semantic maximum.
    ///
    /// It is a performance hint to the underlying allocator only.
    ///
    /// Allocation failure is handled by Rust's normal allocation behavior.
    /// Compiler-level resource policies should be implemented outside the AST
    /// representation.
    pub fn reserve_children(&mut self, additional: usize) {
        self.children.reserve(additional);
    }

    /// Reserves capacity for at least `additional` more children without
    /// changing the program.
    ///
    /// This is the fallible variant useful to infrastructure that wants to
    /// handle allocation failure as a normal error rather than relying on the
    /// process-level allocation policy.
    pub fn try_reserve_children(
        &mut self,
        additional: usize,
    ) -> Result<(), ProgramCapacityError> {
        self.children
            .try_reserve(additional)
            .map_err(ProgramCapacityError::from)
    }

    /// Validates the complete structural contract of the program root.
    ///
    /// This validates only information owned by `Program`.
    ///
    /// It does not attempt to resolve child IDs against an external node
    /// storage because the storage abstraction is intentionally outside this
    /// file.
    ///
    /// The validator checks:
    ///
    /// - root kind;
    /// - root self-reference;
    /// - duplicate direct children.
    ///
    /// Storage-level validation must additionally check that every child ID
    /// resolves to exactly one concrete node and that the resolved node is
    /// structurally legal in a program-root position.
    pub fn validate(&self) -> Result<(), ProgramValidationError> {
        Self::validate_root_node(&self.node)
            .map_err(ProgramValidationError::InvalidRoot)?;

        if self.children.iter().any(|id| *id == self.id()) {
            return Err(ProgramValidationError::SelfReference {
                id: self.id(),
            });
        }

        let mut seen = HashSet::with_capacity(self.children.len());

        for &child in &self.children {
            if !seen.insert(child) {
                return Err(ProgramValidationError::DuplicateChild { id: child });
            }
        }

        Ok(())
    }

    /// Returns the AST program schema version.
    #[inline]
    pub const fn schema_version() -> u16 {
        PROGRAM_AST_SCHEMA_VERSION
    }

    /// Returns a source-level diagnostic summary.
    ///
    /// This intentionally excludes metadata so diagnostics cannot accidentally
    /// materialize potentially large extension payloads.
    #[inline]
    pub fn diagnostic_summary(&self) -> ProgramDiagnosticSummary {
        ProgramDiagnosticSummary {
            id: self.id(),
            child_count: self.child_count(),
            span: self.span().clone(),
        }
    }

    /// Validates that a supplied node is the canonical program root.
    fn validate_root_node(
        node: &Node,
    ) -> Result<(), ProgramConstructionError> {
        let expected = NodeKind::core(CoreNodeKind::Program);

        if node.kind() != &expected {
            return Err(ProgramConstructionError::InvalidRootKind {
                actual: node.kind_owned(),
            });
        }

        Ok(())
    }

    /// Validates direct child identities without resolving them.
    fn validate_child_ids(
        root_id: NodeId,
        children: &[NodeId],
    ) -> Result<(), ProgramConstructionError> {
        let mut seen = HashSet::with_capacity(children.len());

        for &child in children {
            if child == root_id {
                return Err(ProgramConstructionError::SelfReference {
                    id: child,
                });
            }

            if !seen.insert(child) {
                return Err(ProgramConstructionError::DuplicateChild {
                    id: child,
                });
            }
        }

        Ok(())
    }
}

impl AstNode for Program {
    /// Returns the common program-root node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common program-root node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Default for Program {
    /// Creates an empty, structurally valid program root using the foundational
    /// default values.
    ///
    /// Parser production paths should normally prefer [`Program::new`] so the
    /// caller explicitly supplies the deterministic root ID and source span.
    fn default() -> Self {
        Self {
            node: Node::new(
                NodeId::default(),
                NodeKind::core(CoreNodeKind::Program),
                Span::default(),
                NodeMetadata::default(),
            ),
            children: Vec::new(),
        }
    }
}

/// Errors produced while constructing a [`Program`].
///
/// These are local structural errors. Semantic and target errors belong to
/// later compilation layers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramConstructionError {
    /// The supplied root node is not `CoreNodeKind::Program`.
    InvalidRootKind {
        /// Actual supplied node kind.
        actual: NodeKind,
    },

    /// The program root was accidentally inserted as its own child.
    SelfReference {
        /// Invalid child/root identity.
        id: NodeId,
    },

    /// A direct child identity occurred more than once.
    DuplicateChild {
        /// Duplicated identity.
        id: NodeId,
    },
}

impl fmt::Display for ProgramConstructionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRootKind { actual } => write!(
                formatter,
                "invalid AST program root kind: expected zamani::program, got {}",
                actual
            ),
            Self::SelfReference { id } => write!(
                formatter,
                "AST program root cannot contain itself as child: node {id}"
            ),
            Self::DuplicateChild { id } => write!(
                formatter,
                "AST program contains duplicate direct child node: {id}"
            ),
        }
    }
}

impl std::error::Error for ProgramConstructionError {}

/// Errors produced when mutating a [`Program`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramMutationError {
    /// A program cannot directly contain itself.
    SelfReference {
        /// Invalid child identity.
        id: NodeId,
    },

    /// A direct child identity is already present.
    DuplicateChild {
        /// Duplicated identity.
        id: NodeId,
    },

    /// An insertion index lies outside the current child sequence.
    ChildIndexOutOfBounds {
        /// Requested insertion index.
        index: usize,

        /// Current number of children.
        len: usize,
    },
}

impl fmt::Display for ProgramMutationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfReference { id } => write!(
                formatter,
                "AST program cannot contain itself as child: node {id}"
            ),
            Self::DuplicateChild { id } => write!(
                formatter,
                "AST program already contains direct child node: {id}"
            ),
            Self::ChildIndexOutOfBounds { index, len } => write!(
                formatter,
                "AST program child insertion index {index} is out of bounds for {len} children"
            ),
        }
    }
}

impl std::error::Error for ProgramMutationError {}

/// Errors produced by program structural validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramValidationError {
    /// The root node violates the program-root contract.
    InvalidRoot(ProgramConstructionError),

    /// The program directly contains itself.
    SelfReference {
        /// Invalid root identity.
        id: NodeId,
    },

    /// A direct child occurs more than once.
    DuplicateChild {
        /// Duplicated identity.
        id: NodeId,
    },
}

impl fmt::Display for ProgramValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoot(error) => write!(
                formatter,
                "invalid AST program root: {error}"
            ),
            Self::SelfReference { id } => write!(
                formatter,
                "AST program contains self-reference through node {id}"
            ),
            Self::DuplicateChild { id } => write!(
                formatter,
                "AST program contains duplicate direct child node {id}"
            ),
        }
    }
}

impl std::error::Error for ProgramValidationError {}

/// Allocation failure while reserving program-child storage.
///
/// This is deliberately a thin wrapper around the standard allocator error.
/// The AST does not invent a semantic maximum program size.
#[derive(Debug)]
pub struct ProgramCapacityError {
    source: std::collections::TryReserveError,
}

impl From<std::collections::TryReserveError> for ProgramCapacityError {
    #[inline]
    fn from(source: std::collections::TryReserveError) -> Self {
        Self { source }
    }
}

impl fmt::Display for ProgramCapacityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unable to reserve AST program child storage: {}",
            self.source
        )
    }
}

impl std::error::Error for ProgramCapacityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Compact diagnostic information for a program root.
///
/// Metadata is intentionally excluded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramDiagnosticSummary {
    /// Stable AST root identity.
    pub id: NodeId,

    /// Number of direct top-level nodes.
    pub child_count: usize,

    /// Source span of the complete program.
    pub span: Span,
}

impl fmt::Display for ProgramDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "program {} with {} top-level nodes at {}",
            self.id,
            self.child_count,
            self.span
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeIdAllocator;

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn new_program_has_program_kind() {
        let id = NodeId::first();

        let program = Program::without_metadata(
            id,
            test_span(),
        );

        assert_eq!(
            program.kind(),
            &NodeKind::core(CoreNodeKind::Program)
        );
        assert_eq!(program.id(), id);
        assert!(program.is_empty());
        assert_eq!(program.child_count(), 0);
    }

    #[test]
    fn empty_program_is_structurally_valid() {
        let id = NodeId::first();

        let program = Program::without_metadata(
            id,
            test_span(),
        );

        assert!(program.validate().is_ok());
    }

    #[test]
    fn children_preserve_source_order() {
        let mut allocator = NodeIdAllocator::new();

        let root = allocator.allocate().expect("root ID");
        let first = allocator.allocate().expect("first child ID");
        let second = allocator.allocate().expect("second child ID");
        let third = allocator.allocate().expect("third child ID");

        let mut program = Program::without_metadata(
            root,
            test_span(),
        );

        program.push_child(first).expect("first child");
        program.push_child(second).expect("second child");
        program.push_child(third).expect("third child");

        assert_eq!(
            program.children(),
            &[first, second, third]
        );
    }

    #[test]
    fn duplicate_children_are_rejected() {
        let root = NodeId::first();

        let child = root
            .checked_next()
            .expect("second ID");

        let mut program = Program::without_metadata(
            root,
            test_span(),
        );

        program.push_child(child).expect("first insertion");

        let error = program
            .push_child(child)
            .expect_err("duplicate must fail");

        assert_eq!(
            error,
            ProgramMutationError::DuplicateChild { id: child }
        );
    }

    #[test]
    fn root_self_reference_is_rejected() {
        let root = NodeId::first();

        let mut program = Program::without_metadata(
            root,
            test_span(),
        );

        let error = program
            .push_child(root)
            .expect_err("self-reference must fail");

        assert_eq!(
            error,
            ProgramMutationError::SelfReference { id: root }
        );
    }

    #[test]
    fn insert_and_remove_preserve_order() {
        let mut allocator = NodeIdAllocator::new();

        let root = allocator.allocate().expect("root");
        let first = allocator.allocate().expect("first");
        let second = allocator.allocate().expect("second");
        let middle = allocator.allocate().expect("middle");

        let mut program = Program::without_metadata(
            root,
            test_span(),
        );

        program.push_child(first).expect("first");
        program.push_child(second).expect("second");

        program
            .insert_child(1, middle)
            .expect("middle insertion");

        assert_eq!(
            program.children(),
            &[first, middle, second]
        );

        assert_eq!(
            program.remove_child(1),
            Some(middle)
        );

        assert_eq!(
            program.children(),
            &[first, second]
        );
    }

    #[test]
    fn remove_child_by_id_works() {
        let mut allocator = NodeIdAllocator::new();

        let root = allocator.allocate().expect("root");
        let first = allocator.allocate().expect("first");
        let second = allocator.allocate().expect("second");

        let mut program = Program::without_metadata(
            root,
            test_span(),
        );

        program.push_child(first).expect("first");
        program.push_child(second).expect("second");

        assert!(program.remove_child_by_id(first));
        assert!(!program.contains_child(first));
        assert_eq!(program.children(), &[second]);

        assert!(!program.remove_child_by_id(first));
    }

    #[test]
    fn validation_rejects_duplicate_children_after_direct_mutation_is_not_possible() {
        let mut allocator = NodeIdAllocator::new();

        let root = allocator.allocate().expect("root");
        let first = allocator.allocate().expect("first");

        let program = Program {
            node: Node::new(
                root,
                NodeKind::core(CoreNodeKind::Program),
                test_span(),
                NodeMetadata::default(),
            ),
            children: vec![first, first],
        };

        assert_eq!(
            program.validate(),
            Err(ProgramValidationError::DuplicateChild {
                id: first
            })
        );
    }

    #[test]
    fn diagnostic_summary_is_compact() {
        let root = NodeId::first();

        let program = Program::without_metadata(
            root,
            test_span(),
        );

        let summary = program.diagnostic_summary();

        assert_eq!(summary.id, root);
        assert_eq!(summary.child_count, 0);
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(
            Program::schema_version(),
            PROGRAM_AST_SCHEMA_VERSION
        );
    }

    #[test]
    fn deterministic_allocator_produces_deterministic_program_structure() {
        let mut left_allocator = NodeIdAllocator::new();
        let mut right_allocator = NodeIdAllocator::new();

        let left_root = left_allocator
            .allocate()
            .expect("left root");

        let right_root = right_allocator
            .allocate()
            .expect("right root");

        let mut left = Program::without_metadata(
            left_root,
            test_span(),
        );

        let mut right = Program::without_metadata(
            right_root,
            test_span(),
        );

        for _ in 0..3 {
            let left_id = left_allocator
                .allocate()
                .expect("left child");

            let right_id = right_allocator
                .allocate()
                .expect("right child");

            left.push_child(left_id)
                .expect("left insertion");

            right.push_child(right_id)
                .expect("right insertion");
        }

        assert_eq!(left, right);
    }

    #[test]
    fn ast_node_contract_is_available() {
        let root = NodeId::first();

        let program = Program::without_metadata(
            root,
            test_span(),
        );

        assert_eq!(AstNode::id(&program), root);
        assert_eq!(
            AstNode::kind(&program),
            &NodeKind::core(CoreNodeKind::Program)
        );
    }
}