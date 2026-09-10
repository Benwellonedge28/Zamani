//! # Zamani Frontend AST — Canonical Child Enumeration
//!
//! `src/frontend/ast/node/traversal/children.rs`
//!
//! ## Purpose
//!
//! This module owns the canonical, domain-neutral contract for enumerating the
//! direct children of a Zamani frontend AST node.
//!
//! It deliberately separates:
//!
//! ```text
//! AST storage
//!      │
//!      ▼
//! child relationships
//!      │
//!      ▼
//! this module
//!      │
//!      ├── preorder traversal
//!      ├── postorder traversal
//!      ├── validation
//!      ├── visitors
//!      ├── folds
//!      └── tooling
//! ```
//!
//! This module answers only:
//!
//! > "What are the direct child NodeIds of this AST node, and in what
//! > canonical order?"
//!
//! It does NOT answer:
//!
//! - how children are traversed recursively;
//! - how nodes are interpreted;
//! - what a node means semantically;
//! - how quantum operations execute;
//! - how resources are allocated;
//! - how hardware is selected;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how QEC is implemented;
//! - how calibration is performed;
//! - how a backend executes a program.
//!
//! Those responsibilities belong to later layers.
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
//! native AST
//!      │
//!      ├── NodeId
//!      ├── Node
//!      └── child relationships
//!              │
//!              ▼
//!        ┌───────────────┐
//!        │   children.rs │
//!        └───────┬───────┘
//!                │
//!       ┌────────┼────────┐
//!       ▼        ▼        ▼
//!   preorder  postorder  fold
//!       │        │        │
//!       └────────┼────────┘
//!                ▼
//!        semantic analysis
//!                │
//!                ▼
//!               ZUIR
//! ```
//!
//! ## POCO-REAF
//!
//! Child enumeration contains no machine-size assumptions.
//!
//! It does not contain:
//!
//! - maximum qubit counts;
//! - maximum register sizes;
//! - machine topology;
//! - processor architecture;
//! - vendor identifiers;
//! - backend identifiers;
//! - gate-set assumptions;
//! - quantum-specific child categories;
//! - fixed computational-domain lists.
//!
//! Therefore the same child relationship mechanism can describe:
//!
//! - classical programs;
//! - quantum programs;
//! - hybrid programs;
//! - HDL programs;
//! - accelerator programs;
//! - distributed programs;
//! - AI programs;
//! - future computational domains.
//!
//! This is required for:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## Why this module exists
//!
//! The AST already represents relationships using `NodeId` rather than
//! recursively embedding every descendant in the parent structure. Concrete
//! AST nodes also expose direct-child information in their own modules.
//!
//! For example, a node with a list of children can expose those children in
//! source order without knowing anything about the eventual compilation
//! target.
//!
//! The traversal layer should not repeatedly invent slightly different
//! representations of this contract.
//!
//! This module therefore provides one reusable abstraction for:
//!
//! - direct-child enumeration;
//! - child cursors;
//! - optional child-count accounting;
//! - deterministic ordering contracts;
//! - empty-child handling;
//! - resource-aware iteration.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - [`AstChildrenProvider`];
//! - [`ChildCursor`];
//! - [`ChildCursorError`];
//! - [`ChildEnumeration`];
//! - child enumeration invariants;
//! - child-order documentation;
//! - reusable iterator-level accounting.
//!
//! This file does NOT own:
//!
//! - AST node storage;
//! - `NodeId` allocation;
//! - `Node` definitions;
//! - recursive traversal;
//! - visitor callbacks;
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - runtime execution.
//!
//! ## Dependency contract
//!
//! Allowed dependencies:
//!
//! - [`NodeId`](super::super::node_id::NodeId);
//! - Rust standard library.
//!
//! Deliberately forbidden:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - hardware modules;
//! - backend modules;
//! - runtime modules;
//! - asynchronous runtimes;
//! - vendor SDKs.
//!
//! This keeps child enumeration at the lowest structural traversal layer.
//!
//! ## Canonical ordering
//!
//! A provider MUST return direct children in the AST's canonical structural
//! order.
//!
//! For source-derived ASTs, that normally means source order.
//!
//! This module never:
//!
//! - sorts children;
//! - reverses children;
//! - deduplicates children;
//! - filters children;
//! - interprets children.
//!
//! Those operations belong to higher-level traversal policies.
//!
//! This is critical for deterministic compilation and diagnostics.
//!
//! ## Tree versus DAG
//!
//! Child enumeration itself does not reject repeated `NodeId`s.
//!
//! A repeated child can be:
//!
//! - a malformed AST cycle;
//! - a shared DAG reference;
//! - a deliberately shared structural object;
//! - an error in an AST provider.
//!
//! Determining which interpretation is valid is the responsibility of the
//! traversal/validation layer.
//!
//! Consequently this module does NOT maintain a global visited set.
//!
//! This is intentional.
//!
//! A provider must remain usable by both:
//!
//! ```text
//! tree traversal
//! DAG-aware traversal
//! graph validation
//! shared-node analysis
//! ```
//!
//! without imposing one graph policy on every consumer.
//!
//! ## Scalability
//!
//! There is no:
//!
//! ```text
//! MAX_CHILDREN
//! MAX_NODES
//! MAX_DEPTH
//! MAX_QUBITS
//! MAX_REGISTERS
//! ```
//!
//! in this module.
//!
//! A node may have any number of direct children representable by the host
//! process and the underlying AST storage.
//!
//! The provider may expose millions or more child references without this
//! abstraction changing.
//!
//! No child collection is materialized by the default cursor.
//!
//! This is particularly important for high-fan-out AST nodes.
//!
//! ## Deep programs
//!
//! This module does not recurse.
//!
//! A child cursor only produces the next direct child.
//!
//! Consequently deeply nested ASTs do not consume Rust call-stack frames merely
//! because their children are being enumerated.
//!
//! Iterative depth-first traversal remains the responsibility of
//! `preorder.rs`/`postorder.rs`.
//!
//! ## Wide programs
//!
//! A provider may expose children through an iterator backed by existing AST
//! storage.
//!
//! No `Vec<NodeId>` is required by this abstraction.
//!
//! This allows a node containing a very large number of children to be
//! traversed incrementally.
//!
//! ## Determinism
//!
//! Given:
//!
//! ```text
//! same AST
//! + same provider state
//! + same provider child order
//! ```
//!
//! the cursor yields the same `NodeId` sequence.
//!
//! The cursor contains no:
//!
//! - randomness;
//! - timestamps;
//! - process-global state;
//! - memory-address identity;
//! - hardware state;
//! - thread-local traversal state.
//!
//! ## Mutation
//!
//! This module exposes only read-only child enumeration.
//!
//! AST mutation belongs to:
//!
//! - mutable visitors;
//! - folds;
//! - explicit AST transformation infrastructure.
//!
//! A mutable child relationship API should not be introduced here because it
//! would mix storage mutation with structural traversal.
//!
//! ## Error model
//!
//! The provider owns storage-specific errors.
//!
//! The cursor adds only traversal-local accounting errors where a caller has
//! explicitly supplied a limit.
//!
//! No error is generated merely because a node has many children.
//!
//! ## Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Safety
//!
//! This module explicitly forbids unsafe code.
//!
//! ```text
//! unsafe code = forbidden
//! ```
//!
//! ## Integration contract
//!
//! The intended integration is:
//!
//! ```text
//! concrete AST node
//!       │
//!       ▼
//! AST storage/provider
//!       │
//!       ▼
//! AstChildrenProvider
//!       │
//!       ▼
//! ChildCursor
//!       │
//!       ├── preorder.rs
//!       ├── postorder.rs
//!       ├── fold.rs
//!       └── validation
//! ```
//!
//! Existing traversal implementations that currently define their own
//! child-provider contract should migrate to this contract rather than
//! maintaining multiple incompatible child abstractions.
//!
//! That migration is mechanical:
//!
//! ```text
//! old provider::children()
//!          │
//!          ▼
//! AstChildrenProvider::children()
//!          │
//!          ▼
//! ChildCursor::new(...)
//! ```
//!
//! The concrete AST node implementations do not need to know whether the
//! consumer is performing preorder, postorder, validation, or folding.
//!
//! ## File completion contract
//!
//! Once this file is complete:
//!
//! - adding a new AST node does not require changing this file;
//! - adding a new quantum operation does not require changing this file;
//! - adding a new computational domain does not require changing this file;
//! - adding a new backend does not require changing this file;
//! - changing AST storage from a vector to an arena does not require changing
//!   this file;
//! - changing AST storage from an arena to persistent storage does not require
//!   changing this file.
//!
//! A future change to this file should therefore be required only if the
//! canonical child-enumeration contract itself changes.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::super::node_id::NodeId;

// =============================================================================
// Child provider
// =============================================================================

/// Canonical read-only provider of direct AST children.
///
/// The provider owns AST storage; this trait owns only the interface by which
/// structural consumers obtain direct child identities.
///
/// # Required invariant
///
/// [`Self::children`] MUST return children in canonical AST order.
///
/// The returned iterator MUST contain direct children only. It MUST NOT
/// recursively enumerate descendants.
///
/// # Storage independence
///
/// Implementations may store nodes in any representation, including:
///
/// - indexed vectors;
/// - arenas;
/// - persistent structures;
/// - immutable stores;
/// - incremental stores;
/// - memory-efficient custom containers.
///
/// The traversal layer must not depend on the storage representation.
///
/// # Example
///
/// A node backed by a `Vec<NodeId>` can implement the contract as:
///
/// ```ignore
/// impl AstChildrenProvider for AstStore {
///     type Error = AstStoreError;
///
///     type Children<'a>
///         = std::iter::Copied<std::slice::Iter<'a, NodeId>>
///     where
///         Self: 'a;
///
///     fn children<'a>(
///         &'a self,
///         node_id: NodeId,
///     ) -> Result<Self::Children<'a>, Self::Error> {
///         Ok(self.node(node_id)?.children.iter().copied())
///     }
/// }
/// ```
///
/// The actual AST storage implementation remains outside this module.
pub trait AstChildrenProvider {
    /// Storage/provider-specific error.
    type Error;

    /// Iterator over direct children.
    ///
    /// The GAT allows the iterator to borrow the provider without requiring
    /// heap allocation or a boxed trait object.
    type Children<'a>: Iterator<Item = NodeId> + 'a
    where
        Self: 'a;

    /// Returns the direct children of `node_id` in canonical order.
    ///
    /// Returning an error means the provider could not enumerate the requested
    /// node's children.
    fn children<'a>(
        &'a self,
        node_id: NodeId,
    ) -> Result<Self::Children<'a>, Self::Error>;
}

// =============================================================================
// Child enumeration
// =============================================================================

/// Metadata describing one direct-child enumeration operation.
///
/// This structure contains measurements only. Its values are not language
/// limits and must never be interpreted as such.
///
/// It is useful for diagnostics, metrics, testing, and traversal reporting.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ChildEnumeration {
    /// Number of child references yielded so far.
    yielded: usize,
}

impl ChildEnumeration {
    /// Creates an empty enumeration counter.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { yielded: 0 }
    }

    /// Returns the number of child references yielded.
    #[inline]
    #[must_use]
    pub const fn yielded(self) -> usize {
        self.yielded
    }

    /// Records one successfully yielded child.
    ///
    /// Returns `None` when the host `usize` representation cannot represent
    /// another count.
    #[inline]
    fn record_yield(&mut self) -> Option<()> {
        self.yielded = self.yielded.checked_add(1)?;
        Some(())
    }
}

// =============================================================================
// Cursor errors
// =============================================================================

/// Errors produced by [`ChildCursor`].
///
/// Provider errors remain owned by the provider. This error type only reports
/// cursor-local accounting failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ChildCursorError {
    /// The caller configured a maximum number of child references and that
    /// maximum has been reached.
    LimitExceeded {
        /// Number of children already yielded.
        yielded: usize,

        /// Caller-configured maximum.
        maximum: usize,
    },

    /// The host `usize` representation cannot represent another child count.
    ///
    /// This is a representational boundary, not a Zamani language limit.
    CountOverflow,
}

impl fmt::Display for ChildCursorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitExceeded { yielded, maximum } => {
                write!(
                    formatter,
                    "AST child enumeration limit exceeded: yielded {yielded}, maximum {maximum}"
                )
            }

            Self::CountOverflow => {
                formatter.write_str(
                    "AST child enumeration count exceeded host usize representation",
                )
            }
        }
    }
}

impl std::error::Error for ChildCursorError {}

// =============================================================================
// Child cursor
// =============================================================================

/// Incremental cursor over the direct children of one AST node.
///
/// `ChildCursor` intentionally does not collect children into a `Vec`.
///
/// This means a high-fan-out node can be processed incrementally:
///
/// ```text
/// child 0
/// child 1
/// child 2
/// ...
/// child N
/// ```
///
/// without first materializing:
///
/// ```text
/// Vec<NodeId>
/// ```
///
/// # Ordering
///
/// The cursor preserves the order produced by the provider.
///
/// It does not sort, reverse, deduplicate, or filter.
///
/// # Limits
///
/// `maximum` is optional.
///
/// `None` means no cursor-level limit.
///
/// This is essential for POCO-REAF because a default hard-coded maximum would
/// turn an implementation detail into an artificial language limitation.
///
/// # Complexity
///
/// `next()` is O(1) plus the provider iterator's own cost.
///
/// Additional cursor memory is O(1), excluding the provider's iterator state.
pub struct ChildCursor<I> {
    iterator: I,
    enumeration: ChildEnumeration,
    maximum: Option<usize>,
}

impl<I> ChildCursor<I>
where
    I: Iterator<Item = NodeId>,
{
    /// Creates an unlimited child cursor.
    ///
    /// "Unlimited" means that this cursor imposes no artificial child-count
    /// policy. Actual execution remains constrained by available resources and
    /// the provider.
    #[inline]
    #[must_use]
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            enumeration: ChildEnumeration::new(),
            maximum: None,
        }
    }

    /// Creates a child cursor with an explicit caller-provided limit.
    ///
    /// A limit of `Some(0)` means that no child may be yielded.
    #[inline]
    #[must_use]
    pub const fn with_maximum(
        iterator: I,
        maximum: Option<usize>,
    ) -> Self {
        Self {
            iterator,
            enumeration: ChildEnumeration::new(),
            maximum,
        }
    }

    /// Returns the number of child references yielded so far.
    #[inline]
    #[must_use]
    pub const fn yielded(&self) -> usize {
        self.enumeration.yielded()
    }

    /// Returns the configured child limit, if any.
    #[inline]
    #[must_use]
    pub const fn maximum(&self) -> Option<usize> {
        self.maximum
    }

    /// Returns the current enumeration statistics.
    #[inline]
    #[must_use]
    pub const fn enumeration(&self) -> ChildEnumeration {
        self.enumeration
    }

    /// Returns the next child.
    ///
    /// `Ok(None)` means enumeration is complete.
    ///
    /// `Err(...)` means the caller's explicit child budget or the host
    /// representation prevented another child from being yielded.
    ///
    /// Provider errors are not generated here because the provider's iterator
    /// contract is intentionally a normal Rust `Iterator`. Providers that can
    /// fail while producing children should encode that failure at the
    /// provider boundary before constructing the cursor, or use a dedicated
    /// fallible iterator adapter in their storage layer.
    pub fn next_child(
        &mut self,
    ) -> Result<Option<NodeId>, ChildCursorError> {
        if let Some(maximum) = self.maximum {
            if self.enumeration.yielded() >= maximum {
                return Ok(None);
            }
        }

        match self.iterator.next() {
            Some(child) => {
                self.enumeration
                    .record_yield()
                    .ok_or(ChildCursorError::CountOverflow)?;

                Ok(Some(child))
            }

            None => Ok(None),
        }
    }

    /// Returns the underlying iterator.
    ///
    /// This is useful when a traversal engine needs to integrate the cursor
    /// with another iterator abstraction.
    ///
    /// The returned iterator begins at the cursor's current position.
    #[inline]
    pub fn into_inner(self) -> I {
        self.iterator
    }
}

impl<I> Iterator for ChildCursor<I>
where
    I: Iterator<Item = NodeId>,
{
    type Item = Result<NodeId, ChildCursorError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_child() {
            Ok(Some(child)) => Some(Ok(child)),
            Ok(None) => None,
            Err(error) => Some(Err(error)),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iterator.size_hint();

        match self.maximum {
            None => (lower, upper),

            Some(maximum) => {
                let remaining = maximum.saturating_sub(self.enumeration.yielded());

                let lower = lower.min(remaining);
                let upper = upper.map(|value| value.min(remaining));

                (lower, upper)
            }
        }
    }
}

// =============================================================================
// Helper functions
// =============================================================================

/// Creates an unlimited child cursor from an iterator.
///
/// This is the preferred concise construction for traversal implementations.
///
/// ```ignore
/// let mut children = child_cursor(provider.children(node_id)?);
///
/// while let Some(child) = children.next()? {
///     // process child
/// }
/// ```
#[inline]
#[must_use]
pub fn child_cursor<I>(
    iterator: I,
) -> ChildCursor<I>
where
    I: Iterator<Item = NodeId>,
{
    ChildCursor::new(iterator)
}

/// Creates a child cursor with an explicit caller-provided maximum.
///
/// The maximum is an operational safety policy. It is not a language limit.
#[inline]
#[must_use]
pub fn child_cursor_with_maximum<I>(
    iterator: I,
    maximum: Option<usize>,
) -> ChildCursor<I>
where
    I: Iterator<Item = NodeId>,
{
    ChildCursor::with_maximum(iterator, maximum)
}

/// Returns an empty child iterator.
///
/// This helper is useful for leaf nodes and storage adapters.
///
/// It is intentionally allocation-free.
#[inline]
pub fn no_children() -> std::iter::Empty<NodeId> {
    std::iter::empty()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> NodeId {
        NodeId::from_u64(value)
    }

    #[test]
    fn empty_children_are_empty() {
        let mut cursor = ChildCursor::new(no_children());

        assert_eq!(cursor.next_child(), Ok(None));
        assert_eq!(cursor.yielded(), 0);
    }

    #[test]
    fn cursor_preserves_provider_order() {
        let children = [
            id(1),
            id(2),
            id(3),
            id(4),
        ];

        let mut cursor = ChildCursor::new(children.into_iter());

        assert_eq!(cursor.next_child(), Ok(Some(id(1))));
        assert_eq!(cursor.next_child(), Ok(Some(id(2))));
        assert_eq!(cursor.next_child(), Ok(Some(id(3))));
        assert_eq!(cursor.next_child(), Ok(Some(id(4))));
        assert_eq!(cursor.next_child(), Ok(None));

        assert_eq!(cursor.yielded(), 4);
    }

    #[test]
    fn cursor_does_not_deduplicate_children() {
        let children = [
            id(7),
            id(7),
            id(9),
        ];

        let collected: Vec<_> = ChildCursor::new(children.into_iter())
            .map(|result| result.expect("cursor must succeed"))
            .collect();

        assert_eq!(
            collected,
            vec![id(7), id(7), id(9)]
        );
    }

    #[test]
    fn cursor_can_be_bounded_by_caller_policy() {
        let children = [
            id(1),
            id(2),
            id(3),
        ];

        let mut cursor =
            ChildCursor::with_maximum(children.into_iter(), Some(2));

        assert_eq!(cursor.next_child(), Ok(Some(id(1))));
        assert_eq!(cursor.next_child(), Ok(Some(id(2))));

        // Reaching the caller-provided boundary ends enumeration without
        // inventing a language-level error.
        assert_eq!(cursor.next_child(), Ok(None));
        assert_eq!(cursor.yielded(), 2);
    }

    #[test]
    fn unlimited_cursor_has_no_artificial_limit() {
        let children = (0_u64..10_000).map(NodeId::from_u64);

        let mut cursor = ChildCursor::new(children);

        for expected in 0_u64..10_000 {
            assert_eq!(
                cursor.next_child(),
                Ok(Some(id(expected)))
            );
        }

        assert_eq!(cursor.next_child(), Ok(None));
        assert_eq!(cursor.yielded(), 10_000);
    }

    #[test]
    fn iterator_adapter_preserves_results() {
        let children = [
            id(10),
            id(20),
            id(30),
        ];

        let collected: Vec<_> =
            ChildCursor::new(children.into_iter()).collect();

        assert_eq!(
            collected,
            vec![
                Ok(id(10)),
                Ok(id(20)),
                Ok(id(30)),
            ]
        );
    }

    #[test]
    fn size_hint_respects_explicit_limit() {
        let children = [id(1), id(2), id(3), id(4)];

        let mut cursor =
            ChildCursor::with_maximum(children.into_iter(), Some(2));

        assert_eq!(cursor.size_hint(), (2, Some(2)));

        assert_eq!(cursor.next_child(), Ok(Some(id(1))));
        assert_eq!(cursor.size_hint(), (1, Some(1)));

        assert_eq!(cursor.next_child(), Ok(Some(id(2))));
        assert_eq!(cursor.size_hint(), (0, Some(0)));

        assert_eq!(cursor.next_child(), Ok(None));
    }
}