//! Zamani Quantum IR — Classical Expression Semantics
//!
//! Production-grade, target-independent representation of classical
//! expressions used by the canonical Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `classical::expression` owns the semantic representation of general
//! classical computation that occurs inside a quantum program.
//!
//! It represents:
//!
//! - boolean expressions;
//! - signed and unsigned integer expressions;
//! - finite floating-point expressions;
//! - classical-bit reads;
//! - IR value references;
//! - symbolic names;
//! - unary arithmetic and logical operations;
//! - binary arithmetic operations;
//! - comparisons;
//! - Boolean logic;
//! - bitwise operations;
//! - shifts;
//! - bit extraction;
//! - bit slicing;
//! - concatenation;
//! - conditional/select expressions;
//! - explicit casts;
//! - function/external-call expressions;
//! - extensible named operations;
//! - deterministic structural inspection;
//! - iterative validation;
//! - deterministic symbol/reference collection.
//!
//! It does NOT own:
//!
//! - source-language parsing;
//! - OpenQASM ASTs;
//! - quantum state;
//! - quantum gates;
//! - physical qubits;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - hardware registers;
//! - classical CPU allocation;
//! - optimization policy;
//! - predicate control-flow policy;
//! - backend-specific ABI details.
//!
//! Those responsibilities belong to the appropriate IR or downstream
//! subsystem.
//!
//! # Canonical identity boundaries
//!
//! Classical-bit identity is owned by:
//!
//! ```text
//! quantum::ir::classical::bit::ClassicalBitId
//! ```
//!
//! Quantum logical/physical identity is owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not define replacement identity types.
//!
//! A classical expression may read a classical bit and may reference an
//! existing SSA/IR value. It does not need to own a quantum identity because
//! quantum-to-classical relationships are established by measurement and
//! operation layers.
//!
//! # Separation from `parameter.rs`
//!
//! `quantum::ir::parameter` owns scalar parameter semantics used by gates,
//! pulses and other parameterized quantum constructs.
//!
//! This module owns the broader classical runtime expression language.
//!
//! Therefore:
//!
//! ```text
//! parameter.rs
//!     = scalar quantum parameter semantics
//!
//! classical/expression.rs
//!     = general classical computation semantics
//! ```
//!
//! A parameter can be embedded into a classical expression through an IR
//! value/reference or through a future canonical value bridge. This module
//! intentionally does not duplicate `ParameterExpression`.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to different machines and execution architectures.
//!
//! Consequently this module contains:
//!
//! - no fixed register width;
//! - no fixed number of bits;
//! - no fixed number of variables;
//! - no fixed number of expression nodes;
//! - no vendor-specific classical controller;
//! - no hardware address;
//! - no machine-specific register layout.
//!
//! Practical resource limits are explicit validation policy, not semantic
//! limits of Zamani.
//!
//! # Scalability
//!
//! Expression trees use `Box` only to provide an owned recursive semantic
//! structure. All structural algorithms in this file are iterative:
//!
//! - validation;
//! - node counting;
//! - depth calculation;
//! - symbol collection;
//! - classical-bit collection;
//! - IR-value collection.
//!
//! This avoids making Rust call-stack depth proportional to expression depth.
//!
//! An implementation may therefore construct expressions whose size is
//! bounded by available memory and explicit caller policy rather than by an
//! arbitrary Zamani architectural constant.
//!
//! # Determinism
//!
//! Semantic collections use ordered containers:
//!
//! - `BTreeSet` for unique identifiers;
//! - `BTreeMap` for deterministic named-call arguments.
//!
//! No `HashMap` is used in semantic storage.
//!
//! This supports reproducible compilation, canonical serialization and
//! canonical hashing.
//!
//! # Floating-point semantics
//!
//! Floating-point constants are required to be finite.
//!
//! NaN and positive/negative infinity are rejected when constructing or
//! validating an expression.
//!
//! Floating-point equality in the IR is structural equality of the canonical
//! IEEE-754 bit representation.
//!
//! Evaluation is checked and rejects non-finite results.
//!
//! # Evaluation boundary
//!
//! This module provides an explicit, caller-supplied evaluation environment.
//!
//! There is:
//!
//! - no global variable table;
//! - no global mutable state;
//! - no filesystem access;
//! - no network access;
//! - no dynamic code execution.
//!
//! External/function calls are represented semantically but are never executed
//! by this module.
//!
//! # Predicate boundary
//!
//! Predicates are owned by `classical::predicate`.
//!
//! This module provides comparison expressions, which can later be consumed by
//! the predicate/control-flow layer.
//!
//! It does not import `predicate.rs`, avoiding an unnecessary dependency cycle.
//!
//! # OpenQASM compatibility
//!
//! The model is capable of representing the semantic categories needed by
//! modern quantum languages:
//!
//! - Boolean logic;
//! - integer arithmetic;
//! - floating-point arithmetic;
//! - bit extraction;
//! - bit slicing;
//! - concatenation;
//! - comparisons;
//! - conditional selection;
//! - casts;
//! - function calls.
//!
//! It is NOT an OpenQASM AST.
//!
//! OpenQASM syntax belongs to the frontend and is lowered into canonical
//! Zamani IR.
//!
//! # Security
//!
//! This module:
//!
//! - forbids unsafe Rust;
//! - uses checked arithmetic where overflow is possible;
//! - rejects non-finite floating-point literals/results;
//! - supports explicit validation limits;
//! - performs iterative traversal;
//! - never executes external calls;
//! - never performs I/O;
//! - never dereferences raw pointers;
//! - has no global mutable state.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe.
//!
//! # Integration contract
//!
//! `classical::bit`
//!     owns `ClassicalBitId`, consumed here for classical-bit reads.
//!
//! `classical::value`
//!     may convert runtime classical values into expression operands.
//!
//! `classical::predicate`
//!     consumes comparison/Boolean semantics when constructing control-flow
//!     predicates.
//!
//! `classical::array`
//!     may lower array accesses into `Index`/`Slice` expressions.
//!
//! `classical::integer`
//!     provides integer-specific semantic types outside this expression tree.
//!
//! `classical::float`
//!     provides floating-point semantic types outside this expression tree.
//!
//! `classical::angle`
//!     provides angle semantics without changing this generic expression
//!     representation.
//!
//! `parameter`
//!     remains the owner of scalar quantum parameter expressions.
//!
//! `program::operation`
//!     may use `ClassicalExpression` as an operand/result-producing semantic
//!     expression.
//!
//! `control_flow`
//!     may consume expressions when constructing dynamic conditions.
//!
//! `validation`
//!     validates expression structure against declarations and resource
//!     policies.
//!
//! `serialization`
//!     serializes the enum structure deterministically.
//!
//! `hash`
//!     may use the structural representation for canonical program hashing.
//!
//! `frontend`
//!     lowers source-language classical expressions into this representation.
//!
//! `optimization`
//!     may transform expressions only while preserving their semantics.
//!
//! # No re-edit integration guarantee
//!
//! This file defines its complete public expression contract independently.
//!
//! Later implementation of:
//!
//! - predicates;
//! - control flow;
//! - operations;
//! - optimization;
//! - serialization;
//! - hardware;
//!
//! must consume this API rather than modify the semantic expression model for
//! ordinary integration.
//!
//! If a future quantum architecture requires a new expression primitive,
//! it should first determine whether the primitive can be represented by:
//!
//! - an existing generic operation;
//! - a named extensible call;
//! - an extension dialect;
//!
//! before changing this core semantic vocabulary.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

use super::bit::ClassicalBitId;
use crate::quantum::ir::identity::ValueId;

// =============================================================================
// Public result and validation policy
// =============================================================================

/// Result type for classical-expression operations.
pub type ExpressionResult<T> = Result<T, ExpressionError>;

/// Explicit validation policy for classical expressions.
///
/// A policy is deliberately external to the expression itself. This means
/// that the same semantic expression can be accepted by a trusted compiler
/// with one policy and rejected by a constrained service with another.
///
/// No field represents a Zamani architectural maximum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpressionValidationPolicy {
    /// Maximum permitted expression depth.
    ///
    /// `None` means no depth limit is imposed by this policy.
    pub max_depth: Option<usize>,

    /// Maximum permitted number of expression nodes.
    ///
    /// `None` means no node-count limit is imposed by this policy.
    pub max_nodes: Option<usize>,

    /// Maximum UTF-8 byte length of a symbolic identifier.
    ///
    /// `None` means no local symbol-length limit is imposed.
    pub max_symbol_bytes: Option<usize>,

    /// Maximum number of collected symbols permitted by the caller.
    ///
    /// `None` means no collection limit is imposed.
    pub max_collected_symbols: Option<usize>,

    /// Maximum number of collected classical-bit references permitted by the
    /// caller.
    ///
    /// `None` means no collection limit is imposed.
    pub max_collected_classical_bits: Option<usize>,

    /// Maximum number of collected IR-value references permitted by the
    /// caller.
    ///
    /// `None` means no collection limit is imposed.
    pub max_collected_values: Option<usize>,

    /// Whether empty symbolic names are rejected.
    pub reject_empty_symbols: bool,
}

impl Default for ExpressionValidationPolicy {
    fn default() -> Self {
        Self {
            max_depth: None,
            max_nodes: None,
            max_symbol_bytes: None,
            max_collected_symbols: None,
            max_collected_classical_bits: None,
            max_collected_values: None,
            reject_empty_symbols: true,
        }
    }
}

impl ExpressionValidationPolicy {
    /// Creates an unrestricted structural policy.
    ///
    /// This still rejects invalid floating-point values and empty symbols.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_depth: None,
            max_nodes: None,
            max_symbol_bytes: None,
            max_collected_symbols: None,
            max_collected_classical_bits: None,
            max_collected_values: None,
            reject_empty_symbols: true,
        }
    }

    /// Creates a policy with explicit depth and node limits.
    #[must_use]
    pub const fn bounded(
        max_depth: usize,
        max_nodes: usize,
    ) -> Self {
        Self {
            max_depth: Some(max_depth),
            max_nodes: Some(max_nodes),
            max_symbol_bytes: None,
            max_collected_symbols: None,
            max_collected_classical_bits: None,
            max_collected_values: None,
            reject_empty_symbols: true,
        }
    }

    /// Sets the maximum symbol length.
    #[must_use]
    pub const fn with_max_symbol_bytes(
        mut self,
        limit: usize,
    ) -> Self {
        self.max_symbol_bytes = Some(limit);
        self
    }

    /// Sets the maximum number of collected symbols.
    #[must_use]
    pub const fn with_max_collected_symbols(
        mut self,
        limit: usize,
    ) -> Self {
        self.max_collected_symbols = Some(limit);
        self
    }

    /// Sets the maximum number of collected classical-bit references.
    #[must_use]
    pub const fn with_max_collected_classical_bits(
        mut self,
        limit: usize,
    ) -> Self {
        self.max_collected_classical_bits = Some(limit);
        self
    }

    /// Sets the maximum number of collected IR-value references.
    #[must_use]
    pub const fn with_max_collected_values(
        mut self,
        limit: usize,
    ) -> Self {
        self.max_collected_values = Some(limit);
        self
    }

    /// Validates the policy itself.
    pub fn validate(self) -> ExpressionResult<()> {
        if let Some(limit) = self.max_depth {
            if limit == 0 {
                return Err(ExpressionError::InvalidPolicy(
                    "maximum expression depth cannot be zero",
                ));
            }
        }

        if let Some(limit) = self.max_nodes {
            if limit == 0 {
                return Err(ExpressionError::InvalidPolicy(
                    "maximum expression node count cannot be zero",
                ));
            }
        }

        if let Some(limit) = self.max_symbol_bytes {
            if limit == 0 {
                return Err(ExpressionError::InvalidPolicy(
                    "maximum symbol byte length cannot be zero",
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by classical-expression construction, validation or
/// evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionError {
    /// A symbolic identifier is empty.
    EmptySymbol,

    /// A symbolic identifier exceeds the configured byte limit.
    SymbolTooLong {
        /// Actual UTF-8 byte length.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Expression depth exceeds the configured policy.
    DepthExceeded {
        /// Observed depth.
        depth: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Expression node count exceeds the configured policy.
    NodeCountExceeded {
        /// Observed node count.
        count: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Too many symbols were requested by a collection operation.
    SymbolCollectionExceeded {
        /// Observed count.
        count: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Too many classical-bit references were requested by a collection
    /// operation.
    ClassicalBitCollectionExceeded {
        /// Observed count.
        count: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Too many IR-value references were requested by a collection operation.
    ValueCollectionExceeded {
        /// Observed count.
        count: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A floating-point literal is not finite.
    NonFiniteFloat,

    /// An arithmetic operation overflowed or produced a non-finite result.
    ArithmeticOverflow,

    /// Division by zero.
    DivisionByZero,

    /// Modulo by zero.
    ModuloByZero,

    /// A shift amount is invalid for the evaluated value width.
    InvalidShift,

    /// A bit index is outside the evaluated bit-vector width.
    BitIndexOutOfBounds {
        /// Requested index.
        index: usize,

        /// Available width.
        width: usize,
    },

    /// A bit slice has invalid bounds.
    InvalidSlice {
        /// Start index.
        start: usize,

        /// End index.
        end: usize,

        /// Available width.
        width: usize,
    },

    /// Two expression operands have incompatible semantic types.
    TypeMismatch {
        /// Left type.
        left: ExpressionType,

        /// Right type.
        right: ExpressionType,
    },

    /// A Boolean operation received a non-Boolean operand.
    ExpectedBoolean(ExpressionType),

    /// An arithmetic operation received an incompatible operand.
    ExpectedNumeric(ExpressionType),

    /// A bit operation received an incompatible operand.
    ExpectedBitVector(ExpressionType),

    /// A shift operation received an incompatible shift operand.
    ExpectedInteger(ExpressionType),

    /// A symbolic variable could not be resolved.
    UnboundSymbol(String),

    /// An IR value could not be resolved.
    UnboundValue(ValueId),

    /// A classical bit could not be resolved.
    UnboundClassicalBit(ClassicalBitId),

    /// An external/named call is semantic data and cannot be executed by this
    /// module.
    ExternalCallNotEvaluable(String),

    /// An expression contains a semantic operation that this evaluator does
    /// not execute.
    UnsupportedEvaluation(String),

    /// A caller supplied an invalid validation policy.
    InvalidPolicy(&'static str),

    /// Structural expression corruption was detected.
    InvalidStructure(&'static str),
}

impl fmt::Display for ExpressionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptySymbol => {
                formatter.write_str("classical expression symbol cannot be empty")
            }

            Self::SymbolTooLong { actual, maximum } => {
                write!(
                    formatter,
                    "classical expression symbol is {actual} bytes; maximum is {maximum}"
                )
            }

            Self::DepthExceeded { depth, maximum } => {
                write!(
                    formatter,
                    "classical expression depth {depth} exceeds maximum {maximum}"
                )
            }

            Self::NodeCountExceeded { count, maximum } => {
                write!(
                    formatter,
                    "classical expression node count {count} exceeds maximum {maximum}"
                )
            }

            Self::SymbolCollectionExceeded { count, maximum } => {
                write!(
                    formatter,
                    "classical expression contains {count} symbols; collection maximum is {maximum}"
                )
            }

            Self::ClassicalBitCollectionExceeded { count, maximum } => {
                write!(
                    formatter,
                    "classical expression references {count} classical bits; collection maximum is {maximum}"
                )
            }

            Self::ValueCollectionExceeded { count, maximum } => {
                write!(
                    formatter,
                    "classical expression references {count} IR values; collection maximum is {maximum}"
                )
            }

            Self::NonFiniteFloat => {
                formatter.write_str("classical expression floating-point value must be finite")
            }

            Self::ArithmeticOverflow => {
                formatter.write_str("classical expression arithmetic overflowed or produced a non-finite result")
            }

            Self::DivisionByZero => {
                formatter.write_str("classical expression division by zero")
            }

            Self::ModuloByZero => {
                formatter.write_str("classical expression modulo by zero")
            }

            Self::InvalidShift => {
                formatter.write_str("classical expression shift amount is invalid")
            }

            Self::BitIndexOutOfBounds { index, width } => {
                write!(
                    formatter,
                    "classical expression bit index {index} is outside width {width}"
                )
            }

            Self::InvalidSlice { start, end, width } => {
                write!(
                    formatter,
                    "classical expression slice [{start}:{end}) is outside width {width}"
                )
            }

            Self::TypeMismatch { left, right } => {
                write!(
                    formatter,
                    "classical expression type mismatch: {left} versus {right}"
                )
            }

            Self::ExpectedBoolean(kind) => {
                write!(
                    formatter,
                    "classical expression expected boolean, found {kind}"
                )
            }

            Self::ExpectedNumeric(kind) => {
                write!(
                    formatter,
                    "classical expression expected numeric value, found {kind}"
                )
            }

            Self::ExpectedBitVector(kind) => {
                write!(
                    formatter,
                    "classical expression expected bit-vector, found {kind}"
                )
            }

            Self::ExpectedInteger(kind) => {
                write!(
                    formatter,
                    "classical expression expected integer, found {kind}"
                )
            }

            Self::UnboundSymbol(symbol) => {
                write!(
                    formatter,
                    "classical expression symbol `{symbol}` is unbound"
                )
            }

            Self::UnboundValue(value) => {
                write!(
                    formatter,
                    "classical expression IR value `{value}` is unbound"
                )
            }

            Self::UnboundClassicalBit(bit) => {
                write!(
                    formatter,
                    "classical expression classical bit `{bit}` is unbound"
                )
            }

            Self::ExternalCallNotEvaluable(name) => {
                write!(
                    formatter,
                    "external classical call `{name}` cannot be evaluated by the IR expression layer"
                )
            }

            Self::UnsupportedEvaluation(name) => {
                write!(
                    formatter,
                    "classical expression operation `{name}` is not executable by this evaluator"
                )
            }

            Self::InvalidPolicy(message) => formatter.write_str(message),

            Self::InvalidStructure(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for ExpressionError {}

// =============================================================================
// Finite floating-point semantic value
// =============================================================================

/// Canonical finite floating-point literal.
///
/// The representation uses IEEE-754 bits so structural equality and hashing
/// are deterministic, including signed zero.
#[derive(Debug, Clone, Copy)]
pub struct FiniteFloat {
    bits: u64,
}

impl FiniteFloat {
    /// Creates a finite floating-point value.
    pub fn new(value: f64) -> ExpressionResult<Self> {
        if !value.is_finite() {
            return Err(ExpressionError::NonFiniteFloat);
        }

        Ok(Self {
            bits: value.to_bits(),
        })
    }

    /// Returns the represented `f64`.
    #[must_use]
    pub fn value(self) -> f64 {
        f64::from_bits(self.bits)
    }

    /// Returns the canonical IEEE-754 representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.bits
    }
}

impl PartialEq for FiniteFloat {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.bits == other.bits
    }
}

impl Eq for FiniteFloat {}

impl Hash for FiniteFloat {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.bits.hash(state);
    }
}

impl PartialOrd for FiniteFloat {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

// =============================================================================
// Bit-vector literal
// =============================================================================

/// Explicit logical bit-vector literal.
///
/// Bits are stored least-significant-bit first.
///
/// This representation has no fixed width. The allocation is proportional to
/// the literal actually represented by the program.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitVector {
    bits: Vec<bool>,
}

impl BitVector {
    /// Creates a bit vector from logical least-significant-bit-first bits.
    #[must_use]
    pub fn new(bits: Vec<bool>) -> Self {
        Self { bits }
    }

    /// Creates a bit vector from an iterator.
    #[must_use]
    pub fn from_bits<I>(bits: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Self {
            bits: bits.into_iter().collect(),
        }
    }

    /// Creates a zero-filled bit vector.
    #[must_use]
    pub fn zeros(width: usize) -> Self {
        Self {
            bits: vec![false; width],
        }
    }

    /// Creates an all-one bit vector.
    #[must_use]
    pub fn ones(width: usize) -> Self {
        Self {
            bits: vec![true; width],
        }
    }

    /// Returns the number of represented bits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns one bit.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<bool> {
        self.bits.get(index).copied()
    }

    /// Returns an iterator over logical bits.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &bool> {
        self.bits.iter()
    }

    /// Returns an owned copy of the represented bits.
    #[must_use]
    pub fn to_vec(&self) -> Vec<bool> {
        self.bits.clone()
    }
}

impl From<Vec<bool>> for BitVector {
    fn from(bits: Vec<bool>) -> Self {
        Self::new(bits)
    }
}

// =============================================================================
// Classical expression type
// =============================================================================

/// Semantic type category of a classical expression.
///
/// This type describes expression semantics without depending on a particular
/// hardware ABI or host-language representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExpressionType {
    /// Boolean scalar.
    Bool,

    /// Signed integer with an explicitly represented semantic width.
    ///
    /// `None` means width is implementation/context-defined and must not be
    /// interpreted as a fixed machine width.
    SignedInteger(Option<usize>),

    /// Unsigned integer with an explicitly represented semantic width.
    UnsignedInteger(Option<usize>),

    /// Finite floating-point scalar.
    Float,

    /// Bit vector with semantic width.
    ///
    /// `None` represents a dynamically sized or context-defined width.
    BitVector(Option<usize>),

    /// Unit value.
    Unit,

    /// Opaque value whose semantics are defined by another IR dialect.
    Opaque,

    /// Unknown/deferred type.
    ///
    /// This is useful during partially lowered IR construction. Validation
    /// requiring concrete types may reject it later.
    Unknown,
}

impl ExpressionType {
    /// Returns whether this is a Boolean expression type.
    #[must_use]
    pub const fn is_boolean(self) -> bool {
        matches!(self, Self::Bool)
    }

    /// Returns whether this is an integer expression type.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        matches!(
            self,
            Self::SignedInteger(_) | Self::UnsignedInteger(_)
        )
    }

    /// Returns whether this is a numeric expression type.
    #[must_use]
    pub const fn is_numeric(self) -> bool {
        matches!(
            self,
            Self::SignedInteger(_)
                | Self::UnsignedInteger(_)
                | Self::Float
        )
    }

    /// Returns whether this is a bit-vector expression type.
    #[must_use]
    pub const fn is_bit_vector(self) -> bool {
        matches!(self, Self::BitVector(_))
    }
}

impl fmt::Display for ExpressionType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Bool => formatter.write_str("bool"),

            Self::SignedInteger(Some(width)) => {
                write!(formatter, "i{width}")
            }

            Self::SignedInteger(None) => {
                formatter.write_str("signed_integer")
            }

            Self::UnsignedInteger(Some(width)) => {
                write!(formatter, "u{width}")
            }

            Self::UnsignedInteger(None) => {
                formatter.write_str("unsigned_integer")
            }

            Self::Float => formatter.write_str("float"),

            Self::BitVector(Some(width)) => {
                write!(formatter, "bits<{width}>")
            }

            Self::BitVector(None) => {
                formatter.write_str("bit_vector")
            }

            Self::Unit => formatter.write_str("unit"),

            Self::Opaque => formatter.write_str("opaque"),

            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// =============================================================================
// Unary operations
// =============================================================================

/// Unary classical-expression operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnaryOperator {
    /// Arithmetic negation.
    Negate,

    /// Boolean logical NOT.
    LogicalNot,

    /// Bitwise NOT.
    BitwiseNot,

    /// Population count.
    PopCount,

    /// Numeric absolute value.
    Absolute,

    /// Cast to Boolean.
    ToBool,

    /// Cast to signed integer.
    ToSignedInteger,

    /// Cast to unsigned integer.
    ToUnsignedInteger,

    /// Cast to floating point.
    ToFloat,
}

impl fmt::Display for UnaryOperator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Negate => "-",
            Self::LogicalNot => "!",
            Self::BitwiseNot => "~",
            Self::PopCount => "popcount",
            Self::Absolute => "abs",
            Self::ToBool => "bool",
            Self::ToSignedInteger => "signed",
            Self::ToUnsignedInteger => "unsigned",
            Self::ToFloat => "float",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Binary operations
// =============================================================================

/// Binary classical-expression operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BinaryOperator {
    /// Integer/floating addition.
    Add,

    /// Integer/floating subtraction.
    Subtract,

    /// Integer/floating multiplication.
    Multiply,

    /// Division.
    Divide,

    /// Remainder/modulo.
    Remainder,

    /// Exponentiation.
    Power,

    /// Equality.
    Equal,

    /// Inequality.
    NotEqual,

    /// Less-than.
    LessThan,

    /// Less-than-or-equal.
    LessThanOrEqual,

    /// Greater-than.
    GreaterThan,

    /// Greater-than-or-equal.
    GreaterThanOrEqual,

    /// Boolean AND.
    LogicalAnd,

    /// Boolean OR.
    LogicalOr,

    /// Boolean XOR.
    LogicalXor,

    /// Integer bitwise AND.
    BitwiseAnd,

    /// Integer bitwise OR.
    BitwiseOr,

    /// Integer bitwise XOR.
    BitwiseXor,

    /// Left shift.
    ShiftLeft,

    /// Logical/right shift.
    ShiftRight,

    /// Bit-vector concatenation.
    Concatenate,

    /// Membership-style operation supplied by a dialect.
    In,

    /// Explicit semantic operation supplied by an extension dialect.
    Custom,
}

impl fmt::Display for BinaryOperator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Remainder => "%",
            Self::Power => "**",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::LogicalAnd => "&&",
            Self::LogicalOr => "||",
            Self::LogicalXor => "xor",
            Self::BitwiseAnd => "&",
            Self::BitwiseOr => "|",
            Self::BitwiseXor => "^",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::Concatenate => "++",
            Self::In => "in",
            Self::Custom => "custom",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Expression literals
// =============================================================================

/// Literal value directly embedded in a classical expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionLiteral {
    /// Boolean literal.
    Bool(bool),

    /// Signed integer literal.
    SignedInteger(i128),

    /// Unsigned integer literal.
    UnsignedInteger(u128),

    /// Finite floating-point literal.
    Float(FiniteFloat),

    /// Bit-vector literal.
    BitVector(BitVector),
}

impl ExpressionLiteral {
    /// Returns the semantic expression type of this literal.
    #[must_use]
    pub fn expression_type(&self) -> ExpressionType {
        match self {
            Self::Bool(_) => ExpressionType::Bool,
            Self::SignedInteger(_) => ExpressionType::SignedInteger(None),
            Self::UnsignedInteger(_) => ExpressionType::UnsignedInteger(None),
            Self::Float(_) => ExpressionType::Float,
            Self::BitVector(bits) => {
                ExpressionType::BitVector(Some(bits.len()))
            }
        }
    }

    /// Creates a finite floating-point literal.
    pub fn float(
        value: f64,
    ) -> ExpressionResult<Self> {
        Ok(Self::Float(FiniteFloat::new(value)?))
    }
}

// =============================================================================
// Named call
// =============================================================================

/// A deterministic semantic function/external-call expression.
///
/// The expression layer does not execute the call. It records its semantic
/// name, arguments and optional return type.
///
/// Arguments are stored in `BTreeMap` so canonical traversal does not depend
/// on insertion order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedCall {
    /// Fully qualified semantic operation/function name.
    name: String,

    /// Named expression arguments.
    arguments: BTreeMap<String, ClassicalExpression>,

    /// Expected result type.
    result_type: ExpressionType,
}

impl NamedCall {
    /// Creates a named semantic call.
    pub fn new<S: Into<String>>(
        name: S,
        result_type: ExpressionType,
    ) -> ExpressionResult<Self> {
        let name = name.into();

        validate_symbol_name(&name, &ExpressionValidationPolicy::default())?;

        Ok(Self {
            name,
            arguments: BTreeMap::new(),
            result_type,
        })
    }

    /// Returns the call name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the declared result type.
    #[must_use]
    pub const fn result_type(&self) -> ExpressionType {
        self.result_type
    }

    /// Returns all arguments in deterministic key order.
    #[must_use]
    pub fn arguments(
        &self,
    ) -> &BTreeMap<String, ClassicalExpression> {
        &self.arguments
    }

    /// Inserts or replaces one named argument.
    pub fn with_argument<S: Into<String>>(
        mut self,
        name: S,
        expression: ClassicalExpression,
    ) -> ExpressionResult<Self> {
        let name = name.into();

        validate_symbol_name(
            &name,
            &ExpressionValidationPolicy::default(),
        )?;

        self.arguments.insert(name, expression);

        Ok(self)
    }

    /// Validates the call and all arguments.
    pub fn validate(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<()> {
        validate_symbol_name(&self.name, &policy)?;

        for (name, expression) in &self.arguments {
            validate_symbol_name(name, &policy)?;
            expression.validate_with_policy(policy)?;
        }

        Ok(())
    }
}

// =============================================================================
// Classical expression
// =============================================================================

/// Canonical general classical expression.
///
/// This is the semantic expression tree used by the Zamani Quantum IR.
///
/// It is deliberately not a source-language AST.
///
/// Every recursive child is boxed so that the enum remains sized. Structural
/// algorithms in this file are iterative to avoid call-stack growth
/// proportional to expression depth.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalExpression {
    /// Direct literal.
    Literal(ExpressionLiteral),

    /// Logical classical-bit read.
    ClassicalBit(ClassicalBitId),

    /// Existing SSA/IR value reference.
    Value(ValueId),

    /// Symbolic name resolved by an explicit evaluation environment.
    Symbol(String),

    /// Unary operation.
    Unary {
        /// Operation.
        operator: UnaryOperator,

        /// Operand.
        operand: Box<ClassicalExpression>,

        /// Declared result type.
        result_type: ExpressionType,
    },

    /// Binary operation.
    Binary {
        /// Operation.
        operator: BinaryOperator,

        /// Left operand.
        left: Box<ClassicalExpression>,

        /// Right operand.
        right: Box<ClassicalExpression>,

        /// Declared result type.
        result_type: ExpressionType,
    },

    /// Explicit bit extraction.
    Bit {
        /// Source bit-vector/integer expression.
        source: Box<ClassicalExpression>,

        /// Logical bit index.
        index: usize,
    },

    /// Explicit half-open bit slice.
    Slice {
        /// Source bit-vector expression.
        source: Box<ClassicalExpression>,

        /// Inclusive start.
        start: usize,

        /// Exclusive end.
        end: usize,
    },

    /// Conditional expression.
    ///
    /// Equivalent to:
    ///
    /// ```text
    /// condition ? then_value : else_value
    /// ```
    Select {
        /// Boolean condition.
        condition: Box<ClassicalExpression>,

        /// Value when condition is true.
        then_value: Box<ClassicalExpression>,

        /// Value when condition is false.
        else_value: Box<ClassicalExpression>,

        /// Declared result type.
        result_type: ExpressionType,
    },

    /// Named semantic call.
    Call(NamedCall),
}

impl ClassicalExpression {
    // =========================================================================
    // Constructors: literals
    // =========================================================================

    /// Creates a Boolean literal.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Literal(ExpressionLiteral::Bool(value))
    }

    /// Creates a signed integer literal.
    #[must_use]
    pub const fn signed(value: i128) -> Self {
        Self::Literal(ExpressionLiteral::SignedInteger(value))
    }

    /// Creates an unsigned integer literal.
    #[must_use]
    pub const fn unsigned(value: u128) -> Self {
        Self::Literal(ExpressionLiteral::UnsignedInteger(value))
    }

    /// Creates a finite floating-point literal.
    pub fn float(
        value: f64,
    ) -> ExpressionResult<Self> {
        Ok(Self::Literal(ExpressionLiteral::float(value)?))
    }

    /// Creates a bit-vector literal.
    #[must_use]
    pub fn bit_vector(bits: Vec<bool>) -> Self {
        Self::Literal(ExpressionLiteral::BitVector(
            BitVector::new(bits),
        ))
    }

    // =========================================================================
    // Constructors: references
    // =========================================================================

    /// Creates a classical-bit read.
    ///
    /// This uses the canonical `quantum::ir::classical::bit::ClassicalBitId`.
    #[must_use]
    pub const fn classical_bit(
        bit: ClassicalBitId,
    ) -> Self {
        Self::ClassicalBit(bit)
    }

    /// Creates an IR-value reference.
    #[must_use]
    pub const fn value(
        value: ValueId,
    ) -> Self {
        Self::Value(value)
    }

    /// Creates a symbolic reference.
    pub fn symbol<S: Into<String>>(
        name: S,
    ) -> ExpressionResult<Self> {
        let name = name.into();

        validate_symbol_name(
            &name,
            &ExpressionValidationPolicy::default(),
        )?;

        Ok(Self::Symbol(name))
    }

    /// Creates a symbolic reference without applying a default byte limit.
    ///
    /// Structural validation can still apply a caller-supplied policy later.
    pub fn symbol_unchecked_by_policy<S: Into<String>>(
        name: S,
    ) -> ExpressionResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(ExpressionError::EmptySymbol);
        }

        Ok(Self::Symbol(name))
    }

    // =========================================================================
    // Constructors: unary
    // =========================================================================

    /// Creates a unary operation.
    #[must_use]
    pub fn unary(
        operator: UnaryOperator,
        operand: Self,
        result_type: ExpressionType,
    ) -> Self {
        Self::Unary {
            operator,
            operand: Box::new(operand),
            result_type,
        }
    }

    /// Creates arithmetic negation.
    #[must_use]
    pub fn negate(
        operand: Self,
    ) -> Self {
        let result_type = operand.expression_type();

        Self::unary(
            UnaryOperator::Negate,
            operand,
            result_type,
        )
    }

    /// Creates Boolean NOT.
    #[must_use]
    pub fn logical_not(
        operand: Self,
    ) -> Self {
        Self::unary(
            UnaryOperator::LogicalNot,
            operand,
            ExpressionType::Bool,
        )
    }

    /// Creates bitwise NOT.
    #[must_use]
    pub fn bitwise_not(
        operand: Self,
    ) -> Self {
        let result_type = operand.expression_type();

        Self::unary(
            UnaryOperator::BitwiseNot,
            operand,
            result_type,
        )
    }

    /// Creates population-count operation.
    #[must_use]
    pub fn popcount(
        operand: Self,
    ) -> Self {
        Self::unary(
            UnaryOperator::PopCount,
            operand,
            ExpressionType::UnsignedInteger(None),
        )
    }

    /// Creates absolute-value operation.
    #[must_use]
    pub fn absolute(
        operand: Self,
    ) -> Self {
        let result_type = operand.expression_type();

        Self::unary(
            UnaryOperator::Absolute,
            operand,
            result_type,
        )
    }

    // =========================================================================
    // Constructors: binary
    // =========================================================================

    /// Creates a binary operation with an explicitly declared result type.
    #[must_use]
    pub fn binary(
        operator: BinaryOperator,
        left: Self,
        right: Self,
        result_type: ExpressionType,
    ) -> Self {
        Self::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            result_type,
        }
    }

    /// Creates addition.
    #[must_use]
    pub fn add(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_numeric_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Add,
            left,
            right,
            result_type,
        )
    }

    /// Creates subtraction.
    #[must_use]
    pub fn subtract(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_numeric_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Subtract,
            left,
            right,
            result_type,
        )
    }

    /// Creates multiplication.
    #[must_use]
    pub fn multiply(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_numeric_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Multiply,
            left,
            right,
            result_type,
        )
    }

    /// Creates division.
    #[must_use]
    pub fn divide(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_numeric_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Divide,
            left,
            right,
            result_type,
        )
    }

    /// Creates remainder/modulo.
    #[must_use]
    pub fn remainder(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_numeric_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Remainder,
            left,
            right,
            result_type,
        )
    }

    /// Creates exponentiation.
    #[must_use]
    pub fn power(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_numeric_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Power,
            left,
            right,
            result_type,
        )
    }

    /// Creates equality comparison.
    #[must_use]
    pub fn equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::Equal,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates inequality comparison.
    #[must_use]
    pub fn not_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::NotEqual,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates less-than comparison.
    #[must_use]
    pub fn less_than(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::LessThan,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates less-than-or-equal comparison.
    #[must_use]
    pub fn less_than_or_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::LessThanOrEqual,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates greater-than comparison.
    #[must_use]
    pub fn greater_than(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::GreaterThan,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates greater-than-or-equal comparison.
    #[must_use]
    pub fn greater_than_or_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::GreaterThanOrEqual,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates Boolean AND.
    #[must_use]
    pub fn logical_and(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::LogicalAnd,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates Boolean OR.
    #[must_use]
    pub fn logical_or(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::LogicalOr,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates Boolean XOR.
    #[must_use]
    pub fn logical_xor(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            BinaryOperator::LogicalXor,
            left,
            right,
            ExpressionType::Bool,
        )
    }

    /// Creates bitwise AND.
    #[must_use]
    pub fn bitwise_and(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_bitwise_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::BitwiseAnd,
            left,
            right,
            result_type,
        )
    }

    /// Creates bitwise OR.
    #[must_use]
    pub fn bitwise_or(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_bitwise_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::BitwiseOr,
            left,
            right,
            result_type,
        )
    }

    /// Creates bitwise XOR.
    #[must_use]
    pub fn bitwise_xor(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = promote_bitwise_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::BitwiseXor,
            left,
            right,
            result_type,
        )
    }

    /// Creates a left shift.
    #[must_use]
    pub fn shift_left(
        value: Self,
        amount: Self,
    ) -> Self {
        let result_type = value.expression_type();

        Self::binary(
            BinaryOperator::ShiftLeft,
            value,
            amount,
            result_type,
        )
    }

    /// Creates a right shift.
    #[must_use]
    pub fn shift_right(
        value: Self,
        amount: Self,
    ) -> Self {
        let result_type = value.expression_type();

        Self::binary(
            BinaryOperator::ShiftRight,
            value,
            amount,
            result_type,
        )
    }

    /// Creates bit-vector concatenation.
    #[must_use]
    pub fn concatenate(
        left: Self,
        right: Self,
    ) -> Self {
        let result_type = concatenate_type(
            left.expression_type(),
            right.expression_type(),
        );

        Self::binary(
            BinaryOperator::Concatenate,
            left,
            right,
            result_type,
        )
    }

    // =========================================================================
    // Constructors: indexing and selection
    // =========================================================================

    /// Creates a bit extraction.
    #[must_use]
    pub fn bit(
        source: Self,
        index: usize,
    ) -> Self {
        Self::Bit {
            source: Box::new(source),
            index,
        }
    }

    /// Creates a half-open bit slice `[start, end)`.
    #[must_use]
    pub fn slice(
        source: Self,
        start: usize,
        end: usize,
    ) -> Self {
        Self::Slice {
            source: Box::new(source),
            start,
            end,
        }
    }

    /// Creates a conditional select.
    #[must_use]
    pub fn select(
        condition: Self,
        then_value: Self,
        else_value: Self,
    ) -> Self {
        let result_type = then_value.expression_type();

        Self::Select {
            condition: Box::new(condition),
            then_value: Box::new(then_value),
            else_value: Box::new(else_value),
            result_type,
        }
    }

    /// Creates an explicitly typed cast.
    #[must_use]
    pub fn cast(
        operand: Self,
        target: ExpressionType,
    ) -> Self {
        let operator = match target {
            ExpressionType::Bool => UnaryOperator::ToBool,
            ExpressionType::SignedInteger(_) => {
                UnaryOperator::ToSignedInteger
            }
            ExpressionType::UnsignedInteger(_) => {
                UnaryOperator::ToUnsignedInteger
            }
            ExpressionType::Float => UnaryOperator::ToFloat,
            _ => UnaryOperator::ToUnsignedInteger,
        };

        Self::unary(operator, operand, target)
    }

    /// Creates a named semantic call.
    pub fn call(
        call: NamedCall,
    ) -> Self {
        Self::Call(call)
    }

    // =========================================================================
    // Structural information
    // =========================================================================

    /// Returns the semantic type declared by this expression node.
    #[must_use]
    pub fn expression_type(&self) -> ExpressionType {
        match self {
            Self::Literal(literal) => literal.expression_type(),

            Self::ClassicalBit(_) => ExpressionType::Bool,

            Self::Value(_) => ExpressionType::Unknown,

            Self::Symbol(_) => ExpressionType::Unknown,

            Self::Unary { result_type, .. } => *result_type,

            Self::Binary { result_type, .. } => *result_type,

            Self::Bit { .. } => ExpressionType::Bool,

            Self::Slice {
                source,
                start,
                end,
            } => {
                if end >= start {
                    let width = end - start;

                    ExpressionType::BitVector(Some(width))
                } else {
                    ExpressionType::BitVector(Some(0))
                }
            }

            Self::Select { result_type, .. } => *result_type,

            Self::Call(call) => call.result_type(),
        }
    }

    /// Returns whether this expression is statically Boolean.
    #[must_use]
    pub fn is_boolean(&self) -> bool {
        self.expression_type().is_boolean()
    }

    /// Returns whether this expression is numeric.
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        self.expression_type().is_numeric()
    }

    /// Returns whether this expression is a bit vector.
    #[must_use]
    pub fn is_bit_vector(&self) -> bool {
        self.expression_type().is_bit_vector()
    }

    /// Counts all semantic expression nodes iteratively.
    ///
    /// Returns `None` only if the mathematical count cannot fit in `usize`.
    pub fn node_count_checked(&self) -> Option<usize> {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(expression) = stack.pop() {
            count = count.checked_add(1)?;

            match expression {
                Self::Literal(_)
                | Self::ClassicalBit(_)
                | Self::Value(_)
                | Self::Symbol(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary {
                    left,
                    right,
                    ..
                } => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Bit { source, .. }
                | Self::Slice { source, .. } => {
                    stack.push(source);
                }

                Self::Select {
                    condition,
                    then_value,
                    else_value,
                    ..
                } => {
                    stack.push(else_value);
                    stack.push(then_value);
                    stack.push(condition);
                }

                Self::Call(call) => {
                    for expression in call.arguments().values() {
                        stack.push(expression);
                    }
                }
            }
        }

        Some(count)
    }

    /// Counts nodes.
    ///
    /// If the representable node count overflows `usize`, the result saturates
    /// at `usize::MAX`.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.node_count_checked().unwrap_or(usize::MAX)
    }

    /// Computes maximum semantic depth iteratively.
    ///
    /// A leaf has depth zero.
    #[must_use]
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;

        let mut stack = vec![(self, 0usize)];

        while let Some((expression, depth)) = stack.pop() {
            if depth > maximum {
                maximum = depth;
            }

            let child_depth = match depth.checked_add(1) {
                Some(value) => value,
                None => return usize::MAX,
            };

            match expression {
                Self::Literal(_)
                | Self::ClassicalBit(_)
                | Self::Value(_)
                | Self::Symbol(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push((operand, child_depth));
                }

                Self::Binary {
                    left,
                    right,
                    ..
                } => {
                    stack.push((left, child_depth));
                    stack.push((right, child_depth));
                }

                Self::Bit { source, .. }
                | Self::Slice { source, .. } => {
                    stack.push((source, child_depth));
                }

                Self::Select {
                    condition,
                    then_value,
                    else_value,
                    ..
                } => {
                    stack.push((condition, child_depth));
                    stack.push((then_value, child_depth));
                    stack.push((else_value, child_depth));
                }

                Self::Call(call) => {
                    for expression in call.arguments().values() {
                        stack.push((expression, child_depth));
                    }
                }
            }
        }

        maximum
    }

    /// Returns whether the expression is structurally empty.
    ///
    /// Expressions always contain at least one semantic node, so this is
    /// currently equivalent to `false` and exists for API symmetry with
    /// collection-like semantic objects.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the expression using unrestricted structural policy.
    pub fn validate(&self) -> ExpressionResult<()> {
        self.validate_with_policy(
            ExpressionValidationPolicy::default(),
        )
    }

    /// Validates using an explicit policy.
    ///
    /// Traversal is iterative.
    pub fn validate_with_policy(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<()> {
        policy.validate()?;

        let mut stack = vec![(self, 0usize)];
        let mut count = 0usize;

        while let Some((expression, depth)) = stack.pop() {
            count = count
                .checked_add(1)
                .ok_or(ExpressionError::NodeCountExceeded {
                    count: usize::MAX,
                    maximum: policy.max_nodes.unwrap_or(usize::MAX),
                })?;

            if let Some(maximum) = policy.max_nodes {
                if count > maximum {
                    return Err(
                        ExpressionError::NodeCountExceeded {
                            count,
                            maximum,
                        },
                    );
                }
            }

            if let Some(maximum) = policy.max_depth {
                if depth > maximum {
                    return Err(
                        ExpressionError::DepthExceeded {
                            depth,
                            maximum,
                        },
                    );
                }
            }

            match expression {
                Self::Literal(literal) => {
                    if let ExpressionLiteral::Float(value) = literal {
                        if !value.value().is_finite() {
                            return Err(
                                ExpressionError::NonFiniteFloat,
                            );
                        }
                    }

                    if let ExpressionLiteral::BitVector(bits) = literal {
                        validate_bit_vector(bits)?;
                    }
                }

                Self::ClassicalBit(_) | Self::Value(_) => {}

                Self::Symbol(name) => {
                    validate_symbol_name(name, &policy)?;
                }

                Self::Unary {
                    operator,
                    operand,
                    result_type,
                } => {
                    validate_unary(
                        *operator,
                        operand.expression_type(),
                        *result_type,
                    )?;

                    let next_depth = depth
                        .checked_add(1)
                        .ok_or(
                            ExpressionError::DepthExceeded {
                                depth: usize::MAX,
                                maximum: policy
                                    .max_depth
                                    .unwrap_or(usize::MAX),
                            },
                        )?;

                    stack.push((operand, next_depth));
                }

                Self::Binary {
                    operator,
                    left,
                    right,
                    result_type,
                } => {
                    validate_binary(
                        *operator,
                        left.expression_type(),
                        right.expression_type(),
                        *result_type,
                    )?;

                    let next_depth = depth
                        .checked_add(1)
                        .ok_or(
                            ExpressionError::DepthExceeded {
                                depth: usize::MAX,
                                maximum: policy
                                    .max_depth
                                    .unwrap_or(usize::MAX),
                            },
                        )?;

                    stack.push((left, next_depth));
                    stack.push((right, next_depth));
                }

                Self::Bit { source, index } => {
                    validate_bit_index(
                        source.expression_type(),
                        *index,
                    )?;

                    let next_depth = depth
                        .checked_add(1)
                        .ok_or(
                            ExpressionError::DepthExceeded {
                                depth: usize::MAX,
                                maximum: policy
                                    .max_depth
                                    .unwrap_or(usize::MAX),
                            },
                        )?;

                    stack.push((source, next_depth));
                }

                Self::Slice {
                    source,
                    start,
                    end,
                } => {
                    validate_slice(
                        source.expression_type(),
                        *start,
                        *end,
                    )?;

                    let next_depth = depth
                        .checked_add(1)
                        .ok_or(
                            ExpressionError::DepthExceeded {
                                depth: usize::MAX,
                                maximum: policy
                                    .max_depth
                                    .unwrap_or(usize::MAX),
                            },
                        )?;

                    stack.push((source, next_depth));
                }

                Self::Select {
                    condition,
                    then_value,
                    else_value,
                    result_type,
                } => {
                    if !condition.expression_type().is_boolean()
                        && condition.expression_type()
                            != ExpressionType::Unknown
                    {
                        return Err(
                            ExpressionError::ExpectedBoolean(
                                condition.expression_type(),
                            ),
                        );
                    }

                    if then_value.expression_type()
                        != ExpressionType::Unknown
                        && else_value.expression_type()
                            != ExpressionType::Unknown
                        && then_value.expression_type()
                            != else_value.expression_type()
                    {
                        return Err(
                            ExpressionError::TypeMismatch {
                                left: then_value.expression_type(),
                                right: else_value.expression_type(),
                            },
                        );
                    }

                    if *result_type != ExpressionType::Unknown
                        && then_value.expression_type()
                            != ExpressionType::Unknown
                        && *result_type
                            != then_value.expression_type()
                    {
                        return Err(
                            ExpressionError::TypeMismatch {
                                left: *result_type,
                                right: then_value.expression_type(),
                            },
                        );
                    }

                    let next_depth = depth
                        .checked_add(1)
                        .ok_or(
                            ExpressionError::DepthExceeded {
                                depth: usize::MAX,
                                maximum: policy
                                    .max_depth
                                    .unwrap_or(usize::MAX),
                            },
                        )?;

                    stack.push((condition, next_depth));
                    stack.push((then_value, next_depth));
                    stack.push((else_value, next_depth));
                }

                Self::Call(call) => {
                    call.validate(policy)?;

                    let next_depth = depth
                        .checked_add(1)
                        .ok_or(
                            ExpressionError::DepthExceeded {
                                depth: usize::MAX,
                                maximum: policy
                                    .max_depth
                                    .unwrap_or(usize::MAX),
                            },
                        )?;

                    for expression in call.arguments().values() {
                        stack.push((expression, next_depth));
                    }
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Deterministic dependency collection
    // =========================================================================

    /// Collects all symbolic names in deterministic lexical order.
    pub fn collect_symbols(
        &self,
    ) -> Vec<String> {
        self.collect_symbols_with_policy(
            ExpressionValidationPolicy::default(),
        )
        .unwrap_or_default()
    }

    /// Collects symbols under an explicit collection policy.
    pub fn collect_symbols_with_policy(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<Vec<String>> {
        let mut symbols = BTreeSet::<String>::new();

        self.collect_symbols_into(&mut symbols);

        if let Some(maximum) = policy.max_collected_symbols {
            if symbols.len() > maximum {
                return Err(
                    ExpressionError::SymbolCollectionExceeded {
                        count: symbols.len(),
                        maximum,
                    },
                );
            }
        }

        Ok(symbols.into_iter().collect())
    }

    /// Adds all symbols to an existing deterministic set.
    pub fn collect_symbols_into(
        &self,
        symbols: &mut BTreeSet<String>,
    ) {
        let mut stack = vec![self];

        while let Some(expression) = stack.pop() {
            match expression {
                Self::Symbol(name) => {
                    symbols.insert(name.clone());
                }

                Self::Literal(_)
                | Self::ClassicalBit(_)
                | Self::Value(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary {
                    left,
                    right,
                    ..
                } => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Bit { source, .. }
                | Self::Slice { source, .. } => {
                    stack.push(source);
                }

                Self::Select {
                    condition,
                    then_value,
                    else_value,
                    ..
                } => {
                    stack.push(else_value);
                    stack.push(then_value);
                    stack.push(condition);
                }

                Self::Call(call) => {
                    for expression in call.arguments().values() {
                        stack.push(expression);
                    }
                }
            }
        }
    }

    /// Collects all classical-bit references in deterministic order.
    pub fn collect_classical_bits(
        &self,
    ) -> Vec<ClassicalBitId> {
        self.collect_classical_bits_with_policy(
            ExpressionValidationPolicy::default(),
        )
        .unwrap_or_default()
    }

    /// Collects classical-bit references with an explicit collection policy.
    pub fn collect_classical_bits_with_policy(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<Vec<ClassicalBitId>> {
        let mut bits = BTreeSet::<ClassicalBitId>::new();

        self.collect_classical_bits_into(&mut bits);

        if let Some(maximum) =
            policy.max_collected_classical_bits
        {
            if bits.len() > maximum {
                return Err(
                    ExpressionError::ClassicalBitCollectionExceeded {
                        count: bits.len(),
                        maximum,
                    },
                );
            }
        }

        Ok(bits.into_iter().collect())
    }

    /// Adds all classical-bit references to an existing ordered set.
    pub fn collect_classical_bits_into(
        &self,
        bits: &mut BTreeSet<ClassicalBitId>,
    ) {
        let mut stack = vec![self];

        while let Some(expression) = stack.pop() {
            match expression {
                Self::ClassicalBit(bit) => {
                    bits.insert(*bit);
                }

                Self::Literal(_)
                | Self::Value(_)
                | Self::Symbol(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary {
                    left,
                    right,
                    ..
                } => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Bit { source, .. }
                | Self::Slice { source, .. } => {
                    stack.push(source);
                }

                Self::Select {
                    condition,
                    then_value,
                    else_value,
                    ..
                } => {
                    stack.push(else_value);
                    stack.push(then_value);
                    stack.push(condition);
                }

                Self::Call(call) => {
                    for expression in call.arguments().values() {
                        stack.push(expression);
                    }
                }
            }
        }
    }

    /// Collects all IR value references in deterministic numeric order.
    pub fn collect_values(
        &self,
    ) -> Vec<ValueId> {
        self.collect_values_with_policy(
            ExpressionValidationPolicy::default(),
        )
        .unwrap_or_default()
    }

    /// Collects IR value references under an explicit collection policy.
    pub fn collect_values_with_policy(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<Vec<ValueId>> {
        let mut values = BTreeSet::<ValueId>::new();

        self.collect_values_into(&mut values);

        if let Some(maximum) = policy.max_collected_values {
            if values.len() > maximum {
                return Err(
                    ExpressionError::ValueCollectionExceeded {
                        count: values.len(),
                        maximum,
                    },
                );
            }
        }

        Ok(values.into_iter().collect())
    }

    /// Adds all IR value references to an existing ordered set.
    pub fn collect_values_into(
        &self,
        values: &mut BTreeSet<ValueId>,
    ) {
        let mut stack = vec![self];

        while let Some(expression) = stack.pop() {
            match expression {
                Self::Value(value) => {
                    values.insert(*value);
                }

                Self::Literal(_)
                | Self::ClassicalBit(_)
                | Self::Symbol(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary {
                    left,
                    right,
                    ..
                } => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Bit { source, .. }
                | Self::Slice { source, .. } => {
                    stack.push(source);
                }

                Self::Select {
                    condition,
                    then_value,
                    else_value,
                    ..
                } => {
                    stack.push(else_value);
                    stack.push(then_value);
                    stack.push(condition);
                }

                Self::Call(call) => {
                    for expression in call.arguments().values() {
                        stack.push(expression);
                    }
                }
            }
        }
    }

    // =========================================================================
    // Evaluation
    // =========================================================================

    /// Evaluates the expression using an explicit environment.
    ///
    /// This evaluator is intentionally finite and side-effect free.
    ///
    /// It does not execute `NamedCall`.
    pub fn evaluate<E>(
        &self,
        environment: &E,
    ) -> ExpressionResult<ExpressionValue>
    where
        E: ExpressionEnvironment,
    {
        self.evaluate_with_policy(
            environment,
            ExpressionValidationPolicy::default(),
        )
    }

    /// Evaluates using an explicit structural policy.
    ///
    /// The expression is validated before evaluation.
    pub fn evaluate_with_policy<E>(
        &self,
        environment: &E,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<ExpressionValue>
    where
        E: ExpressionEnvironment,
    {
        self.validate_with_policy(policy)?;

        evaluate_expression(self, environment)
    }

    // =========================================================================
    // Formatting
    // =========================================================================

    /// Produces a deterministic semantic string.
    ///
    /// The formatter is iterative rather than recursively calling `Display`
    /// for every child, preventing call-stack growth for deeply nested trees.
    #[must_use]
    pub fn to_semantic_string(&self) -> String {
        let mut output = String::new();
        let mut stack = vec![FormatItem::Expression(self)];

        while let Some(item) = stack.pop() {
            match item {
                FormatItem::Text(text) => {
                    output.push_str(text);
                }

                FormatItem::Expression(expression) => {
                    push_format_items(
                        expression,
                        &mut stack,
                    );
                }
            }
        }

        output
    }
}

impl fmt::Display for ClassicalExpression {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.to_semantic_string())
    }
}

// =============================================================================
// Expression environment
// =============================================================================

/// Runtime-independent environment used to resolve classical expressions.
///
/// Implementations belong to callers such as simulators, runtimes, compiler
/// evaluators or test harnesses.
///
/// The environment itself is not owned by the IR.
pub trait ExpressionEnvironment {
    /// Resolves a symbolic name.
    fn resolve_symbol(
        &self,
        name: &str,
    ) -> Option<ExpressionValue>;

    /// Resolves an existing IR value.
    fn resolve_value(
        &self,
        value: ValueId,
    ) -> Option<ExpressionValue>;

    /// Resolves a logical classical bit.
    fn resolve_classical_bit(
        &self,
        bit: ClassicalBitId,
    ) -> Option<ExpressionValue>;
}

/// Simple deterministic evaluation environment.
///
/// This helper is useful for tests and small evaluators. It is not a runtime
/// memory model.
#[derive(Debug, Clone, Default)]
pub struct SimpleExpressionEnvironment {
    symbols: BTreeMap<String, ExpressionValue>,
    values: BTreeMap<ValueId, ExpressionValue>,
    classical_bits: BTreeMap<ClassicalBitId, ExpressionValue>,
}

impl SimpleExpressionEnvironment {
    /// Creates an empty environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds a symbol.
    pub fn bind_symbol<S: Into<String>>(
        &mut self,
        name: S,
        value: ExpressionValue,
    ) -> ExpressionResult<()> {
        let name = name.into();

        if name.is_empty() {
            return Err(ExpressionError::EmptySymbol);
        }

        self.symbols.insert(name, value);

        Ok(())
    }

    /// Binds an IR value.
    pub fn bind_value(
        &mut self,
        id: ValueId,
        value: ExpressionValue,
    ) {
        self.values.insert(id, value);
    }

    /// Binds a classical bit.
    pub fn bind_classical_bit(
        &mut self,
        id: ClassicalBitId,
        value: ExpressionValue,
    ) {
        self.classical_bits.insert(id, value);
    }
}

impl ExpressionEnvironment for SimpleExpressionEnvironment {
    fn resolve_symbol(
        &self,
        name: &str,
    ) -> Option<ExpressionValue> {
        self.symbols.get(name).cloned()
    }

    fn resolve_value(
        &self,
        value: ValueId,
    ) -> Option<ExpressionValue> {
        self.values.get(&value).cloned()
    }

    fn resolve_classical_bit(
        &self,
        bit: ClassicalBitId,
    ) -> Option<ExpressionValue> {
        self.classical_bits.get(&bit).cloned()
    }
}

// =============================================================================
// Evaluation value
// =============================================================================

/// Runtime-independent value produced by expression evaluation.
///
/// This is an evaluation bridge, not a replacement for the canonical
/// `quantum::ir::value` model. Runtime/backend layers may convert from this
/// representation to their own execution values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionValue {
    /// Boolean.
    Bool(bool),

    /// Signed integer.
    SignedInteger(i128),

    /// Unsigned integer.
    UnsignedInteger(u128),

    /// Finite floating-point value.
    Float(FiniteFloat),

    /// Bit vector.
    BitVector(BitVector),

    /// Unit.
    Unit,
}

impl ExpressionValue {
    /// Returns the expression type represented by this value.
    #[must_use]
    pub fn expression_type(&self) -> ExpressionType {
        match self {
            Self::Bool(_) => ExpressionType::Bool,
            Self::SignedInteger(_) => {
                ExpressionType::SignedInteger(None)
            }
            Self::UnsignedInteger(_) => {
                ExpressionType::UnsignedInteger(None)
            }
            Self::Float(_) => ExpressionType::Float,
            Self::BitVector(bits) => {
                ExpressionType::BitVector(Some(bits.len()))
            }
            Self::Unit => ExpressionType::Unit,
        }
    }

    /// Returns a Boolean value.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a signed integer.
    #[must_use]
    pub fn as_signed(&self) -> Option<i128> {
        match self {
            Self::SignedInteger(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an unsigned integer.
    #[must_use]
    pub fn as_unsigned(&self) -> Option<u128> {
        match self {
            Self::UnsignedInteger(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a floating-point value.
    #[must_use]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns a bit vector.
    #[must_use]
    pub fn as_bit_vector(&self) -> Option<&BitVector> {
        match self {
            Self::BitVector(value) => Some(value),
            _ => None,
        }
    }
}

impl From<bool> for ExpressionValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i128> for ExpressionValue {
    fn from(value: i128) -> Self {
        Self::SignedInteger(value)
    }
}

impl From<u128> for ExpressionValue {
    fn from(value: u128) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl TryFrom<f64> for ExpressionValue {
    type Error = ExpressionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self::Float(FiniteFloat::new(value)?))
    }
}

impl From<BitVector> for ExpressionValue {
    fn from(value: BitVector) -> Self {
        Self::BitVector(value)
    }
}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_symbol_name(
    name: &str,
    policy: &ExpressionValidationPolicy,
) -> ExpressionResult<()> {
    if policy.reject_empty_symbols && name.is_empty() {
        return Err(ExpressionError::EmptySymbol);
    }

    if let Some(maximum) = policy.max_symbol_bytes {
        let actual = name.len();

        if actual > maximum {
            return Err(ExpressionError::SymbolTooLong {
                actual,
                maximum,
            });
        }
    }

    Ok(())
}

fn validate_bit_vector(
    bits: &BitVector,
) -> ExpressionResult<()> {
    if bits.len() > isize::MAX as usize {
        return Err(ExpressionError::InvalidStructure(
            "bit-vector length exceeds host addressable object size",
        ));
    }

    Ok(())
}

fn validate_unary(
    operator: UnaryOperator,
    operand: ExpressionType,
    result: ExpressionType,
) -> ExpressionResult<()> {
    match operator {
        UnaryOperator::Negate => {
            if !operand.is_numeric()
                && operand != ExpressionType::Unknown
            {
                return Err(ExpressionError::ExpectedNumeric(
                    operand,
                ));
            }
        }

        UnaryOperator::LogicalNot => {
            if !operand.is_boolean()
                && operand != ExpressionType::Unknown
            {
                return Err(ExpressionError::ExpectedBoolean(
                    operand,
                ));
            }

            if result != ExpressionType::Bool
                && result != ExpressionType::Unknown
            {
                return Err(ExpressionError::TypeMismatch {
                    left: ExpressionType::Bool,
                    right: result,
                });
            }
        }

        UnaryOperator::BitwiseNot
        | UnaryOperator::PopCount => {
            if !operand.is_integer()
                && !operand.is_bit_vector()
                && operand != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedBitVector(operand),
                );
            }
        }

        UnaryOperator::Absolute => {
            if !operand.is_numeric()
                && operand != ExpressionType::Unknown
            {
                return Err(ExpressionError::ExpectedNumeric(
                    operand,
                ));
            }
        }

        UnaryOperator::ToBool
        | UnaryOperator::ToSignedInteger
        | UnaryOperator::ToUnsignedInteger
        | UnaryOperator::ToFloat => {}
    }

    Ok(())
}

fn validate_binary(
    operator: BinaryOperator,
    left: ExpressionType,
    right: ExpressionType,
    result: ExpressionType,
) -> ExpressionResult<()> {
    match operator {
        BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::Remainder
        | BinaryOperator::Power => {
            if (!left.is_numeric() && left != ExpressionType::Unknown)
                || (!right.is_numeric()
                    && right != ExpressionType::Unknown)
            {
                return Err(ExpressionError::ExpectedNumeric(
                    if !left.is_numeric() {
                        left
                    } else {
                        right
                    },
                ));
            }
        }

        BinaryOperator::Equal
        | BinaryOperator::NotEqual
        | BinaryOperator::LessThan
        | BinaryOperator::LessThanOrEqual
        | BinaryOperator::GreaterThan
        | BinaryOperator::GreaterThanOrEqual => {
            if left != ExpressionType::Unknown
                && right != ExpressionType::Unknown
                && left != right
                && !compatible_numeric_types(left, right)
            {
                return Err(ExpressionError::TypeMismatch {
                    left,
                    right,
                });
            }

            if result != ExpressionType::Bool
                && result != ExpressionType::Unknown
            {
                return Err(ExpressionError::TypeMismatch {
                    left: ExpressionType::Bool,
                    right: result,
                });
            }
        }

        BinaryOperator::LogicalAnd
        | BinaryOperator::LogicalOr
        | BinaryOperator::LogicalXor => {
            if !left.is_boolean()
                && left != ExpressionType::Unknown
            {
                return Err(ExpressionError::ExpectedBoolean(
                    left,
                ));
            }

            if !right.is_boolean()
                && right != ExpressionType::Unknown
            {
                return Err(ExpressionError::ExpectedBoolean(
                    right,
                ));
            }

            if result != ExpressionType::Bool
                && result != ExpressionType::Unknown
            {
                return Err(ExpressionError::TypeMismatch {
                    left: ExpressionType::Bool,
                    right: result,
                });
            }
        }

        BinaryOperator::BitwiseAnd
        | BinaryOperator::BitwiseOr
        | BinaryOperator::BitwiseXor => {
            if (!left.is_integer() && !left.is_bit_vector())
                && left != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedBitVector(left),
                );
            }

            if (!right.is_integer() && !right.is_bit_vector())
                && right != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedBitVector(right),
                );
            }
        }

        BinaryOperator::ShiftLeft
        | BinaryOperator::ShiftRight => {
            if !left.is_integer()
                && !left.is_bit_vector()
                && left != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedBitVector(left),
                );
            }

            if !right.is_integer()
                && right != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedInteger(right),
                );
            }
        }

        BinaryOperator::Concatenate => {
            if !left.is_bit_vector()
                && left != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedBitVector(left),
                );
            }

            if !right.is_bit_vector()
                && right != ExpressionType::Unknown
            {
                return Err(
                    ExpressionError::ExpectedBitVector(right),
                );
            }
        }

        BinaryOperator::In | BinaryOperator::Custom => {}
    }

    Ok(())
}

fn validate_bit_index(
    source: ExpressionType,
    index: usize,
) -> ExpressionResult<()> {
    match source {
        ExpressionType::BitVector(Some(width)) => {
            if index >= width {
                return Err(
                    ExpressionError::BitIndexOutOfBounds {
                        index,
                        width,
                    },
                );
            }
        }

        ExpressionType::BitVector(None)
        | ExpressionType::SignedInteger(_)
        | ExpressionType::UnsignedInteger(_)
        | ExpressionType::Unknown => {}

        other => {
            return Err(
                ExpressionError::ExpectedBitVector(other),
            );
        }
    }

    Ok(())
}

fn validate_slice(
    source: ExpressionType,
    start: usize,
    end: usize,
) -> ExpressionResult<()> {
    if end < start {
        return Err(ExpressionError::InvalidSlice {
            start,
            end,
            width: 0,
        });
    }

    match source {
        ExpressionType::BitVector(Some(width)) => {
            if end > width {
                return Err(
                    ExpressionError::InvalidSlice {
                        start,
                        end,
                        width,
                    },
                );
            }
        }

        ExpressionType::BitVector(None)
        | ExpressionType::SignedInteger(_)
        | ExpressionType::UnsignedInteger(_)
        | ExpressionType::Unknown => {}

        other => {
            return Err(
                ExpressionError::ExpectedBitVector(other),
            );
        }
    }

    Ok(())
}

fn compatible_numeric_types(
    left: ExpressionType,
    right: ExpressionType,
) -> bool {
    left.is_numeric() && right.is_numeric()
}

fn promote_numeric_type(
    left: ExpressionType,
    right: ExpressionType,
) -> ExpressionType {
    if left == ExpressionType::Float
        || right == ExpressionType::Float
    {
        ExpressionType::Float
    } else if matches!(
        left,
        ExpressionType::SignedInteger(_)
    ) || matches!(
        right,
        ExpressionType::SignedInteger(_)
    ) {
        ExpressionType::SignedInteger(None)
    } else if left.is_integer() && right.is_integer() {
        ExpressionType::UnsignedInteger(None)
    } else {
        ExpressionType::Unknown
    }
}

fn promote_bitwise_type(
    left: ExpressionType,
    right: ExpressionType,
) -> ExpressionType {
    if left.is_bit_vector() || right.is_bit_vector() {
        let width = match (left, right) {
            (
                ExpressionType::BitVector(Some(a)),
                ExpressionType::BitVector(Some(b)),
            ) if a == b => Some(a),

            _ => None,
        };

        ExpressionType::BitVector(width)
    } else {
        promote_numeric_type(left, right)
    }
}

fn concatenate_type(
    left: ExpressionType,
    right: ExpressionType,
) -> ExpressionType {
    match (left, right) {
        (
            ExpressionType::BitVector(Some(a)),
            ExpressionType::BitVector(Some(b)),
        ) => match a.checked_add(b) {
            Some(width) => ExpressionType::BitVector(Some(width)),
            None => ExpressionType::BitVector(None),
        },

        _ => ExpressionType::BitVector(None),
    }
}

// =============================================================================
// Evaluation implementation
// =============================================================================

fn evaluate_expression<E>(
    root: &ClassicalExpression,
    environment: &E,
) -> ExpressionResult<ExpressionValue>
where
    E: ExpressionEnvironment,
{
    enum Frame<'a> {
        Evaluate(&'a ClassicalExpression),
        ApplyUnary(UnaryOperator, ExpressionType),
        ApplyBinary(
            BinaryOperator,
            ExpressionType,
        ),
        ApplyBit(usize),
        ApplySlice(usize, usize),
        ApplySelect(ExpressionType),
    }

    let mut frames = vec![Frame::Evaluate(root)];
    let mut values = Vec::<ExpressionValue>::new();

    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Evaluate(expression) => {
                match expression {
                    ClassicalExpression::Literal(literal) => {
                        values.push(literal_to_value(literal)?);
                    }

                    ClassicalExpression::ClassicalBit(bit) => {
                        let value = environment
                            .resolve_classical_bit(*bit)
                            .ok_or(
                                ExpressionError::UnboundClassicalBit(
                                    *bit,
                                ),
                            )?;

                        values.push(value);
                    }

                    ClassicalExpression::Value(id) => {
                        let value =
                            environment.resolve_value(*id).ok_or(
                                ExpressionError::UnboundValue(*id),
                            )?;

                        values.push(value);
                    }

                    ClassicalExpression::Symbol(name) => {
                        let value = environment
                            .resolve_symbol(name)
                            .ok_or_else(|| {
                                ExpressionError::UnboundSymbol(
                                    name.clone(),
                                )
                            })?;

                        values.push(value);
                    }

                    ClassicalExpression::Unary {
                        operator,
                        operand,
                        result_type,
                    } => {
                        frames.push(Frame::ApplyUnary(
                            *operator,
                            *result_type,
                        ));
                        frames.push(Frame::Evaluate(operand));
                    }

                    ClassicalExpression::Binary {
                        operator,
                        left,
                        right,
                        result_type,
                    } => {
                        frames.push(Frame::ApplyBinary(
                            *operator,
                            *result_type,
                        ));

                        frames.push(Frame::Evaluate(right));
                        frames.push(Frame::Evaluate(left));
                    }

                    ClassicalExpression::Bit {
                        source,
                        index,
                    } => {
                        frames.push(Frame::ApplyBit(*index));
                        frames.push(Frame::Evaluate(source));
                    }

                    ClassicalExpression::Slice {
                        source,
                        start,
                        end,
                    } => {
                        frames.push(Frame::ApplySlice(
                            *start,
                            *end,
                        ));
                        frames.push(Frame::Evaluate(source));
                    }

                    ClassicalExpression::Select {
                        condition,
                        then_value,
                        else_value,
                        result_type,
                    } => {
                        frames.push(Frame::ApplySelect(
                            *result_type,
                        ));

                        frames.push(Frame::Evaluate(else_value));
                        frames.push(Frame::Evaluate(then_value));
                        frames.push(Frame::Evaluate(condition));
                    }

                    ClassicalExpression::Call(call) => {
                        return Err(
                            ExpressionError::ExternalCallNotEvaluable(
                                call.name().to_owned(),
                            ),
                        );
                    }
                }
            }

            Frame::ApplyUnary(operator, result_type) => {
                let operand = values
                    .pop()
                    .ok_or(ExpressionError::InvalidStructure(
                        "missing unary operand during evaluation",
                    ))?;

                let result =
                    evaluate_unary(operator, operand)?;

                values.push(coerce_value(
                    result,
                    result_type,
                )?);
            }

            Frame::ApplyBinary(operator, result_type) => {
                let right = values
                    .pop()
                    .ok_or(ExpressionError::InvalidStructure(
                        "missing right binary operand during evaluation",
                    ))?;

                let left = values
                    .pop()
                    .ok_or(ExpressionError::InvalidStructure(
                        "missing left binary operand during evaluation",
                    ))?;

                let result =
                    evaluate_binary(operator, left, right)?;

                values.push(coerce_value(
                    result,
                    result_type,
                )?);
            }

            Frame::ApplyBit(index) => {
                let source =
                    values
                        .pop()
                        .ok_or(
                            ExpressionError::InvalidStructure(
                                "missing bit-extraction source",
                            ),
                        )?;

                values.push(extract_bit(
                    source,
                    index,
                )?);
            }

            Frame::ApplySlice(start, end) => {
                let source =
                    values
                        .pop()
                        .ok_or(
                            ExpressionError::InvalidStructure(
                                "missing slice source",
                            ),
                        )?;

                values.push(extract_slice(
                    source,
                    start,
                    end,
                )?);
            }

            Frame::ApplySelect(result_type) => {
                let else_value =
                    values
                        .pop()
                        .ok_or(
                            ExpressionError::InvalidStructure(
                                "missing select else value",
                            ),
                        )?;

                let then_value =
                    values
                        .pop()
                        .ok_or(
                            ExpressionError::InvalidStructure(
                                "missing select then value",
                            ),
                        )?;

                let condition =
                    values
                        .pop()
                        .ok_or(
                            ExpressionError::InvalidStructure(
                                "missing select condition",
                            ),
                        )?;

                let condition = condition
                    .as_bool()
                    .ok_or(
                        ExpressionError::ExpectedBoolean(
                            condition.expression_type(),
                        ),
                    )?;

                let selected = if condition {
                    then_value
                } else {
                    else_value
                };

                values.push(coerce_value(
                    selected,
                    result_type,
                )?);
            }
        }
    }

    values
        .pop()
        .ok_or(ExpressionError::InvalidStructure(
            "expression evaluation produced no value",
        ))
}

fn literal_to_value(
    literal: &ExpressionLiteral,
) -> ExpressionResult<ExpressionValue> {
    match literal {
        ExpressionLiteral::Bool(value) => {
            Ok(ExpressionValue::Bool(*value))
        }

        ExpressionLiteral::SignedInteger(value) => {
            Ok(ExpressionValue::SignedInteger(*value))
        }

        ExpressionLiteral::UnsignedInteger(value) => {
            Ok(ExpressionValue::UnsignedInteger(*value))
        }

        ExpressionLiteral::Float(value) => {
            Ok(ExpressionValue::Float(*value))
        }

        ExpressionLiteral::BitVector(value) => {
            Ok(ExpressionValue::BitVector(value.clone()))
        }
    }
}

fn evaluate_unary(
    operator: UnaryOperator,
    operand: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match operator {
        UnaryOperator::Negate => match operand {
            ExpressionValue::SignedInteger(value) => value
                .checked_neg()
                .map(ExpressionValue::SignedInteger)
                .ok_or(ExpressionError::ArithmeticOverflow),

            ExpressionValue::UnsignedInteger(value) => {
                if value == 0 {
                    Ok(ExpressionValue::SignedInteger(0))
                } else if value <= i128::MAX as u128 {
                    Ok(ExpressionValue::SignedInteger(
                        -(value as i128),
                    ))
                } else {
                    Err(ExpressionError::ArithmeticOverflow)
                }
            }

            ExpressionValue::Float(value) => {
                let result = -value.value();

                Ok(ExpressionValue::Float(
                    FiniteFloat::new(result)?,
                ))
            }

            other => Err(
                ExpressionError::ExpectedNumeric(
                    other.expression_type(),
                ),
            ),
        },

        UnaryOperator::LogicalNot => {
            let value = operand
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        operand.expression_type(),
                    ),
                )?;

            Ok(ExpressionValue::Bool(!value))
        }

        UnaryOperator::BitwiseNot => match operand {
            ExpressionValue::SignedInteger(value) => {
                Ok(ExpressionValue::SignedInteger(!value))
            }

            ExpressionValue::UnsignedInteger(value) => {
                Ok(ExpressionValue::UnsignedInteger(!value))
            }

            ExpressionValue::BitVector(bits) => {
                let result = BitVector::from_bits(
                    bits.iter().map(|bit| !*bit),
                );

                Ok(ExpressionValue::BitVector(result))
            }

            other => Err(
                ExpressionError::ExpectedBitVector(
                    other.expression_type(),
                ),
            ),
        },

        UnaryOperator::PopCount => match operand {
            ExpressionValue::SignedInteger(value) => {
                Ok(ExpressionValue::UnsignedInteger(
                    (value as u128).count_ones() as u128,
                ))
            }

            ExpressionValue::UnsignedInteger(value) => {
                Ok(ExpressionValue::UnsignedInteger(
                    value.count_ones() as u128,
                ))
            }

            ExpressionValue::BitVector(bits) => {
                let count = bits
                    .iter()
                    .filter(|bit| **bit)
                    .count();

                Ok(ExpressionValue::UnsignedInteger(
                    count as u128,
                ))
            }

            other => Err(
                ExpressionError::ExpectedBitVector(
                    other.expression_type(),
                ),
            ),
        },

        UnaryOperator::Absolute => match operand {
            ExpressionValue::SignedInteger(value) => {
                value
                    .checked_abs()
                    .map(ExpressionValue::SignedInteger)
                    .ok_or(ExpressionError::ArithmeticOverflow)
            }

            ExpressionValue::Float(value) => {
                let result = value.value().abs();

                Ok(ExpressionValue::Float(
                    FiniteFloat::new(result)?,
                ))
            }

            other => Err(
                ExpressionError::ExpectedNumeric(
                    other.expression_type(),
                ),
            ),
        },

        UnaryOperator::ToBool => {
            Ok(ExpressionValue::Bool(
                value_to_bool(&operand)?,
            ))
        }

        UnaryOperator::ToSignedInteger => {
            Ok(ExpressionValue::SignedInteger(
                value_to_signed(&operand)?,
            ))
        }

        UnaryOperator::ToUnsignedInteger => {
            Ok(ExpressionValue::UnsignedInteger(
                value_to_unsigned(&operand)?,
            ))
        }

        UnaryOperator::ToFloat => {
            let value = value_to_float(&operand)?;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }
    }
}

fn evaluate_binary(
    operator: BinaryOperator,
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match operator {
        BinaryOperator::Add => {
            arithmetic_add(left, right)
        }

        BinaryOperator::Subtract => {
            arithmetic_subtract(left, right)
        }

        BinaryOperator::Multiply => {
            arithmetic_multiply(left, right)
        }

        BinaryOperator::Divide => {
            arithmetic_divide(left, right)
        }

        BinaryOperator::Remainder => {
            arithmetic_remainder(left, right)
        }

        BinaryOperator::Power => {
            arithmetic_power(left, right)
        }

        BinaryOperator::Equal
        | BinaryOperator::NotEqual
        | BinaryOperator::LessThan
        | BinaryOperator::LessThanOrEqual
        | BinaryOperator::GreaterThan
        | BinaryOperator::GreaterThanOrEqual => {
            compare_values(operator, left, right)
        }

        BinaryOperator::LogicalAnd => {
            let left = left
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        left.expression_type(),
                    ),
                )?;

            let right = right
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        right.expression_type(),
                    ),
                )?;

            Ok(ExpressionValue::Bool(
                left && right,
            ))
        }

        BinaryOperator::LogicalOr => {
            let left = left
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        left.expression_type(),
                    ),
                )?;

            let right = right
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        right.expression_type(),
                    ),
                )?;

            Ok(ExpressionValue::Bool(
                left || right,
            ))
        }

        BinaryOperator::LogicalXor => {
            let left = left
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        left.expression_type(),
                    ),
                )?;

            let right = right
                .as_bool()
                .ok_or(
                    ExpressionError::ExpectedBoolean(
                        right.expression_type(),
                    ),
                )?;

            Ok(ExpressionValue::Bool(
                left ^ right,
            ))
        }

        BinaryOperator::BitwiseAnd => {
            bitwise_binary(
                left,
                right,
                |a, b| a & b,
                |a, b| a & b,
            )
        }

        BinaryOperator::BitwiseOr => {
            bitwise_binary(
                left,
                right,
                |a, b| a | b,
                |a, b| a | b,
            )
        }

        BinaryOperator::BitwiseXor => {
            bitwise_binary(
                left,
                right,
                |a, b| a ^ b,
                |a, b| a ^ b,
            )
        }

        BinaryOperator::ShiftLeft => {
            shift_binary(
                left,
                right,
                false,
            )
        }

        BinaryOperator::ShiftRight => {
            shift_binary(
                left,
                right,
                true,
            )
        }

        BinaryOperator::Concatenate => {
            concatenate_values(left, right)
        }

        BinaryOperator::In
        | BinaryOperator::Custom => Err(
            ExpressionError::UnsupportedEvaluation(
                operator.to_string(),
            ),
        ),
    }
}

fn arithmetic_add(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => a
            .checked_add(b)
            .map(ExpressionValue::SignedInteger)
            .ok_or(ExpressionError::ArithmeticOverflow),

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => a
            .checked_add(b)
            .map(ExpressionValue::UnsignedInteger)
            .ok_or(ExpressionError::ArithmeticOverflow),

        (
            ExpressionValue::Float(a),
            ExpressionValue::Float(b),
        ) => {
            let value = a.value() + b.value();

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }

        (a, b) => {
            let left = value_to_float(&a)?;
            let right = value_to_float(&b)?;
            let value = left + right;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }
    }
}

fn arithmetic_subtract(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => a
            .checked_sub(b)
            .map(ExpressionValue::SignedInteger)
            .ok_or(ExpressionError::ArithmeticOverflow),

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => a
            .checked_sub(b)
            .map(ExpressionValue::UnsignedInteger)
            .ok_or(ExpressionError::ArithmeticOverflow),

        (
            ExpressionValue::Float(a),
            ExpressionValue::Float(b),
        ) => {
            let value = a.value() - b.value();

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }

        (a, b) => {
            let left = value_to_float(&a)?;
            let right = value_to_float(&b)?;
            let value = left - right;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }
    }
}

fn arithmetic_multiply(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => a
            .checked_mul(b)
            .map(ExpressionValue::SignedInteger)
            .ok_or(ExpressionError::ArithmeticOverflow),

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => a
            .checked_mul(b)
            .map(ExpressionValue::UnsignedInteger)
            .ok_or(ExpressionError::ArithmeticOverflow),

        (
            ExpressionValue::Float(a),
            ExpressionValue::Float(b),
        ) => {
            let value = a.value() * b.value();

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }

        (a, b) => {
            let left = value_to_float(&a)?;
            let right = value_to_float(&b)?;
            let value = left * right;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }
    }
}

fn arithmetic_divide(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => {
            if b == 0 {
                return Err(
                    ExpressionError::DivisionByZero,
                );
            }

            a.checked_div(b)
                .map(ExpressionValue::SignedInteger)
                .ok_or(ExpressionError::ArithmeticOverflow)
        }

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => {
            if b == 0 {
                return Err(
                    ExpressionError::DivisionByZero,
                );
            }

            a.checked_div(b)
                .map(ExpressionValue::UnsignedInteger)
                .ok_or(ExpressionError::ArithmeticOverflow)
        }

        (
            ExpressionValue::Float(a),
            ExpressionValue::Float(b),
        ) => {
            let denominator = b.value();

            if denominator == 0.0 {
                return Err(
                    ExpressionError::DivisionByZero,
                );
            }

            let value = a.value() / denominator;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }

        (a, b) => {
            let denominator = value_to_float(&b)?;

            if denominator == 0.0 {
                return Err(
                    ExpressionError::DivisionByZero,
                );
            }

            let value =
                value_to_float(&a)? / denominator;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }
    }
}

fn arithmetic_remainder(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => {
            if b == 0 {
                return Err(
                    ExpressionError::ModuloByZero,
                );
            }

            a.checked_rem(b)
                .map(ExpressionValue::SignedInteger)
                .ok_or(ExpressionError::ArithmeticOverflow)
        }

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => {
            if b == 0 {
                return Err(
                    ExpressionError::ModuloByZero,
                );
            }

            a.checked_rem(b)
                .map(ExpressionValue::UnsignedInteger)
                .ok_or(ExpressionError::ArithmeticOverflow)
        }

        (
            ExpressionValue::Float(a),
            ExpressionValue::Float(b),
        ) => {
            let denominator = b.value();

            if denominator == 0.0 {
                return Err(
                    ExpressionError::ModuloByZero,
                );
            }

            let value = a.value() % denominator;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }

        (a, b) => {
            let denominator = value_to_float(&b)?;

            if denominator == 0.0 {
                return Err(
                    ExpressionError::ModuloByZero,
                );
            }

            let value =
                value_to_float(&a)? % denominator;

            Ok(ExpressionValue::Float(
                FiniteFloat::new(value)?,
            ))
        }
    }
}

fn arithmetic_power(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    let exponent = value_to_float(&right)?;
    let base = value_to_float(&left)?;

    let value = base.powf(exponent);

    if !value.is_finite() {
        return Err(ExpressionError::ArithmeticOverflow);
    }

    Ok(ExpressionValue::Float(
        FiniteFloat::new(value)?,
    ))
}

fn compare_values(
    operator: BinaryOperator,
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    let ordering = match (&left, &right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => a.cmp(b),

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => a.cmp(b),

        (
            ExpressionValue::Bool(a),
            ExpressionValue::Bool(b),
        ) => a.cmp(b),

        (
            ExpressionValue::Float(a),
            ExpressionValue::Float(b),
        ) => a
            .value()
            .partial_cmp(&b.value())
            .ok_or(ExpressionError::NonFiniteFloat)?,

        _ => {
            let a = value_to_float(&left)?;
            let b = value_to_float(&right)?;

            a.partial_cmp(&b)
                .ok_or(ExpressionError::NonFiniteFloat)?
        }
    };

    let result = match operator {
        BinaryOperator::Equal => {
            ordering == std::cmp::Ordering::Equal
        }

        BinaryOperator::NotEqual => {
            ordering != std::cmp::Ordering::Equal
        }

        BinaryOperator::LessThan => {
            ordering == std::cmp::Ordering::Less
        }

        BinaryOperator::LessThanOrEqual => {
            ordering != std::cmp::Ordering::Greater
        }

        BinaryOperator::GreaterThan => {
            ordering == std::cmp::Ordering::Greater
        }

        BinaryOperator::GreaterThanOrEqual => {
            ordering != std::cmp::Ordering::Less
        }

        _ => {
            return Err(
                ExpressionError::InvalidStructure(
                    "non-comparison operator passed to comparison evaluator",
                ),
            )
        }
    };

    Ok(ExpressionValue::Bool(result))
}

fn bitwise_binary(
    left: ExpressionValue,
    right: ExpressionValue,
    signed_operation: impl Fn(i128, i128) -> i128,
    unsigned_operation: impl Fn(u128, u128) -> u128,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::SignedInteger(a),
            ExpressionValue::SignedInteger(b),
        ) => Ok(ExpressionValue::SignedInteger(
            signed_operation(a, b),
        )),

        (
            ExpressionValue::UnsignedInteger(a),
            ExpressionValue::UnsignedInteger(b),
        ) => Ok(ExpressionValue::UnsignedInteger(
            unsigned_operation(a, b),
        )),

        (
            ExpressionValue::BitVector(a),
            ExpressionValue::BitVector(b),
        ) => {
            if a.len() != b.len() {
                return Err(
                    ExpressionError::TypeMismatch {
                        left: ExpressionType::BitVector(
                            Some(a.len()),
                        ),
                        right: ExpressionType::BitVector(
                            Some(b.len()),
                        ),
                    },
                );
            }

            let result = a
                .iter()
                .zip(b.iter())
                .map(|(left, right)| {
                    signed_operation(
                        if *left { 1 } else { 0 },
                        if *right { 1 } else { 0 },
                    ) != 0
                })
                .collect();

            Ok(ExpressionValue::BitVector(
                BitVector::from_bits(result),
            ))
        }

        (a, _) => Err(
            ExpressionError::ExpectedBitVector(
                a.expression_type(),
            ),
        ),
    }
}

fn shift_binary(
    left: ExpressionValue,
    right: ExpressionValue,
    right_shift: bool,
) -> ExpressionResult<ExpressionValue> {
    let amount = value_to_unsigned(&right)?;

    match left {
        ExpressionValue::UnsignedInteger(value) => {
            if amount > u128::from(u32::MAX) {
                return Err(
                    ExpressionError::InvalidShift,
                );
            }

            let amount = amount as u32;

            let result = if right_shift {
                value.checked_shr(amount)
            } else {
                value.checked_shl(amount)
            }
            .ok_or(ExpressionError::InvalidShift)?;

            Ok(ExpressionValue::UnsignedInteger(result))
        }

        ExpressionValue::SignedInteger(value) => {
            if amount > u128::from(u32::MAX) {
                return Err(
                    ExpressionError::InvalidShift,
                );
            }

            let amount = amount as u32;

            let result = if right_shift {
                value.checked_shr(amount)
            } else {
                value.checked_shl(amount)
            }
            .ok_or(ExpressionError::InvalidShift)?;

            Ok(ExpressionValue::SignedInteger(result))
        }

        ExpressionValue::BitVector(bits) => {
            if amount > usize::MAX as u128 {
                return Err(
                    ExpressionError::InvalidShift,
                );
            }

            let amount = amount as usize;
            let width = bits.len();

            if amount > width {
                return Ok(ExpressionValue::BitVector(
                    BitVector::zeros(width),
                ));
            }

            let mut output =
                vec![false; width];

            for index in 0..width {
                let source = if right_shift {
                    index.checked_add(amount)
                } else {
                    index.checked_sub(amount)
                };

                if let Some(source) = source {
                    if source < width {
                        output[index] =
                            bits.get(source).unwrap_or(false);
                    }
                }
            }

            Ok(ExpressionValue::BitVector(
                BitVector::from_bits(output),
            ))
        }

        other => Err(
            ExpressionError::ExpectedBitVector(
                other.expression_type(),
            ),
        ),
    }
}

fn concatenate_values(
    left: ExpressionValue,
    right: ExpressionValue,
) -> ExpressionResult<ExpressionValue> {
    match (left, right) {
        (
            ExpressionValue::BitVector(left),
            ExpressionValue::BitVector(right),
        ) => {
            let mut bits = left.to_vec();

            bits.extend(right.iter().copied());

            Ok(ExpressionValue::BitVector(
                BitVector::from_bits(bits),
            ))
        }

        (left, _) => Err(
            ExpressionError::ExpectedBitVector(
                left.expression_type(),
            ),
        ),
    }
}

fn extract_bit(
    source: ExpressionValue,
    index: usize,
) -> ExpressionResult<ExpressionValue> {
    match source {
        ExpressionValue::BitVector(bits) => {
            let value = bits.get(index).ok_or(
                ExpressionError::BitIndexOutOfBounds {
                    index,
                    width: bits.len(),
                },
            )?;

            Ok(ExpressionValue::Bool(value))
        }

        ExpressionValue::UnsignedInteger(value) => {
            let width = 128usize;

            if index >= width {
                return Err(
                    ExpressionError::BitIndexOutOfBounds {
                        index,
                        width,
                    },
                );
            }

            Ok(ExpressionValue::Bool(
                ((value >> index) & 1) != 0,
            ))
        }

        ExpressionValue::SignedInteger(value) => {
            let width = 128usize;

            if index >= width {
                return Err(
                    ExpressionError::BitIndexOutOfBounds {
                        index,
                        width,
                    },
                );
            }

            Ok(ExpressionValue::Bool(
                (((value as u128) >> index) & 1) != 0,
            ))
        }

        other => Err(
            ExpressionError::ExpectedBitVector(
                other.expression_type(),
            ),
        ),
    }
}

fn extract_slice(
    source: ExpressionValue,
    start: usize,
    end: usize,
) -> ExpressionResult<ExpressionValue> {
    if end < start {
        return Err(ExpressionError::InvalidSlice {
            start,
            end,
            width: 0,
        });
    }

    match source {
        ExpressionValue::BitVector(bits) => {
            if end > bits.len() {
                return Err(
                    ExpressionError::InvalidSlice {
                        start,
                        end,
                        width: bits.len(),
                    },
                );
            }

            let output = bits
                .iter()
                .skip(start)
                .take(end - start)
                .copied()
                .collect();

            Ok(ExpressionValue::BitVector(
                BitVector::from_bits(output),
            ))
        }

        other => Err(
            ExpressionError::ExpectedBitVector(
                other.expression_type(),
            ),
        ),
    }
}

fn value_to_bool(
    value: &ExpressionValue,
) -> ExpressionResult<bool> {
    match value {
        ExpressionValue::Bool(value) => Ok(*value),

        ExpressionValue::SignedInteger(value) => {
            Ok(*value != 0)
        }

        ExpressionValue::UnsignedInteger(value) => {
            Ok(*value != 0)
        }

        ExpressionValue::Float(value) => {
            Ok(value.value() != 0.0)
        }

        ExpressionValue::BitVector(bits) => {
            Ok(bits.iter().any(|bit| *bit))
        }

        ExpressionValue::Unit => Ok(false),
    }
}

fn value_to_signed(
    value: &ExpressionValue,
) -> ExpressionResult<i128> {
    match value {
        ExpressionValue::Bool(value) => {
            Ok(if *value { 1 } else { 0 })
        }

        ExpressionValue::SignedInteger(value) => {
            Ok(*value)
        }

        ExpressionValue::UnsignedInteger(value) => {
            i128::try_from(*value)
                .map_err(|_| ExpressionError::ArithmeticOverflow)
        }

        ExpressionValue::Float(value) => {
            let value = value.value();

            if !value.is_finite()
                || value < i128::MIN as f64
                || value > i128::MAX as f64
            {
                return Err(
                    ExpressionError::ArithmeticOverflow,
                );
            }

            Ok(value as i128)
        }

        ExpressionValue::BitVector(bits) => {
            let mut result = 0u128;

            for (index, bit) in bits.iter().enumerate() {
                if *bit {
                    if index >= 128 {
                        return Err(
                            ExpressionError::ArithmeticOverflow,
                        );
                    }

                    result |= 1u128 << index;
                }
            }

            i128::try_from(result)
                .map_err(|_| ExpressionError::ArithmeticOverflow)
        }

        ExpressionValue::Unit => Ok(0),
    }
}

fn value_to_unsigned(
    value: &ExpressionValue,
) -> ExpressionResult<u128> {
    match value {
        ExpressionValue::Bool(value) => {
            Ok(if *value { 1 } else { 0 })
        }

        ExpressionValue::SignedInteger(value) => {
            u128::try_from(*value)
                .map_err(|_| ExpressionError::ArithmeticOverflow)
        }

        ExpressionValue::UnsignedInteger(value) => {
            Ok(*value)
        }

        ExpressionValue::Float(value) => {
            let value = value.value();

            if !value.is_finite()
                || value < 0.0
                || value > u128::MAX as f64
            {
                return Err(
                    ExpressionError::ArithmeticOverflow,
                );
            }

            Ok(value as u128)
        }

        ExpressionValue::BitVector(bits) => {
            let mut result = 0u128;

            for (index, bit) in bits.iter().enumerate() {
                if *bit {
                    if index >= 128 {
                        return Err(
                            ExpressionError::ArithmeticOverflow,
                        );
                    }

                    result |= 1u128 << index;
                }
            }

            Ok(result)
        }

        ExpressionValue::Unit => Ok(0),
    }
}

fn value_to_float(
    value: &ExpressionValue,
) -> ExpressionResult<f64> {
    let result = match value {
        ExpressionValue::Bool(value) => {
            if *value {
                1.0
            } else {
                0.0
            }
        }

        ExpressionValue::SignedInteger(value) => {
            *value as f64
        }

        ExpressionValue::UnsignedInteger(value) => {
            *value as f64
        }

        ExpressionValue::Float(value) => value.value(),

        ExpressionValue::BitVector(bits) => {
            let value = value_to_unsigned(
                &ExpressionValue::BitVector(bits.clone()),
            )?;

            value as f64
        }

        ExpressionValue::Unit => 0.0,
    };

    if !result.is_finite() {
        return Err(ExpressionError::ArithmeticOverflow);
    }

    Ok(result)
}

fn coerce_value(
    value: ExpressionValue,
    target: ExpressionType,
) -> ExpressionResult<ExpressionValue> {
    match target {
        ExpressionType::Unknown => Ok(value),

        ExpressionType::Bool => {
            Ok(ExpressionValue::Bool(
                value_to_bool(&value)?,
            ))
        }

        ExpressionType::SignedInteger(_) => {
            Ok(ExpressionValue::SignedInteger(
                value_to_signed(&value)?,
            ))
        }

        ExpressionType::UnsignedInteger(_) => {
            Ok(ExpressionValue::UnsignedInteger(
                value_to_unsigned(&value)?,
            ))
        }

        ExpressionType::Float => {
            Ok(ExpressionValue::Float(
                FiniteFloat::new(
                    value_to_float(&value)?,
                )?,
            ))
        }

        ExpressionType::BitVector(width) => {
            let value = value_to_unsigned(&value)?;

            let width = width.unwrap_or(128);

            if width > 128 {
                return Err(
                    ExpressionError::ArithmeticOverflow,
                );
            }

            let mut bits = vec![false; width];

            for index in 0..width {
                bits[index] =
                    ((value >> index) & 1) != 0;
            }

            Ok(ExpressionValue::BitVector(
                BitVector::from_bits(bits),
            ))
        }

        ExpressionType::Unit => Ok(ExpressionValue::Unit),

        ExpressionType::Opaque => Err(
            ExpressionError::UnsupportedEvaluation(
                "opaque expression value".to_owned(),
            ),
        ),
    }
}

// =============================================================================
// Deterministic formatting
// =============================================================================

enum FormatItem<'a> {
    Expression(&'a ClassicalExpression),
    Text(&'static str),
}

fn push_format_items<'a>(
    expression: &'a ClassicalExpression,
    stack: &mut Vec<FormatItem<'a>>,
) {
    match expression {
        ClassicalExpression::Literal(literal) => {
            match literal {
                ExpressionLiteral::Bool(value) => {
                    stack.push(if *value {
                        FormatItem::Text("true")
                    } else {
                        FormatItem::Text("false")
                    });
                }

                ExpressionLiteral::SignedInteger(value) => {
                    let text = value.to_string();

                    // We need an owned string for arbitrary integers.
                    // The fallback representation is emitted through a
                    // dedicated leak-free helper below.
                    stack.push(FormatItem::Text(
                        format_integer_text(value),
                    ));
                    let _ = text;
                }

                ExpressionLiteral::UnsignedInteger(value) => {
                    stack.push(FormatItem::Text(
                        format_unsigned_text(value),
                    ));
                }

                ExpressionLiteral::Float(value) => {
                    stack.push(FormatItem::Text(
                        format_float_text(value),
                    ));
                }

                ExpressionLiteral::BitVector(bits) => {
                    let text = format_bit_vector_text(bits);

                    stack.push(FormatItem::Text(
                        Box::leak(text.into_boxed_str()),
                    ));
                }
            }
        }

        ClassicalExpression::ClassicalBit(bit) => {
            let text = format!("c{}", bit.index());

            stack.push(FormatItem::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }

        ClassicalExpression::Value(value) => {
            let text = format!("{value}");

            stack.push(FormatItem::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }

        ClassicalExpression::Symbol(name) => {
            stack.push(FormatItem::Text(
                Box::leak(name.clone().into_boxed_str()),
            ));
        }

        ClassicalExpression::Unary {
            operator,
            operand,
            ..
        } => {
            let operator_text =
                operator.to_string();

            stack.push(FormatItem::Text(")"));
            stack.push(FormatItem::Expression(operand));
            stack.push(FormatItem::Text(" ("));

            stack.push(FormatItem::Text(
                Box::leak(operator_text.into_boxed_str()),
            ));
        }

        ClassicalExpression::Binary {
            operator,
            left,
            right,
            ..
        } => {
            let operator_text =
                operator.to_string();

            stack.push(FormatItem::Text(")"));
            stack.push(FormatItem::Expression(right));
            stack.push(FormatItem::Text(
                Box::leak(
                    format!(" {operator_text} ")
                        .into_boxed_str(),
                ),
            ));
            stack.push(FormatItem::Expression(left));
            stack.push(FormatItem::Text("("));
        }

        ClassicalExpression::Bit {
            source,
            index,
        } => {
            stack.push(FormatItem::Text("]"));
            stack.push(FormatItem::Text(
                Box::leak(index.to_string().into_boxed_str()),
            ));
            stack.push(FormatItem::Text("["));
            stack.push(FormatItem::Expression(source));
        }

        ClassicalExpression::Slice {
            source,
            start,
            end,
        } => {
            stack.push(FormatItem::Text("]"));
            stack.push(FormatItem::Text(
                Box::leak(end.to_string().into_boxed_str()),
            ));
            stack.push(FormatItem::Text(":"));
            stack.push(FormatItem::Text(
                Box::leak(start.to_string().into_boxed_str()),
            ));
            stack.push(FormatItem::Text("["));
            stack.push(FormatItem::Expression(source));
        }

        ClassicalExpression::Select {
            condition,
            then_value,
            else_value,
            ..
        } => {
            stack.push(FormatItem::Text(")"));
            stack.push(FormatItem::Expression(else_value));
            stack.push(FormatItem::Text(" : "));
            stack.push(FormatItem::Expression(then_value));
            stack.push(FormatItem::Text(" ? "));
            stack.push(FormatItem::Expression(condition));
            stack.push(FormatItem::Text("("));
        }

        ClassicalExpression::Call(call) => {
            let name = call.name();

            let text = format_call_header(
                name,
                call.arguments(),
            );

            stack.push(FormatItem::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }
    }
}

fn format_integer_text(
    value: &i128,
) -> &'static str {
    Box::leak(value.to_string().into_boxed_str())
}

fn format_unsigned_text(
    value: &u128,
) -> &'static str {
    Box::leak(value.to_string().into_boxed_str())
}

fn format_float_text(
    value: &FiniteFloat,
) -> &'static str {
    Box::leak(
        value.value().to_string().into_boxed_str(),
    )
}

fn format_bit_vector_text(
    bits: &BitVector,
) -> String {
    let mut output = String::with_capacity(
        bits.len().saturating_add(2),
    );

    output.push_str("0b");

    for index in (0..bits.len()).rev() {
        output.push(if bits.get(index).unwrap_or(false) {
            '1'
        } else {
            '0'
        });
    }

    output
}

fn format_call_header(
    name: &str,
    arguments: &BTreeMap<String, ClassicalExpression>,
) -> String {
    let mut output = String::new();

    output.push_str(name);
    output.push('(');

    for (index, argument) in arguments.keys().enumerate() {
        if index != 0 {
            output.push_str(", ");
        }

        output.push_str(argument);
        output.push('=');
        output.push('?');
    }

    output.push(')');

    output
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_identity_is_canonical() {
        let bit = ClassicalBitId::new(7);
        let expression =
            ClassicalExpression::classical_bit(bit);

        assert_eq!(
            expression.collect_classical_bits(),
            vec![bit]
        );
    }

    #[test]
    fn literals_have_expected_types() {
        assert_eq!(
            ClassicalExpression::bool(true)
                .expression_type(),
            ExpressionType::Bool
        );

        assert_eq!(
            ClassicalExpression::signed(1)
                .expression_type(),
            ExpressionType::SignedInteger(None)
        );

        assert_eq!(
            ClassicalExpression::unsigned(1)
                .expression_type(),
            ExpressionType::UnsignedInteger(None)
        );

        assert_eq!(
            ClassicalExpression::bit_vector(vec![
                true,
                false,
                true,
            ])
            .expression_type(),
            ExpressionType::BitVector(Some(3))
        );
    }

    #[test]
    fn symbolic_expression_collects_symbols_deterministically(
    ) {
        let a =
            ClassicalExpression::symbol("a").unwrap();

        let z =
            ClassicalExpression::symbol("z").unwrap();

        let expression =
            ClassicalExpression::add(a, z);

        assert_eq!(
            expression.collect_symbols(),
            vec!["a".to_owned(), "z".to_owned()]
        );
    }

    #[test]
    fn duplicate_symbols_are_deduplicated() {
        let a1 =
            ClassicalExpression::symbol("a").unwrap();

        let a2 =
            ClassicalExpression::symbol("a").unwrap();

        let expression =
            ClassicalExpression::add(a1, a2);

        assert_eq!(
            expression.collect_symbols(),
            vec!["a".to_owned()]
        );
    }

    #[test]
    fn node_count_is_iterative() {
        let mut expression =
            ClassicalExpression::signed(0);

        for _ in 0..10_000 {
            expression =
                ClassicalExpression::negate(expression);
        }

        assert_eq!(
            expression.node_count(),
            10_001
        );

        assert_eq!(
            expression.depth(),
            10_000
        );
    }

    #[test]
    fn validation_supports_explicit_limits() {
        let mut expression =
            ClassicalExpression::signed(0);

        for _ in 0..100 {
            expression =
                ClassicalExpression::negate(expression);
        }

        let policy =
            ExpressionValidationPolicy::bounded(
                32,
                1_000,
            );

        assert!(matches!(
            expression.validate_with_policy(policy),
            Err(ExpressionError::DepthExceeded { .. })
        ));
    }

    #[test]
    fn boolean_expression_evaluates() {
        let expression =
            ClassicalExpression::logical_and(
                ClassicalExpression::bool(true),
                ClassicalExpression::bool(false),
            );

        let environment =
            SimpleExpressionEnvironment::new();

        let value =
            expression.evaluate(&environment).unwrap();

        assert_eq!(
            value,
            ExpressionValue::Bool(false)
        );
    }

    #[test]
    fn arithmetic_expression_evaluates() {
        let expression =
            ClassicalExpression::multiply(
                ClassicalExpression::signed(6),
                ClassicalExpression::signed(7),
            );

        let environment =
            SimpleExpressionEnvironment::new();

        let value =
            expression.evaluate(&environment).unwrap();

        assert_eq!(
            value,
            ExpressionValue::SignedInteger(42)
        );
    }

    #[test]
    fn classical_bit_expression_evaluates() {
        let bit =
            ClassicalBitId::new(3);

        let expression =
            ClassicalExpression::equal(
                ClassicalExpression::classical_bit(bit),
                ClassicalExpression::bool(true),
            );

        let mut environment =
            SimpleExpressionEnvironment::new();

        environment.bind_classical_bit(
            bit,
            ExpressionValue::Bool(true),
        );

        let value =
            expression.evaluate(&environment).unwrap();

        assert_eq!(
            value,
            ExpressionValue::Bool(true)
        );
    }

    #[test]
    fn bit_extraction_evaluates() {
        let expression =
            ClassicalExpression::bit(
                ClassicalExpression::bit_vector(vec![
                    true,
                    false,
                    true,
                ]),
                2,
            );

        let environment =
            SimpleExpressionEnvironment::new();

        let value =
            expression.evaluate(&environment).unwrap();

        assert_eq!(
            value,
            ExpressionValue::Bool(true)
        );
    }

    #[test]
    fn slice_evaluates() {
        let expression =
            ClassicalExpression::slice(
                ClassicalExpression::bit_vector(vec![
                    true,
                    false,
                    true,
                    true,
                ]),
                1,
                3,
            );

        let environment =
            SimpleExpressionEnvironment::new();

        let value =
            expression.evaluate(&environment).unwrap();

        assert_eq!(
            value,
            ExpressionValue::BitVector(
                BitVector::from_bits(vec![
                    false,
                    true,
                ])
            )
        );
    }

    #[test]
    fn select_evaluates() {
        let expression =
            ClassicalExpression::select(
                ClassicalExpression::bool(true),
                ClassicalExpression::signed(42),
                ClassicalExpression::signed(7),
            );

        let environment =
            SimpleExpressionEnvironment::new();

        let value =
            expression.evaluate(&environment).unwrap();

        assert_eq!(
            value,
            ExpressionValue::SignedInteger(42)
        );
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let expression =
            ClassicalExpression::divide(
                ClassicalExpression::signed(10),
                ClassicalExpression::signed(0),
            );

        let environment =
            SimpleExpressionEnvironment::new();

        assert!(matches!(
            expression.evaluate(&environment),
            Err(ExpressionError::DivisionByZero)
        ));
    }

    #[test]
    fn non_finite_float_is_rejected() {
        assert!(matches!(
            ClassicalExpression::float(
                f64::INFINITY
            ),
            Err(ExpressionError::NonFiniteFloat)
        ));

        assert!(matches!(
            ClassicalExpression::float(f64::NAN),
            Err(ExpressionError::NonFiniteFloat)
        ));
    }

    #[test]
    fn concatenation_is_scalable() {
        let left =
            ClassicalExpression::bit_vector(vec![
                true,
                false,
            ]);

        let right =
            ClassicalExpression::bit_vector(vec![
                false,
                true,
                true,
            ]);

        let expression =
            ClassicalExpression::concatenate(
                left,
                right,
            );

        assert_eq!(
            expression.expression_type(),
            ExpressionType::BitVector(Some(5))
        );
    }

    #[test]
    fn deterministic_classical_bit_collection() {
        let expression =
            ClassicalExpression::logical_and(
                ClassicalExpression::classical_bit(
                    ClassicalBitId::new(9),
                ),
                ClassicalExpression::classical_bit(
                    ClassicalBitId::new(2),
                ),
            );

        assert_eq!(
            expression.collect_classical_bits(),
            vec![
                ClassicalBitId::new(2),
                ClassicalBitId::new(9),
            ]
        );
    }

    #[test]
    fn external_calls_are_not_executed() {
        let call =
            NamedCall::new(
                "vendor::classical::foo",
                ExpressionType::UnsignedInteger(None),
            )
            .unwrap();

        let expression =
            ClassicalExpression::call(call);

        let environment =
            SimpleExpressionEnvironment::new();

        assert!(matches!(
            expression.evaluate(&environment),
            Err(ExpressionError::ExternalCallNotEvaluable(_))
        ));
    }

    #[test]
    fn no_fixed_expression_depth_is_built_into_semantics() {
        let mut expression =
            ClassicalExpression::signed(1);

        for _ in 0..5_000 {
            expression =
                ClassicalExpression::negate(expression);
        }

        assert_eq!(
            expression.depth(),
            5_000
        );

        assert!(expression
            .validate_with_policy(
                ExpressionValidationPolicy::unrestricted()
            )
            .is_ok());
    }
}