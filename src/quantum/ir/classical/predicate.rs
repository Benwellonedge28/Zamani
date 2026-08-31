//! Zamani Quantum IR — Classical Predicate Semantics
//!
//! Production-grade, hardware-independent representation of predicates used
//! by classical control in the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `predicate.rs` owns the semantic representation of Boolean conditions used
//! by hybrid classical/quantum programs.
//!
//! It represents:
//!
//! - Boolean constants;
//! - classical-bit tests;
//! - typed scalar comparisons;
//! - equality and inequality;
//! - ordering comparisons;
//! - Boolean NOT;
//! - AND;
//! - OR;
//! - XOR;
//! - implication;
//! - equivalence;
//! - membership in finite sets;
//! - explicit predicate operands;
//! - deterministic structural validation;
//! - bounded structural analysis;
//! - deterministic formatting;
//! - simplification of semantics-preserving Boolean structure.
//!
//! It does NOT own:
//!
//! - classical expression parsing;
//! - source-language syntax;
//! - classical register allocation;
//! - quantum measurement;
//! - quantum state;
//! - physical qubits;
//! - routing;
//! - scheduling;
//! - backend execution;
//! - hardware topology;
//! - hardware capabilities;
//! - simulator state;
//! - optimization policy;
//! - QEC decoding.
//!
//! Those responsibilities belong to the appropriate IR/downstream modules.
//!
//! # Canonical classical identity
//!
//! The canonical classical-bit identity is:
//!
//! ```text
//! quantum::ir::classical::bit::ClassicalBitId
//! ```
//!
//! This module intentionally does not define another classical-bit identity.
//!
//! # Quantum integration
//!
//! Classical predicates commonly control quantum operations:
//!
//! ```text
//! measure(q0) -> c0
//!
//! predicate:
//!     c0 == 1
//!
//! controlled operation:
//!     x(q1)
//! ```
//!
//! The predicate therefore consumes the classical result of a measurement.
//! It does not directly own or inspect quantum state.
//!
//! In particular, this module deliberately does not import `QubitId`.
//! `quantum::ir::qubit::QubitId` remains the canonical logical-qubit identity,
//! while measurement and control-flow modules connect quantum operations to
//! classical predicates.
//!
//! # Universal-program principle
//!
//! A Zamani program must be expressible independently of the eventual quantum
//! machine.
//!
//! Therefore this module contains no limits such as:
//!
//! ```text
//! 63
//! 64
//! 127
//! 256
//! 4096
//! 1_000_000
//! ```
//!
//! as semantic limits.
//!
//! Predicate size is determined by the program and by explicit compilation
//! or security policies supplied by the caller.
//!
//! The same predicate model therefore applies to:
//!
//! ```text
//! one classical bit
//! thousands of classical bits
//! millions of classical bits
//! arbitrarily large finite classical namespaces
//! ```
//!
//! subject only to available resources and explicit safety policies.
//!
//! # Design principle
//!
//! A predicate answers:
//!
//! ```text
//! WHAT condition must be true?
//! ```
//!
//! It does not answer:
//!
//! ```text
//! WHERE is the condition evaluated?
//! WHEN is it evaluated?
//! HOW does hardware evaluate it?
//! ```
//!
//! Those decisions belong to later compiler/runtime/backend layers.
//!
//! # Expression boundary
//!
//! This file intentionally owns predicate semantics rather than the complete
//! classical expression language.
//!
//! A classical expression may eventually produce a value which is consumed by
//! a predicate.
//!
//! The dependency direction is therefore:
//!
//! ```text
//! classical expression
//!          │
//!          ▼
//! predicate operand
//!          │
//!          ▼
//! ClassicalPredicate
//!          │
//!          ▼
//! control flow
//! ```
//!
//! `expression.rs` can construct [`PredicateOperand`] and
//! [`ClassicalPredicate`] without requiring this module to depend on the
//! expression AST.
//!
//! # OpenQASM compatibility
//!
//! The representation is intentionally capable of expressing the semantic
//! class of conditions required by modern quantum languages, including:
//!
//! - boolean logic;
//! - bit comparison;
//! - register/value comparison;
//! - integer comparison;
//! - ordered comparison;
//! - equality/inequality;
//! - finite-set membership.
//!
//! It is not an OpenQASM AST. OpenQASM has its own frontend AST and is lowered
//! into this canonical representation.
//!
//! # Determinism
//!
//! Predicate nodes use ordered collections where ordering is semantically
//! observable or useful for canonical serialization.
//!
//! The type therefore avoids unordered collections in semantic predicate
//! storage.
//!
//! # Validation
//!
//! Structural validation is always available without requiring hardware.
//!
//! Resource-sensitive validation is controlled by [`PredicateValidationLimits`].
//!
//! This is important because malformed or hostile IR must not be able to force
//! unbounded recursive traversal without an explicit policy boundary.
//!
//! # Security
//!
//! This file:
//!
//! - uses checked arithmetic;
//! - rejects non-finite floating-point predicate literals;
//! - validates recursive depth;
//! - validates node count;
//! - validates membership-set size;
//! - contains no unsafe code;
//! - performs no I/O;
//! - performs no dynamic code execution;
//! - has no global mutable state.
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

use std::fmt;

use super::bit::ClassicalBitId;
use super::super::identity::ValueId;

// =============================================================================
// Result type
// =============================================================================

/// Result type used by classical-predicate construction and validation.
pub type PredicateResult<T> = Result<T, PredicateError>;

// =============================================================================
// Floating-point semantic wrapper
// =============================================================================

/// A finite IEEE-754 floating-point literal used by predicates.
///
/// NaN and positive/negative infinity are rejected because they do not provide
/// a stable total ordering suitable for canonical predicate semantics.
#[derive(Debug, Clone, Copy)]
pub struct PredicateFloat {
    bits: u64,
}

impl PredicateFloat {
    /// Creates a finite predicate floating-point value.
    pub fn new(value: f64) -> PredicateResult<Self> {
        if !value.is_finite() {
            return Err(PredicateError::NonFiniteFloat);
        }

        Ok(Self {
            bits: value.to_bits(),
        })
    }

    /// Returns the represented floating-point value.
    #[must_use]
    pub fn value(self) -> f64 {
        f64::from_bits(self.bits)
    }

    /// Returns the canonical IEEE-754 bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.bits
    }
}

impl PartialEq for PredicateFloat {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl Eq for PredicateFloat {}

impl std::hash::Hash for PredicateFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

impl PartialOrd for PredicateFloat {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

// =============================================================================
// Boolean value
// =============================================================================

/// Canonical Boolean value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PredicateBool {
    /// Boolean false.
    False,

    /// Boolean true.
    True,
}

impl PredicateBool {
    /// Returns the native Rust Boolean.
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::False => false,
            Self::True => true,
        }
    }

    /// Returns the logical negation.
    #[must_use]
    pub const fn not(self) -> Self {
        match self {
            Self::False => Self::True,
            Self::True => Self::False,
        }
    }
}

impl From<bool> for PredicateBool {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<PredicateBool> for bool {
    fn from(value: PredicateBool) -> Self {
        value.as_bool()
    }
}

// =============================================================================
// Bit-vector literal
// =============================================================================

/// Immutable logical bit-vector literal.
///
/// Bits are stored in logical least-significant-bit-first order:
///
/// ```text
/// BitVector([false, true, false])
/// ```
///
/// represents the three-bit value whose bit 1 is set.
///
/// The vector is explicit data and therefore scales with the requested
/// representation rather than imposing a fixed machine width.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PredicateBitVector {
    bits: Vec<bool>,
}

impl PredicateBitVector {
    /// Creates a bit-vector from logical bits.
    ///
    /// No architectural width is imposed.
    #[must_use]
    pub fn new(bits: Vec<bool>) -> Self {
        Self { bits }
    }

    /// Creates a bit-vector from an iterator.
    #[must_use]
    pub fn from_bits<I>(bits: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Self {
            bits: bits.into_iter().collect(),
        }
    }

    /// Creates a bit-vector containing `width` zero bits.
    ///
    /// The allocation is explicit and proportional to the requested value
    /// representation.
    #[must_use]
    pub fn zeros(width: usize) -> Self {
        Self {
            bits: vec![false; width],
        }
    }

    /// Creates a bit-vector containing `width` one bits.
    #[must_use]
    pub fn ones(width: usize) -> Self {
        Self {
            bits: vec![true; width],
        }
    }

    /// Returns the number of bits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the vector contains no bits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns one bit by logical index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<bool> {
        self.bits.get(index).copied()
    }

    /// Returns the bits in logical order.
    pub fn iter(&self) -> impl Iterator<Item = &bool> {
        self.bits.iter()
    }

    /// Returns an owned copy of the logical bits.
    #[must_use]
    pub fn to_vec(&self) -> Vec<bool> {
        self.bits.clone()
    }
}

// =============================================================================
// Predicate operand
// =============================================================================

/// Semantic value that may participate in a classical predicate.
///
/// This is deliberately smaller than a complete classical-expression AST.
/// Complex arithmetic and expression construction belong to `expression.rs`.
///
/// `Value(ValueId)` permits predicates to compare SSA/IR values without
/// requiring this file to know how those values are produced.
///
/// `ClassicalBit` directly represents the extremely common dynamic-circuit
/// case:
///
/// ```text
/// if c0 == 1
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateOperand {
    /// Boolean literal.
    Bool(PredicateBool),

    /// Signed integer literal.
    SignedInteger(i128),

    /// Unsigned integer literal.
    UnsignedInteger(u128),

    /// Finite floating-point literal.
    Float(PredicateFloat),

    /// Logical classical bit.
    ClassicalBit(ClassicalBitId),

    /// Bit-vector literal.
    BitVector(PredicateBitVector),

    /// Reference to an already-defined IR value.
    Value(ValueId),
}

impl PredicateOperand {
    /// Creates a Boolean operand.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(if value {
            PredicateBool::True
        } else {
            PredicateBool::False
        })
    }

    /// Creates a signed integer operand.
    #[must_use]
    pub const fn signed(value: i128) -> Self {
        Self::SignedInteger(value)
    }

    /// Creates an unsigned integer operand.
    #[must_use]
    pub const fn unsigned(value: u128) -> Self {
        Self::UnsignedInteger(value)
    }

    /// Creates a finite floating-point operand.
    pub fn float(value: f64) -> PredicateResult<Self> {
        Ok(Self::Float(PredicateFloat::new(value)?))
    }

    /// Creates a classical-bit operand.
    #[must_use]
    pub const fn classical_bit(bit: ClassicalBitId) -> Self {
        Self::ClassicalBit(bit)
    }

    /// Creates a bit-vector operand.
    #[must_use]
    pub fn bit_vector(bits: Vec<bool>) -> Self {
        Self::BitVector(PredicateBitVector::new(bits))
    }

    /// Creates an IR-value operand.
    #[must_use]
    pub const fn value(value: ValueId) -> Self {
        Self::Value(value)
    }

    /// Returns the broad operand kind.
    #[must_use]
    pub const fn kind(&self) -> PredicateOperandKind {
        match self {
            Self::Bool(_) => PredicateOperandKind::Bool,
            Self::SignedInteger(_) => PredicateOperandKind::SignedInteger,
            Self::UnsignedInteger(_) => PredicateOperandKind::UnsignedInteger,
            Self::Float(_) => PredicateOperandKind::Float,
            Self::ClassicalBit(_) => PredicateOperandKind::ClassicalBit,
            Self::BitVector(_) => PredicateOperandKind::BitVector,
            Self::Value(_) => PredicateOperandKind::Value,
        }
    }

    /// Returns the referenced classical bit when this is a bit operand.
    #[must_use]
    pub const fn classical_bit_id(&self) -> Option<ClassicalBitId> {
        match self {
            Self::ClassicalBit(bit) => Some(*bit),
            _ => None,
        }
    }

    /// Returns the referenced IR value when this is an IR-value operand.
    #[must_use]
    pub const fn value_id(&self) -> Option<ValueId> {
        match self {
            Self::Value(value) => Some(*value),
            _ => None,
        }
    }
}

/// Broad semantic category of a predicate operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PredicateOperandKind {
    /// Boolean.
    Bool,

    /// Signed integer.
    SignedInteger,

    /// Unsigned integer.
    UnsignedInteger,

    /// Floating point.
    Float,

    /// Classical bit.
    ClassicalBit,

    /// Bit vector.
    BitVector,

    /// Existing IR value.
    Value,
}

impl fmt::Display for PredicateOperandKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Bool => "bool",
            Self::SignedInteger => "signed_integer",
            Self::UnsignedInteger => "unsigned_integer",
            Self::Float => "float",
            Self::ClassicalBit => "classical_bit",
            Self::BitVector => "bit_vector",
            Self::Value => "value",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Comparison operator
// =============================================================================

/// Relational comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComparisonOperator {
    /// Equality.
    Equal,

    /// Inequality.
    NotEqual,

    /// Strictly less than.
    LessThan,

    /// Less than or equal.
    LessOrEqual,

    /// Strictly greater than.
    GreaterThan,

    /// Greater than or equal.
    GreaterOrEqual,
}

impl ComparisonOperator {
    /// Returns the logical inverse comparison operator.
    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::LessThan => Self::GreaterOrEqual,
            Self::LessOrEqual => Self::GreaterThan,
            Self::GreaterThan => Self::LessOrEqual,
            Self::GreaterOrEqual => Self::LessThan,
        }
    }

    /// Returns whether this operator is an equality-family comparison.
    #[must_use]
    pub const fn is_equality(self) -> bool {
        matches!(self, Self::Equal | Self::NotEqual)
    }

    /// Returns whether this operator is an ordering comparison.
    #[must_use]
    pub const fn is_ordering(self) -> bool {
        matches!(
            self,
            Self::LessThan
                | Self::LessOrEqual
                | Self::GreaterThan
                | Self::GreaterOrEqual
        )
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let symbol = match self {
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::LessOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterOrEqual => ">=",
        };

        formatter.write_str(symbol)
    }
}

// =============================================================================
// Logical operator
// =============================================================================

/// N-ary logical predicate operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalOperator {
    /// Conjunction.
    And,

    /// Disjunction.
    Or,

    /// Exclusive disjunction.
    Xor,
}

impl fmt::Display for LogicalOperator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let symbol = match self {
            Self::And => "&&",
            Self::Or => "||",
            Self::Xor => "xor",
        };

        formatter.write_str(symbol)
    }
}

// =============================================================================
// Classical predicate
// =============================================================================

/// Canonical Boolean predicate used by classical and quantum control flow.
///
/// The representation is deliberately target-independent.
///
/// Examples:
///
/// ```text
/// c0
/// !c0
/// c0 == 1
/// c0 != 0
/// x >= 10
/// x < y
/// x in {1, 2, 3}
/// (c0 && c1) || c2
/// !(x == 0)
/// ```
///
/// Logical operators are represented as vectors so the IR can preserve
/// n-ary structure without artificially restricting predicates to binary
/// trees.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalPredicate {
    /// Constant Boolean predicate.
    Constant(PredicateBool),

    /// Tests whether a classical bit is logically true.
    Bit(ClassicalBitId),

    /// Compares two operands.
    Compare {
        /// Left-hand operand.
        left: PredicateOperand,

        /// Comparison operator.
        operator: ComparisonOperator,

        /// Right-hand operand.
        right: PredicateOperand,
    },

    /// Logical negation.
    Not(Box<Self>),

    /// N-ary conjunction.
    And(Vec<Self>),

    /// N-ary disjunction.
    Or(Vec<Self>),

    /// N-ary exclusive disjunction.
    Xor(Vec<Self>),

    /// Logical implication.
    Implies {
        /// Antecedent.
        antecedent: Box<Self>,

        /// Consequent.
        consequent: Box<Self>,
    },

    /// Logical equivalence.
    Equivalent {
        /// Left predicate.
        left: Box<Self>,

        /// Right predicate.
        right: Box<Self>,
    },

    /// Membership test.
    ///
    /// Semantically:
    ///
    /// ```text
    /// value in {candidate_0, candidate_1, ...}
    /// ```
    InSet {
        /// Value being tested.
        value: PredicateOperand,

        /// Ordered candidate set.
        candidates: Vec<PredicateOperand>,
    },
}

impl ClassicalPredicate {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    /// Creates `true`.
    #[must_use]
    pub const fn always() -> Self {
        Self::Constant(PredicateBool::True)
    }

    /// Creates `false`.
    #[must_use]
    pub const fn never() -> Self {
        Self::Constant(PredicateBool::False)
    }

    /// Creates a Boolean constant.
    #[must_use]
    pub const fn constant(value: bool) -> Self {
        if value {
            Self::always()
        } else {
            Self::never()
        }
    }

    // -------------------------------------------------------------------------
    // Classical bit
    // -------------------------------------------------------------------------

    /// Creates a predicate testing whether a classical bit is one/true.
    #[must_use]
    pub const fn bit(bit: ClassicalBitId) -> Self {
        Self::Bit(bit)
    }

    /// Creates `bit == value`.
    #[must_use]
    pub const fn bit_equals(
        bit: ClassicalBitId,
        value: bool,
    ) -> Self {
        Self::Compare {
            left: PredicateOperand::ClassicalBit(bit),
            operator: ComparisonOperator::Equal,
            right: PredicateOperand::Bool(if value {
                PredicateBool::True
            } else {
                PredicateBool::False
            }),
        }
    }

    /// Creates `bit != value`.
    #[must_use]
    pub const fn bit_not_equals(
        bit: ClassicalBitId,
        value: bool,
    ) -> Self {
        Self::Compare {
            left: PredicateOperand::ClassicalBit(bit),
            operator: ComparisonOperator::NotEqual,
            right: PredicateOperand::Bool(if value {
                PredicateBool::True
            } else {
                PredicateBool::False
            }),
        }
    }

    // -------------------------------------------------------------------------
    // Comparison
    // -------------------------------------------------------------------------

    /// Creates a comparison predicate.
    ///
    /// No target-specific type coercion is performed here. Type checking and
    /// language-specific conversion rules belong to the validation/type system.
    pub fn compare(
        left: PredicateOperand,
        operator: ComparisonOperator,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::validate_comparison_operands(&left, operator, &right)?;

        Ok(Self::Compare {
            left,
            operator,
            right,
        })
    }

    /// Creates an equality predicate.
    pub fn equal(
        left: PredicateOperand,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::compare(left, ComparisonOperator::Equal, right)
    }

    /// Creates an inequality predicate.
    pub fn not_equal(
        left: PredicateOperand,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::compare(left, ComparisonOperator::NotEqual, right)
    }

    /// Creates a less-than predicate.
    pub fn less_than(
        left: PredicateOperand,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::compare(left, ComparisonOperator::LessThan, right)
    }

    /// Creates a less-than-or-equal predicate.
    pub fn less_or_equal(
        left: PredicateOperand,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::compare(left, ComparisonOperator::LessOrEqual, right)
    }

    /// Creates a greater-than predicate.
    pub fn greater_than(
        left: PredicateOperand,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::compare(left, ComparisonOperator::GreaterThan, right)
    }

    /// Creates a greater-than-or-equal predicate.
    pub fn greater_or_equal(
        left: PredicateOperand,
        right: PredicateOperand,
    ) -> PredicateResult<Self> {
        Self::compare(left, ComparisonOperator::GreaterOrEqual, right)
    }

    // -------------------------------------------------------------------------
    // Logical operators
    // -------------------------------------------------------------------------

    /// Creates a logical NOT.
    #[must_use]
    pub fn not(predicate: Self) -> Self {
        Self::Not(Box::new(predicate))
    }

    /// Creates an n-ary AND.
    ///
    /// At least one predicate is required.
    pub fn and(
        predicates: Vec<Self>,
    ) -> PredicateResult<Self> {
        Self::validate_non_empty_terms(&predicates)?;

        Ok(Self::And(predicates))
    }

    /// Creates an n-ary OR.
    ///
    /// At least one predicate is required.
    pub fn or(
        predicates: Vec<Self>,
    ) -> PredicateResult<Self> {
        Self::validate_non_empty_terms(&predicates)?;

        Ok(Self::Or(predicates))
    }

    /// Creates an n-ary XOR.
    ///
    /// At least one predicate is required.
    pub fn xor(
        predicates: Vec<Self>,
    ) -> PredicateResult<Self> {
        Self::validate_non_empty_terms(&predicates)?;

        Ok(Self::Xor(predicates))
    }

    /// Creates logical implication.
    #[must_use]
    pub fn implies(
        antecedent: Self,
        consequent: Self,
    ) -> Self {
        Self::Implies {
            antecedent: Box::new(antecedent),
            consequent: Box::new(consequent),
        }
    }

    /// Creates logical equivalence.
    #[must_use]
    pub fn equivalent(
        left: Self,
        right: Self,
    ) -> Self {
        Self::Equivalent {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    // -------------------------------------------------------------------------
    // Membership
    // -------------------------------------------------------------------------

    /// Creates an `in` membership predicate.
    ///
    /// The candidate list must not be empty and must not contain duplicate
    /// semantic operands.
    pub fn in_set(
        value: PredicateOperand,
        candidates: Vec<PredicateOperand>,
    ) -> PredicateResult<Self> {
        if candidates.is_empty() {
            return Err(PredicateError::EmptyMembershipSet);
        }

        for candidate in &candidates {
            Self::validate_membership_operands(&value, candidate)?;
        }

        for index in 0..candidates.len() {
            for other in (index + 1)..candidates.len() {
                if candidates[index] == candidates[other] {
                    return Err(PredicateError::DuplicateMembershipCandidate {
                        index,
                        duplicate_index: other,
                    });
                }
            }
        }

        Ok(Self::InSet { value, candidates })
    }

    // -------------------------------------------------------------------------
    // Structural information
    // -------------------------------------------------------------------------

    /// Returns the root predicate kind.
    #[must_use]
    pub const fn kind(&self) -> PredicateKind {
        match self {
            Self::Constant(_) => PredicateKind::Constant,
            Self::Bit(_) => PredicateKind::Bit,
            Self::Compare { .. } => PredicateKind::Compare,
            Self::Not(_) => PredicateKind::Not,
            Self::And(_) => PredicateKind::And,
            Self::Or(_) => PredicateKind::Or,
            Self::Xor(_) => PredicateKind::Xor,
            Self::Implies { .. } => PredicateKind::Implies,
            Self::Equivalent { .. } => PredicateKind::Equivalent,
            Self::InSet { .. } => PredicateKind::InSet,
        }
    }

    /// Returns whether this predicate is a constant.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Returns the constant value if this predicate is constant.
    #[must_use]
    pub const fn constant_value(&self) -> Option<bool> {
        match self {
            Self::Constant(value) => Some(value.as_bool()),
            _ => None,
        }
    }

    /// Returns the maximum structural depth.
    ///
    /// Depth is measured from the root as depth one.
    ///
    /// This method does not impose a maximum; callers that need bounded
    /// validation should use [`Self::validate_with_limits`].
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::Constant(_) | Self::Bit(_) | Self::Compare { .. } => 1,

            Self::Not(predicate) => 1usize.saturating_add(predicate.depth()),

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                let child_depth = predicates
                    .iter()
                    .map(Self::depth)
                    .max()
                    .unwrap_or(0);

                1usize.saturating_add(child_depth)
            }

            Self::Implies {
                antecedent,
                consequent,
            }
            | Self::Equivalent {
                left: antecedent,
                right: consequent,
            } => {
                let child_depth = antecedent
                    .depth()
                    .max(consequent.depth());

                1usize.saturating_add(child_depth)
            }

            Self::InSet { .. } => 1,
        }
    }

    /// Returns the number of predicate nodes.
    ///
    /// This operation is structural analysis and does not impose a semantic
    /// maximum.
    #[must_use]
    pub fn node_count(&self) -> usize {
        match self {
            Self::Constant(_) | Self::Bit(_) | Self::Compare { .. } => 1,

            Self::Not(predicate) => {
                1usize.saturating_add(predicate.node_count())
            }

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                predicates.iter().fold(1usize, |total, predicate| {
                    total.saturating_add(predicate.node_count())
                })
            }

            Self::Implies {
                antecedent,
                consequent,
            }
            | Self::Equivalent {
                left: antecedent,
                right: consequent,
            } => 1usize
                .saturating_add(antecedent.node_count())
                .saturating_add(consequent.node_count()),

            Self::InSet { .. } => 1,
        }
    }

    /// Returns the number of distinct classical bits directly referenced.
    ///
    /// The returned count is structural and may count the same bit only once.
    pub fn referenced_classical_bits(
        &self,
    ) -> std::collections::BTreeSet<ClassicalBitId> {
        let mut result = std::collections::BTreeSet::new();
        self.collect_classical_bits(&mut result);
        result
    }

    /// Returns whether the predicate directly references a classical bit.
    #[must_use]
    pub fn references_classical_bit(
        &self,
        bit: ClassicalBitId,
    ) -> bool {
        self.referenced_classical_bits().contains(&bit)
    }

    /// Returns whether the predicate contains an IR value reference.
    #[must_use]
    pub fn contains_value_reference(&self) -> bool {
        match self {
            Self::Constant(_) | Self::Bit(_) => false,

            Self::Compare { left, right, .. } => {
                Self::operand_contains_value_reference(left)
                    || Self::operand_contains_value_reference(right)
            }

            Self::Not(predicate) => predicate.contains_value_reference(),

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => predicates
                .iter()
                .any(Self::contains_value_reference),

            Self::Implies {
                antecedent,
                consequent,
            }
            | Self::Equivalent {
                left: antecedent,
                right: consequent,
            } => {
                antecedent.contains_value_reference()
                    || consequent.contains_value_reference()
            }

            Self::InSet { value, candidates } => {
                Self::operand_contains_value_reference(value)
                    || candidates
                        .iter()
                        .any(Self::operand_contains_value_reference)
            }
        }
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates using the default structural policy.
    ///
    /// The default policy is deliberately conservative enough to protect a
    /// compiler from malformed recursive IR while not defining an architectural
    /// limit on Zamani programs.
    pub fn validate(&self) -> PredicateResult<()> {
        self.validate_with_limits(&PredicateValidationLimits::default())
    }

    /// Validates this predicate under an explicit resource/security policy.
    pub fn validate_with_limits(
        &self,
        limits: &PredicateValidationLimits,
    ) -> PredicateResult<()> {
        limits.validate()?;

        let mut state = ValidationState {
            nodes: 0,
            membership_candidates: 0,
        };

        self.validate_recursive(1, limits, &mut state)
    }

    /// Performs a semantic simplification that preserves predicate meaning.
    ///
    /// This method performs only local, deterministic transformations. It is
    /// not a general optimizer.
    #[must_use]
    pub fn simplify(self) -> Self {
        match self {
            Self::Constant(value) => Self::Constant(value),

            Self::Bit(bit) => Self::Bit(bit),

            Self::Compare {
                left,
                operator,
                right,
            } => {
                if left == right {
                    return Self::constant(match operator {
                        ComparisonOperator::Equal
                        | ComparisonOperator::LessOrEqual
                        | ComparisonOperator::GreaterOrEqual => true,

                        ComparisonOperator::NotEqual
                        | ComparisonOperator::LessThan
                        | ComparisonOperator::GreaterThan => false,
                    });
                }

                Self::Compare {
                    left,
                    operator,
                    right,
                }
            }

            Self::Not(predicate) => {
                let predicate = predicate.simplify();

                match predicate {
                    Self::Constant(value) => {
                        Self::Constant(value.not())
                    }

                    Self::Not(inner) => *inner,

                    Self::Compare {
                        left,
                        operator,
                        right,
                    } => Self::Compare {
                        left,
                        operator: operator.inverse(),
                        right,
                    },

                    other => Self::Not(Box::new(other)),
                }
            }

            Self::And(predicates) => {
                let mut simplified = Vec::with_capacity(predicates.len());

                for predicate in predicates {
                    let predicate = predicate.simplify();

                    match predicate {
                        Self::Constant(PredicateBool::False) => {
                            return Self::never();
                        }

                        Self::Constant(PredicateBool::True) => {}

                        other => simplified.push(other),
                    }
                }

                match simplified.len() {
                    0 => Self::always(),
                    1 => simplified
                        .pop()
                        .unwrap_or_else(Self::always),
                    _ => Self::And(simplified),
                }
            }

            Self::Or(predicates) => {
                let mut simplified = Vec::with_capacity(predicates.len());

                for predicate in predicates {
                    let predicate = predicate.simplify();

                    match predicate {
                        Self::Constant(PredicateBool::True) => {
                            return Self::always();
                        }

                        Self::Constant(PredicateBool::False) => {}

                        other => simplified.push(other),
                    }
                }

                match simplified.len() {
                    0 => Self::never(),
                    1 => simplified
                        .pop()
                        .unwrap_or_else(Self::never),
                    _ => Self::Or(simplified),
                }
            }

            Self::Xor(predicates) => {
                let mut simplified = Vec::with_capacity(predicates.len());
                let mut parity = false;

                for predicate in predicates {
                    match predicate.simplify() {
                        Self::Constant(value) => {
                            parity ^= value.as_bool();
                        }

                        other => simplified.push(other),
                    }
                }

                if parity {
                    simplified.push(Self::always());
                }

                match simplified.len() {
                    0 => Self::never(),

                    1 => simplified
                        .pop()
                        .unwrap_or_else(Self::never),

                    _ => Self::Xor(simplified),
                }
            }

            Self::Implies {
                antecedent,
                consequent,
            } => {
                let antecedent = antecedent.simplify();
                let consequent = consequent.simplify();

                match (&antecedent, &consequent) {
                    (Self::Constant(PredicateBool::False), _)
                    | (_, Self::Constant(PredicateBool::True)) => {
                        Self::always()
                    }

                    (Self::Constant(PredicateBool::True), _) => {
                        consequent
                    }

                    (_, Self::Constant(PredicateBool::False)) => {
                        Self::not(antecedent).simplify()
                    }

                    _ => Self::Implies {
                        antecedent: Box::new(antecedent),
                        consequent: Box::new(consequent),
                    },
                }
            }

            Self::Equivalent { left, right } => {
                let left = left.simplify();
                let right = right.simplify();

                if left == right {
                    return Self::always();
                }

                match (&left, &right) {
                    (
                        Self::Constant(left_value),
                        Self::Constant(right_value),
                    ) => Self::constant(
                        left_value.as_bool() == right_value.as_bool(),
                    ),

                    _ => Self::Equivalent {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }

            Self::InSet {
                value,
                candidates,
            } => Self::InSet {
                value,
                candidates,
            },
        }
    }

    // -------------------------------------------------------------------------
    // Private validation
    // -------------------------------------------------------------------------

    fn validate_recursive(
        &self,
        depth: usize,
        limits: &PredicateValidationLimits,
        state: &mut ValidationState,
    ) -> PredicateResult<()> {
        if depth > limits.max_depth {
            return Err(PredicateError::DepthLimitExceeded {
                depth,
                maximum: limits.max_depth,
            });
        }

        state.nodes = state
            .nodes
            .checked_add(1)
            .ok_or(PredicateError::NodeCountOverflow)?;

        if state.nodes > limits.max_nodes {
            return Err(PredicateError::NodeLimitExceeded {
                nodes: state.nodes,
                maximum: limits.max_nodes,
            });
        }

        match self {
            Self::Constant(_) | Self::Bit(_) => Ok(()),

            Self::Compare {
                left,
                operator,
                right,
            } => {
                Self::validate_comparison_operands(
                    left,
                    *operator,
                    right,
                )
            }

            Self::Not(predicate) => {
                predicate.validate_recursive(
                    depth.saturating_add(1),
                    limits,
                    state,
                )
            }

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                Self::validate_non_empty_terms(predicates)?;

                for predicate in predicates {
                    predicate.validate_recursive(
                        depth.saturating_add(1),
                        limits,
                        state,
                    )?;
                }

                Ok(())
            }

            Self::Implies {
                antecedent,
                consequent,
            }
            | Self::Equivalent {
                left: antecedent,
                right: consequent,
            } => {
                antecedent.validate_recursive(
                    depth.saturating_add(1),
                    limits,
                    state,
                )?;

                consequent.validate_recursive(
                    depth.saturating_add(1),
                    limits,
                    state,
                )
            }

            Self::InSet { value, candidates } => {
                if candidates.is_empty() {
                    return Err(PredicateError::EmptyMembershipSet);
                }

                if candidates.len() > limits.max_membership_candidates {
                    return Err(
                        PredicateError::MembershipLimitExceeded {
                            requested: candidates.len(),
                            maximum: limits.max_membership_candidates,
                        },
                    );
                }

                state.membership_candidates = state
                    .membership_candidates
                    .checked_add(candidates.len())
                    .ok_or(
                        PredicateError::MembershipCountOverflow,
                    )?;

                for candidate in candidates {
                    Self::validate_membership_operands(
                        value,
                        candidate,
                    )?;
                }

                for index in 0..candidates.len() {
                    for other in (index + 1)..candidates.len() {
                        if candidates[index] == candidates[other] {
                            return Err(
                                PredicateError::DuplicateMembershipCandidate {
                                    index,
                                    duplicate_index: other,
                                },
                            );
                        }
                    }
                }

                Ok(())
            }
        }
    }

    fn validate_non_empty_terms(
        predicates: &[Self],
    ) -> PredicateResult<()> {
        if predicates.is_empty() {
            return Err(PredicateError::EmptyLogicalOperandList);
        }

        Ok(())
    }

    fn validate_comparison_operands(
        left: &PredicateOperand,
        operator: ComparisonOperator,
        right: &PredicateOperand,
    ) -> PredicateResult<()> {
        let left_kind = left.kind();
        let right_kind = right.kind();

        // Value references are intentionally unresolved at this level.
        //
        // Their actual types are supplied by the enclosing IR/program
        // validation pass.
        if matches!(left_kind, PredicateOperandKind::Value)
            || matches!(right_kind, PredicateOperandKind::Value)
        {
            return Ok(());
        }

        let compatible = match (left, right) {
            (PredicateOperand::Bool(_), PredicateOperand::Bool(_)) => {
                operator.is_equality()
            }

            (
                PredicateOperand::SignedInteger(_),
                PredicateOperand::SignedInteger(_),
            )
            | (
                PredicateOperand::UnsignedInteger(_),
                PredicateOperand::UnsignedInteger(_),
            )
            | (
                PredicateOperand::Float(_),
                PredicateOperand::Float(_),
            ) => true,

            (
                PredicateOperand::ClassicalBit(_),
                PredicateOperand::Bool(_),
            )
            | (
                PredicateOperand::Bool(_),
                PredicateOperand::ClassicalBit(_),
            ) => operator.is_equality(),

            (
                PredicateOperand::ClassicalBit(_),
                PredicateOperand::ClassicalBit(_),
            ) => operator.is_equality(),

            (
                PredicateOperand::BitVector(left),
                PredicateOperand::BitVector(right),
            ) => {
                left.len() == right.len()
                    && operator.is_equality()
            }

            _ => false,
        };

        if !compatible {
            return Err(PredicateError::IncompatibleOperands {
                left: left_kind,
                operator,
                right: right_kind,
            });
        }

        Ok(())
    }

    fn validate_membership_operands(
        value: &PredicateOperand,
        candidate: &PredicateOperand,
    ) -> PredicateResult<()> {
        let value_kind = value.kind();
        let candidate_kind = candidate.kind();

        if matches!(value_kind, PredicateOperandKind::Value)
            || matches!(candidate_kind, PredicateOperandKind::Value)
        {
            return Ok(());
        }

        let compatible = match (value, candidate) {
            (PredicateOperand::Bool(_), PredicateOperand::Bool(_)) => {
                true
            }

            (
                PredicateOperand::SignedInteger(_),
                PredicateOperand::SignedInteger(_),
            )
            | (
                PredicateOperand::UnsignedInteger(_),
                PredicateOperand::UnsignedInteger(_),
            )
            | (
                PredicateOperand::Float(_),
                PredicateOperand::Float(_),
            )
            | (
                PredicateOperand::ClassicalBit(_),
                PredicateOperand::ClassicalBit(_),
            ) => true,

            (
                PredicateOperand::BitVector(left),
                PredicateOperand::BitVector(right),
            ) => left.len() == right.len(),

            (
                PredicateOperand::ClassicalBit(_),
                PredicateOperand::Bool(_),
            )
            | (
                PredicateOperand::Bool(_),
                PredicateOperand::ClassicalBit(_),
            ) => true,

            _ => false,
        };

        if !compatible {
            return Err(PredicateError::IncompatibleMembershipOperands {
                value: value_kind,
                candidate: candidate_kind,
            });
        }

        Ok(())
    }

    fn operand_contains_value_reference(
        operand: &PredicateOperand,
    ) -> bool {
        matches!(operand, PredicateOperand::Value(_))
    }

    fn collect_classical_bits(
        &self,
        output: &mut std::collections::BTreeSet<ClassicalBitId>,
    ) {
        match self {
            Self::Constant(_) => {}

            Self::Bit(bit) => {
                output.insert(*bit);
            }

            Self::Compare { left, right, .. } => {
                Self::collect_operand_classical_bit(left, output);
                Self::collect_operand_classical_bit(right, output);
            }

            Self::Not(predicate) => {
                predicate.collect_classical_bits(output);
            }

            Self::And(predicates)
            | Self::Or(predicates)
            | Self::Xor(predicates) => {
                for predicate in predicates {
                    predicate.collect_classical_bits(output);
                }
            }

            Self::Implies {
                antecedent,
                consequent,
            } => {
                antecedent.collect_classical_bits(output);
                consequent.collect_classical_bits(output);
            }

            Self::Equivalent { left, right } => {
                left.collect_classical_bits(output);
                right.collect_classical_bits(output);
            }

            Self::InSet { value, candidates } => {
                Self::collect_operand_classical_bit(value, output);

                for candidate in candidates {
                    Self::collect_operand_classical_bit(
                        candidate,
                        output,
                    );
                }
            }
        }
    }

    fn collect_operand_classical_bit(
        operand: &PredicateOperand,
        output: &mut std::collections::BTreeSet<ClassicalBitId>,
    ) {
        if let PredicateOperand::ClassicalBit(bit) = operand {
            output.insert(*bit);
        }
    }
}

// =============================================================================
// Predicate kind
// =============================================================================

/// Root structural category of a classical predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PredicateKind {
    /// Constant.
    Constant,

    /// Classical-bit truth test.
    Bit,

    /// Relational comparison.
    Compare,

    /// Logical negation.
    Not,

    /// Logical conjunction.
    And,

    /// Logical disjunction.
    Or,

    /// Logical exclusive-or.
    Xor,

    /// Logical implication.
    Implies,

    /// Logical equivalence.
    Equivalent,

    /// Membership test.
    InSet,
}

impl fmt::Display for PredicateKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Constant => "constant",
            Self::Bit => "bit",
            Self::Compare => "compare",
            Self::Not => "not",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Implies => "implies",
            Self::Equivalent => "equivalent",
            Self::InSet => "in_set",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Validation limits
// =============================================================================

/// Explicit safety/resource policy for predicate validation.
///
/// These values are validation policies, not Zamani language or quantum
/// machine limits.
///
/// A compiler, service, sandbox, or backend may create a policy appropriate
/// for its available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PredicateValidationLimits {
    /// Maximum recursive structural depth accepted during validation.
    pub max_depth: usize,

    /// Maximum number of predicate nodes.
    pub max_nodes: usize,

    /// Maximum number of candidates in one membership predicate.
    pub max_membership_candidates: usize,
}

impl PredicateValidationLimits {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_depth: usize,
        max_nodes: usize,
        max_membership_candidates: usize,
    ) -> Self {
        Self {
            max_depth,
            max_nodes,
            max_membership_candidates,
        }
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> PredicateResult<()> {
        if self.max_depth == 0 {
            return Err(PredicateError::InvalidValidationLimit {
                field: "max_depth",
            });
        }

        if self.max_nodes == 0 {
            return Err(PredicateError::InvalidValidationLimit {
                field: "max_nodes",
            });
        }

        if self.max_membership_candidates == 0 {
            return Err(PredicateError::InvalidValidationLimit {
                field: "max_membership_candidates",
            });
        }

        Ok(())
    }
}

impl Default for PredicateValidationLimits {
    fn default() -> Self {
        Self {
            max_depth: 1_024,
            max_nodes: 1_000_000,
            max_membership_candidates: 1_000_000,
        }
    }
}

// =============================================================================
// Validation state
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct ValidationState {
    nodes: usize,
    membership_candidates: usize,
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating predicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateError {
    /// A floating-point literal was NaN or infinite.
    NonFiniteFloat,

    /// A logical operator received no operands.
    EmptyLogicalOperandList,

    /// A membership predicate has no candidates.
    EmptyMembershipSet,

    /// Membership candidates contain duplicates.
    DuplicateMembershipCandidate {
        /// First candidate position.
        index: usize,

        /// Duplicate candidate position.
        duplicate_index: usize,
    },

    /// Two predicate operands have incompatible semantic categories.
    IncompatibleOperands {
        /// Left operand category.
        left: PredicateOperandKind,

        /// Requested comparison.
        operator: ComparisonOperator,

        /// Right operand category.
        right: PredicateOperandKind,
    },

    /// Membership operands have incompatible categories.
    IncompatibleMembershipOperands {
        /// Tested value category.
        value: PredicateOperandKind,

        /// Candidate category.
        candidate: PredicateOperandKind,
    },

    /// Predicate depth exceeds the explicit validation policy.
    DepthLimitExceeded {
        /// Requested depth.
        depth: usize,

        /// Maximum allowed depth.
        maximum: usize,
    },

    /// Predicate node count exceeds the explicit validation policy.
    NodeLimitExceeded {
        /// Requested number of nodes.
        nodes: usize,

        /// Maximum allowed number of nodes.
        maximum: usize,
    },

    /// Membership candidate count exceeds the explicit validation policy.
    MembershipLimitExceeded {
        /// Requested candidate count.
        requested: usize,

        /// Maximum allowed candidate count.
        maximum: usize,
    },

    /// Node-count arithmetic overflowed.
    NodeCountOverflow,

    /// Membership-count arithmetic overflowed.
    MembershipCountOverflow,

    /// A validation policy contains an invalid zero limit.
    InvalidValidationLimit {
        /// Name of invalid field.
        field: &'static str,
    },
}

impl fmt::Display for PredicateError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NonFiniteFloat => {
                formatter.write_str(
                    "predicate floating-point literal must be finite",
                )
            }

            Self::EmptyLogicalOperandList => {
                formatter.write_str(
                    "logical predicate operator requires at least one operand",
                )
            }

            Self::EmptyMembershipSet => {
                formatter.write_str(
                    "membership predicate requires at least one candidate",
                )
            }

            Self::DuplicateMembershipCandidate {
                index,
                duplicate_index,
            } => {
                write!(
                    formatter,
                    "membership candidate at index {duplicate_index} \
                     duplicates candidate at index {index}"
                )
            }

            Self::IncompatibleOperands {
                left,
                operator,
                right,
            } => {
                write!(
                    formatter,
                    "incompatible predicate operands: {left} {operator} {right}"
                )
            }

            Self::IncompatibleMembershipOperands {
                value,
                candidate,
            } => {
                write!(
                    formatter,
                    "incompatible membership operands: {value} and {candidate}"
                )
            }

            Self::DepthLimitExceeded {
                depth,
                maximum,
            } => {
                write!(
                    formatter,
                    "predicate depth limit exceeded: \
                     requested {depth}, maximum {maximum}"
                )
            }

            Self::NodeLimitExceeded {
                nodes,
                maximum,
            } => {
                write!(
                    formatter,
                    "predicate node limit exceeded: \
                     requested {nodes}, maximum {maximum}"
                )
            }

            Self::MembershipLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "predicate membership limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::NodeCountOverflow => {
                formatter.write_str(
                    "predicate node-count arithmetic overflowed",
                )
            }

            Self::MembershipCountOverflow => {
                formatter.write_str(
                    "predicate membership-count arithmetic overflowed",
                )
            }

            Self::InvalidValidationLimit { field } => {
                write!(
                    formatter,
                    "predicate validation limit `{field}` must be non-zero"
                )
            }
        }
    }
}

impl std::error::Error for PredicateError {}

// =============================================================================
// Display implementation
// =============================================================================

impl fmt::Display for PredicateOperand {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Bool(value) => {
                formatter.write_str(if value.as_bool() {
                    "true"
                } else {
                    "false"
                })
            }

            Self::SignedInteger(value) => {
                write!(formatter, "{value}")
            }

            Self::UnsignedInteger(value) => {
                write!(formatter, "{value}")
            }

            Self::Float(value) => {
                write!(formatter, "{}", value.value())
            }

            Self::ClassicalBit(bit) => {
                write!(formatter, "{bit}")
            }

            Self::BitVector(bits) => {
                formatter.write_str("\"")?;

                for bit in bits.iter() {
                    formatter.write_str(if *bit { "1" } else { "0" })?;
                }

                formatter.write_str("\"")
            }

            Self::Value(value) => {
                write!(formatter, "{value}")
            }
        }
    }
}

impl fmt::Display for ClassicalPredicate {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Constant(value) => {
                formatter.write_str(if value.as_bool() {
                    "true"
                } else {
                    "false"
                })
            }

            Self::Bit(bit) => {
                write!(formatter, "{bit}")
            }

            Self::Compare {
                left,
                operator,
                right,
            } => {
                write!(
                    formatter,
                    "{left} {operator} {right}"
                )
            }

            Self::Not(predicate) => {
                write!(formatter, "!({predicate})")
            }

            Self::And(predicates) => {
                Self::fmt_logical_list(
                    formatter,
                    predicates,
                    " && ",
                )
            }

            Self::Or(predicates) => {
                Self::fmt_logical_list(
                    formatter,
                    predicates,
                    " || ",
                )
            }

            Self::Xor(predicates) => {
                Self::fmt_logical_list(
                    formatter,
                    predicates,
                    " xor ",
                )
            }

            Self::Implies {
                antecedent,
                consequent,
            } => {
                write!(
                    formatter,
                    "({antecedent}) -> ({consequent})"
                )
            }

            Self::Equivalent { left, right } => {
                write!(
                    formatter,
                    "({left}) <-> ({right})"
                )
            }

            Self::InSet {
                value,
                candidates,
            } => {
                write!(formatter, "{value} in {{")?;

                for (index, candidate) in candidates.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{candidate}")?;
                }

                formatter.write_str("}")
            }
        }
    }
}

impl ClassicalPredicate {
    fn fmt_logical_list(
        formatter: &mut fmt::Formatter<'_>,
        predicates: &[Self],
        separator: &str,
    ) -> fmt::Result {
        formatter.write_str("(")?;

        for (index, predicate) in predicates.iter().enumerate() {
            if index != 0 {
                formatter.write_str(separator)?;
            }

            write!(formatter, "{predicate}")?;
        }

        formatter.write_str(")")
    }
}

// =============================================================================
// Standard conversions
// =============================================================================

impl From<ClassicalBitId> for PredicateOperand {
    fn from(bit: ClassicalBitId) -> Self {
        Self::ClassicalBit(bit)
    }
}

impl From<bool> for PredicateOperand {
    fn from(value: bool) -> Self {
        Self::bool(value)
    }
}

impl From<i8> for PredicateOperand {
    fn from(value: i8) -> Self {
        Self::signed(i128::from(value))
    }
}

impl From<i16> for PredicateOperand {
    fn from(value: i16) -> Self {
        Self::signed(i128::from(value))
    }
}

impl From<i32> for PredicateOperand {
    fn from(value: i32) -> Self {
        Self::signed(i128::from(value))
    }
}

impl From<i64> for PredicateOperand {
    fn from(value: i64) -> Self {
        Self::signed(i128::from(value))
    }
}

impl From<i128> for PredicateOperand {
    fn from(value: i128) -> Self {
        Self::signed(value)
    }
}

impl From<u8> for PredicateOperand {
    fn from(value: u8) -> Self {
        Self::unsigned(u128::from(value))
    }
}

impl From<u16> for PredicateOperand {
    fn from(value: u16) -> Self {
        Self::unsigned(u128::from(value))
    }
}

impl From<u32> for PredicateOperand {
    fn from(value: u32) -> Self {
        Self::unsigned(u128::from(value))
    }
}

impl From<u64> for PredicateOperand {
    fn from(value: u64) -> Self {
        Self::unsigned(u128::from(value))
    }
}

impl From<u128> for PredicateOperand {
    fn from(value: u128) -> Self {
        Self::unsigned(value)
    }
}

impl From<ValueId> for PredicateOperand {
    fn from(value: ValueId) -> Self {
        Self::Value(value)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn bit(index: usize) -> ClassicalBitId {
        ClassicalBitId::new(index)
    }

    #[test]
    fn constants_are_correct() {
        assert_eq!(
            ClassicalPredicate::always().constant_value(),
            Some(true)
        );

        assert_eq!(
            ClassicalPredicate::never().constant_value(),
            Some(false)
        );
    }

    #[test]
    fn bit_predicate_is_constructible() {
        let predicate = ClassicalPredicate::bit(bit(0));

        assert_eq!(predicate.kind(), PredicateKind::Bit);
        assert!(predicate.references_classical_bit(bit(0)));
    }

    #[test]
    fn bit_equality_is_constructible() {
        let predicate =
            ClassicalPredicate::bit_equals(bit(0), true);

        assert_eq!(
            predicate.to_string(),
            "c0 == true"
        );

        assert!(predicate.validate().is_ok());
    }

    #[test]
    fn comparisons_are_constructible() {
        let predicate = ClassicalPredicate::equal(
            PredicateOperand::signed(10),
            PredicateOperand::signed(10),
        )
        .expect("valid comparison");

        assert_eq!(predicate.kind(), PredicateKind::Compare);
        assert!(predicate.validate().is_ok());
    }

    #[test]
    fn incompatible_ordering_is_rejected() {
        let result = ClassicalPredicate::compare(
            PredicateOperand::bool(true),
            ComparisonOperator::LessThan,
            PredicateOperand::bool(false),
        );

        assert!(matches!(
            result,
            Err(PredicateError::IncompatibleOperands { .. })
        ));
    }

    #[test]
    fn boolean_logic_is_constructible() {
        let predicate = ClassicalPredicate::and(vec![
            ClassicalPredicate::bit(bit(0)),
            ClassicalPredicate::not(ClassicalPredicate::bit(bit(1))),
        ])
        .expect("valid conjunction");

        assert!(predicate.validate().is_ok());
        assert_eq!(predicate.node_count(), 3);
    }

    #[test]
    fn membership_is_constructible() {
        let predicate = ClassicalPredicate::in_set(
            PredicateOperand::unsigned(3),
            vec![
                PredicateOperand::unsigned(1),
                PredicateOperand::unsigned(2),
                PredicateOperand::unsigned(3),
            ],
        )
        .expect("valid membership");

        assert!(predicate.validate().is_ok());
    }

    #[test]
    fn duplicate_membership_is_rejected() {
        let result = ClassicalPredicate::in_set(
            PredicateOperand::unsigned(3),
            vec![
                PredicateOperand::unsigned(1),
                PredicateOperand::unsigned(1),
            ],
        );

        assert!(matches!(
            result,
            Err(PredicateError::DuplicateMembershipCandidate { .. })
        ));
    }

    #[test]
    fn non_finite_float_is_rejected() {
        assert!(matches!(
            PredicateOperand::float(f64::NAN),
            Err(PredicateError::NonFiniteFloat)
        ));

        assert!(matches!(
            PredicateOperand::float(f64::INFINITY),
            Err(PredicateError::NonFiniteFloat)
        ));
    }

    #[test]
    fn simplification_of_not_constant_works() {
        assert_eq!(
            ClassicalPredicate::not(
                ClassicalPredicate::always()
            )
            .simplify(),
            ClassicalPredicate::never()
        );
    }

    #[test]
    fn simplification_of_and_works() {
        let predicate =
            ClassicalPredicate::and(vec![
                ClassicalPredicate::always(),
                ClassicalPredicate::bit(bit(0)),
            ])
            .expect("valid conjunction")
            .simplify();

        assert_eq!(
            predicate,
            ClassicalPredicate::bit(bit(0))
        );
    }

    #[test]
    fn simplification_of_or_works() {
        let predicate =
            ClassicalPredicate::or(vec![
                ClassicalPredicate::never(),
                ClassicalPredicate::bit(bit(0)),
            ])
            .expect("valid disjunction")
            .simplify();

        assert_eq!(
            predicate,
            ClassicalPredicate::bit(bit(0))
        );
    }

    #[test]
    fn referenced_bits_are_deterministic() {
        let predicate =
            ClassicalPredicate::and(vec![
                ClassicalPredicate::bit(bit(5)),
                ClassicalPredicate::bit(bit(1)),
                ClassicalPredicate::bit(bit(5)),
            ])
            .expect("valid conjunction");

        let bits = predicate.referenced_classical_bits();

        let values: Vec<usize> =
            bits.iter().map(|id| id.index()).collect();

        assert_eq!(values, vec![1, 5]);
    }

    #[test]
    fn explicit_validation_limits_are_enforced() {
        let predicate =
            ClassicalPredicate::not(
                ClassicalPredicate::bit(bit(0)),
            );

        let limits =
            PredicateValidationLimits::new(1, 100, 100);

        assert!(matches!(
            predicate.validate_with_limits(&limits),
            Err(PredicateError::DepthLimitExceeded { .. })
        ));
    }

    #[test]
    fn display_is_deterministic() {
        let predicate =
            ClassicalPredicate::and(vec![
                ClassicalPredicate::bit(bit(0)),
                ClassicalPredicate::bit_equals(bit(1), true),
            ])
            .expect("valid conjunction");

        assert_eq!(
            predicate.to_string(),
            "(c0 && c1 == true)"
        );
    }

    #[test]
    fn value_reference_is_supported() {
        let value = ValueId::new(42);

        let predicate = ClassicalPredicate::equal(
            PredicateOperand::value(value),
            PredicateOperand::signed(1),
        )
        .expect("value references are type-resolved later");

        assert!(predicate.contains_value_reference());
        assert!(predicate.validate().is_ok());
    }

    #[test]
    fn bit_vector_width_is_checked() {
        let result = ClassicalPredicate::equal(
            PredicateOperand::bit_vector(vec![true, false]),
            PredicateOperand::bit_vector(vec![true]),
        );

        assert!(matches!(
            result,
            Err(PredicateError::IncompatibleOperands { .. })
        ));
    }
}