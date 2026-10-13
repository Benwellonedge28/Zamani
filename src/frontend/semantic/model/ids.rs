
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Identity Infrastructure
//!
//! Path: `src/frontend/semantic/model/ids.rs`
//! Status: Proposed implementation; integration and workspace verification
//! are required before this file can be marked frozen.
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Define strongly typed identities and explicit deterministic allocators for
//! entities owned by the semantic model.
//!
//! ## AUTHORITY
//!
//! This module follows the semantic-model and grammar-authority specifications
//! under `grammar/specification/`, and the identity boundary established by
//! `src/frontend/ast/node/node_id.rs`.
//!
//! ## OWNS
//!
//! - Semantic identity newtypes.
//! - Checked conversion between raw numeric IDs and semantic IDs.
//! - Explicit, deterministic, per-owner ID allocation.
//! - Checked allocator exhaustion.
//!
//! ## DOES_NOT_OWN
//!
//! - AST node identities (`NodeId`).
//! - Symbol tables, scopes, types, or semantic facts themselves.
//! - Source spans, diagnostics, provenance payloads, or AST traversal.
//! - Target, hardware, runtime, IR, or device identities.
//! - Global ID allocation or cross-compilation persistent identity.
//!
//! ## INTEGRATION CONTRACT
//!
//! `model/ids.rs` is a foundational semantic-model module. It may depend only
//! on the Rust standard library. Higher-level semantic modules may import its
//! IDs; this module must never import those higher-level modules.
//!
//! `NodeId` and every semantic ID are distinct types. A conversion between
//! them must not be implicit. An AST node can be referenced by `NodeId` while
//! a declaration, type, obligation, or fact receives its own semantic ID.
//!
//! ## SCALABILITY AND SAFETY
//!
//! IDs use non-zero `u64` values. Zero is reserved and rejected. There is no
//! language-level or hardware-capacity limit in this module. Allocation
//! exhaustion is reported explicitly, and an identity is never reused by
//! wrapping the counter.
//!
//! The finite numeric representation is an implementation representation
//! limit, not a limit on the number of qubits, devices, registers, tensor
//! dimensions, or other resources a Zamani program may describe.
//!
//! Each allocator is an ordinary value owned by its caller. Independent
//! allocators may issue equal numeric values; IDs are unique within the
//! allocation domain governed by their owner. Callers must not merge IDs from
//! independent domains without an explicit remapping or namespace policy.
//!
//! IDs are deterministic for a given allocator origin and allocation order.
//! They are not promised to remain stable across unrelated compilations,
//! changed traversal orders, or changed source programs.
//!
//! ## FREEZE CRITERIA
//!
//! - Public ID types are distinct and checked.
//! - Allocators have no global state and never wrap.
//! - Exhaustion is an explicit error.
//! - The allocator can start at a checked, caller-selected ID.
//! - Tests cover parsing, ordering, determinism, exhaustion, and type separation.
//! - `cargo fmt`, relevant tests, and workspace checks pass after integration.

use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU64;
use core::str::FromStr;

/// The first numeric value permitted for a semantic identity.
///
/// This is an identity-format invariant, not a program-size or hardware limit.
const FIRST_SEMANTIC_ID: u64 = 1;

/// Common behavior required by the generic semantic ID allocator.
///
/// Implementations should normally be the built-in ID newtypes declared in
/// this module. External implementations must preserve the invariant that an
/// ID contains a non-zero integer and that `get()` returns that integer.
pub trait SemanticIdValue:
    Copy + Eq + Ord + core::hash::Hash + fmt::Debug + fmt::Display
{
    /// Construct an ID from an already validated non-zero integer.
    #[doc(hidden)]
    fn from_nonzero(value: NonZeroU64) -> Self;

    /// Return the integer representation of this ID.
    fn get(self) -> u64;

    /// Construct an ID from a raw integer, rejecting zero.
    #[inline]
    fn from_raw(value: u64) -> Result<Self, SemanticIdParseError> {
        NonZeroU64::new(value)
            .map(Self::from_nonzero)
            .ok_or(SemanticIdParseError::Zero)
    }

    /// Whether this ID has the first permitted numeric value.
    #[inline]
    fn is_first(self) -> bool {
        self.get() == FIRST_SEMANTIC_ID
    }
}

/// Declare a distinct semantic ID type without duplicating its invariants.
macro_rules! define_semantic_id {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        ///
        /// This is a semantic-layer identity, not an AST node ID, physical
        /// resource ID, runtime handle, or persistent cross-build identifier.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(NonZeroU64);

        impl $name {
            /// Create an ID from a non-zero integer.
            ///
            /// Returns `None` for zero.
            #[inline]
            pub const fn new(value: u64) -> Option<Self> {
                match NonZeroU64::new(value) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Return the raw numeric representation.
            #[inline]
            pub const fn get(self) -> u64 {
                self.0.get()
            }

            /// Return whether this is the first permitted ID value.
            #[inline]
            pub const fn is_first(self) -> bool {
                self.get() == FIRST_SEMANTIC_ID
            }

            /// Convert to a host index when representable.
            ///
            /// This checked conversion must be used before indexing a
            /// collection with the numeric identity.
            #[inline]
            pub fn try_as_usize(self) -> Option<usize> {
                usize::try_from(self.get()).ok()
            }
        }

        impl SemanticIdValue for $name {
            #[inline]
            fn from_nonzero(value: NonZeroU64) -> Self {
                Self(value)
            }

            #[inline]
            fn get(self) -> u64 {
                self.get()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.get())
                    .finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.get())
            }
        }

        impl From<$name> for u64 {
            #[inline]
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl TryFrom<u64> for $name {
            type Error = SemanticIdParseError;

            #[inline]
            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Self::from_raw(value)
            }
        }

        impl FromStr for $name {
            type Err = SemanticIdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let raw = value
                    .parse::<u64>()
                    .map_err(SemanticIdParseError::InvalidInteger)?;
                Self::from_raw(raw)
            }
        }
    };
}

define_semantic_id!(
    SemanticId,
    "General-purpose identity for a semantic-model entity that does not need a more specific identity type."
);

define_semantic_id!(
    SymbolId,
    "Identity of a semantic symbol or declaration record."
);

define_semantic_id!(
    ScopeId,
    "Identity of a semantic scope."
);

define_semantic_id!(
    TypeId,
    "Identity of an interned or otherwise explicitly identified semantic type."
);

define_semantic_id!(
    ModuleId,
    "Identity of a module in the semantic model."
);

define_semantic_id!(
    FactId,
    "Identity of a semantic fact when facts require identity independent of their AST owner."
);

define_semantic_id!(
    ObligationId,
    "Identity of a semantic obligation, such as a type, effect, contract, or constraint obligation."
);

define_semantic_id!(
    CapabilityId,
    "Identity of a capability definition or registered capability record."
);

define_semantic_id!(
    ResourceId,
    "Identity of a semantic resource definition or resource record, not a physical allocation."
);

define_semantic_id!(
    ContractId,
    "Identity of a semantic contract definition or contract record."
);

define_semantic_id!(
    ProvenanceId,
    "Identity of a provenance record in the semantic model."
);

define_semantic_id!(
    ExtensionId,
    "Identity of a registered semantic extension within its documented registry scope."
);

/// Error returned when a semantic ID is parsed or constructed incorrectly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticIdParseError {
    /// Zero is reserved and cannot represent an assigned identity.
    Zero,

    /// The textual representation is not a valid unsigned 64-bit integer.
    InvalidInteger(core::num::ParseIntError),
}

impl fmt::Display for SemanticIdParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(formatter, "semantic ID must be greater than zero"),
            Self::InvalidInteger(error) => {
                write!(formatter, "invalid semantic ID: {error}")
            }
        }
    }
}

impl std::error::Error for SemanticIdParseError {}

/// Error returned when an allocator cannot issue another identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticIdAllocationError {
    /// The numeric identity space for this allocator has been exhausted.
    Exhausted,

    /// The allocator's accounting representation cannot be advanced.
    ///
    /// This is defensive: with the current `u64` identity representation and
    /// a valid starting ID, ordinary allocation should exhaust the ID space
    /// no later than this counter.
    AllocationCountExhausted,
}

impl fmt::Display for SemanticIdAllocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exhausted => write!(formatter, "semantic ID space exhausted"),
            Self::AllocationCountExhausted => {
                write!(formatter, "semantic ID allocation counter exhausted")
            }
        }
    }
}

impl std::error::Error for SemanticIdAllocationError {}

/// Explicit deterministic allocator for one semantic ID namespace.
///
/// Each allocator owns its own sequence. Use one allocator per identity
/// category or registry scope. Numeric equality across independent allocators
/// does not imply identity equality because the ID types and owner scopes are
/// part of the identity contract.
///
/// The allocator stores no allocated-ID collection, so its bookkeeping memory
/// is constant with respect to the number of IDs issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdAllocator<I: SemanticIdValue> {
    /// The next ID to issue, or `None` after issuing the maximum value.
    next: Option<NonZeroU64>,

    /// Number of IDs issued by this allocator.
    allocated_count: u64,

    /// Makes the ID namespace part of the allocator's static type.
    marker: PhantomData<fn() -> I>,
}

impl<I: SemanticIdValue> SemanticIdAllocator<I> {
    /// Create an allocator starting at the first valid ID.
    #[inline]
    pub const fn new() -> Self {
        Self {
            next: NonZeroU64::new(FIRST_SEMANTIC_ID),
            allocated_count: 0,
            marker: PhantomData,
        }
    }

    /// Create an allocator whose first result is `first`.
    ///
    /// This is useful for deterministic, separately constructed fragments.
    /// The caller remains responsible for ensuring that separately allocated
    /// ranges do not collide when their results are combined.
    #[inline]
    pub fn starting_at(first: I) -> Self {
        Self {
            next: NonZeroU64::new(first.get()),
            allocated_count: 0,
            marker: PhantomData,
        }
    }

    /// Create an allocator from a raw starting value, rejecting zero.
    #[inline]
    pub fn try_starting_at(first: u64) -> Result<Self, SemanticIdParseError> {
        let first = I::from_raw(first)?;
        Ok(Self::starting_at(first))
    }

    /// Allocate the next ID.
    ///
    /// Allocation is monotonic and never wraps. If the final representable ID
    /// has already been issued, this returns `Exhausted` without changing the
    /// allocator.
    pub fn allocate(&mut self) -> Result<I, SemanticIdAllocationError> {
        let current = self
            .next
            .ok_or(SemanticIdAllocationError::Exhausted)?;

        // Calculate all fallible bookkeeping before changing allocator state.
        let new_count = self
            .allocated_count
            .checked_add(1)
            .ok_or(SemanticIdAllocationError::AllocationCountExhausted)?;

        let successor = current
            .get()
            .checked_add(1)
            .and_then(NonZeroU64::new);

        let id = I::from_nonzero(current);
        self.next = successor;
        self.allocated_count = new_count;

        Ok(id)
    }

    /// Return the next ID without allocating it.
    ///
    /// Returns `None` once the allocator is exhausted.
    #[inline]
    pub fn peek(&self) -> Option<I> {
        self.next.map(I::from_nonzero)
    }

    /// Return the number of IDs successfully issued by this allocator.
    #[inline]
    pub const fn allocated_count(&self) -> u64 {
        self.allocated_count
    }

    /// Whether no further ID can be allocated.
    #[inline]
    pub const fn is_exhausted(&self) -> bool {
        self.next.is_none()
    }

    /// Return the next raw integer for explicit state persistence.
    ///
    /// `None` represents the exhausted state. Any persisted allocator state
    /// must be versioned and validated by the owning serialization layer.
    #[inline]
    pub const fn next_raw(&self) -> Option<u64> {
        match self.next {
            Some(value) => Some(value.get()),
            None => None,
        }
    }
}

impl<I: SemanticIdValue> Default for SemanticIdAllocator<I> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_id_types_are_distinct_and_orderable() {
        let symbol = SymbolId::new(1).expect("non-zero symbol ID");
        let other_symbol = SymbolId::new(2).expect("non-zero symbol ID");
        let ty = TypeId::new(1).expect("non-zero type ID");

        assert_ne!(symbol, other_symbol);
        assert!(symbol < other_symbol);
        assert_eq!(symbol.get(), ty.get());

        // SymbolId and TypeId are distinct Rust types; they cannot be
        // compared directly or passed to one another's typed APIs.
    }

    #[test]
    fn zero_is_rejected_for_all_id_types() {
        assert_eq!(SemanticId::new(0), None);
        assert_eq!(SymbolId::new(0), None);
        assert_eq!(ScopeId::new(0), None);
        assert_eq!(TypeId::new(0), None);
        assert_eq!(ModuleId::new(0), None);
        assert_eq!(FactId::new(0), None);
        assert_eq!(ObligationId::new(0), None);
        assert_eq!(CapabilityId::new(0), None);
        assert_eq!(ResourceId::new(0), None);
        assert_eq!(ContractId::new(0), None);
        assert_eq!(ProvenanceId::new(0), None);
        assert_eq!(ExtensionId::new(0), None);
    }

    #[test]
    fn allocator_is_deterministic_for_the_same_start_and_order() {
        let mut first = SemanticIdAllocator::<SymbolId>::new();
        let mut second = SemanticIdAllocator::<SymbolId>::new();

        for expected in 1..=128 {
            let left = first.allocate().expect("ID allocation");
            let right = second.allocate().expect("ID allocation");

            assert_eq!(left, right);
            assert_eq!(left.get(), expected);
        }

        assert_eq!(first.allocated_count(), 128);
        assert_eq!(second.allocated_count(), 128);
    }

    #[test]
    fn allocator_supports_an_explicit_start() {
        let start = TypeId::new(900).expect("valid ID");
        let mut allocator = SemanticIdAllocator::<TypeId>::starting_at(start);

        assert_eq!(allocator.peek(), Some(start));
        assert_eq!(allocator.allocate().expect("allocation"), start);
        assert_eq!(allocator.allocate().expect("allocation").get(), 901);
        assert_eq!(allocator.allocated_count(), 2);
    }

    #[test]
    fn raw_starting_value_is_checked() {
        assert!(SemanticIdAllocator::<ScopeId>::try_starting_at(0).is_err());

        let allocator =
            SemanticIdAllocator::<ScopeId>::try_starting_at(42).expect("valid start");

        assert_eq!(allocator.peek().map(ScopeId::get), Some(42));
    }

    #[test]
    fn allocator_exhaustion_never_wraps_or_reuses_an_id() {
        let last = ObligationId::new(u64::MAX).expect("maximum non-zero ID");
        let mut allocator = SemanticIdAllocator::<ObligationId>::starting_at(last);

        assert_eq!(allocator.peek(), Some(last));
        assert_eq!(allocator.allocate(), Ok(last));
        assert!(allocator.is_exhausted());
        assert_eq!(allocator.peek(), None);
        assert_eq!(
            allocator.allocate(),
            Err(SemanticIdAllocationError::Exhausted)
        );
        assert_eq!(allocator.allocated_count(), 1);
    }

    #[test]
    fn failed_allocation_does_not_change_exhausted_state() {
        let last = ResourceId::new(u64::MAX).expect("maximum non-zero ID");
        let mut allocator = SemanticIdAllocator::<ResourceId>::starting_at(last);

        assert_eq!(allocator.allocate(), Ok(last));
        let before = allocator.clone();

        assert_eq!(
            allocator.allocate(),
            Err(SemanticIdAllocationError::Exhausted)
        );
        assert_eq!(allocator, before);
    }

    #[test]
    fn IDs_round_trip_through_display_and_parsing() {
        let id = CapabilityId::new(123_456).expect("valid ID");
        let text = id.to_string();
        let parsed: CapabilityId = text.parse().expect("parse ID");

        assert_eq!(parsed, id);
        assert_eq!(u64::from(id), 123_456);
        assert_eq!(id.try_as_usize(), usize::try_from(123_456_u64).ok());
    }

    #[test]
    fn parsing_rejects_zero_and_invalid_text() {
        assert_eq!(
            "0".parse::<SemanticId>(),
            Err(SemanticIdParseError::Zero)
        );
        assert!("not-an-id".parse::<SemanticId>().is_err());
        assert!(u64::MAX.to_string().parse::<SemanticId>().is_ok());
    }

    #[test]
    fn allocators_for_different_namespaces_are_independent() {
        let mut symbols = SemanticIdAllocator::<SymbolId>::new();
        let mut types = SemanticIdAllocator::<TypeId>::new();

        assert_eq!(symbols.allocate().expect("symbol").get(), 1);
        assert_eq!(types.allocate().expect("type").get(), 1);
        assert_eq!(symbols.allocate().expect("symbol").get(), 2);
        assert_eq!(types.allocate().expect("type").get(), 2);
    }

    #[test]
    fn allocator_state_is_explicit_and_cloneable() {
        let mut original = SemanticIdAllocator::<FactId>::new();
        assert_eq!(original.allocate().expect("fact").get(), 1);

        let mut fork = original.clone();

        assert_eq!(original.allocate().expect("fact").get(), 2);
        assert_eq!(fork.allocate().expect("fact").get(), 2);
        assert_eq!(original, fork);
    }
}
