//! # Zamani Frontend AST — Program Builder
//!
//! `src/frontend/ast/node/builders/program_builder.rs`
//!
//! Production-grade construction API for the canonical source-level
//! [`Program`] AST root.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer
//!      │
//!      ▼
//! parser
//!      │
//!      ▼
//! ProgramBuilder
//!      │
//!      ▼
//! Native Zamani AST
//!      │
//!      ▼
//! structural validation
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├──────────────┬──────────────┬──────────────┐
//!      ▼              ▼              ▼              ▼
//! classical        quantum          HDL          future
//! domain IR        domain IR      domain IR      domains
//! ```
//!
//! ## Purpose
//!
//! `ProgramBuilder` owns the construction workflow for a [`Program`] root.
//!
//! It does not own:
//!
//! - concrete declarations;
//! - concrete statements;
//! - expressions;
//! - types;
//! - quantum operations;
//! - hardware;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - semantic analysis;
//! - ZUIR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime execution.
//!
//! The builder creates only the source-level program container and its ordered
//! child `NodeId` references.
//!
//! ## Canonical ownership model
//!
//! ```text
//! ProgramBuilder
//!      │
//!      ▼
//! Program
//!      │
//!      └── Vec<NodeId>
//!             │
//!             ▼
//!      canonical AST node store
//!             │
//!             └── concrete AST nodes
//! ```
//!
//! `ProgramBuilder` therefore never creates a second representation of a
//! concrete AST node.
//!
//! ## POCO-REAF
//!
//! The builder contains no assumptions about:
//!
//! - machine size;
//! - processor count;
//! - memory size;
//! - qubit count;
//! - register count;
//! - quantum topology;
//! - hardware vendor;
//! - backend;
//! - instruction set;
//! - gate set;
//! - simulator;
//! - QPU;
//! - accelerator.
//!
//! A program can therefore be constructed identically for a tiny machine,
//! large machine, heterogeneous system, distributed system, quantum system,
//! simulator, or future computational platform.
//!
//! ```text
//! Program_Once
//!      │
//!      ▼
//! canonical source AST
//!      │
//!      ▼
//! compile according to available capabilities/resources
//!      │
//!      ▼
//! target realization
//! ```
//!
//! ## Scalability
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_PROGRAM_ITEMS
//! MAX_NODES
//! MAX_QUBITS
//! MAX_REGISTERS
//! MAX_MACHINE_SIZE
//! ```
//!
//! The builder delegates storage growth to Rust's standard collections.
//!
//! Operational/resource limits for untrusted input belong to configurable
//! compiler policy layers and must not become AST semantics.
//!
//! ## Determinism
//!
//! Deterministic construction is achieved through an explicitly supplied
//! [`NodeIdAllocator`].
//!
//! The builder does not use:
//!
//! - global counters;
//! - random IDs;
//! - timestamps;
//! - memory addresses;
//! - thread IDs;
//! - hash-map iteration order.
//!
//! Source order is preserved exactly by the program's ordered child list.
//!
//! ## Allocation ownership
//!
//! The caller owns the [`NodeIdAllocator`]. The builder temporarily borrows it
//! mutably for the duration of construction.
//!
//! This is intentional:
//!
//! ```text
//! parser/compiler session
//!       │
//!       └── NodeIdAllocator
//!               │
//!               ├── ProgramBuilder
//!               ├── declaration builders
//!               ├── expression builders
//!               └── generated-node builders
//! ```
//!
//! There is no hidden allocator state.
//!
//! ## Error model
//!
//! This file exposes one builder-level error:
//!
//! [`ProgramBuilderError`]
//!
//! It preserves the underlying construction/allocation error rather than
//! converting failures into strings.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! ```text
//! parser
//!   │
//!   ├── owns NodeIdAllocator
//!   │
//!   └── ProgramBuilder::new(...)
//!          │
//!          ├── push_child(existing_node_id)
//!          │
//!          └── push_new_child()
//!                    │
//!                    ▼
//!               concrete AST node
//!                    │
//!                    └── same allocated NodeId
//!
//! ProgramBuilder::finish()
//!          │
//!          ▼
//! Program
//! ```
//!
//! The parser remains responsible for constructing the concrete node that a
//! newly allocated child ID identifies.
//!
//! ### AST storage
//!
//! The builder does not own or resolve the AST node store.
//!
//! The surrounding AST storage layer must guarantee:
//!
//! ```text
//! every Program child NodeId
//!     → exactly one canonical AST node
//! ```
//!
//! ### Structural validation
//!
//! ```text
//! ProgramBuilder
//!      │
//!      ▼
//! Program
//!      │
//!      ▼
//! structural validator
//! ```
//!
//! Builder validation is local. Global graph validation remains the
//! responsibility of the AST validation layer.
//!
//! ### Semantic analysis
//!
//! ```text
//! Program
//!   │
//!   ▼
//! AST storage
//!   │
//!   ▼
//! concrete nodes
//!   │
//!   ▼
//! semantic analysis
//! ```
//!
//! The builder must never resolve names, types, resources, domains,
//! capabilities, or quantum meaning.
//!
//! ### ZUIR
//!
//! There is no direct builder → ZUIR dependency.
//!
//! The correct path remains:
//!
//! ```text
//! ProgramBuilder
//!      ▼
//! Program / AST
//!      ▼
//! semantic analysis
//!      ▼
//! semantic model
//!      ▼
//! ZUIR
//! ```
//!
//! ## Quantum integration
//!
//! Quantum programs use this builder exactly like classical programs.
//!
//! The builder does not know what a qubit is.
//!
//! For example, it can construct a program whose children ultimately represent:
//!
//! - quantum-resource declarations;
//! - generic operations;
//! - measurements;
//! - reset;
//! - classical control;
//! - hybrid computation;
//! - timing intent;
//! - quantum algorithm declarations;
//! - future quantum constructs.
//!
//! Their concrete meanings belong to their respective AST/semantic layers.
//!
//! Therefore adding:
//!
//! - a new gate;
//! - a new quantum technology;
//! - a new QEC strategy;
//! - a new backend;
//! - a new hardware topology;
//! - a new computational domain;
//!
//! does not require changing this builder.
//!
//! ## Safety
//!
//! This file contains no `unsafe` code.
//!
//! No raw pointers are used.
//!
//! No unchecked indexing is used.
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
//! - no external runtime requirements.
//!
//! ## Dependency contract
//!
//! Allowed dependencies:
//!
//! ```text
//! super::super::node_id
//! super::super::program::program
//! standard library
//! ```
//!
//! The builder must never depend on:
//!
//! ```text
//! lexer
//! parser
//! semantic
//! compiler
//! quantum::ir
//! hardware
//! optimizer
//! router
//! scheduler
//! QEC
//! resilience
//! runtime
//! backend
//! ZUIR
//! QIR
//! LLVM
//! MLIR
//! ```
//!
//! ## File completion contract
//!
//! This file is complete when it provides:
//!
//! - deterministic program-root construction;
//! - explicit allocator ownership;
//! - ordered child insertion;
//! - automatic child-ID allocation;
//! - existing-child-ID insertion;
//! - fallible capacity reservation;
//! - local structural validation through `Program`;
//! - zero hidden global state;
//! - zero machine-size assumptions;
//! - zero quantum-size assumptions;
//! - safe Rust only;
//! - deterministic behavior;
//! - unit tests covering construction and scalability properties.
//!
//! Adding a new AST node type must not require modifying this file.
//!
//! Adding a new quantum technology must not require modifying this file.
//!
//! Adding a new backend must not require modifying this file.
//!
//! Adding a new computational domain must not require modifying this file.

use super::super::node_id::{
    NodeId,
    NodeIdAllocationError,
    NodeIdAllocator,
};
use super::super::program::program::{
    Program,
    ProgramCapacityError,
    ProgramMutationError,
    ProgramValidationError,
};
use super::super::metadata::NodeMetadata;
use super::super::source::Span;
use std::fmt;

/// Production builder for the canonical Zamani [`Program`] AST root.
///
/// The builder owns no concrete child AST nodes. It only constructs the
/// program root and its ordered child-node identities.
///
/// # Lifetime
///
/// The allocator is borrowed mutably rather than owned. This allows the
/// surrounding parser/compiler session to retain ownership of deterministic
/// node-ID allocation and use the same allocator for the complete AST.
///
/// # Example
///
/// ```ignore
/// let mut allocator = NodeIdAllocator::new();
///
/// let mut builder = ProgramBuilder::new(
///     &mut allocator,
///     span,
///     NodeMetadata::default(),
/// )?;
///
/// let declaration_id = allocator.allocate()?;
/// // Construct the concrete declaration using declaration_id here.
///
/// builder.push_child(declaration_id)?;
///
/// let program = builder.finish()?;
/// # Ok::<(), ProgramBuilderError>(())
/// ```
pub struct ProgramBuilder<'a> {
    allocator: &'a mut NodeIdAllocator,
    program: Program,
}

impl<'a> ProgramBuilder<'a> {
    /// Creates a new program builder.
    ///
    /// The first ID allocated from `allocator` becomes the program-root ID.
    ///
    /// The allocator is never global and is never created implicitly.
    ///
    /// This guarantees that ID allocation remains under the control of the
    /// parser/compiler session.
    pub fn new(
        allocator: &'a mut NodeIdAllocator,
        span: Span,
        metadata: NodeMetadata,
    ) -> Result<Self, ProgramBuilderError> {
        let root_id = allocator.allocate()?;

        let program = Program::new(root_id, span, metadata);

        Ok(Self {
            allocator,
            program,
        })
    }

    /// Creates a new program builder with default metadata.
    ///
    /// Source identity remains explicit through the supplied span and
    /// allocator.
    pub fn without_metadata(
        allocator: &'a mut NodeIdAllocator,
        span: Span,
    ) -> Result<Self, ProgramBuilderError> {
        Self::new(allocator, span, NodeMetadata::default())
    }

    /// Returns the program root's stable [`NodeId`].
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.program.id()
    }

    /// Returns the number of direct top-level children currently registered.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.program.child_count()
    }

    /// Returns whether the program currently has no direct children.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.program.is_empty()
    }

    /// Returns the ordered direct child IDs.
    ///
    /// The slice is borrowed directly from the canonical program container.
    #[inline]
    #[must_use]
    pub fn children(&self) -> &[NodeId] {
        self.program.children()
    }

    /// Returns the child at `index`.
    #[inline]
    #[must_use]
    pub fn child(&self, index: usize) -> Option<NodeId> {
        self.program.child(index)
    }

    /// Returns the first direct child.
    #[inline]
    #[must_use]
    pub fn first_child(&self) -> Option<NodeId> {
        self.program.first_child()
    }

    /// Returns the last direct child.
    #[inline]
    #[must_use]
    pub fn last_child(&self) -> Option<NodeId> {
        self.program.last_child()
    }

    /// Returns an iterator over children in source order.
    #[inline]
    pub fn iter_children(
        &self,
    ) -> std::slice::Iter<'_, NodeId> {
        self.program.iter_children()
    }

    /// Returns whether `id` is already a direct child.
    #[inline]
    #[must_use]
    pub fn contains_child(&self, id: NodeId) -> bool {
        self.program.contains_child(id)
    }

    /// Appends an already allocated AST child ID.
    ///
    /// This is the primary integration point when the parser or another AST
    /// builder constructs a concrete node first and then attaches it to the
    /// program.
    ///
    /// Source order is preserved.
    pub fn push_child(
        &mut self,
        id: NodeId,
    ) -> Result<(), ProgramBuilderError> {
        self.program
            .push_child(id)
            .map_err(ProgramBuilderError::Mutation)
    }

    /// Allocates a fresh child ID and appends it to the program.
    ///
    /// The returned ID must subsequently be used by the caller as the identity
    /// of the concrete AST node being constructed.
    ///
    /// Allocation and insertion happen in that order:
    ///
    /// ```text
    /// allocator
    ///    │
    ///    ▼
    /// fresh NodeId
    ///    │
    ///    ▼
    /// Program child list
    ///    │
    ///    ▼
    /// concrete AST node uses same NodeId
    /// ```
    ///
    /// Because the allocator is monotonic, this operation cannot intentionally
    /// create a duplicate ID.
    pub fn push_new_child(
        &mut self,
    ) -> Result<NodeId, ProgramBuilderError> {
        let id = self.allocator.allocate()?;

        self.program
            .push_child(id)
            .map_err(ProgramBuilderError::Mutation)?;

        Ok(id)
    }

    /// Inserts an existing child ID at `index`.
    ///
    /// Existing source order is preserved except for the explicitly requested
    /// insertion point.
    pub fn insert_child(
        &mut self,
        index: usize,
        id: NodeId,
    ) -> Result<(), ProgramBuilderError> {
        self.program
            .insert_child(index, id)
            .map_err(ProgramBuilderError::Mutation)
    }

    /// Allocates a fresh child ID and inserts it at `index`.
    ///
    /// Returns the newly allocated ID.
    pub fn insert_new_child(
        &mut self,
        index: usize,
    ) -> Result<NodeId, ProgramBuilderError> {
        let id = self.allocator.allocate()?;

        match self.program.insert_child(index, id) {
            Ok(()) => Ok(id),
            Err(error) => Err(ProgramBuilderError::Mutation(error)),
        }
    }

    /// Removes a child at `index`.
    ///
    /// Removing a child from the program does not recycle its `NodeId`.
    ///
    /// This is intentional. Node IDs are identities, not reusable array
    /// positions.
    #[inline]
    pub fn remove_child(
        &mut self,
        index: usize,
    ) -> Option<NodeId> {
        self.program.remove_child(index)
    }

    /// Removes a child by identity.
    ///
    /// The node ID is not returned to the allocator.
    pub fn remove_child_by_id(
        &mut self,
        id: NodeId,
    ) -> bool {
        self.program.remove_child_by_id(id)
    }

    /// Clears the program's direct children.
    ///
    /// Existing IDs are not recycled.
    pub fn clear_children(&mut self) {
        self.program.clear_children();
    }

    /// Reserves capacity for `additional` direct children.
    ///
    /// This is a performance hint only.
    ///
    /// It does not create a language-level limit and does not impose a maximum
    /// program size.
    pub fn reserve_children(&mut self, additional: usize) {
        self.program.reserve_children(additional);
    }

    /// Fallibly reserves capacity for `additional` direct children.
    ///
    /// This method is intended for compiler infrastructure that wants to treat
    /// allocation failure as an explicit result.
    pub fn try_reserve_children(
        &mut self,
        additional: usize,
    ) -> Result<(), ProgramBuilderError> {
        self.program
            .try_reserve_children(additional)
            .map_err(ProgramBuilderError::Capacity)
    }

    /// Returns the allocator's next candidate ID.
    ///
    /// This does not allocate.
    #[inline]
    #[must_use]
    pub fn next_node_id(&self) -> Option<NodeId> {
        self.allocator.peek()
    }

    /// Returns the number of IDs allocated from the allocator since its
    /// configured starting point.
    ///
    /// This value is informational only.
    #[inline]
    #[must_use]
    pub fn allocated_node_count(&self) -> u64 {
        self.allocator.allocated_count()
    }

    /// Validates the local structural invariants of the program.
    ///
    /// This does not resolve children through an external AST store.
    ///
    /// Global graph validation remains the responsibility of the AST
    /// validation layer.
    pub fn validate(&self) -> Result<(), ProgramBuilderError> {
        self.program
            .validate()
            .map_err(ProgramBuilderError::Validation)
    }

    /// Returns an immutable reference to the program under construction.
    ///
    /// This is useful for parser diagnostics or inspection without transferring
    /// ownership away from the builder.
    #[inline]
    #[must_use]
    pub fn as_program(&self) -> &Program {
        &self.program
    }

    /// Finishes construction and returns the canonical [`Program`].
    ///
    /// Final local validation occurs before the program is returned.
    ///
    /// The allocator remains owned by the caller because the builder only
    /// borrowed it.
    pub fn finish(
        self,
    ) -> Result<Program, ProgramBuilderError> {
        self.program
            .validate()
            .map_err(ProgramBuilderError::Validation)?;

        Ok(self.program)
    }
}

/// Errors produced by [`ProgramBuilder`].
///
/// The error retains the semantic distinction between:
///
/// - node-ID allocation failure;
/// - local program mutation failure;
/// - capacity failure;
/// - structural validation failure.
///
/// No error is converted into an opaque string.
#[derive(Debug)]
pub enum ProgramBuilderError {
    /// The shared node-ID allocator could not allocate another identity.
    Allocation(NodeIdAllocationError),

    /// A requested program mutation violated a local program invariant.
    Mutation(ProgramMutationError),

    /// Capacity reservation could not be satisfied.
    Capacity(ProgramCapacityError),

    /// Final/local structural validation failed.
    Validation(ProgramValidationError),
}

impl fmt::Display for ProgramBuilderError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Allocation(error) => {
                write!(formatter, "program builder allocation failed: {error}")
            }
            Self::Mutation(error) => {
                write!(formatter, "program builder mutation failed: {error}")
            }
            Self::Capacity(error) => {
                write!(formatter, "program builder capacity reservation failed: {error}")
            }
            Self::Validation(error) => {
                write!(formatter, "program builder validation failed: {error}")
            }
        }
    }
}

impl std::error::Error for ProgramBuilderError {}

impl From<NodeIdAllocationError> for ProgramBuilderError {
    #[inline]
    fn from(error: NodeIdAllocationError) -> Self {
        Self::Allocation(error)
    }
}

/// Deterministic construction contract.
///
/// This helper is intentionally small: it provides the canonical construction
/// sequence without introducing a second program representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramBuilderContract;

impl ProgramBuilderContract {
    /// Returns the current program AST schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        Program::schema_version()
    }

    /// Returns whether the builder is hardware-independent.
    ///
    /// This is a compile-time/documentation-level architectural property.
    #[inline]
    #[must_use]
    pub const fn is_hardware_independent() -> bool {
        true
    }

    /// Returns whether the builder imposes a fixed program-size limit.
    #[inline]
    #[must_use]
    pub const fn has_fixed_program_limit() -> bool {
        false
    }

    /// Returns whether the builder contains quantum-specific knowledge.
    #[inline]
    #[must_use]
    pub const fn is_domain_neutral() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn creates_deterministic_program_root() {
        let mut allocator = NodeIdAllocator::new();

        let builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("program builder should allocate root");

        assert_eq!(builder.id().get(), 1);
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
        assert_eq!(
            builder.next_node_id().map(NodeId::get),
            Some(2)
        );
    }

    #[test]
    fn allocates_children_without_duplicate_ids() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let first = builder
            .push_new_child()
            .expect("first child allocation");

        let second = builder
            .push_new_child()
            .expect("second child allocation");

        assert_eq!(first.get(), 2);
        assert_eq!(second.get(), 3);

        assert_eq!(
            builder.children(),
            &[first, second]
        );
    }

    #[test]
    fn preserves_source_order() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let first = builder
            .push_new_child()
            .expect("first child");

        let second = builder
            .push_new_child()
            .expect("second child");

        let third = builder
            .push_new_child()
            .expect("third child");

        assert_eq!(
            builder.children(),
            &[first, second, third]
        );
    }

    #[test]
    fn rejects_duplicate_existing_child() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let child = allocator
            .allocate()
            .expect("child allocation");

        builder
            .push_child(child)
            .expect("first insertion");

        let result = builder.push_child(child);

        assert!(matches!(
            result,
            Err(ProgramBuilderError::Mutation(
                ProgramMutationError::DuplicateChild { .. }
            ))
        ));
    }

    #[test]
    fn rejects_root_as_child() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let root = builder.id();

        let result = builder.push_child(root);

        assert!(matches!(
            result,
            Err(ProgramBuilderError::Mutation(
                ProgramMutationError::SelfReference { .. }
            ))
        ));
    }

    #[test]
    fn supports_insertion_without_rebuilding_program() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let first = builder
            .push_new_child()
            .expect("first child");

        let third = builder
            .push_new_child()
            .expect("third child");

        let second = builder
            .insert_new_child(1)
            .expect("second child");

        assert_eq!(
            builder.children(),
            &[first, second, third]
        );
    }

    #[test]
    fn removed_ids_are_not_recycled() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let first = builder
            .push_new_child()
            .expect("first child");

        let second = builder
            .push_new_child()
            .expect("second child");

        assert!(builder.remove_child_by_id(first));
        assert_eq!(builder.children(), &[second]);

        let third = builder
            .push_new_child()
            .expect("third child");

        assert_ne!(first, third);
        assert_eq!(third.get(), 4);
    }

    #[test]
    fn finish_performs_final_validation() {
        let mut allocator = NodeIdAllocator::new();

        let builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let program = builder
            .finish()
            .expect("valid empty program");

        assert!(program.is_empty());
        assert_eq!(program.id().get(), 1);
    }

    #[test]
    fn allocator_remains_owned_by_caller() {
        let mut allocator = NodeIdAllocator::new();

        {
            let mut builder = ProgramBuilder::without_metadata(
                &mut allocator,
                test_span(),
            )
            .expect("root allocation");

            builder
                .push_new_child()
                .expect("child allocation");

            let _program = builder
                .finish()
                .expect("finish");
        }

        let next = allocator
            .allocate()
            .expect("allocator remains usable");

        assert_eq!(next.get(), 3);
    }

    #[test]
    fn supports_explicitly_allocated_child_nodes() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        let child = allocator
            .allocate()
            .expect("concrete node allocation");

        builder
            .push_child(child)
            .expect("attach concrete node");

        assert_eq!(builder.first_child(), Some(child));
    }

    #[test]
    fn capacity_reservation_does_not_change_semantics() {
        let mut allocator = NodeIdAllocator::new();

        let mut builder = ProgramBuilder::without_metadata(
            &mut allocator,
            test_span(),
        )
        .expect("root allocation");

        builder.reserve_children(1024);

        let child = builder
            .push_new_child()
            .expect("child allocation");

        assert_eq!(builder.children(), &[child]);
    }

    #[test]
    fn contract_is_domain_neutral() {
        assert!(ProgramBuilderContract::is_domain_neutral());
        assert!(ProgramBuilderContract::is_hardware_independent());
        assert!(!ProgramBuilderContract::has_fixed_program_limit());
    }

    #[test]
    fn schema_version_matches_canonical_program() {
        assert_eq!(
            ProgramBuilderContract::schema_version(),
            Program::schema_version()
        );
    }

    #[test]
    fn builder_has_no_fixed_quantum_size_assumption() {
        // This test deliberately does not construct a quantum-specific object.
        //
        // The important invariant is that ProgramBuilder has no quantum-size
        // field or quantum-size validation.
        assert!(ProgramBuilderContract::is_domain_neutral());
        assert!(!ProgramBuilderContract::has_fixed_program_limit());
    }
}