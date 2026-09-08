//! Stable identity infrastructure for the native Zamani frontend AST.
//!
//! # Architectural role
//!
//! `NodeId` identifies a node in the native Zamani AST.  It is deliberately
//! independent of:
//!
//! - source-file paths;
//! - byte offsets;
//! - memory addresses;
//! - parser implementation details;
//! - semantic-analysis identities;
//! - ZUIR identities;
//! - quantum IR identities;
//! - hardware identities;
//! - backend identities;
//! - operating-system process IDs;
//! - thread IDs.
//!
//! The AST owns syntax identity.  Later compiler stages may maintain side
//! tables keyed by `NodeId`, but `NodeId` itself does not know anything about
//! those stages.
//!
//! # Design goals
//!
//! This module provides:
//!
//! - deterministic IDs;
//! - no global mutable state;
//! - no randomness;
//! - no unsafe code;
//! - no pointer-derived identity;
//! - no fixed maximum AST size imposed by the language;
//! - explicit allocation ownership;
//! - stable equality and hashing;
//! - compact representation;
//! - serialization-friendly representation;
//! - deterministic construction;
//! - support for generated/desugared AST nodes;
//! - support for independently constructed AST fragments;
//! - safe checked allocation;
//! - compatibility with Rust 1.97 / 1.97.1.
//!
//! # Important scalability property
//!
//! `NodeId` does **not** represent the number of qubits, registers, machine
//! cores, hardware devices, or any other computational resource.  It is only
//! an identity for an AST node.
//!
//! There is intentionally no `MAX_NODE_ID`, `MAX_AST_NODES`, `MAX_QUBITS`, or
//! similar language-level limit in this module.
//!
//! A compiler may impose configurable resource limits elsewhere for denial-of-
//! service protection or operational policy.  Such limits must never be
//! encoded as semantic limits by this type.
//!
//! # Determinism
//!
//! The allocator starts from an explicit caller-selected origin and advances
//! monotonically.  Therefore two AST constructions that consume the same
//! sequence of allocations produce the same IDs.
//!
//! There is no process-global counter.  This is important for:
//!
//! - reproducible builds;
//! - parallel compilation;
//! - compiler servers;
//! - incremental compilation;
//! - testing;
//! - deterministic serialization.
//!
//! # Ownership
//!
//! `NodeId` is a value.  `NodeIdAllocator` owns allocation state.
//!
//! AST nodes should normally obtain their IDs from an allocator owned by the
//! AST builder/parser/program-construction layer.
//!
//! The allocator must not be hidden inside `NodeId`, because doing so would
//! introduce global mutable state and make deterministic parallel compilation
//! substantially harder.
//!
//! # Integration contract
//!
//! ```text
//! source/span.rs
//!       │
//!       ├───────────────┐
//!       ▼               ▼
//! node/node_id.rs   node/metadata.rs
//!       │               │
//!       └──────┬────────┘
//!              ▼
//!          node/node.rs
//!              │
//!       ┌──────┼───────────────┐
//!       ▼      ▼               ▼
//!   builders traversal     validation
//!       │      │               │
//!       └──────┼───────────────┘
//!              ▼
//!          parser / AST
//!              │
//!              ▼
//!          semantic analysis
//!              │
//!              ▼
//!             ZUIR
//! ```
//!
//! `NodeId` must never introduce a dependency in the reverse direction.
//!
//! # Semantic boundary
//!
//! A `NodeId` is not:
//!
//! - a symbol ID;
//! - a type ID;
//! - a resource ID;
//! - a quantum-qubit ID;
//! - a quantum operation ID;
//! - a ZUIR value ID;
//! - a backend job ID.
//!
//! Those identities belong to their respective compilation layers.
//!
//! # Serialization
//!
//! `NodeId` deliberately uses a primitive integer representation so that it
//! can be serialized by the enclosing AST serialization layer without
//! requiring this module to depend on a particular serialization framework.
//!
//! The AST serialization layer is responsible for recording its schema
//! version and validating serialized IDs.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! This file uses only stable standard-library facilities and contains no
//! `unsafe` code.
//! ```

use core::fmt;
use core::num::NonZeroU64;
use core::str::FromStr;

/// The first valid AST node identity.
///
/// Zero is deliberately reserved as an invalid/unassigned value.
///
/// Starting at one also makes accidental zero-initialization distinguishable
/// from a legitimately allocated node.
const FIRST_NODE_ID: u64 = 1;

/// Stable identity of a node in the native Zamani frontend AST.
///
/// `NodeId` is intentionally opaque.  Consumers should compare, hash, store,
/// and serialize it, but should not attach semantic meaning to the numeric
/// value.
///
/// The internal `NonZeroU64` representation provides a compact representation
/// while ensuring that a valid `NodeId` can never contain zero.
///
/// # Stability
///
/// A `NodeId` is stable for the lifetime of the AST graph in which it was
/// allocated.  It is not globally stable across unrelated compiler
/// invocations unless the same deterministic construction order is used.
///
/// Persistent cross-compilation identities belong to a separate layer and
/// should not be conflated with AST node identity.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(NonZeroU64);

impl NodeId {
    /// Creates a `NodeId` from a positive integer.
    ///
    /// Returns `None` for zero because zero is reserved as the invalid/unset
    /// representation.
    #[inline]
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates the first valid node ID.
    #[inline]
    pub const fn first() -> Self {
        // SAFETY: FIRST_NODE_ID is a compile-time constant equal to one.
        //
        // This implementation deliberately does not use `unsafe`; the value
        // is constructed through `NonZeroU64::new` and matched explicitly.
        match NonZeroU64::new(FIRST_NODE_ID) {
            Some(value) => Self(value),
            None => unreachable!(),
        }
    }

    /// Returns the numeric representation of this node ID.
    ///
    /// The numeric representation is useful for:
    ///
    /// - serialization;
    /// - deterministic ordering;
    /// - diagnostics/debugging;
    /// - compact side-table indexing.
    ///
    /// It must not be interpreted as a memory address or hardware identity.
    #[inline]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Returns whether this ID is the first valid AST node ID.
    #[inline]
    pub const fn is_first(self) -> bool {
        self.get() == FIRST_NODE_ID
    }

    /// Returns the next representable ID.
    ///
    /// This is a checked operation.  At the theoretical `u64` boundary there
    /// is no representable successor, so the function returns `None`.
    ///
    /// Production compilers should treat exhaustion as an allocation failure,
    /// not wrap around and silently reuse an existing identity.
    #[inline]
    pub const fn checked_next(self) -> Option<Self> {
        match self.get().checked_add(1) {
            Some(value) => match NonZeroU64::new(value) {
                Some(value) => Some(Self(value)),
                None => None,
            },
            None => None,
        }
    }

    /// Returns the number of the ID as a `usize` when it can be represented
    /// by the host index type.
    ///
    /// This conversion is intentionally checked.  The AST identity itself is
    /// `u64` and therefore does not inherit the address-space width of the
    /// compilation host.
    #[inline]
    pub fn try_as_usize(self) -> Option<usize> {
        usize::try_from(self.get()).ok()
    }
}

impl Default for NodeId {
    /// Produces the first valid ID.
    ///
    /// `Default` is intentionally deterministic.  It does not allocate from
    /// hidden global state.
    #[inline]
    fn default() -> Self {
        Self::first()
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("NodeId")
            .field(&self.get())
            .finish()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl From<NodeId> for u64 {
    #[inline]
    fn from(value: NodeId) -> Self {
        value.get()
    }
}

impl TryFrom<u64> for NodeId {
    type Error = NodeIdParseError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(NodeIdParseError::Zero)
    }
}

impl FromStr for NodeId {
    type Err = NodeIdParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value
            .parse::<u64>()
            .map_err(NodeIdParseError::InvalidInteger)?;

        Self::try_from(value)
    }
}

/// Errors produced while parsing or constructing a [`NodeId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeIdParseError {
    /// Zero is reserved and cannot represent an AST node.
    Zero,

    /// The supplied textual representation was not a valid `u64`.
    InvalidInteger(core::num::ParseIntError),
}

impl fmt::Display for NodeIdParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(formatter, "AST node ID must be greater than zero"),
            Self::InvalidInteger(error) => {
                write!(formatter, "invalid AST node ID: {error}")
            }
        }
    }
}

impl std::error::Error for NodeIdParseError {}

/// Error returned when a [`NodeIdAllocator`] cannot allocate another ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeIdAllocationError {
    /// The allocator has reached the maximum representable `u64` identity.
    Exhausted,
}

impl fmt::Display for NodeIdAllocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exhausted => {
                write!(formatter, "AST node ID space exhausted")
            }
        }
    }
}

impl std::error::Error for NodeIdAllocationError {}

/// Deterministic allocator for native Zamani AST node IDs.
///
/// The allocator is intentionally an ordinary value rather than a singleton
/// or global service.
///
/// This gives the compiler control over:
///
/// - determinism;
/// - parallel compilation;
/// - incremental compilation;
/// - generated-node allocation;
/// - testing;
/// - AST fragment construction.
///
/// # Example
///
/// ```
/// use zamani::frontend::ast::node::node_id::NodeIdAllocator;
///
/// let mut allocator = NodeIdAllocator::new();
///
/// let first = allocator.allocate().expect("first ID");
/// let second = allocator.allocate().expect("second ID");
///
/// assert_eq!(first.get(), 1);
/// assert_eq!(second.get(), 2);
/// ```
///
/// The actual crate path in the repository may differ during migration.  The
/// type itself deliberately has no dependency on the enclosing crate layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeIdAllocator {
    next: u64,
}

impl NodeIdAllocator {
    /// Creates a deterministic allocator beginning with [`NodeId::first`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            next: FIRST_NODE_ID,
        }
    }

    /// Creates an allocator whose first allocation is `first`.
    ///
    /// Zero is rejected because it is reserved.
    ///
    /// This constructor is useful for deterministic construction of
    /// independently allocated AST fragments.
    #[inline]
    pub fn starting_at(first: NodeId) -> Self {
        Self {
            next: first.get(),
        }
    }

    /// Creates an allocator from a raw starting value.
    ///
    /// This is the checked counterpart to `starting_at` and is convenient for
    /// deserialization or compiler infrastructure that stores allocator state
    /// as an integer.
    #[inline]
    pub fn try_starting_at(first: u64) -> Result<Self, NodeIdParseError> {
        NodeId::try_from(first).map(Self::starting_at)
    }

    /// Allocates the next deterministic node ID.
    ///
    /// Allocation is monotonic and never wraps.
    #[inline]
    pub fn allocate(&mut self) -> Result<NodeId, NodeIdAllocationError> {
        let id = NodeId::new(self.next).ok_or(NodeIdAllocationError::Exhausted)?;

        self.next = match self.next.checked_add(1) {
            Some(next) => next,
            None => u64::MAX,
        };

        Ok(id)
    }

    /// Allocates a contiguous range of node IDs.
    ///
    /// The returned range contains exactly `count` IDs.
    ///
    /// This is useful for builders that know in advance that a generated
    /// construct will create multiple nodes.
    ///
    /// The operation is atomic with respect to this allocator: if the entire
    /// requested range cannot be represented, the allocator is left unchanged.
    #[inline]
    pub fn allocate_range(
        &mut self,
        count: u64,
    ) -> Result<NodeIdRange, NodeIdAllocationError> {
        if count == 0 {
            return Ok(NodeIdRange::empty());
        }

        let start = NodeId::new(self.next).ok_or(NodeIdAllocationError::Exhausted)?;

        let end_value = self
            .next
            .checked_add(count - 1)
            .ok_or(NodeIdAllocationError::Exhausted)?;

        // The next allocation after the requested range must also be
        // representable.  If the range ends at u64::MAX, this allocator is
        // exhausted after the range, which is a valid state.
        self.next = end_value
            .checked_add(1)
            .unwrap_or(u64::MAX);

        Ok(NodeIdRange {
            start,
            end: NodeId::new(end_value).ok_or(NodeIdAllocationError::Exhausted)?,
            count,
        })
    }

    /// Returns the ID that would be allocated next without allocating it.
    ///
    /// Returns `None` after exhaustion.
    #[inline]
    pub fn peek(&self) -> Option<NodeId> {
        NodeId::new(self.next)
    }

    /// Returns the number of IDs already allocated when the allocator began
    /// at [`NodeId::first`].
    ///
    /// This is primarily useful for diagnostics and tests.
    ///
    /// The result is checked because the allocator can represent the terminal
    /// exhausted state.
    #[inline]
    pub fn allocated_count(&self) -> u64 {
        self.next.saturating_sub(FIRST_NODE_ID)
    }

    /// Returns the allocator's next raw numeric value.
    ///
    /// This method exists for deterministic serialization of compiler
    /// construction state.  The surrounding serialization layer must version
    /// that representation.
    #[inline]
    pub const fn next_raw(&self) -> u64 {
        self.next
    }
}

impl Default for NodeIdAllocator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// A contiguous range of AST node IDs.
///
/// The range is represented by its inclusive endpoints and its cardinality.
/// It does not materialize every ID, so allocating a large range does not
/// itself require memory proportional to the number of nodes.
///
/// This is useful for scalable AST builders and generated constructs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeIdRange {
    start: NodeId,
    end: NodeId,
    count: u64,
}

impl NodeIdRange {
    const fn empty() -> Self {
        Self {
            start: NodeId::first(),
            end: NodeId::first(),
            count: 0,
        }
    }

    /// Returns the first ID in the range.
    ///
    /// Returns `None` for an empty range.
    #[inline]
    pub const fn start(&self) -> Option<NodeId> {
        if self.count == 0 {
            None
        } else {
            Some(self.start)
        }
    }

    /// Returns the final ID in the range.
    ///
    /// Returns `None` for an empty range.
    #[inline]
    pub const fn end(&self) -> Option<NodeId> {
        if self.count == 0 {
            None
        } else {
            Some(self.end)
        }
    }

    /// Returns the number of IDs represented by the range.
    #[inline]
    pub const fn len(&self) -> u64 {
        self.count
    }

    /// Returns whether the range contains no IDs.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns an iterator over the IDs in this range.
    ///
    /// The iterator is lazy and therefore does not allocate storage
    /// proportional to the range size.
    #[inline]
    pub fn iter(&self) -> NodeIdRangeIter {
        NodeIdRangeIter {
            next: self.start(),
            remaining: self.count,
        }
    }
}

/// Lazy iterator over a [`NodeIdRange`].
#[derive(Debug, Clone)]
pub struct NodeIdRangeIter {
    next: Option<NodeId>,
    remaining: u64,
}

impl Iterator for NodeIdRangeIter {
    type Item = NodeId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            self.next = None;
            return None;
        }

        let current = self.next?;

        self.remaining -= 1;

        if self.remaining == 0 {
            self.next = None;
        } else {
            self.next = current.checked_next();
        }

        Some(current)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining;

        match usize::try_from(remaining) {
            Ok(value) => (value, Some(value)),
            Err(_) => (usize::MAX, None),
        }
    }
}

impl ExactSizeIterator for NodeIdRangeIter {
    #[inline]
    fn len(&self) -> usize {
        usize::try_from(self.remaining).unwrap_or(usize::MAX)
    }
}

impl std::iter::FusedIterator for NodeIdRangeIter {}

/// A deterministic allocator checkpoint.
///
/// Checkpoints allow a builder to reserve a logical allocation position and
/// later restore it when an operation is abandoned during parser recovery or
/// speculative AST construction.
///
/// Restoration is explicit and therefore cannot silently mutate unrelated
/// allocator state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeIdCheckpoint {
    next: u64,
}

impl NodeIdCheckpoint {
    /// Returns the raw allocator position represented by this checkpoint.
    #[inline]
    pub const fn next_raw(self) -> u64 {
        self.next
    }
}

impl NodeIdAllocator {
    /// Creates a checkpoint of the current allocation state.
    #[inline]
    pub const fn checkpoint(&self) -> NodeIdCheckpoint {
        NodeIdCheckpoint { next: self.next }
    }

    /// Restores an earlier checkpoint.
    ///
    /// The caller is responsible for ensuring that no externally retained AST
    /// node depends on IDs allocated after the checkpoint.
    ///
    /// This method is intentionally explicit because reusing IDs that are
    /// already present in a live AST would violate AST identity uniqueness.
    #[inline]
    pub fn restore(&mut self, checkpoint: NodeIdCheckpoint) {
        self.next = checkpoint.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_not_a_valid_node_id() {
        assert_eq!(NodeId::new(0), None);
        assert!(matches!(
            NodeId::try_from(0),
            Err(NodeIdParseError::Zero)
        ));
    }

    #[test]
    fn first_id_is_one() {
        let id = NodeId::first();

        assert_eq!(id.get(), 1);
        assert!(id.is_first());
    }

    #[test]
    fn IDs_are_deterministic() {
        let mut left = NodeIdAllocator::new();
        let mut right = NodeIdAllocator::new();

        for _ in 0..1024 {
            assert_eq!(left.allocate(), right.allocate());
        }
    }

    #[test]
    fn IDs_are_monotonic() {
        let mut allocator = NodeIdAllocator::new();

        let first = allocator.allocate().expect("first ID");
        let second = allocator.allocate().expect("second ID");
        let third = allocator.allocate().expect("third ID");

        assert!(first < second);
        assert!(second < third);
        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), 2);
        assert_eq!(third.get(), 3);
    }

    #[test]
    fn allocator_has_no_hidden_global_state() {
        let mut first = NodeIdAllocator::new();
        let mut second = NodeIdAllocator::new();

        assert_eq!(first.allocate().expect("first").get(), 1);
        assert_eq!(second.allocate().expect("second").get(), 1);

        assert_eq!(first.allocate().expect("first").get(), 2);
        assert_eq!(second.allocate().expect("second").get(), 2);
    }

    #[test]
    fn allocator_can_start_at_explicit_id() {
        let start = NodeId::new(100).expect("valid ID");
        let mut allocator = NodeIdAllocator::starting_at(start);

        assert_eq!(allocator.allocate().expect("ID").get(), 100);
        assert_eq!(allocator.allocate().expect("ID").get(), 101);
    }

    #[test]
    fn allocator_rejects_zero_start() {
        assert!(matches!(
            NodeIdAllocator::try_starting_at(0),
            Err(NodeIdParseError::Zero)
        ));
    }

    #[test]
    fn peek_does_not_allocate() {
        let allocator = NodeIdAllocator::new();

        assert_eq!(allocator.peek().expect("peek").get(), 1);
        assert_eq!(allocator.peek().expect("peek").get(), 1);
    }

    #[test]
    fn allocated_count_is_deterministic() {
        let mut allocator = NodeIdAllocator::new();

        assert_eq!(allocator.allocated_count(), 0);

        allocator.allocate().expect("ID");
        assert_eq!(allocator.allocated_count(), 1);

        allocator.allocate().expect("ID");
        assert_eq!(allocator.allocated_count(), 2);
    }

    #[test]
    fn raw_conversion_round_trips() {
        let original = NodeId::new(42).expect("valid ID");
        let raw: u64 = original.into();
        let reconstructed = NodeId::try_from(raw).expect("valid ID");

        assert_eq!(original, reconstructed);
    }

    #[test]
    fn display_is_stable() {
        let id = NodeId::new(42).expect("valid ID");

        assert_eq!(id.to_string(), "42");
        assert_eq!("42".parse::<NodeId>().expect("valid ID"), id);
    }

    #[test]
    fn zero_text_is_rejected() {
        assert!(matches!(
            "0".parse::<NodeId>(),
            Err(NodeIdParseError::Zero)
        ));
    }

    #[test]
    fn invalid_text_is_rejected() {
        assert!(matches!(
            "not-a-node-id".parse::<NodeId>(),
            Err(NodeIdParseError::InvalidInteger(_))
        ));
    }

    #[test]
    fn checked_next_is_safe() {
        let id = NodeId::new(u64::MAX - 1).expect("valid ID");

        assert_eq!(id.checked_next().expect("next").get(), u64::MAX);
        assert_eq!(NodeId::new(u64::MAX).expect("valid ID").checked_next(), None);
    }

    #[test]
    fn range_allocation_is_contiguous() {
        let mut allocator = NodeIdAllocator::new();

        let range = allocator.allocate_range(5).expect("range");

        assert_eq!(range.len(), 5);
        assert_eq!(range.start().expect("start").get(), 1);
        assert_eq!(range.end().expect("end").get(), 5);

        let ids: Vec<u64> = range.iter().map(NodeId::get).collect();

        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
        assert_eq!(allocator.allocate().expect("next").get(), 6);
    }

    #[test]
    fn zero_length_range_is_empty() {
        let mut allocator = NodeIdAllocator::new();

        let range = allocator.allocate_range(0).expect("empty range");

        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
        assert_eq!(range.start(), None);
        assert_eq!(range.end(), None);

        // A zero-length allocation must not consume an ID.
        assert_eq!(allocator.allocate().expect("first").get(), 1);
    }

    #[test]
    fn large_ranges_are_lazy() {
        let mut allocator = NodeIdAllocator::new();

        let range = allocator.allocate_range(1_000_000).expect("large range");

        assert_eq!(range.len(), 1_000_000);

        let mut iterator = range.iter();

        assert_eq!(iterator.next().expect("first").get(), 1);
        assert_eq!(iterator.next().expect("second").get(), 2);

        // The range itself does not materialize one million NodeId values.
        assert_eq!(range.len(), 1_000_000);
    }

    #[test]
    fn checkpoint_and_restore_are_deterministic() {
        let mut allocator = NodeIdAllocator::new();

        let first = allocator.allocate().expect("first");
        let checkpoint = allocator.checkpoint();

        let second = allocator.allocate().expect("second");
        let third = allocator.allocate().expect("third");

        allocator.restore(checkpoint);

        let second_again = allocator.allocate().expect("second again");
        let third_again = allocator.allocate().expect("third again");

        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), second_again.get());
        assert_eq!(third.get(), third_again.get());
    }

    #[test]
    fn allocator_state_is_cloneable() {
        let mut original = NodeIdAllocator::new();

        original.allocate().expect("first");

        let mut cloned = original.clone();

        assert_eq!(
            original.allocate().expect("original").get(),
            cloned.allocate().expect("clone").get()
        );
    }

    #[test]
    fn node_ids_are_hashable() {
        use std::collections::HashSet;

        let first = NodeId::new(1).expect("ID");
        let second = NodeId::new(2).expect("ID");

        let mut ids = HashSet::new();
        ids.insert(first);
        ids.insert(second);

        assert!(ids.contains(&first));
        assert!(ids.contains(&second));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn node_ids_have_stable_ordering() {
        let first = NodeId::new(1).expect("ID");
        let second = NodeId::new(2).expect("ID");

        assert!(first < second);
        assert!(second > first);
    }

    #[test]
    fn no_unsafe_code_is_required() {
        // This test documents an architectural requirement rather than
        // testing compiler internals.
        //
        // The module intentionally constructs NonZeroU64 values through
        // checked constructors and contains no unsafe blocks.
        assert_eq!(NodeId::first().get(), 1);
    }
}