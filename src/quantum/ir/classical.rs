//! Zamani Quantum IR — Canonical Classical Resource Model
//!
//! Hardware-independent representation of classical resources used by
//! quantum programs.
//!
//! # Architectural role
//!
//! `classical.rs` owns the semantic model for classical information inside
//! the canonical Quantum IR:
//!
//! - classical-bit identity;
//! - classical registers;
//! - sparse and deterministic classical-bit collections;
//! - classical scalar values;
//! - classical expressions;
//! - classical predicates;
//! - classical assignments;
//! - classical resource requirements;
//! - logical classical control dependencies.
//!
//! It does NOT own:
//!
//! - CPU registers;
//! - hardware memory addresses;
//! - FPGA registers;
//! - ADC/DAC memory;
//! - device-specific readout buffers;
//! - hardware transport;
//! - scheduling;
//! - routing;
//! - QPU execution;
//! - simulator state;
//! - compiler optimization policy;
//! - frontend parsing.
//!
//! Those responsibilities belong to the appropriate downstream subsystem.
//!
//! # Canonical identity
//!
//! The canonical classical-bit identity is:
//!
//! ```text
//! quantum::ir::classical::ClassicalBitId
//! ```
//!
//! A classical bit is a logical IR resource. Its identity does not imply:
//!
//! - a hardware register;
//! - a byte address;
//! - a CPU register;
//! - a particular memory location;
//! - a physical readout channel.
//!
//! # Quantum integration
//!
//! Classical control is intrinsically connected to quantum operations.
//! Consequently, this module uses the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! for explicit quantum-to-classical dependencies such as measurement
//! destinations and conditional predicates.
//!
//! The dependency is deliberately one-way:
//!
//! ```text
//! classical.rs ──► qubit.rs
//! ```
//!
//! `qubit.rs` does not depend on `classical.rs`.
//!
//! This prevents a circular dependency while allowing dynamic quantum
//! programs to express:
//!
//! ```text
//! measure(q0) -> c0
//! if c0 == 1 {
//!     x(q1)
//! }
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written independently of the target machine.
//!
//! Therefore this module has no architectural classical-resource ceiling.
//!
//! A program may contain:
//!
//! - one classical bit;
//! - thousands of classical bits;
//! - millions of classical bits;
//! - arbitrarily many finite classical resources,
//!
//! subject only to the address space and explicit resource/security policies
//! of the compilation or execution environment.
//!
//! No value such as `63`, `64`, `4096`, or `1_000_000` is a semantic maximum.
//!
//! # Important scalability property
//!
//! `ClassicalRegister` stores a logical resource count rather than eagerly
//! allocating one Rust object per bit.
//!
//! This is important because:
//!
//! ```text
//! classical register of 1_000_000 bits
//! ```
//!
//! does not inherently require a `Vec<ClassicalBit>` containing one million
//! objects merely to describe the resource.
//!
//! Sparse collections use `BTreeSet` and deterministic ranges use compact
//! range representations.
//!
//! Actual memory consumption remains governed by the host platform and
//! explicit compiler/security policies.
//!
//! # Classical semantics
//!
//! The IR distinguishes:
//!
//! ```text
//! ClassicalBitId
//!     identity
//!
//! ClassicalRegister
//!     logical namespace
//!
//! ClassicalValue
//!     runtime value
//!
//! ClassicalExpression
//!     deterministic computation
//!
//! ClassicalPredicate
//!     boolean condition
//!
//! ClassicalAssignment
//!     state update
//! ```
//!
//! These types do not execute classical computation themselves. They describe
//! the semantics that later compiler/runtime layers execute.
//!
//! # Dynamic quantum programs
//!
//! This module is designed to support mid-circuit measurement and dynamic
//! quantum control without making measurement itself responsible for classical
//! expression semantics.
//!
//! Example semantic flow:
//!
//! ```text
//! q0
//!  │
//!  ▼
//! measurement
//!  │
//!  ▼
//! c0
//!  │
//!  ▼
//! ClassicalPredicate::Equal
//!  │
//!  ▼
//! conditional quantum operation
//! ```
//!
//! # Integration contracts
//!
//! `measurement.rs`
//!     uses `ClassicalBitId` as the canonical measurement destination.
//!
//! `control_flow.rs`
//!     consumes `ClassicalPredicate`.
//!
//! `operation.rs`
//!     consumes `ClassicalAssignment` and classical conditions.
//!
//! `program.rs`
//!     owns collections of classical registers and declarations.
//!
//! `value.rs`
//!     may later wrap or generalize `ClassicalValue` without changing the
//!     identity semantics defined here.
//!
//! `type.rs`
//!     may later expose the corresponding canonical classical types.
//!
//! `validation.rs`
//!     validates identifiers, expressions, assignments and resource
//!     requirements against program declarations and explicit limits.
//!
//! `analysis.rs`
//!     may count classical resources and dependencies.
//!
//! `serialization.rs`
//!     may serialize these structures using their deterministic enum and
//!     identifier representation.
//!
//! `hash.rs`
//!     may hash the structural representation for canonical program identity.
//!
//! `frontend/`
//!     lowers Zamani source-language classical constructs into these types.
//!
//! `hardware/`
//!     resolves logical classical resources to actual implementation details.
//!
//! # Boundary rule
//!
//! This module describes:
//!
//!     WHAT classical computation means.
//!
//! It does not decide:
//!
//!     WHERE classical computation runs.
//!
//!     WHEN it runs.
//!
//!     HOW a hardware device implements it.
//!
//! Those decisions belong to routing, scheduling, hardware, runtime and
//! backend layers.
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

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;

// =============================================================================
// Classical-bit identity
// =============================================================================

/// Stable logical classical-bit identifier.
///
/// `ClassicalBitId` identifies a classical bit in the canonical Zamani
/// Quantum IR.
///
/// It is intentionally independent from:
///
/// - CPU registers;
/// - physical memory addresses;
/// - FPGA registers;
/// - device readout addresses;
/// - backend-specific memory identifiers.
///
/// The numeric value is a logical namespace index.
///
/// `usize` is used because compiler-side collections commonly index resources
/// using `usize`. It does NOT impose an architectural classical-resource
/// maximum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalBitId(usize);

impl ClassicalBitId {
    /// Creates a classical-bit identifier.
    ///
    /// This does not establish membership in any register.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying logical index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next identifier if the underlying index can be incremented
    /// without overflow.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(index) => Some(Self(index)),
            None => None,
        }
    }
}

impl From<usize> for ClassicalBitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<ClassicalBitId> for usize {
    fn from(bit: ClassicalBitId) -> Self {
        bit.index()
    }
}

impl fmt::Display for ClassicalBitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "c{}", self.0)
    }
}

// =============================================================================
// Classical resource reference
// =============================================================================

/// A logical classical resource reference.
///
/// This is intentionally distinct from a raw integer so that APIs cannot
/// accidentally confuse a classical value with a classical-bit identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalRef {
    /// One logical classical bit.
    Bit(ClassicalBitId),
}

impl ClassicalRef {
    /// Returns the referenced classical bit.
    #[must_use]
    pub const fn bit(self) -> ClassicalBitId {
        match self {
            Self::Bit(id) => id,
        }
    }
}

impl From<ClassicalBitId> for ClassicalRef {
    fn from(id: ClassicalBitId) -> Self {
        Self::Bit(id)
    }
}

impl fmt::Display for ClassicalRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bit(id) => write!(formatter, "{id}"),
        }
    }
}

// =============================================================================
// Classical register
// =============================================================================

/// Logical classical register.
///
/// A register declares a contiguous logical namespace:
///
/// ```text
/// ClassicalRegister::new(0, 4)
/// ```
///
/// represents:
///
/// ```text
/// c0 c1 c2 c3
/// ```
///
/// The register does not allocate a vector of values.
///
/// Runtime values are maintained by the runtime/execution layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalRegister {
    start: ClassicalBitId,
    len: usize,
}

impl ClassicalRegister {
    /// Creates a register containing `len` classical bits beginning at
    /// `start`.
    ///
    /// This operation performs no allocation proportional to `len`.
    ///
    /// An error is returned if the exclusive end of the register cannot be
    /// represented.
    pub fn new(
        start: ClassicalBitId,
        len: usize,
    ) -> Result<Self, ClassicalRegisterError> {
        start
            .index()
            .checked_add(len)
            .ok_or(ClassicalRegisterError::IndexOverflow)?;

        Ok(Self { start, len })
    }

    /// Creates a register starting at `c0`.
    ///
    /// This is a convenience constructor and does not allocate the register's
    /// runtime storage.
    pub fn zero_based(
        len: usize,
    ) -> Result<Self, ClassicalRegisterError> {
        Self::new(ClassicalBitId::new(0), len)
    }

    /// Creates an empty register at `start`.
    #[must_use]
    pub const fn empty(start: ClassicalBitId) -> Self {
        Self { start, len: 0 }
    }

    /// Returns the first classical-bit identifier.
    #[must_use]
    pub const fn start(&self) -> ClassicalBitId {
        self.start
    }

    /// Returns the number of bits in the register.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the register contains no bits.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the exclusive end index as a classical-bit identifier.
    ///
    /// For a register containing `c0..c3`, this returns `c4`.
    pub fn end(&self) -> Result<ClassicalBitId, ClassicalRegisterError> {
        self.start
            .index()
            .checked_add(self.len)
            .map(ClassicalBitId::new)
            .ok_or(ClassicalRegisterError::IndexOverflow)
    }

    /// Returns the final classical-bit identifier if the register is
    /// non-empty.
    pub fn last(&self) -> Option<ClassicalBitId> {
        if self.len == 0 {
            return None;
        }

        self.start
            .index()
            .checked_add(self.len - 1)
            .map(ClassicalBitId::new)
    }

    /// Returns whether this register contains `bit`.
    #[must_use]
    pub fn contains(&self, bit: ClassicalBitId) -> bool {
        match self.start.index().checked_add(self.len) {
            Some(end) => bit.index() >= self.start.index() && bit.index() < end,
            None => false,
        }
    }

    /// Returns the zero-based offset of `bit` within this register.
    pub fn offset(
        &self,
        bit: ClassicalBitId,
    ) -> Option<usize> {
        if !self.contains(bit) {
            return None;
        }

        Some(bit.index() - self.start.index())
    }

    /// Returns a compact Rust range corresponding to this register.
    ///
    /// The range is lazy and therefore does not allocate proportional to the
    /// register size.
    pub fn index_range(
        &self,
    ) -> Result<Range<usize>, ClassicalRegisterError> {
        let end = self
            .start
            .index()
            .checked_add(self.len)
            .ok_or(ClassicalRegisterError::IndexOverflow)?;

        Ok(self.start.index()..end)
    }

    /// Returns a lazy iterator over all logical classical-bit identifiers in
    /// this register.
    ///
    /// No vector containing all identifiers is allocated.
    pub fn iter(
        &self,
    ) -> ClassicalRegisterIter {
        ClassicalRegisterIter {
            next: self.start.index(),
            remaining: self.len,
        }
    }

    /// Validates that the register can represent its declared namespace.
    pub fn validate(&self) -> Result<(), ClassicalRegisterError> {
        self.start
            .index()
            .checked_add(self.len)
            .ok_or(ClassicalRegisterError::IndexOverflow)?;

        Ok(())
    }
}

/// Lazy iterator over a classical register.
#[derive(Debug, Clone)]
pub struct ClassicalRegisterIter {
    next: usize,
    remaining: usize,
}

impl Iterator for ClassicalRegisterIter {
    type Item = ClassicalBitId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let id = ClassicalBitId::new(self.next);

        self.next = self.next.checked_add(1)?;
        self.remaining -= 1;

        Some(id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for ClassicalRegisterIter {}

impl std::iter::FusedIterator for ClassicalRegisterIter {}

// =============================================================================
// Sparse classical-bit set
// =============================================================================

/// Deterministic sparse set of logical classical bits.
///
/// This is useful for programs that operate on a small subset of a very large
/// classical namespace without materializing the complete register.
///
/// `BTreeSet` also provides deterministic iteration order, which is important
/// for reproducible compilation, serialization and hashing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassicalBitSet {
    bits: BTreeSet<ClassicalBitId>,
}

impl ClassicalBitSet {
    /// Creates an empty classical-bit set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a set containing the supplied bits.
    pub fn from_bits<I>(bits: I) -> Self
    where
        I: IntoIterator<Item = ClassicalBitId>,
    {
        Self {
            bits: bits.into_iter().collect(),
        }
    }

    /// Inserts a bit.
    ///
    /// Returns `true` if the bit was not already present.
    pub fn insert(
        &mut self,
        bit: ClassicalBitId,
    ) -> bool {
        self.bits.insert(bit)
    }

    /// Removes a bit.
    ///
    /// Returns `true` if the bit existed.
    pub fn remove(
        &mut self,
        bit: &ClassicalBitId,
    ) -> bool {
        self.bits.remove(bit)
    }

    /// Returns whether the set contains `bit`.
    #[must_use]
    pub fn contains(
        &self,
        bit: &ClassicalBitId,
    ) -> bool {
        self.bits.contains(bit)
    }

    /// Returns the number of unique bits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns an iterator in ascending logical-ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ClassicalBitId> {
        self.bits.iter()
    }

    /// Returns the first bit.
    #[must_use]
    pub fn first(&self) -> Option<ClassicalBitId> {
        self.bits.iter().next().copied()
    }

    /// Returns the last bit.
    #[must_use]
    pub fn last(&self) -> Option<ClassicalBitId> {
        self.bits.iter().next_back().copied()
    }

    /// Removes all bits.
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// Returns whether this set is disjoint from another set.
    #[must_use]
    pub fn is_disjoint(
        &self,
        other: &Self,
    ) -> bool {
        self.bits.is_disjoint(&other.bits)
    }

    /// Returns whether every bit in `other` is contained in this set.
    #[must_use]
    pub fn contains_all(
        &self,
        other: &Self,
    ) -> bool {
        other.bits.is_subset(&self.bits)
    }

    /// Returns the bits as a deterministic vector.
    ///
    /// This is an explicit allocation requested by the caller.
    #[must_use]
    pub fn to_vec(&self) -> Vec<ClassicalBitId> {
        self.bits.iter().copied().collect()
    }

    /// Validates that every bit is a valid identifier.
    ///
    /// All `ClassicalBitId` values are structurally valid by construction, so
    /// this currently serves as a semantic extension point.
    pub fn validate(&self) -> Result<(), ClassicalError> {
        Ok(())
    }
}

// =============================================================================
// Classical scalar value
// =============================================================================

/// Canonical logical classical scalar value.
///
/// This type represents values that can participate in classical control
/// within the Quantum IR.
///
/// It deliberately excludes hardware-specific representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalValue {
    /// Boolean value.
    Bool(bool),

    /// Signed integer.
    ///
    /// `i128` provides substantially more semantic range than machine-sized
    /// integers while remaining safe and deterministic.
    Int(i128),

    /// Unsigned integer.
    UInt(u128),
}

impl ClassicalValue {
    /// Returns the value as a boolean when it is boolean.
    #[must_use]
    pub const fn as_bool(self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(value),
            Self::Int(_) | Self::UInt(_) => None,
        }
    }

    /// Returns the value as an integer when it is signed.
    #[must_use]
    pub const fn as_int(self) -> Option<i128> {
        match self {
            Self::Int(value) => Some(value),
            Self::Bool(_) | Self::UInt(_) => None,
        }
    }

    /// Returns the value as an unsigned integer.
    #[must_use]
    pub const fn as_uint(self) -> Option<u128> {
        match self {
            Self::UInt(value) => Some(value),
            Self::Bool(_) | Self::Int(_) => None,
        }
    }

    /// Returns whether the value is boolean.
    #[must_use]
    pub const fn is_bool(self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Returns whether the value is signed integer.
    #[must_use]
    pub const fn is_int(self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Returns whether the value is unsigned integer.
    #[must_use]
    pub const fn is_uint(self) -> bool {
        matches!(self, Self::UInt(_))
    }

    /// Returns a checked logical NOT for boolean values.
    pub fn logical_not(
        self,
    ) -> Result<Self, ClassicalError> {
        match self {
            Self::Bool(value) => Ok(Self::Bool(!value)),
            Self::Int(_) | Self::UInt(_) => {
                Err(ClassicalError::TypeMismatch {
                    expected: "bool",
                    found: self.type_name(),
                })
            }
        }
    }

    /// Returns the canonical type name.
    #[must_use]
    pub const fn type_name(self) -> &'static str {
        match self {
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::UInt(_) => "uint",
        }
    }
}

impl From<bool> for ClassicalValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i128> for ClassicalValue {
    fn from(value: i128) -> Self {
        Self::Int(value)
    }
}

impl From<u128> for ClassicalValue {
    fn from(value: u128) -> Self {
        Self::UInt(value)
    }
}

impl From<i64> for ClassicalValue {
    fn from(value: i64) -> Self {
        Self::Int(i128::from(value))
    }
}

impl From<u64> for ClassicalValue {
    fn from(value: u64) -> Self {
        Self::UInt(u128::from(value))
    }
}

impl From<i32> for ClassicalValue {
    fn from(value: i32) -> Self {
        Self::Int(i128::from(value))
    }
}

impl From<u32> for ClassicalValue {
    fn from(value: u32) -> Self {
        Self::UInt(u128::from(value))
    }
}

impl From<usize> for ClassicalValue {
    fn from(value: usize) -> Self {
        Self::UInt(value as u128)
    }
}

// =============================================================================
// Classical expression
// =============================================================================

/// Binary arithmetic/logical operator for classical expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalBinaryOp {
    /// Integer addition.
    Add,

    /// Integer subtraction.
    Subtract,

    /// Integer multiplication.
    Multiply,

    /// Integer division.
    Divide,

    /// Integer remainder.
    Remainder,

    /// Equality.
    Equal,

    /// Inequality.
    NotEqual,

    /// Less-than comparison.
    LessThan,

    /// Less-than-or-equal comparison.
    LessThanOrEqual,

    /// Greater-than comparison.
    GreaterThan,

    /// Greater-than-or-equal comparison.
    GreaterThanOrEqual,

    /// Boolean conjunction.
    And,

    /// Boolean disjunction.
    Or,

    /// Bitwise AND for integer values.
    BitAnd,

    /// Bitwise OR for integer values.
    BitOr,

    /// Bitwise XOR for integer values.
    BitXor,
}

impl ClassicalBinaryOp {
    /// Returns whether this operator is a comparison.
    #[must_use]
    pub const fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::NotEqual
                | Self::LessThan
                | Self::LessThanOrEqual
                | Self::GreaterThan
                | Self::GreaterThanOrEqual
        )
    }

    /// Returns whether this operator is boolean-only.
    #[must_use]
    pub const fn is_boolean_operator(self) -> bool {
        matches!(self, Self::And | Self::Or)
    }

    /// Returns whether this operator is integer-only.
    #[must_use]
    pub const fn is_integer_operator(self) -> bool {
        matches!(
            self,
            Self::Add
                | Self::Subtract
                | Self::Multiply
                | Self::Divide
                | Self::Remainder
                | Self::BitAnd
                | Self::BitOr
                | Self::BitXor
        )
    }
}

/// Unary operator for classical expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalUnaryOp {
    /// Boolean NOT.
    Not,

    /// Arithmetic negation.
    Negate,

    /// Bitwise complement.
    BitNot,
}

impl ClassicalUnaryOp {
    /// Returns whether this operator requires a boolean value.
    #[must_use]
    pub const fn requires_bool(self) -> bool {
        matches!(self, Self::Not)
    }

    /// Returns whether this operator requires an integer value.
    #[must_use]
    pub const fn requires_integer(self) -> bool {
        matches!(self, Self::Negate | Self::BitNot)
    }
}

/// Classical expression.
///
/// Expressions are semantic descriptions and do not execute until interpreted
/// by a later compiler/runtime stage.
///
/// Classical expressions can reference:
///
/// - constants;
/// - classical bits;
/// - binary operations;
/// - unary operations.
///
/// The representation is deliberately independent from physical classical
/// hardware.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalExpression {
    /// Constant classical value.
    Value(ClassicalValue),

    /// Read one logical classical bit.
    Bit(ClassicalBitId),

    /// Unary operation.
    Unary {
        /// Operator.
        op: ClassicalUnaryOp,

        /// Operand.
        operand: Box<ClassicalExpression>,
    },

    /// Binary operation.
    Binary {
        /// Operator.
        op: ClassicalBinaryOp,

        /// Left operand.
        left: Box<ClassicalExpression>,

        /// Right operand.
        right: Box<ClassicalExpression>,
    },
}

impl ClassicalExpression {
    /// Creates a constant expression.
    #[must_use]
    pub const fn value(value: ClassicalValue) -> Self {
        Self::Value(value)
    }

    /// Creates a boolean constant.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Value(ClassicalValue::Bool(value))
    }

    /// Creates a signed integer constant.
    #[must_use]
    pub const fn int(value: i128) -> Self {
        Self::Value(ClassicalValue::Int(value))
    }

    /// Creates an unsigned integer constant.
    #[must_use]
    pub const fn uint(value: u128) -> Self {
        Self::Value(ClassicalValue::UInt(value))
    }

    /// References one classical bit.
    #[must_use]
    pub const fn bit(bit: ClassicalBitId) -> Self {
        Self::Bit(bit)
    }

    /// Creates a unary expression.
    #[must_use]
    pub fn unary(
        op: ClassicalUnaryOp,
        operand: Self,
    ) -> Self {
        Self::Unary {
            op,
            operand: Box::new(operand),
        }
    }

    /// Creates a binary expression.
    #[must_use]
    pub fn binary(
        op: ClassicalBinaryOp,
        left: Self,
        right: Self,
    ) -> Self {
        Self::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Creates `left == right`.
    #[must_use]
    pub fn equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Equal, left, right)
    }

    /// Creates `left != right`.
    #[must_use]
    pub fn not_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::NotEqual, left, right)
    }

    /// Creates `left && right`.
    #[must_use]
    pub fn and(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::And, left, right)
    }

    /// Creates `left || right`.
    #[must_use]
    pub fn or(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Or, left, right)
    }

    /// Creates `!operand`.
    #[must_use]
    pub fn not(operand: Self) -> Self {
        Self::unary(ClassicalUnaryOp::Not, operand)
    }

    /// Returns whether the expression directly references classical bits.
    #[must_use]
    pub fn references_bits(&self) -> bool {
        match self {
            Self::Value(_) => false,
            Self::Bit(_) => true,
            Self::Unary { operand, .. } => operand.references_bits(),
            Self::Binary { left, right, .. } => {
                left.references_bits() || right.references_bits()
            }
        }
    }

    /// Returns the number of expression nodes.
    ///
    /// This operation is iterative and therefore does not recursively consume
    /// Rust call-stack depth proportional to expression size.
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(expression) = stack.pop() {
            count = match count.checked_add(1) {
                Some(value) => value,
                None => return usize::MAX,
            };

            match expression {
                Self::Value(_) | Self::Bit(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        count
    }

    /// Returns the maximum expression depth.
    ///
    /// The traversal is iterative.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = vec![(self, 0usize)];

        while let Some((expression, depth)) = stack.pop() {
            if depth > maximum {
                maximum = depth;
            }

            match expression {
                Self::Value(_) | Self::Bit(_) => {}

                Self::Unary { operand, .. } => {
                    let next_depth = depth.saturating_add(1);
                    stack.push((operand, next_depth));
                }

                Self::Binary { left, right, .. } => {
                    let next_depth = depth.saturating_add(1);
                    stack.push((left, next_depth));
                    stack.push((right, next_depth));
                }
            }
        }

        maximum
    }

    /// Collects every referenced classical bit in deterministic ascending
    /// order.
    pub fn collect_bits(&self) -> ClassicalBitSet {
        let mut result = ClassicalBitSet::new();
        let mut stack = vec![self];

        while let Some(expression) = stack.pop() {
            match expression {
                Self::Value(_) => {}

                Self::Bit(bit) => {
                    result.insert(*bit);
                }

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        result
    }

    /// Validates the expression's static type and operator compatibility.
    pub fn validate(&self) -> Result<ClassicalType, ClassicalError> {
        self.validate_with_depth_limit(usize::MAX)
    }

    /// Validates the expression while enforcing an explicit expression-depth
    /// policy.
    ///
    /// This policy is deliberately supplied by the caller. It is not a
    /// semantic limit on Zamani's classical language.
    pub fn validate_with_depth_limit(
        &self,
        max_depth: usize,
    ) -> Result<ClassicalType, ClassicalError> {
        let mut stack = vec![(self, false, 0usize)];
        let mut types = Vec::<ClassicalType>::new();

        while let Some((expression, visited, depth)) = stack.pop() {
            if depth > max_depth {
                return Err(ClassicalError::ExpressionDepthExceeded {
                    depth,
                    maximum: max_depth,
                });
            }

            if !visited {
                match expression {
                    Self::Value(value) => {
                        types.push(value.classical_type());
                    }

                    Self::Bit(_) => {
                        types.push(ClassicalType::Bool);
                    }

                    Self::Unary { operand, .. } => {
                        stack.push((expression, true, depth));
                        stack.push((operand, false, depth.saturating_add(1)));
                    }

                    Self::Binary { left, right, .. } => {
                        stack.push((expression, true, depth));
                        stack.push((right, false, depth.saturating_add(1)));
                        stack.push((left, false, depth.saturating_add(1)));
                    }
                }

                continue;
            }

            match expression {
                Self::Value(_) | Self::Bit(_) => {}

                Self::Unary { op, .. } => {
                    let operand_type = types.pop().ok_or(
                        ClassicalError::InvalidExpression,
                    )?;

                    let result = validate_unary(*op, operand_type)?;
                    types.push(result);
                }

                Self::Binary { op, .. } => {
                    let right = types.pop().ok_or(
                        ClassicalError::InvalidExpression,
                    )?;

                    let left = types.pop().ok_or(
                        ClassicalError::InvalidExpression,
                    )?;

                    let result = validate_binary(*op, left, right)?;
                    types.push(result);
                }
            }
        }

        types.pop().ok_or(ClassicalError::InvalidExpression)
    }

    /// Evaluates the expression using a caller-supplied classical-bit
    /// resolver.
    ///
    /// The resolver has no global state requirement and can therefore be used
    /// safely by concurrent compilation/runtime contexts.
    pub fn evaluate<F>(
        &self,
        resolver: &F,
    ) -> Result<ClassicalValue, ClassicalError>
    where
        F: Fn(ClassicalBitId) -> Option<ClassicalValue>,
    {
        let mut stack = vec![(self, false)];
        let mut values = Vec::<ClassicalValue>::new();

        while let Some((expression, visited)) = stack.pop() {
            if !visited {
                match expression {
                    Self::Value(value) => {
                        values.push(*value);
                    }

                    Self::Bit(bit) => {
                        let value = resolver(*bit).ok_or(
                            ClassicalError::UnboundClassicalBit { bit: *bit },
                        )?;

                        values.push(value);
                    }

                    Self::Unary { operand, .. } => {
                        stack.push((expression, true));
                        stack.push((operand, false));
                    }

                    Self::Binary { left, right, .. } => {
                        stack.push((expression, true));
                        stack.push((right, false));
                        stack.push((left, false));
                    }
                }

                continue;
            }

            match expression {
                Self::Value(_) | Self::Bit(_) => {}

                Self::Unary { op, .. } => {
                    let operand = values.pop().ok_or(
                        ClassicalError::InvalidExpression,
                    )?;

                    values.push(evaluate_unary(*op, operand)?);
                }

                Self::Binary { op, .. } => {
                    let right = values.pop().ok_or(
                        ClassicalError::InvalidExpression,
                    )?;

                    let left = values.pop().ok_or(
                        ClassicalError::InvalidExpression,
                    )?;

                    values.push(evaluate_binary(*op, left, right)?);
                }
            }
        }

        values.pop().ok_or(ClassicalError::InvalidExpression)
    }
}

// =============================================================================
// Classical type
// =============================================================================

/// Static classical value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalType {
    /// Boolean.
    Bool,

    /// Signed integer.
    Int,

    /// Unsigned integer.
    UInt,
}

impl fmt::Display for ClassicalType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::UInt => "uint",
        };

        formatter.write_str(name)
    }
}

impl ClassicalValue {
    /// Returns the corresponding static type.
    #[must_use]
    pub const fn classical_type(self) -> ClassicalType {
        match self {
            Self::Bool(_) => ClassicalType::Bool,
            Self::Int(_) => ClassicalType::Int,
            Self::UInt(_) => ClassicalType::UInt,
        }
    }
}

// =============================================================================
// Classical predicate
// =============================================================================

/// A boolean classical predicate suitable for dynamic quantum control.
///
/// The predicate must statically evaluate to `bool`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassicalPredicate {
    expression: ClassicalExpression,
}

impl ClassicalPredicate {
    /// Creates a predicate after validating that the expression is boolean.
    pub fn new(
        expression: ClassicalExpression,
    ) -> Result<Self, ClassicalError> {
        let expression_type = expression.validate()?;

        if expression_type != ClassicalType::Bool {
            return Err(ClassicalError::PredicateMustBeBoolean {
                found: expression_type,
            });
        }

        Ok(Self { expression })
    }

    /// Creates a predicate with an explicit expression-depth policy.
    pub fn with_depth_limit(
        expression: ClassicalExpression,
        maximum_depth: usize,
    ) -> Result<Self, ClassicalError> {
        let expression_type =
            expression.validate_with_depth_limit(maximum_depth)?;

        if expression_type != ClassicalType::Bool {
            return Err(ClassicalError::PredicateMustBeBoolean {
                found: expression_type,
            });
        }

        Ok(Self { expression })
    }

    /// Returns the underlying expression.
    #[must_use]
    pub fn expression(&self) -> &ClassicalExpression {
        &self.expression
    }

    /// Returns all classical bits referenced by the predicate.
    #[must_use]
    pub fn referenced_bits(&self) -> ClassicalBitSet {
        self.expression.collect_bits()
    }

    /// Returns the number of expression nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.expression.node_count()
    }

    /// Returns the maximum expression depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.expression.depth()
    }

    /// Evaluates the predicate.
    pub fn evaluate<F>(
        &self,
        resolver: &F,
    ) -> Result<bool, ClassicalError>
    where
        F: Fn(ClassicalBitId) -> Option<ClassicalValue>,
    {
        let value = self.expression.evaluate(resolver)?;

        match value {
            ClassicalValue::Bool(result) => Ok(result),
            other => Err(ClassicalError::PredicateMustBeBoolean {
                found: other.classical_type(),
            }),
        }
    }

    /// Creates `bit == value`.
    pub fn equals(
        bit: ClassicalBitId,
        value: ClassicalValue,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::equal(
            ClassicalExpression::bit(bit),
            ClassicalExpression::value(value),
        ))
    }

    /// Creates `bit != value`.
    pub fn not_equals(
        bit: ClassicalBitId,
        value: ClassicalValue,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::not_equal(
            ClassicalExpression::bit(bit),
            ClassicalExpression::value(value),
        ))
    }

    /// Creates a predicate from one classical bit interpreted as a boolean.
    pub fn bit(
        bit: ClassicalBitId,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::bit(bit))
    }

    /// Creates logical conjunction.
    pub fn and(
        left: Self,
        right: Self,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::and(
            left.expression,
            right.expression,
        ))
    }

    /// Creates logical disjunction.
    pub fn or(
        left: Self,
        right: Self,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::or(
            left.expression,
            right.expression,
        ))
    }

    /// Creates logical negation.
    pub fn not(
        predicate: Self,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::not(predicate.expression))
    }
}

// =============================================================================
// Classical assignment
// =============================================================================

/// Semantic assignment to a classical bit.
///
/// The assignment is not executed by the IR. It describes a logical state
/// update for a downstream compiler/runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassicalAssignment {
    target: ClassicalBitId,
    value: ClassicalExpression,
}

impl ClassicalAssignment {
    /// Creates a classical assignment.
    ///
    /// The target bit itself has no static value type beyond the expression
    /// being assigned. Type compatibility with a declared classical variable
    /// is checked by the containing program/type system.
    #[must_use]
    pub fn new(
        target: ClassicalBitId,
        value: ClassicalExpression,
    ) -> Self {
        Self { target, value }
    }

    /// Returns the target bit.
    #[must_use]
    pub const fn target(&self) -> ClassicalBitId {
        self.target
    }

    /// Returns the assigned expression.
    #[must_use]
    pub fn value(&self) -> &ClassicalExpression {
        &self.value
    }

    /// Validates the assigned expression.
    pub fn validate(&self) -> Result<ClassicalType, ClassicalError> {
        self.value.validate()
    }

    /// Returns the classical bits referenced by the assigned expression.
    #[must_use]
    pub fn referenced_bits(&self) -> ClassicalBitSet {
        self.value.collect_bits()
    }
}

// =============================================================================
// Measurement destination
// =============================================================================

/// Explicit semantic destination for a quantum measurement.
///
/// This type provides a bridge between the quantum and classical IR domains
/// without putting measurement behavior into `classical.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementDestination {
    /// Quantum source.
    qubit: super::qubit::QubitId,

    /// Classical destination.
    classical_bit: ClassicalBitId,
}

impl MeasurementDestination {
    /// Creates a quantum-to-classical measurement destination.
    #[must_use]
    pub const fn new(
        qubit: super::qubit::QubitId,
        classical_bit: ClassicalBitId,
    ) -> Self {
        Self {
            qubit,
            classical_bit,
        }
    }

    /// Returns the logical source qubit.
    #[must_use]
    pub const fn qubit(&self) -> super::qubit::QubitId {
        self.qubit
    }

    /// Returns the classical destination.
    #[must_use]
    pub const fn classical_bit(&self) -> ClassicalBitId {
        self.classical_bit
    }
}

// =============================================================================
// Classical resource requirement
// =============================================================================

/// Abstract requirement for logical classical resources.
///
/// This is a semantic resource requirement, not a hardware-memory request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalResourceRequirement {
    /// Number of logical classical bits required.
    count: usize,
}

impl ClassicalResourceRequirement {
    /// Creates a resource requirement.
    ///
    /// No allocation proportional to `count` occurs.
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self { count }
    }

    /// Returns the required number of logical classical bits.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Returns whether no classical bits are required.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Checks whether this requirement can be satisfied by `available`.
    #[must_use]
    pub const fn fits_within(
        &self,
        available: usize,
    ) -> bool {
        self.count <= available
    }
}

// =============================================================================
// Classical errors
// =============================================================================

/// Errors produced by the canonical classical IR model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalError {
    /// A register's end index cannot be represented.
    IndexOverflow,

    /// A classical expression is structurally invalid.
    InvalidExpression,

    /// A value has the wrong static type.
    TypeMismatch {
        /// Required type.
        expected: &'static str,

        /// Actual type.
        found: &'static str,
    },

    /// A binary operation received incompatible operands.
    BinaryTypeMismatch {
        /// Operator.
        operation: ClassicalBinaryOp,

        /// Left operand type.
        left: ClassicalType,

        /// Right operand type.
        right: ClassicalType,
    },

    /// A predicate expression is not boolean.
    PredicateMustBeBoolean {
        /// Actual type.
        found: ClassicalType,
    },

    /// A classical bit has no runtime binding.
    UnboundClassicalBit {
        /// Missing bit.
        bit: ClassicalBitId,
    },

    /// Integer division by zero.
    DivisionByZero,

    /// Integer remainder by zero.
    RemainderByZero,

    /// Signed integer overflow.
    SignedOverflow,

    /// Unsigned integer overflow.
    UnsignedOverflow,

    /// Signed integer underflow.
    SignedUnderflow,

    /// Unsigned integer underflow.
    UnsignedUnderflow,

    /// An expression exceeds the caller's explicit depth policy.
    ExpressionDepthExceeded {
        /// Encountered depth.
        depth: usize,

        /// Configured maximum.
        maximum: usize,
    },
}

impl fmt::Display for ClassicalError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::IndexOverflow => {
                formatter.write_str(
                    "classical register index calculation overflowed",
                )
            }

            Self::InvalidExpression => {
                formatter.write_str("invalid classical expression")
            }

            Self::TypeMismatch { expected, found } => {
                write!(
                    formatter,
                    "classical type mismatch: expected {expected}, found {found}"
                )
            }

            Self::BinaryTypeMismatch {
                operation,
                left,
                right,
            } => {
                write!(
                    formatter,
                    "invalid operands for classical operation {operation:?}: \
                     left={left}, right={right}"
                )
            }

            Self::PredicateMustBeBoolean { found } => {
                write!(
                    formatter,
                    "classical predicate must be bool, found {found}"
                )
            }

            Self::UnboundClassicalBit { bit } => {
                write!(
                    formatter,
                    "classical bit {bit} has no runtime binding"
                )
            }

            Self::DivisionByZero => {
                formatter.write_str(
                    "classical integer division by zero",
                )
            }

            Self::RemainderByZero => {
                formatter.write_str(
                    "classical integer remainder by zero",
                )
            }

            Self::SignedOverflow => {
                formatter.write_str(
                    "signed classical integer overflow",
                )
            }

            Self::UnsignedOverflow => {
                formatter.write_str(
                    "unsigned classical integer overflow",
                )
            }

            Self::SignedUnderflow => {
                formatter.write_str(
                    "signed classical integer underflow",
                )
            }

            Self::UnsignedUnderflow => {
                formatter.write_str(
                    "unsigned classical integer underflow",
                )
            }

            Self::ExpressionDepthExceeded { depth, maximum } => {
                write!(
                    formatter,
                    "classical expression depth {depth} exceeds configured \
                     maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for ClassicalError {}

/// Errors produced while constructing a classical register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassicalRegisterError {
    /// The register's exclusive end cannot be represented.
    IndexOverflow,
}

impl fmt::Display for ClassicalRegisterError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::IndexOverflow => {
                formatter.write_str(
                    "classical register index calculation overflowed",
                )
            }
        }
    }
}

impl std::error::Error for ClassicalRegisterError {}

// =============================================================================
// Operator validation
// =============================================================================

fn validate_unary(
    operation: ClassicalUnaryOp,
    operand: ClassicalType,
) -> Result<ClassicalType, ClassicalError> {
    if operation.requires_bool() {
        if operand != ClassicalType::Bool {
            return Err(ClassicalError::TypeMismatch {
                expected: "bool",
                found: match operand {
                    ClassicalType::Bool => "bool",
                    ClassicalType::Int => "int",
                    ClassicalType::UInt => "uint",
                },
            });
        }

        return Ok(ClassicalType::Bool);
    }

    if operation.requires_integer() {
        match operand {
            ClassicalType::Int | ClassicalType::UInt => {
                Ok(operand)
            }

            ClassicalType::Bool => Err(
                ClassicalError::TypeMismatch {
                    expected: "integer",
                    found: "bool",
                },
            ),
        }
    } else {
        Err(ClassicalError::InvalidExpression)
    }
}

fn validate_binary(
    operation: ClassicalBinaryOp,
    left: ClassicalType,
    right: ClassicalType,
) -> Result<ClassicalType, ClassicalError> {
    match operation {
        ClassicalBinaryOp::And | ClassicalBinaryOp::Or => {
            if left == ClassicalType::Bool
                && right == ClassicalType::Bool
            {
                Ok(ClassicalType::Bool)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::Equal | ClassicalBinaryOp::NotEqual => {
            if left == right {
                Ok(ClassicalType::Bool)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::LessThan
        | ClassicalBinaryOp::LessThanOrEqual
        | ClassicalBinaryOp::GreaterThan
        | ClassicalBinaryOp::GreaterThanOrEqual => {
            if left == right
                && matches!(
                    left,
                    ClassicalType::Int | ClassicalType::UInt
                )
            {
                Ok(ClassicalType::Bool)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::Add
        | ClassicalBinaryOp::Subtract
        | ClassicalBinaryOp::Multiply
        | ClassicalBinaryOp::Divide
        | ClassicalBinaryOp::Remainder => {
            if left == right
                && matches!(
                    left,
                    ClassicalType::Int | ClassicalType::UInt
                )
            {
                Ok(left)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::BitAnd
        | ClassicalBinaryOp::BitOr
        | ClassicalBinaryOp::BitXor => {
            if left == right
                && matches!(
                    left,
                    ClassicalType::Int | ClassicalType::UInt
                )
            {
                Ok(left)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }
    }
}

// =============================================================================
// Operator evaluation
// =============================================================================

fn evaluate_unary(
    operation: ClassicalUnaryOp,
    operand: ClassicalValue,
) -> Result<ClassicalValue, ClassicalError> {
    match operation {
        ClassicalUnaryOp::Not => match operand {
            ClassicalValue::Bool(value) => {
                Ok(ClassicalValue::Bool(!value))
            }

            other => Err(ClassicalError::TypeMismatch {
                expected: "bool",
                found: other.type_name(),
            }),
        },

        ClassicalUnaryOp::Negate => match operand {
            ClassicalValue::Int(value) => value
                .checked_neg()
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedOverflow),

            ClassicalValue::UInt(_) => Err(
                ClassicalError::TypeMismatch {
                    expected: "int",
                    found: "uint",
                },
            ),

            ClassicalValue::Bool(_) => Err(
                ClassicalError::TypeMismatch {
                    expected: "integer",
                    found: "bool",
                },
            ),
        },

        ClassicalUnaryOp::BitNot => match operand {
            ClassicalValue::Int(value) => {
                Ok(ClassicalValue::Int(!value))
            }

            ClassicalValue::UInt(value) => {
                Ok(ClassicalValue::UInt(!value))
            }

            ClassicalValue::Bool(_) => Err(
                ClassicalError::TypeMismatch {
                    expected: "integer",
                    found: "bool",
                },
            ),
        },
    }
}

fn evaluate_binary(
    operation: ClassicalBinaryOp,
    left: ClassicalValue,
    right: ClassicalValue,
) -> Result<ClassicalValue, ClassicalError> {
    match operation {
        ClassicalBinaryOp::And => match (left, right) {
            (ClassicalValue::Bool(a), ClassicalValue::Bool(b)) => {
                Ok(ClassicalValue::Bool(a && b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Or => match (left, right) {
            (ClassicalValue::Bool(a), ClassicalValue::Bool(b)) => {
                Ok(ClassicalValue::Bool(a || b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Equal => Ok(
            ClassicalValue::Bool(left == right)
        ),

        ClassicalBinaryOp::NotEqual => Ok(
            ClassicalValue::Bool(left != right)
        ),

        ClassicalBinaryOp::LessThan => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a < b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a < b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::LessThanOrEqual => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a <= b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a <= b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::GreaterThan => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a > b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a > b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::GreaterThanOrEqual => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a >= b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a >= b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Add => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => a
                .checked_add(b)
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedOverflow),

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => a
                .checked_add(b)
                .map(ClassicalValue::UInt)
                .ok_or(ClassicalError::UnsignedOverflow),

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Subtract => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => a
                .checked_sub(b)
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedUnderflow),

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => a
                .checked_sub(b)
                .map(ClassicalValue::UInt)
                .ok_or(ClassicalError::UnsignedUnderflow),

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Multiply => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => a
                .checked_mul(b)
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedOverflow),

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => a
                .checked_mul(b)
                .map(ClassicalValue::UInt)
                .ok_or(ClassicalError::UnsignedOverflow),

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Divide => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                if b == 0 {
                    return Err(ClassicalError::DivisionByZero);
                }

                a.checked_div(b)
                    .map(ClassicalValue::Int)
                    .ok_or(ClassicalError::SignedOverflow)
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                if b == 0 {
                    return Err(ClassicalError::DivisionByZero);
                }

                Ok(ClassicalValue::UInt(a / b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Remainder => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                if b == 0 {
                    return Err(ClassicalError::RemainderByZero);
                }

                a.checked_rem(b)
                    .map(ClassicalValue::Int)
                    .ok_or(ClassicalError::SignedOverflow)
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                if b == 0 {
                    return Err(ClassicalError::RemainderByZero);
                }

                Ok(ClassicalValue::UInt(a % b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::BitAnd => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Int(a & b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::UInt(a & b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::BitOr => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Int(a | b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::UInt(a | b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::BitXor => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Int(a ^ b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::UInt(a ^ b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },
    }
}

// =============================================================================
// Display implementations
// =============================================================================

impl fmt::Display for ClassicalValue {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(formatter, "{value}"),
            Self::Int(value) => write!(formatter, "{value}"),
            Self::UInt(value) => write!(formatter, "{value}"),
        }
    }
}

impl fmt::Display for ClassicalBinaryOp {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let symbol = match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Remainder => "%",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::And => "&&",
            Self::Or => "||",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
        };

        formatter.write_str(symbol)
    }
}

impl fmt::Display for ClassicalUnaryOp {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let symbol = match self {
            Self::Not => "!",
            Self::Negate => "-",
            Self::BitNot => "~",
        };

        formatter.write_str(symbol)
    }
}

impl fmt::Display for ClassicalExpression {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fn write_expression(
            expression: &ClassicalExpression,
            formatter: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            match expression {
                ClassicalExpression::Value(value) => {
                    write!(formatter, "{value}")
                }

                ClassicalExpression::Bit(bit) => {
                    write!(formatter, "{bit}")
                }

                ClassicalExpression::Unary { op, operand } => {
                    write!(formatter, "{op}(")?;
                    write_expression(operand, formatter)?;
                    formatter.write_str(")")
                }

                ClassicalExpression::Binary {
                    op,
                    left,
                    right,
                } => {
                    formatter.write_str("(")?;
                    write_expression(left, formatter)?;
                    write!(formatter, " {op} ")?;
                    write_expression(right, formatter)?;
                    formatter.write_str(")")
                }
            }
        }

        write_expression(self, formatter)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::qubit::QubitId;

    #[test]
    fn classical_bit_identity_is_stable() {
        let bit = ClassicalBitId::new(42);

        assert_eq!(bit.index(), 42);
        assert_eq!(bit.to_string(), "c42");
        assert_eq!(
            bit.checked_next(),
            Some(ClassicalBitId::new(43))
        );
    }

    #[test]
    fn classical_register_does_not_materialize_bits() {
        let register =
            ClassicalRegister::zero_based(1_000_000).unwrap();

        assert_eq!(register.len(), 1_000_000);
        assert_eq!(
            register.start(),
            ClassicalBitId::new(0)
        );
        assert_eq!(
            register.last(),
            Some(ClassicalBitId::new(999_999))
        );
        assert!(register.contains(ClassicalBitId::new(500_000)));
        assert!(!register.contains(ClassicalBitId::new(1_000_000)));
    }

    #[test]
    fn classical_register_iter_is_lazy() {
        let register =
            ClassicalRegister::zero_based(3).unwrap();

        let bits: Vec<_> = register.iter().collect();

        assert_eq!(
            bits,
            vec![
                ClassicalBitId::new(0),
                ClassicalBitId::new(1),
                ClassicalBitId::new(2),
            ]
        );
    }

    #[test]
    fn sparse_classical_set_is_deterministic() {
        let mut set = ClassicalBitSet::new();

        set.insert(ClassicalBitId::new(100));
        set.insert(ClassicalBitId::new(2));
        set.insert(ClassicalBitId::new(50));

        assert_eq!(
            set.to_vec(),
            vec![
                ClassicalBitId::new(2),
                ClassicalBitId::new(50),
                ClassicalBitId::new(100),
            ]
        );
    }

    #[test]
    fn classical_values_have_stable_types() {
        assert_eq!(
            ClassicalValue::Bool(true).classical_type(),
            ClassicalType::Bool
        );

        assert_eq!(
            ClassicalValue::Int(-1).classical_type(),
            ClassicalType::Int
        );

        assert_eq!(
            ClassicalValue::UInt(1).classical_type(),
            ClassicalType::UInt
        );
    }

    #[test]
    fn boolean_predicate_works() {
        let expression = ClassicalExpression::and(
            ClassicalExpression::bool(true),
            ClassicalExpression::bool(false),
        );

        let predicate =
            ClassicalPredicate::new(expression).unwrap();

        assert!(!predicate.evaluate(&|_| None).unwrap());
    }

    #[test]
    fn classical_bit_predicate_works() {
        let bit = ClassicalBitId::new(0);

        let predicate =
            ClassicalPredicate::equals(
                bit,
                ClassicalValue::UInt(1),
            )
            .unwrap();

        assert!(
            predicate
                .evaluate(&|requested| {
                    if requested == bit {
                        Some(ClassicalValue::UInt(1))
                    } else {
                        None
                    }
                })
                .unwrap()
        );
    }

    #[test]
    fn classical_expression_evaluation_is_checked() {
        let expression = ClassicalExpression::binary(
            ClassicalBinaryOp::Add,
            ClassicalExpression::int(10),
            ClassicalExpression::int(20),
        );

        assert_eq!(
            expression.evaluate(&|_| None).unwrap(),
            ClassicalValue::Int(30)
        );
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let expression = ClassicalExpression::binary(
            ClassicalBinaryOp::Divide,
            ClassicalExpression::int(10),
            ClassicalExpression::int(0),
        );

        assert_eq!(
            expression.evaluate(&|_| None),
            Err(ClassicalError::DivisionByZero)
        );
    }

    #[test]
    fn type_mismatch_is_rejected() {
        let expression = ClassicalExpression::binary(
            ClassicalBinaryOp::And,
            ClassicalExpression::int(1),
            ClassicalExpression::int(2),
        );

        assert!(matches!(
            expression.validate(),
            Err(ClassicalError::BinaryTypeMismatch {
                operation: ClassicalBinaryOp::And,
                ..
            })
        ));
    }

    #[test]
    fn measurement_destination_preserves_identity_domains() {
        let q = QubitId::new(7);
        let c = ClassicalBitId::new(3);

        let destination =
            MeasurementDestination::new(q, c);

        assert_eq!(destination.qubit(), q);
        assert_eq!(
            destination.classical_bit(),
            c
        );
    }

    #[test]
    fn expression_collects_unique_bits() {
        let c0 = ClassicalBitId::new(0);
        let c1 = ClassicalBitId::new(1);

        let expression = ClassicalExpression::and(
            ClassicalExpression::bit(c0),
            ClassicalExpression::or(
                ClassicalExpression::bit(c1),
                ClassicalExpression::bit(c0),
            ),
        );

        let bits = expression.collect_bits();

        assert_eq!(
            bits.to_vec(),
            vec![c0, c1]
        );
    }

    #[test]
    fn expression_depth_is_iterative() {
        let mut expression =
            ClassicalExpression::bool(true);

        for _ in 0..128 {
            expression =
                ClassicalExpression::not(expression);
        }

        assert_eq!(expression.depth(), 128);
        assert_eq!(
            expression.validate_with_depth_limit(128),
            Ok(ClassicalType::Bool)
        );
        assert!(matches!(
            expression.validate_with_depth_limit(127),
            Err(ClassicalError::ExpressionDepthExceeded {
                depth: 128,
                maximum: 127
            })
        ));
    }

    #[test]
    fn resource_requirement_has_no_machine_size_limit() {
        let requirement =
            ClassicalResourceRequirement::new(usize::MAX);

        assert_eq!(requirement.count(), usize::MAX);
        assert!(requirement.fits_within(usize::MAX));
        assert!(!requirement.fits_within(
            usize::MAX.saturating_sub(1)
        ));
    }
}