//! Zamani Quantum IR — Classical Bit Identity
//!
//! Canonical, target-independent representation of a logical classical bit.
//!
//! # Architectural role
//!
//! This module is the SINGLE OWNER of the canonical classical-bit identity
//! used by the Zamani Quantum IR.
//!
//! The canonical type is:
//!
//! ```text
//! quantum::ir::classical::bit::ClassicalBitId
//! ```
//!
//! The parent classical module should re-export it:
//!
//! ```text
//! quantum::ir::classical::ClassicalBitId
//! ```
//!
//! Legacy modules such as `measurement.rs` must import or re-export this
//! canonical type instead of defining another `ClassicalBitId`.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - logical classical-bit identity;
//! - safe construction of classical-bit identifiers;
//! - checked identifier arithmetic;
//! - deterministic ordering;
//! - conversion to and from the host indexing representation;
//! - textual representation;
//! - lightweight classical-bit references;
//! - identity-level validation;
//! - identity-level tests and invariants.
//!
//! This file does NOT own:
//!
//! - classical runtime values;
//! - classical registers;
//! - classical arrays;
//! - classical expressions;
//! - predicates;
//! - assignments;
//! - classical memory allocation;
//! - CPU registers;
//! - FPGA registers;
//! - device readout buffers;
//! - measurement semantics;
//! - quantum operations;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulation;
//! - serialization framing;
//! - hashing policy;
//! - compiler optimization;
//! - frontend parsing.
//!
//! Those responsibilities belong to their respective IR modules.
//!
//! # Universal-program principle
//!
//! A `ClassicalBitId` identifies a logical bit in the Zamani program namespace.
//!
//! It does NOT mean:
//!
//! - one physical memory cell;
//! - one byte;
//! - one CPU register;
//! - one FPGA register;
//! - one hardware readout location;
//! - one simulator array element.
//!
//! The same semantic program may therefore be lowered toward machines with
//! radically different classical controllers and memory architectures.
//!
//! No fixed value such as:
//!
//! ```text
//! 8
//! 32
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! is a semantic maximum.
//!
//! The practical maximum is determined by the host representation, compiler
//! resource policy, target capabilities, available memory, and execution
//! environment.
//!
//! # Relationship with quantum IR
//!
//! This file deliberately does NOT depend on `QubitId`.
//!
//! A classical bit can exist independently of a quantum bit. Quantum-to-
//! classical relationships are represented by higher-level constructs such as
//! measurement and classical feedback.
//!
//! For example:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!             |
//!             v
//!       measurement
//!             |
//!             v
//! quantum::ir::classical::bit::ClassicalBitId
//!             |
//!             v
//!       classical predicate
//!             |
//!             v
//!       quantum operation
//! ```
//!
//! The dependency direction therefore remains:
//!
//! ```text
//! qubit.rs
//!     ^
//!     |
//! measurement.rs / operation.rs / control_flow.rs
//!     |
//!     v
//! classical/bit.rs
//! ```
//!
//! `bit.rs` itself does not require `qubit.rs`, preventing unnecessary
//! coupling and circular dependencies.
//!
//! # Identity versus value
//!
//! This distinction is fundamental:
//!
//! ```text
//! ClassicalBitId
//!     = WHERE a logical classical bit is identified
//!
//! ClassicalBitRef
//!     = a typed reference to that identity
//!
//! ClassicalValue
//!     = WHAT value the bit currently carries
//! ```
//!
//! `ClassicalBitId` therefore must never be used as a boolean value.
//!
//! For example:
//!
//! ```text
//! ClassicalBitId::new(0)
//! ```
//!
//! does NOT mean:
//!
//! ```text
//! false
//! ```
//!
//! and:
//!
//! ```text
//! ClassicalBitId::new(1)
//! ```
//!
//! does NOT mean:
//!
//! ```text
//! true
//! ```
//!
//! The identifier and the value belong to different semantic domains.
//!
//! # Determinism
//!
//! The type implements deterministic ordering and hashing.
//!
//! This permits higher-level modules to use it safely with:
//!
//! - `BTreeMap`;
//! - `BTreeSet`;
//! - deterministic dependency structures;
//! - canonical serialization;
//! - canonical hashing;
//! - reproducible compilation.
//!
//! This file does not itself define the canonical serialization format.
//!
//! # Overflow safety
//!
//! Identifier arithmetic is checked.
//!
//! No public operation in this module may silently wrap a classical-bit
//! identifier.
//!
//! `checked_next()` returns `None` at the representational boundary.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `classical/mod.rs`
//!     must re-export `ClassicalBitId` and `ClassicalBitRef`.
//!
//! `classical/value.rs`
//!     owns the value stored by a classical bit.
//!
//! `classical/register.rs`
//!     owns collections/registers of classical bits.
//!
//! `classical/expression.rs`
//!     may reference `ClassicalBitId` when expressions read bits.
//!
//! `classical/predicate.rs`
//!     may reference `ClassicalBitId` when predicates inspect bits.
//!
//! `classical/assignment.rs`
//!     may reference `ClassicalBitId` as an assignment destination.
//!
//! `quantum/measurement.rs` or the repository's measurement module
//!     uses `ClassicalBitId` as a logical measurement destination.
//!
//! `control/*`
//!     uses `ClassicalBitId` for dynamic classical feedback.
//!
//! `program/operand.rs`
//!     may use `ClassicalBitRef` as a typed operand reference.
//!
//! `program/result.rs`
//!     may use `ClassicalBitRef` for classical results.
//!
//! `validation/*`
//!     validates whether an identifier belongs to a declared classical
//!     namespace.
//!
//! `serialization/*`
//!     serializes the numeric identity using its canonical integer
//!     representation.
//!
//! `hashing/*`
//!     incorporates the identity into canonical semantic hashes.
//!
//! `frontend/*`
//!     lowers source-level classical-bit declarations and references into
//!     this type.
//!
//! `hardware/*`
//!     may map the logical identity to target-specific storage, but must not
//!     redefine this type.
//!
//! # Migration rule
//!
//! There MUST be exactly one canonical `ClassicalBitId` definition in the
//! canonical IR.
//!
//! The following pattern is forbidden:
//!
//! ```text
//! measurement.rs  -> defines ClassicalBitId
//! classical.rs    -> defines ClassicalBitId
//! classical/bit.rs -> defines ClassicalBitId
//! ```
//!
//! That creates incompatible Rust types even if they have identical fields.
//!
//! The required architecture is:
//!
//! ```text
//! classical/bit.rs
//!        |
//!        +----> canonical ClassicalBitId
//!        |
//!        +----> canonical ClassicalBitRef
//!
//! classical/mod.rs
//!        |
//!        +----> re-export
//!
//! measurement.rs
//!        |
//!        +----> import canonical type
//!
//! operation.rs
//!        |
//!        +----> import canonical type
//! ```
//!
//! # Security boundary
//!
//! An identifier is not proof of resource existence.
//!
//! For example:
//!
//! ```rust
//! let id = ClassicalBitId::new(10_000);
//! ```
//!
//! is valid as an identity value, but does not prove that classical bit
//! `c10000` has been declared by a program.
//!
//! Declaration membership must be checked by the owning program/register
//! namespace.
//!
//! This separation prevents constructors from becoming hidden global-resource
//! allocators and allows large logical namespaces without eager allocation.
//!
//! # No execution semantics
//!
//! This module never:
//!
//! - reads a bit;
//! - writes a bit;
//! - allocates memory;
//! - communicates with a QPU;
//! - performs measurement;
//! - evaluates an expression;
//! - executes a predicate.
//!
//! It only represents identity.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use core::fmt;

/// Stable logical classical-bit identifier.
///
/// `ClassicalBitId` identifies a classical bit in the canonical Zamani
/// Quantum IR logical namespace.
///
/// It is intentionally distinct from:
///
/// - classical values;
/// - register offsets;
/// - physical memory addresses;
/// - CPU registers;
/// - device readout locations;
/// - simulator storage.
///
/// The underlying `usize` is a host-side representation suitable for
/// compiler data structures. It is NOT a Zamani architectural resource limit.
///
/// A target with a larger resource namespace can use the same semantic model;
/// concrete compilation and execution limits are enforced elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalBitId(usize);

impl ClassicalBitId {
    /// Creates a logical classical-bit identifier.
    ///
    /// This constructor does not allocate the bit and does not establish
    /// declaration membership.
    ///
    /// Membership must be established by the owning classical namespace,
    /// register, or program.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying logical identifier index.
    ///
    /// This value is suitable for interaction with compiler-side collections.
    ///
    /// It must not be interpreted as:
    ///
    /// - a physical memory address;
    /// - a byte offset;
    /// - a device register;
    /// - a quantum measurement result.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next logical classical-bit identifier when representable.
    ///
    /// No wrapping is permitted.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(index) => Some(Self::new(index)),
            None => None,
        }
    }

    /// Returns whether this identifier is the zero-based first identifier.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Creates an identifier from a checked addition to this identifier.
    ///
    /// This is useful when a compiler is constructing logical namespaces
    /// without allowing integer overflow.
    #[must_use]
    pub const fn checked_add(self, offset: usize) -> Option<Self> {
        match self.0.checked_add(offset) {
            Some(index) => Some(Self::new(index)),
            None => None,
        }
    }

    /// Creates an identifier from a checked subtraction from this identifier.
    ///
    /// This is useful for namespace calculations where an offset must remain
    /// inside the representable logical identifier domain.
    #[must_use]
    pub const fn checked_sub(self, offset: usize) -> Option<Self> {
        match self.0.checked_sub(offset) {
            Some(index) => Some(Self::new(index)),
            None => None,
        }
    }

    /// Returns the distance from `self` to `other` when `other` is not below
    /// `self`.
    ///
    /// This is an identity-space calculation, not a register-membership
    /// calculation.
    #[must_use]
    pub const fn checked_distance_to(self, other: Self) -> Option<usize> {
        other.index().checked_sub(self.index())
    }
}

impl From<usize> for ClassicalBitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<ClassicalBitId> for usize {
    fn from(id: ClassicalBitId) -> Self {
        id.index()
    }
}

impl fmt::Display for ClassicalBitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "c{}", self.0)
    }
}

/// Typed reference to a logical classical bit.
///
/// This type exists to prevent APIs from accidentally accepting a raw integer
/// where a classical-bit reference is required.
///
/// It is intentionally lightweight and copyable.
///
/// `ClassicalBitRef` does not imply that the referenced bit has been declared.
/// Declaration validation remains the responsibility of the owning program
/// or register namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalBitRef {
    id: ClassicalBitId,
}

impl ClassicalBitRef {
    /// Creates a typed reference to a classical bit.
    #[must_use]
    pub const fn new(id: ClassicalBitId) -> Self {
        Self { id }
    }

    /// Returns the referenced classical-bit identifier.
    #[must_use]
    pub const fn id(self) -> ClassicalBitId {
        self.id
    }

    /// Returns the logical index of the referenced classical bit.
    #[must_use]
    pub const fn index(self) -> usize {
        self.id.index()
    }
}

impl From<ClassicalBitId> for ClassicalBitRef {
    fn from(id: ClassicalBitId) -> Self {
        Self::new(id)
    }
}

impl From<ClassicalBitRef> for ClassicalBitId {
    fn from(reference: ClassicalBitRef) -> Self {
        reference.id()
    }
}

impl fmt::Display for ClassicalBitRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(formatter)
    }
}

/// Canonical result of validating a classical-bit identity against a
/// half-open logical namespace.
///
/// This type is deliberately small so callers can perform identity
/// membership checks without allocating.
///
/// It is useful when a higher-level register/program API wants to distinguish
/// "identifier exists in namespace" from "identifier is merely representable".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassicalBitMembership {
    /// The identifier belongs to the checked namespace.
    Present,

    /// The identifier is representable but is not declared by the checked
    /// namespace.
    Absent,
}

impl ClassicalBitMembership {
    /// Returns `true` when the identifier is present.
    #[must_use]
    pub const fn is_present(self) -> bool {
        matches!(self, Self::Present)
    }

    /// Returns `true` when the identifier is absent.
    #[must_use]
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::Absent)
    }
}

/// Checks whether a classical-bit identifier belongs to a half-open namespace.
///
/// The namespace is:
///
/// ```text
/// [start, end)
/// ```
///
/// Therefore:
///
/// ```text
/// contains(c0, 0, 4) == true
/// contains(c3, 0, 4) == true
/// contains(c4, 0, 4) == false
/// ```
///
/// This helper does not allocate and does not impose a fixed namespace size.
///
/// # Arguments
///
/// * `id` - logical classical-bit identifier to test.
/// * `start` - inclusive namespace start.
/// * `end` - exclusive namespace end.
///
/// # Panics
///
/// This function never panics.
///
/// # Overflow
///
/// No arithmetic is performed on `end`; callers are expected to provide an
/// already validated half-open boundary.
#[must_use]
pub const fn contains_in_range(
    id: ClassicalBitId,
    start: ClassicalBitId,
    end: ClassicalBitId,
) -> ClassicalBitMembership {
    if id.index() >= start.index() && id.index() < end.index() {
        ClassicalBitMembership::Present
    } else {
        ClassicalBitMembership::Absent
    }
}

/// Checks whether a classical-bit identifier is the first identifier in a
/// namespace.
///
/// This is a convenience helper for namespace implementations and does not
/// establish declaration membership.
#[must_use]
pub const fn is_first_in_namespace(
    id: ClassicalBitId,
    namespace_start: ClassicalBitId,
) -> bool {
    id.index() == namespace_start.index()
}

/// Computes the exclusive end identifier for a namespace.
///
/// The namespace starts at `start` and contains `length` identifiers.
///
/// For example:
///
/// ```text
/// start = c10
/// length = 4
///
/// namespace = c10, c11, c12, c13
/// exclusive end = c14
/// ```
///
/// `None` indicates that the namespace would exceed the representable
/// identifier domain.
///
/// This function does not allocate.
#[must_use]
pub const fn checked_namespace_end(
    start: ClassicalBitId,
    length: usize,
) -> Option<ClassicalBitId> {
    start.checked_add(length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_construction_is_stable() {
        let id = ClassicalBitId::new(42);

        assert_eq!(id.index(), 42);
        assert_eq!(usize::from(id), 42);
    }

    #[test]
    fn identity_display_is_deterministic() {
        assert_eq!(ClassicalBitId::new(0).to_string(), "c0");
        assert_eq!(ClassicalBitId::new(7).to_string(), "c7");
        assert_eq!(ClassicalBitId::new(123_456).to_string(), "c123456");
    }

    #[test]
    fn identity_ordering_is_numeric() {
        let low = ClassicalBitId::new(1);
        let high = ClassicalBitId::new(2);

        assert!(low < high);
        assert!(high > low);
    }

    #[test]
    fn checked_next_handles_normal_case() {
        let id = ClassicalBitId::new(41);

        assert_eq!(id.checked_next(), Some(ClassicalBitId::new(42)));
    }

    #[test]
    fn checked_next_does_not_wrap() {
        let id = ClassicalBitId::new(usize::MAX);

        assert_eq!(id.checked_next(), None);
    }

    #[test]
    fn checked_add_is_overflow_safe() {
        let id = ClassicalBitId::new(10);

        assert_eq!(
            id.checked_add(5),
            Some(ClassicalBitId::new(15))
        );

        assert_eq!(
            ClassicalBitId::new(usize::MAX).checked_add(1),
            None
        );
    }

    #[test]
    fn checked_sub_is_underflow_safe() {
        let id = ClassicalBitId::new(10);

        assert_eq!(
            id.checked_sub(5),
            Some(ClassicalBitId::new(5))
        );

        assert_eq!(
            ClassicalBitId::new(0).checked_sub(1),
            None
        );
    }

    #[test]
    fn checked_distance_is_directional() {
        let first = ClassicalBitId::new(10);
        let second = ClassicalBitId::new(15);

        assert_eq!(
            first.checked_distance_to(second),
            Some(5)
        );

        assert_eq!(
            second.checked_distance_to(first),
            None
        );
    }

    #[test]
    fn typed_reference_preserves_identity() {
        let id = ClassicalBitId::new(17);
        let reference = ClassicalBitRef::new(id);

        assert_eq!(reference.id(), id);
        assert_eq!(reference.index(), 17);
        assert_eq!(
            ClassicalBitId::from(reference),
            id
        );
    }

    #[test]
    fn typed_reference_display_is_canonical() {
        let reference = ClassicalBitRef::new(ClassicalBitId::new(9));

        assert_eq!(reference.to_string(), "c9");
    }

    #[test]
    fn range_membership_is_half_open() {
        let start = ClassicalBitId::new(0);
        let end = ClassicalBitId::new(4);

        assert!(
            contains_in_range(
                ClassicalBitId::new(0),
                start,
                end
            )
            .is_present()
        );

        assert!(
            contains_in_range(
                ClassicalBitId::new(3),
                start,
                end
            )
            .is_present()
        );

        assert!(
            contains_in_range(
                ClassicalBitId::new(4),
                start,
                end
            )
            .is_absent()
        );
    }

    #[test]
    fn range_membership_handles_nonzero_namespace() {
        let start = ClassicalBitId::new(100);
        let end = ClassicalBitId::new(105);

        assert!(
            contains_in_range(
                ClassicalBitId::new(100),
                start,
                end
            )
            .is_present()
        );

        assert!(
            contains_in_range(
                ClassicalBitId::new(104),
                start,
                end
            )
            .is_present()
        );

        assert!(
            contains_in_range(
                ClassicalBitId::new(99),
                start,
                end
            )
            .is_absent()
        );

        assert!(
            contains_in_range(
                ClassicalBitId::new(105),
                start,
                end
            )
            .is_absent()
        );
    }

    #[test]
    fn namespace_end_is_checked() {
        assert_eq!(
            checked_namespace_end(
                ClassicalBitId::new(10),
                5
            ),
            Some(ClassicalBitId::new(15))
        );

        assert_eq!(
            checked_namespace_end(
                ClassicalBitId::new(usize::MAX),
                1
            ),
            None
        );
    }

    #[test]
    fn zero_identity_is_detected() {
        assert!(ClassicalBitId::new(0).is_zero());
        assert!(!ClassicalBitId::new(1).is_zero());
    }

    #[test]
    fn first_namespace_helper_is_exact() {
        let namespace_start = ClassicalBitId::new(50);

        assert!(
            is_first_in_namespace(
                ClassicalBitId::new(50),
                namespace_start
            )
        );

        assert!(
            !is_first_in_namespace(
                ClassicalBitId::new(51),
                namespace_start
            )
        );
    }

    #[test]
    fn identity_is_copy_and_hashable() {
        use std::collections::BTreeSet;

        let mut ids = BTreeSet::new();

        ids.insert(ClassicalBitId::new(2));
        ids.insert(ClassicalBitId::new(1));
        ids.insert(ClassicalBitId::new(2));

        let ordered: Vec<_> = ids.into_iter().collect();

        assert_eq!(
            ordered,
            vec![
                ClassicalBitId::new(1),
                ClassicalBitId::new(2)
            ]
        );
    }
}