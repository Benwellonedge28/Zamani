//! Zamani Quantum IR — Canonical Semantic Type System
//!
//! Path:
//!     src/quantum/ir/core/types.rs
//!
//! # Purpose
//!
//! This module defines the canonical, target-independent semantic type system
//! of the Zamani Quantum IR.
//!
//! It answers:
//!
//!     "What kind of entity/value does this IR node represent?"
//!
//! It does NOT answer:
//!
//!     "Which hardware implements it?"
//!
//! Hardware capabilities, topology, routing, calibration, scheduling,
//! execution, simulation and backend-specific representations belong outside
//! this module.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::identity
//!          │
//!          ├── TypeId
//!          │
//!          ▼
//! quantum::ir::core::types
//!          │
//!          ├───────────────┬────────────────┐
//!          ▼               ▼                ▼
//!      values          operations        programs
//!          │               │                │
//!          └───────────────┼────────────────┘
//!                          ▼
//!                   validation
//!                          │
//!                          ▼
//!                  serialization
//! ```
//!
//! The canonical qubit identities are imported from:
//!
//!     crate::quantum::ir::qubit
//!
//! Specifically:
//!
//!     QubitId
//!     PhysicalQubitId
//!
//! This file intentionally does not define another qubit identifier.
//!
//! # Design principles
//!
//! 1. Types describe semantics, never hardware.
//! 2. No fixed quantum-machine size is encoded here.
//! 3. `usize` is never used as a semantic quantum-resource count.
//! 4. Dynamic and arbitrarily large finite structures are representable.
//! 5. Symbolic dimensions are first-class.
//! 6. Recursive/container types are represented structurally.
//! 7. Named/extensible types are represented through `TypeId`.
//! 8. Hardware-specific types can exist through opaque/named extensions.
//! 9. Type construction is deterministic.
//! 10. Type equality is structural unless explicitly represented by a
//!     declared/named type.
//! 11. No runtime values are stored in this module.
//! 12. No expression evaluation occurs here.
//! 13. No backend is required.
//! 14. No external crate is required.
//! 15. No unsafe code is permitted.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//!     MAX_QUBITS
//!     MAX_CLASSICAL_BITS
//!     MAX_ARRAY_LENGTH
//!     MAX_TUPLE_ARITY
//!     MAX_TYPE_DEPTH
//!
//! in this module.
//!
//! Any operational/resource/security limit belongs to the compilation or
//! execution policy (`QuantumIrLimits` or an equivalent external policy).
//!
//! A type can therefore describe:
//!
//!     qubit[N]
//!     bit[N]
//!     array<T, N>
//!
//! without making N a hardware limit.
//!
//! # Important distinction
//!
//! ```text
//! QubitId
//!     logical qubit identity
//!
//! PhysicalQubitId
//!     physical qubit identity vocabulary
//!
//! QubitType
//!     semantic type of a quantum resource
//!
//! PhysicalQubitType
//!     semantic type used after/at a physical mapping boundary
//! ```
//!
//! The type system never establishes that a physical qubit actually exists.
//! Hardware validation is a downstream responsibility.
//!
//! # Rust contract
//!
//! Supported:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021
//!
//! Requirements:
//!
//!     no nightly features
//!     no external dependencies
//!     no unsafe code
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//!     quantum::ir::identity::TypeId
//!     quantum::ir::qubit::QubitId
//!     quantum::ir::qubit::PhysicalQubitId
//!
//! Downstream consumers may include:
//!
//!     core::value
//!     core::parameter
//!     program::operand
//!     program::result
//!     program::operation
//!     quantum::gate
//!     quantum::measurement
//!     classical::*
//!     control::*
//!     pulse::*
//!     models::*
//!     validation::*
//!     serialization::*
//!     analysis::*
//!
//! None of those modules are required to compile this file.
//!
//! This is intentional: the type system is a foundation layer.
//!
//! # Ownership
//!
//! This file owns:
//!
//!     semantic scalar types
//!     integer widths
//!     floating-point widths
//!     complex types
//!     bit types
//!     angle types
//!     duration/frequency/amplitude/phase semantic types
//!     quantum resource types
//!     arrays
//!     tuples
//!     structs
//!     option/result
//!     function types
//!     unit/never
//!     named/opaque types
//!     type qualifiers
//!     type classification
//!     structural type compatibility
//!
//! This file does NOT own:
//!
//!     runtime values
//!     expressions
//!     symbolic evaluation
//!     gate definitions
//!     measurement definitions
//!     pulse definitions
//!     hardware capabilities
//!     hardware topology
//!     routing
//!     scheduling
//!     calibration data
//!     simulation state
//!     source-language AST
//!
//! Those responsibilities remain elsewhere.
//!
//! # Stability rule
//!
//! Once another IR module consumes one of the public types in this file,
//! changes should be additive or versioned. Do not silently change the meaning
//! of an existing type variant.

#![forbid(unsafe_code)]

use std::fmt;

use super::super::identity::TypeId;
use super::super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Primitive integer types
// =============================================================================

/// Signed integer representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SignedIntegerType {
    /// Platform-sized signed integer.
    ///
    /// This is a language/compiler representation choice, not a quantum
    /// machine-size limit.
    Size,

    /// 8-bit signed integer.
    I8,

    /// 16-bit signed integer.
    I16,

    /// 32-bit signed integer.
    I32,

    /// 64-bit signed integer.
    I64,

    /// 128-bit signed integer.
    I128,

    /// Arbitrary semantic bit width.
    ///
    /// The compiler/backend may impose an explicit implementation limit.
    Arbitrary(u64),
}

impl SignedIntegerType {
    /// Returns the statically represented width where applicable.
    #[must_use]
    pub const fn width(self) -> Option<u64> {
        match self {
            Self::Size => None,
            Self::I8 => Some(8),
            Self::I16 => Some(16),
            Self::I32 => Some(32),
            Self::I64 => Some(64),
            Self::I128 => Some(128),
            Self::Arbitrary(width) => Some(width),
        }
    }

    /// Returns whether this is an explicitly arbitrary-width integer.
    #[must_use]
    pub const fn is_arbitrary(self) -> bool {
        matches!(self, Self::Arbitrary(_))
    }
}

impl Default for SignedIntegerType {
    fn default() -> Self {
        Self::I64
    }
}

impl fmt::Display for SignedIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Size => f.write_str("isize"),
            Self::I8 => f.write_str("i8"),
            Self::I16 => f.write_str("i16"),
            Self::I32 => f.write_str("i32"),
            Self::I64 => f.write_str("i64"),
            Self::I128 => f.write_str("i128"),
            Self::Arbitrary(width) => write!(f, "i{width}"),
        }
    }
}

/// Unsigned integer representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnsignedIntegerType {
    /// Platform-sized unsigned integer.
    Size,

    /// 8-bit unsigned integer.
    U8,

    /// 16-bit unsigned integer.
    U16,

    /// 32-bit unsigned integer.
    U32,

    /// 64-bit unsigned integer.
    U64,

    /// 128-bit unsigned integer.
    U128,

    /// Arbitrary semantic bit width.
    Arbitrary(u64),
}

impl UnsignedIntegerType {
    /// Returns the statically represented width where applicable.
    #[must_use]
    pub const fn width(self) -> Option<u64> {
        match self {
            Self::Size => None,
            Self::U8 => Some(8),
            Self::U16 => Some(16),
            Self::U32 => Some(32),
            Self::U64 => Some(64),
            Self::U128 => Some(128),
            Self::Arbitrary(width) => Some(width),
        }
    }

    /// Returns whether this is an explicitly arbitrary-width integer.
    #[must_use]
    pub const fn is_arbitrary(self) -> bool {
        matches!(self, Self::Arbitrary(_))
    }
}

impl Default for UnsignedIntegerType {
    fn default() -> Self {
        Self::U64
    }
}

impl fmt::Display for UnsignedIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Size => f.write_str("usize"),
            Self::U8 => f.write_str("u8"),
            Self::U16 => f.write_str("u16"),
            Self::U32 => f.write_str("u32"),
            Self::U64 => f.write_str("u64"),
            Self::U128 => f.write_str("u128"),
            Self::Arbitrary(width) => write!(f, "u{width}"),
        }
    }
}

// =============================================================================
// Floating-point types
// =============================================================================

/// Floating-point representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FloatType {
    /// IEEE-754 binary16.
    F16,

    /// IEEE-754 binary32.
    F32,

    /// IEEE-754 binary64.
    F64,

    /// IEEE-754 binary128 semantic representation.
    F128,

    /// Arbitrary semantic precision in bits.
    Arbitrary(u64),
}

impl FloatType {
    /// Returns the precision in bits where statically known.
    #[must_use]
    pub const fn precision_bits(self) -> Option<u64> {
        match self {
            Self::F16 => Some(16),
            Self::F32 => Some(32),
            Self::F64 => Some(64),
            Self::F128 => Some(128),
            Self::Arbitrary(bits) => Some(bits),
        }
    }

    /// Returns whether this is arbitrary precision.
    #[must_use]
    pub const fn is_arbitrary(self) -> bool {
        matches!(self, Self::Arbitrary(_))
    }
}

impl Default for FloatType {
    fn default() -> Self {
        Self::F64
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F16 => f.write_str("f16"),
            Self::F32 => f.write_str("f32"),
            Self::F64 => f.write_str("f64"),
            Self::F128 => f.write_str("f128"),
            Self::Arbitrary(bits) => write!(f, "f{bits}"),
        }
    }
}

// =============================================================================
// Complex types
// =============================================================================

/// Complex-number semantic representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComplexType {
    /// Complex number with the specified real/imaginary component type.
    Float(FloatType),

    /// Arbitrary component precision.
    Arbitrary(u64),
}

impl ComplexType {
    /// Returns component precision where known.
    #[must_use]
    pub const fn component_precision_bits(self) -> Option<u64> {
        match self {
            Self::Float(float) => float.precision_bits(),
            Self::Arbitrary(bits) => Some(bits),
        }
    }
}

impl Default for ComplexType {
    fn default() -> Self {
        Self::Float(FloatType::F64)
    }
}

impl fmt::Display for ComplexType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(float) => write!(f, "complex<{float}>"),
            Self::Arbitrary(bits) => write!(f, "complex<{bits}>"),
        }
    }
}

// =============================================================================
// Classical bit types
// =============================================================================

/// Classical bit representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BitType {
    /// One classical bit.
    Bit,

    /// Statically sized classical bit vector.
    Vector(u64),

    /// Semantically sized bit vector whose implementation representation is
    /// determined downstream.
    Arbitrary(u64),
}

impl BitType {
    /// Returns the semantic width.
    #[must_use]
    pub const fn width(self) -> u64 {
        match self {
            Self::Bit => 1,
            Self::Vector(width) | Self::Arbitrary(width) => width,
        }
    }

    /// Returns whether this is exactly one bit.
    #[must_use]
    pub const fn is_single(self) -> bool {
        matches!(self, Self::Bit)
    }

    /// Returns whether this uses an explicit vector width.
    #[must_use]
    pub const fn is_vector(self) -> bool {
        matches!(self, Self::Vector(_))
    }

    /// Returns whether the vector width is represented as arbitrary semantic
    /// storage.
    #[must_use]
    pub const fn is_arbitrary(self) -> bool {
        matches!(self, Self::Arbitrary(_))
    }
}

impl fmt::Display for BitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bit => f.write_str("bit"),
            Self::Vector(width) => write!(f, "bit[{width}]"),
            Self::Arbitrary(width) => write!(f, "bit[arbitrary:{width}]"),
        }
    }
}

// =============================================================================
// Semantic quantum scalar types
// =============================================================================

/// Semantic quantum-resource type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumType {
    /// Logical qubit resource.
    Qubit,

    /// Explicit physical-qubit resource.
    ///
    /// The identifier is semantic vocabulary only. Existence and hardware
    /// capabilities are checked by downstream hardware/resource layers.
    PhysicalQubit,
}

impl QuantumType {
    /// Returns whether this is a logical qubit type.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Qubit)
    }

    /// Returns whether this is a physical qubit type.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit)
    }
}

impl fmt::Display for QuantumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Qubit => f.write_str("qubit"),
            Self::PhysicalQubit => f.write_str("physical_qubit"),
        }
    }
}

/// Semantic angle representation.
///
/// Angles are kept distinct from generic floating-point numbers so that
/// parameterized quantum operations can preserve their mathematical meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AngleType {
    /// Exact mathematical angle.
    Exact,

    /// Angle represented using a floating-point precision.
    Float(FloatType),
}

impl Default for AngleType {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for AngleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("angle"),
            Self::Float(float) => write!(f, "angle<{float}>"),
        }
    }
}

/// Semantic duration type.
///
/// The numeric representation of an actual duration belongs to the value
/// system. This type only identifies the semantic quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DurationType {
    /// Exact semantic duration.
    Exact,

    /// Duration represented using a floating-point quantity.
    Float(FloatType),
}

impl Default for DurationType {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for DurationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("duration"),
            Self::Float(float) => write!(f, "duration<{float}>"),
        }
    }
}

/// Semantic frequency type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FrequencyType {
    /// Exact frequency.
    Exact,

    /// Floating-point frequency.
    Float(FloatType),
}

impl Default for FrequencyType {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for FrequencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("frequency"),
            Self::Float(float) => write!(f, "frequency<{float}>"),
        }
    }
}

/// Semantic amplitude type.
///
/// Amplitude is deliberately not defined as a vendor-specific DAC code or
/// normalized hardware voltage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AmplitudeType {
    /// Exact semantic amplitude.
    Exact,

    /// Floating-point amplitude.
    Float(FloatType),
}

impl Default for AmplitudeType {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for AmplitudeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("amplitude"),
            Self::Float(float) => write!(f, "amplitude<{float}>"),
        }
    }
}

/// Semantic phase type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PhaseType {
    /// Exact phase.
    Exact,

    /// Floating-point phase.
    Float(FloatType),
}

impl Default for PhaseType {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for PhaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("phase"),
            Self::Float(float) => write!(f, "phase<{float}>"),
        }
    }
}

// =============================================================================
// Dimension system
// =============================================================================

/// Symbolic or concrete dimension.
///
/// This is intentionally separate from runtime values and expressions.
///
/// A dimension may be:
///
///     concrete
///     symbolic
///     dynamic
///
/// A compiler can resolve a symbolic/dynamic dimension later without changing
/// the type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dimension {
    /// Statically known non-negative extent.
    Static(u64),

    /// Symbolic extent identified by a stable parameter/type symbol.
    Symbol(TypeId),

    /// Runtime-dependent extent.
    Dynamic,
}

impl Dimension {
    /// Creates a static dimension.
    #[must_use]
    pub const fn static_(size: u64) -> Self {
        Self::Static(size)
    }

    /// Creates a symbolic dimension.
    #[must_use]
    pub const fn symbol(id: TypeId) -> Self {
        Self::Symbol(id)
    }

    /// Creates a dynamic dimension.
    #[must_use]
    pub const fn dynamic() -> Self {
        Self::Dynamic
    }

    /// Returns the concrete extent if statically known.
    #[must_use]
    pub const fn static_size(&self) -> Option<u64> {
        match self {
            Self::Static(size) => Some(*size),
            Self::Symbol(_) | Self::Dynamic => None,
        }
    }

    /// Returns whether the dimension is statically known.
    #[must_use]
    pub const fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    /// Returns whether the dimension is symbolic.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    /// Returns whether the dimension is runtime dynamic.
    #[must_use]
    pub const fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic)
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(size) => write!(f, "{size}"),
            Self::Symbol(id) => write!(f, "{id}"),
            Self::Dynamic => f.write_str("dynamic"),
        }
    }
}

// =============================================================================
// Array type
// =============================================================================

/// Array type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArrayType {
    element: Box<IrType>,
    dimension: Dimension,
}

impl ArrayType {
    /// Creates an array type.
    #[must_use]
    pub fn new(element: IrType, dimension: Dimension) -> Self {
        Self {
            element: Box::new(element),
            dimension,
        }
    }

    /// Returns the element type.
    #[must_use]
    pub fn element(&self) -> &IrType {
        &self.element
    }

    /// Returns the dimension.
    #[must_use]
    pub const fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    /// Returns the static element count when available.
    #[must_use]
    pub const fn static_size(&self) -> Option<u64> {
        self.dimension.static_size()
    }

    /// Returns whether the array is statically sized.
    #[must_use]
    pub const fn is_static(&self) -> bool {
        self.dimension.is_static()
    }

    /// Returns whether the array is dynamically sized.
    #[must_use]
    pub const fn is_dynamic(&self) -> bool {
        self.dimension.is_dynamic()
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.element, self.dimension)
    }
}

// =============================================================================
// Tuple type
// =============================================================================

/// Heterogeneous tuple type.
///
/// Tuple arity is represented by the length of the vector and therefore has
/// no architectural fixed maximum.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TupleType {
    elements: Vec<IrType>,
}

impl TupleType {
    /// Creates a tuple type.
    #[must_use]
    pub fn new(elements: Vec<IrType>) -> Self {
        Self { elements }
    }

    /// Creates the unit tuple.
    #[must_use]
    pub const fn unit() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Returns all tuple element types.
    #[must_use]
    pub fn elements(&self) -> &[IrType] {
        &self.elements
    }

    /// Returns the number of tuple elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the tuple is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an element type.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&IrType> {
        self.elements.get(index)
    }
}

impl Default for TupleType {
    fn default() -> Self {
        Self::unit()
    }
}

impl fmt::Display for TupleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;

        for (index, element) in self.elements.iter().enumerate() {
            if index != 0 {
                f.write_str(", ")?;
            }

            write!(f, "{element}")?;
        }

        if self.elements.len() == 1 {
            f.write_str(",")?;
        }

        f.write_str(")")
    }
}

// =============================================================================
// Struct fields
// =============================================================================

/// A named field in a structural IR type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructField {
    name: String,
    ty: IrType,
}

impl StructField {
    /// Creates a field.
    ///
    /// Field names must be non-empty and must contain only characters
    /// permitted by the canonical IR identifier policy.
    pub fn new(
        name: impl Into<String>,
        ty: IrType,
    ) -> Result<Self, TypeError> {
        let name = name.into();

        validate_identifier(&name)?;

        Ok(Self { name, ty })
    }

    /// Returns the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field type.
    #[must_use]
    pub fn ty(&self) -> &IrType {
        &self.ty
    }
}

/// Struct type.
///
/// Field order is semantically significant.
///
/// Canonical serialization must therefore preserve the declared order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructType {
    fields: Vec<StructField>,
}

impl StructType {
    /// Creates a struct after validating field names and duplicate names.
    pub fn new(fields: Vec<StructField>) -> Result<Self, TypeError> {
        for (index, field) in fields.iter().enumerate() {
            for other in fields.iter().skip(index + 1) {
                if field.name == other.name {
                    return Err(TypeError::DuplicateFieldName(
                        field.name.clone(),
                    ));
                }
            }
        }

        Ok(Self { fields })
    }

    /// Returns the ordered field list.
    #[must_use]
    pub fn fields(&self) -> &[StructField] {
        &self.fields
    }

    /// Returns the number of fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the struct contains no fields.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Looks up a field by name.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&StructField> {
        self.fields.iter().find(|field| field.name == name)
    }
}

impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("struct {")?;

        for (index, field) in self.fields.iter().enumerate() {
            if index != 0 {
                f.write_str(", ")?;
            }

            write!(f, "{}: {}", field.name, field.ty)?;
        }

        f.write_str("}")
    }
}

// =============================================================================
// Option and Result types
// =============================================================================

/// Optional value type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OptionType {
    inner: Box<IrType>,
}

impl OptionType {
    /// Creates an option type.
    #[must_use]
    pub fn new(inner: IrType) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    /// Returns the wrapped type.
    #[must_use]
    pub fn inner(&self) -> &IrType {
        &self.inner
    }
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "option<{}>", self.inner)
    }
}

/// Result type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResultType {
    ok: Box<IrType>,
    error: Box<IrType>,
}

impl ResultType {
    /// Creates a result type.
    #[must_use]
    pub fn new(ok: IrType, error: IrType) -> Self {
        Self {
            ok: Box::new(ok),
            error: Box::new(error),
        }
    }

    /// Returns the successful result type.
    #[must_use]
    pub fn ok(&self) -> &IrType {
        &self.ok
    }

    /// Returns the error result type.
    #[must_use]
    pub fn error(&self) -> &IrType {
        &self.error
    }
}

impl fmt::Display for ResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "result<{}, {}>", self.ok, self.error)
    }
}

// =============================================================================
// Function type
// =============================================================================

/// Function calling convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FunctionCallingConvention {
    /// Canonical Zamani IR convention.
    Canonical,

    /// External function whose ABI is defined outside this IR.
    Extern,

    /// Target/backend-defined ABI.
    Custom,
}

impl Default for FunctionCallingConvention {
    fn default() -> Self {
        Self::Canonical
    }
}

/// Function type.
///
/// Function types describe semantic signatures only. They do not contain
/// executable code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionType {
    parameters: Vec<IrType>,
    results: Vec<IrType>,
    convention: FunctionCallingConvention,
}

impl FunctionType {
    /// Creates a canonical function type.
    #[must_use]
    pub fn new(parameters: Vec<IrType>, results: Vec<IrType>) -> Self {
        Self {
            parameters,
            results,
            convention: FunctionCallingConvention::Canonical,
        }
    }

    /// Creates a function type with an explicit calling convention.
    #[must_use]
    pub fn with_convention(
        parameters: Vec<IrType>,
        results: Vec<IrType>,
        convention: FunctionCallingConvention,
    ) -> Self {
        Self {
            parameters,
            results,
            convention,
        }
    }

    /// Returns parameter types.
    #[must_use]
    pub fn parameters(&self) -> &[IrType] {
        &self.parameters
    }

    /// Returns result types.
    #[must_use]
    pub fn results(&self) -> &[IrType] {
        &self.results
    }

    /// Returns the calling convention.
    #[must_use]
    pub const fn convention(&self) -> FunctionCallingConvention {
        self.convention
    }

    /// Returns parameter arity.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns result arity.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("fn(")?;

        for (index, parameter) in self.parameters.iter().enumerate() {
            if index != 0 {
                f.write_str(", ")?;
            }

            write!(f, "{parameter}")?;
        }

        f.write_str(") -> ")?;

        if self.results.len() == 1 {
            write!(f, "{}", self.results[0])
        } else {
            write!(f, "{}", TupleType::new(self.results.clone()))
        }
    }
}

// =============================================================================
// Opaque and named types
// =============================================================================

/// A globally declared named type.
///
/// The actual declaration is owned by the program/type-definition layer.
/// `NamedType` merely carries its stable identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamedType {
    id: TypeId,
}

impl NamedType {
    /// Creates a named type reference.
    #[must_use]
    pub const fn new(id: TypeId) -> Self {
        Self { id }
    }

    /// Returns the declaration identity.
    #[must_use]
    pub const fn id(self) -> TypeId {
        self.id
    }
}

impl fmt::Display for NamedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

/// Opaque type identified by a stable type declaration.
///
/// Opaque types are essential for future extensions because the canonical IR
/// does not need to understand every possible quantum technology immediately.
///
/// The type remains opaque until a dialect/extension explicitly defines it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OpaqueType {
    id: TypeId,
}

impl OpaqueType {
    /// Creates an opaque type reference.
    #[must_use]
    pub const fn new(id: TypeId) -> Self {
        Self { id }
    }

    /// Returns the opaque type identity.
    #[must_use]
    pub const fn id(self) -> TypeId {
        self.id
    }
}

impl fmt::Display for OpaqueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "opaque<{:?}>", self.id)
    }
}

// =============================================================================
// Type qualifiers
// =============================================================================

/// Semantic qualifier applied to an IR type.
///
/// Qualifiers describe usage/ownership semantics without changing the
/// underlying type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeQualifier {
    /// Ordinary value/resource.
    Plain,

    /// Value is read-only.
    ReadOnly,

    /// Value is explicitly mutable at the IR level.
    Mutable,

    /// Value is compile-time constant.
    Constant,

    /// Value may be bound at runtime.
    Runtime,

    /// Value participates in compile-time symbolic computation.
    Symbolic,
}

impl Default for TypeQualifier {
    fn default() -> Self {
        Self::Plain
    }
}

impl TypeQualifier {
    /// Returns whether the qualifier permits mutation.
    #[must_use]
    pub const fn permits_mutation(self) -> bool {
        matches!(self, Self::Plain | Self::Mutable)
    }

    /// Returns whether the value is intended to be constant.
    #[must_use]
    pub const fn is_constant(self) -> bool {
        matches!(self, Self::Constant)
    }

    /// Returns whether the value may remain symbolic.
    #[must_use]
    pub const fn is_symbolic(self) -> bool {
        matches!(self, Self::Symbolic)
    }
}

// =============================================================================
// Canonical IR type
// =============================================================================

/// Canonical semantic type used throughout Zamani Quantum IR.
///
/// This is the central type algebra.
///
/// Standard quantum gates are NOT represented as types here. Gate semantics
/// belong to the quantum operation/gate layer.
///
/// Likewise, hardware-specific device types do not become mandatory variants.
/// They may use `Named` or `Opaque`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrType {
    // -------------------------------------------------------------------------
    // Fundamental types
    // -------------------------------------------------------------------------

    /// No value.
    Unit,

    /// Computation that never returns normally.
    Never,

    /// Boolean value.
    Bool,

    /// Signed integer.
    SignedInteger(SignedIntegerType),

    /// Unsigned integer.
    UnsignedInteger(UnsignedIntegerType),

    /// Floating-point value.
    Float(FloatType),

    /// Complex number.
    Complex(ComplexType),

    /// Classical bit or bit vector.
    Bit(BitType),

    // -------------------------------------------------------------------------
    // Quantum types
    // -------------------------------------------------------------------------

    /// Logical quantum resource.
    Qubit,

    /// Explicit physical quantum resource.
    PhysicalQubit,

    /// A concrete logical qubit identity.
    ///
    /// This is useful at IR integration boundaries where the program has
    /// already introduced a specific logical identity.
    ///
    /// It does not imply hardware allocation.
    QubitRef(QubitId),

    /// A concrete physical qubit identity.
    ///
    /// It does not prove that the physical resource exists on a target.
    PhysicalQubitRef(PhysicalQubitId),

    /// Logical-qubit register.
    QubitArray(Dimension),

    /// Physical-qubit register.
    PhysicalQubitArray(Dimension),

    // -------------------------------------------------------------------------
    // Semantic numeric/physical quantities
    // -------------------------------------------------------------------------

    /// Mathematical angle.
    Angle(AngleType),

    /// Time duration.
    Duration(DurationType),

    /// Frequency.
    Frequency(FrequencyType),

    /// Pulse/control amplitude.
    Amplitude(AmplitudeType),

    /// Phase.
    Phase(PhaseType),

    // -------------------------------------------------------------------------
    // Containers
    // -------------------------------------------------------------------------

    /// Homogeneous array.
    Array(ArrayType),

    /// Heterogeneous tuple.
    Tuple(TupleType),

    /// Named structural record.
    Struct(StructType),

    /// Optional value.
    Option(OptionType),

    /// Fallible result.
    Result(ResultType),

    // -------------------------------------------------------------------------
    // Functions
    // -------------------------------------------------------------------------

    /// Function signature.
    Function(FunctionType),

    // -------------------------------------------------------------------------
    // Named/extensible types
    // -------------------------------------------------------------------------

    /// Reference to a declared type.
    Named(NamedType),

    /// Extension-owned opaque type.
    Opaque(OpaqueType),
}

impl IrType {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Returns the unit type.
    #[must_use]
    pub const fn unit() -> Self {
        Self::Unit
    }

    /// Returns the never type.
    #[must_use]
    pub const fn never() -> Self {
        Self::Never
    }

    /// Returns the boolean type.
    #[must_use]
    pub const fn bool() -> Self {
        Self::Bool
    }

    /// Returns the logical-qubit type.
    #[must_use]
    pub const fn qubit() -> Self {
        Self::Qubit
    }

    /// Returns the physical-qubit type.
    #[must_use]
    pub const fn physical_qubit() -> Self {
        Self::PhysicalQubit
    }

    /// Creates a concrete logical-qubit reference type.
    #[must_use]
    pub const fn qubit_ref(id: QubitId) -> Self {
        Self::QubitRef(id)
    }

    /// Creates a concrete physical-qubit reference type.
    #[must_use]
    pub const fn physical_qubit_ref(id: PhysicalQubitId) -> Self {
        Self::PhysicalQubitRef(id)
    }

    /// Creates a logical-qubit array type.
    #[must_use]
    pub const fn qubit_array(dimension: Dimension) -> Self {
        Self::QubitArray(dimension)
    }

    /// Creates a physical-qubit array type.
    #[must_use]
    pub const fn physical_qubit_array(dimension: Dimension) -> Self {
        Self::PhysicalQubitArray(dimension)
    }

    /// Creates an array type.
    #[must_use]
    pub fn array(element: Self, dimension: Dimension) -> Self {
        Self::Array(ArrayType::new(element, dimension))
    }

    /// Creates a tuple type.
    #[must_use]
    pub fn tuple(elements: Vec<Self>) -> Self {
        Self::Tuple(TupleType::new(elements))
    }

    /// Creates an option type.
    #[must_use]
    pub fn option(inner: Self) -> Self {
        Self::Option(OptionType::new(inner))
    }

    /// Creates a result type.
    #[must_use]
    pub fn result(ok: Self, error: Self) -> Self {
        Self::Result(ResultType::new(ok, error))
    }

    /// Creates a canonical function type.
    #[must_use]
    pub fn function(parameters: Vec<Self>, results: Vec<Self>) -> Self {
        Self::Function(FunctionType::new(parameters, results))
    }

    /// Creates a named type reference.
    #[must_use]
    pub const fn named(id: TypeId) -> Self {
        Self::Named(NamedType::new(id))
    }

    /// Creates an opaque type reference.
    #[must_use]
    pub const fn opaque(id: TypeId) -> Self {
        Self::Opaque(OpaqueType::new(id))
    }

    // =========================================================================
    // Classification
    // =========================================================================

    /// Returns the broad semantic category of this type.
    #[must_use]
    pub const fn category(&self) -> TypeCategory {
        match self {
            Self::Unit
            | Self::Never
            | Self::Bool
            | Self::SignedInteger(_)
            | Self::UnsignedInteger(_)
            | Self::Float(_)
            | Self::Complex(_)
            | Self::Bit(_)
            | Self::Angle(_)
            | Self::Duration(_)
            | Self::Frequency(_)
            | Self::Amplitude(_)
            | Self::Phase(_) => TypeCategory::Scalar,

            Self::Qubit
            | Self::PhysicalQubit
            | Self::QubitRef(_)
            | Self::PhysicalQubitRef(_)
            | Self::QubitArray(_)
            | Self::PhysicalQubitArray(_) => TypeCategory::Quantum,

            Self::Array(_)
            | Self::Tuple(_)
            | Self::Struct(_)
            | Self::Option(_)
            | Self::Result(_) => TypeCategory::Container,

            Self::Function(_) => TypeCategory::Function,

            Self::Named(_) | Self::Opaque(_) => TypeCategory::Extensible,
        }
    }

    /// Returns whether this is a scalar type.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(self.category(), TypeCategory::Scalar)
    }

    /// Returns whether this is a quantum type.
    #[must_use]
    pub const fn is_quantum(&self) -> bool {
        matches!(self.category(), TypeCategory::Quantum)
    }

    /// Returns whether this is a classical type.
    #[must_use]
    pub const fn is_classical(&self) -> bool {
        match self {
            Self::Qubit
            | Self::PhysicalQubit
            | Self::QubitRef(_)
            | Self::PhysicalQubitRef(_)
            | Self::QubitArray(_)
            | Self::PhysicalQubitArray(_) => false,

            _ => true,
        }
    }

    /// Returns whether this is a container.
    #[must_use]
    pub const fn is_container(&self) -> bool {
        matches!(self.category(), TypeCategory::Container)
    }

    /// Returns whether this is a function.
    #[must_use]
    pub const fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    /// Returns whether this is named.
    #[must_use]
    pub const fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    /// Returns whether this is opaque.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque(_))
    }

    /// Returns whether this represents a qubit resource.
    #[must_use]
    pub const fn is_qubit(&self) -> bool {
        matches!(
            self,
            Self::Qubit
                | Self::PhysicalQubit
                | Self::QubitRef(_)
                | Self::PhysicalQubitRef(_)
                | Self::QubitArray(_)
                | Self::PhysicalQubitArray(_)
        )
    }

    /// Returns whether this is a logical-qubit type.
    #[must_use]
    pub const fn is_logical_qubit(&self) -> bool {
        matches!(
            self,
            Self::Qubit | Self::QubitRef(_) | Self::QubitArray(_)
        )
    }

    /// Returns whether this is a physical-qubit type.
    #[must_use]
    pub const fn is_physical_qubit(&self) -> bool {
        matches!(
            self,
            Self::PhysicalQubit
                | Self::PhysicalQubitRef(_)
                | Self::PhysicalQubitArray(_)
        )
    }

    /// Returns whether the type is a scalar numeric type.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::SignedInteger(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Complex(_)
                | Self::Angle(_)
                | Self::Duration(_)
                | Self::Frequency(_)
                | Self::Amplitude(_)
                | Self::Phase(_)
        )
    }

    /// Returns whether the type is a classical control predicate candidate.
    #[must_use]
    pub const fn is_predicate_compatible(&self) -> bool {
        matches!(self, Self::Bool | Self::Bit(_))
    }

    // =========================================================================
    // Structural accessors
    // =========================================================================

    /// Returns the array element type if this is an array.
    #[must_use]
    pub fn array_element(&self) -> Option<&Self> {
        match self {
            Self::Array(array) => Some(array.element()),
            _ => None,
        }
    }

    /// Returns the array dimension if this is an array.
    #[must_use]
    pub fn array_dimension(&self) -> Option<&Dimension> {
        match self {
            Self::Array(array) => Some(array.dimension()),
            Self::QubitArray(dimension)
            | Self::PhysicalQubitArray(dimension) => Some(dimension),
            _ => None,
        }
    }

    /// Returns tuple elements if this is a tuple.
    #[must_use]
    pub fn tuple_elements(&self) -> Option<&[Self]> {
        match self {
            Self::Tuple(tuple) => Some(tuple.elements()),
            _ => None,
        }
    }

    /// Returns the wrapped option type.
    #[must_use]
    pub fn option_inner(&self) -> Option<&Self> {
        match self {
            Self::Option(option) => Some(option.inner()),
            _ => None,
        }
    }

    /// Returns function signature information.
    #[must_use]
    pub fn function_signature(&self) -> Option<&FunctionType> {
        match self {
            Self::Function(function) => Some(function),
            _ => None,
        }
    }

    /// Returns the declared `TypeId` for named/opaque types.
    #[must_use]
    pub const fn type_id(&self) -> Option<TypeId> {
        match self {
            Self::Named(named) => Some(named.id()),
            Self::Opaque(opaque) => Some(opaque.id()),
            _ => None,
        }
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Performs structural validation of this type.
    ///
    /// This validates properties owned by the type system only.
    ///
    /// It does NOT validate:
    ///
    ///     hardware existence
    ///     hardware capacity
    ///     routing
    ///     topology
    ///     calibration
    ///     backend support
    pub fn validate(&self) -> Result<(), TypeError> {
        match self {
            Self::Array(array) => {
                array.element().validate()?;
                validate_dimension(array.dimension())
            }

            Self::Tuple(tuple) => {
                for element in tuple.elements() {
                    element.validate()?;
                }

                Ok(())
            }

            Self::Struct(struct_type) => {
                for field in struct_type.fields() {
                    validate_identifier(field.name())?;
                    field.ty().validate()?;
                }

                Ok(())
            }

            Self::Option(option) => option.inner().validate(),

            Self::Result(result) => {
                result.ok().validate()?;
                result.error().validate()
            }

            Self::Function(function) => {
                for parameter in function.parameters() {
                    parameter.validate()?;
                }

                for result in function.results() {
                    result.validate()?;
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }

    /// Returns whether the type is structurally valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    // =========================================================================
    // Compatibility
    // =========================================================================

    /// Returns whether two types are structurally identical.
    ///
    /// This intentionally does not perform implicit numeric conversions.
    #[must_use]
    pub fn is_same_type(&self, other: &Self) -> bool {
        self == other
    }

    /// Returns whether `self` can be used where `target` is expected without
    /// an explicit semantic conversion.
    ///
    /// This operation is deliberately conservative.
    #[must_use]
    pub fn is_assignable_to(&self, target: &Self) -> bool {
        if self == target {
            return true;
        }

        match (self, target) {
            (
                Self::QubitRef(_),
                Self::Qubit,
            )
            | (
                Self::PhysicalQubitRef(_),
                Self::PhysicalQubit,
            ) => true,

            (
                Self::QubitArray(_),
                Self::Qubit,
            )
            | (
                Self::PhysicalQubitArray(_),
                Self::PhysicalQubit,
            ) => false,

            _ => false,
        }
    }

    /// Returns whether an explicit conversion can preserve semantic meaning.
    ///
    /// This is intentionally stricter than generic numeric coercion.
    #[must_use]
    pub fn can_explicitly_convert_to(&self, target: &Self) -> bool {
        if self == target {
            return true;
        }

        match (self, target) {
            (
                Self::SignedInteger(_),
                Self::SignedInteger(_),
            )
            | (
                Self::UnsignedInteger(_),
                Self::UnsignedInteger(_),
            )
            | (
                Self::Float(_),
                Self::Float(_),
            )
            | (
                Self::Complex(_),
                Self::Complex(_),
            ) => true,

            (
                Self::SignedInteger(_),
                Self::Float(_),
            )
            | (
                Self::UnsignedInteger(_),
                Self::Float(_),
            )
            | (
                Self::Float(_),
                Self::Complex(_),
            ) => true,

            _ => false,
        }
    }

    /// Returns a deterministic semantic fingerprint string.
    ///
    /// This is intentionally not a cryptographic hash.
    ///
    /// The hashing subsystem should hash this canonical representation when a
    /// cryptographic identity is required.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => f.write_str("unit"),
            Self::Never => f.write_str("never"),
            Self::Bool => f.write_str("bool"),
            Self::SignedInteger(integer) => write!(f, "{integer}"),
            Self::UnsignedInteger(integer) => write!(f, "{integer}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::Complex(complex) => write!(f, "{complex}"),
            Self::Bit(bit) => write!(f, "{bit}"),
            Self::Qubit => f.write_str("qubit"),
            Self::PhysicalQubit => f.write_str("physical_qubit"),
            Self::QubitRef(id) => write!(f, "qubit<{id}>"),
            Self::PhysicalQubitRef(id) => {
                write!(f, "physical_qubit<{id}>")
            }
            Self::QubitArray(dimension) => {
                write!(f, "qubit[{dimension}]")
            }
            Self::PhysicalQubitArray(dimension) => {
                write!(f, "physical_qubit[{dimension}]")
            }
            Self::Angle(angle) => write!(f, "{angle}"),
            Self::Duration(duration) => write!(f, "{duration}"),
            Self::Frequency(frequency) => write!(f, "{frequency}"),
            Self::Amplitude(amplitude) => write!(f, "{amplitude}"),
            Self::Phase(phase) => write!(f, "{phase}"),
            Self::Array(array) => write!(f, "{array}"),
            Self::Tuple(tuple) => write!(f, "{tuple}"),
            Self::Struct(struct_type) => write!(f, "{struct_type}"),
            Self::Option(option) => write!(f, "{option}"),
            Self::Result(result) => write!(f, "{result}"),
            Self::Function(function) => write!(f, "{function}"),
            Self::Named(named) => write!(f, "{named}"),
            Self::Opaque(opaque) => write!(f, "{opaque}"),
        }
    }
}

// =============================================================================
// Type categories
// =============================================================================

/// Broad semantic type category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeCategory {
    /// Scalar or primitive semantic value.
    Scalar,

    /// Quantum resource.
    Quantum,

    /// Aggregate/container value.
    Container,

    /// Function/signature type.
    Function,

    /// Named or extension-defined type.
    Extensible,
}

impl fmt::Display for TypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar => f.write_str("scalar"),
            Self::Quantum => f.write_str("quantum"),
            Self::Container => f.write_str("container"),
            Self::Function => f.write_str("function"),
            Self::Extensible => f.write_str("extensible"),
        }
    }
}

// =============================================================================
// Type errors
// =============================================================================

/// Errors owned by the canonical type system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    /// Array dimension is invalid.
    InvalidArrayDimension,

    /// A type structure is invalid.
    InvalidStructure,

    /// A required identifier/name is empty.
    EmptyName,

    /// An identifier contains a prohibited character.
    InvalidName,

    /// A struct contains duplicate field names.
    DuplicateFieldName(String),

    /// A conversion is not semantically permitted.
    InvalidConversion {
        /// Source type.
        from: Box<IrType>,

        /// Destination type.
        to: Box<IrType>,
    },

    /// The types are incompatible.
    IncompatibleTypes {
        /// Left type.
        left: Box<IrType>,

        /// Right type.
        right: Box<IrType>,
    },

    /// A declared type reference is structurally invalid.
    InvalidTypeReference(TypeId),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArrayDimension => {
                f.write_str("invalid array dimension")
            }

            Self::InvalidStructure => {
                f.write_str("invalid type structure")
            }

            Self::EmptyName => {
                f.write_str("type name must not be empty")
            }

            Self::InvalidName => {
                f.write_str("type name contains invalid characters")
            }

            Self::DuplicateFieldName(name) => {
                write!(f, "duplicate struct field name: {name}")
            }

            Self::InvalidConversion { from, to } => {
                write!(f, "invalid conversion from {from} to {to}")
            }

            Self::IncompatibleTypes { left, right } => {
                write!(f, "incompatible types: {left} and {right}")
            }

            Self::InvalidTypeReference(id) => {
                write!(f, "invalid type reference: {id}")
            }
        }
    }
}

impl std::error::Error for TypeError {}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a semantic dimension.
fn validate_dimension(dimension: &Dimension) -> Result<(), TypeError> {
    match dimension {
        Dimension::Static(_) => Ok(()),
        Dimension::Symbol(_) => Ok(()),
        Dimension::Dynamic => Ok(()),
    }
}

/// Validates a canonical IR identifier/name.
///
/// This deliberately uses a small language-independent identifier policy:
///
///     first character: ASCII letter or `_`
///     subsequent: ASCII letter, digit, `_`
///
/// Unicode/source-language names can be preserved in frontend metadata or
/// symbol layers without changing this core structural identifier contract.
fn validate_identifier(name: &str) -> Result<(), TypeError> {
    let mut characters = name.chars();

    let first = match characters.next() {
        Some(character) => character,
        None => return Err(TypeError::EmptyName),
    };

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(TypeError::InvalidName);
    }

    if characters.any(|character| {
        !(character == '_' || character.is_ascii_alphanumeric())
    }) {
        return Err(TypeError::InvalidName);
    }

    Ok(())
}

// =============================================================================
// Canonical aliases
// =============================================================================

/// Canonical boolean type alias.
pub type BoolType = IrType;

/// Canonical logical-qubit type alias.
pub type QubitIrType = IrType;

/// Canonical classical-bit type alias.
pub type ClassicalBitType = BitType;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_types_are_structurally_distinct() {
        assert_ne!(IrType::Bool, IrType::Bit(BitType::Bit));
        assert_ne!(
            IrType::SignedInteger(SignedIntegerType::I64),
            IrType::UnsignedInteger(UnsignedIntegerType::U64)
        );
        assert_ne!(IrType::Qubit, IrType::PhysicalQubit);
    }

    #[test]
    fn logical_and_physical_qubits_are_distinct() {
        let logical = IrType::Qubit;
        let physical = IrType::PhysicalQubit;

        assert!(logical.is_logical_qubit());
        assert!(physical.is_physical_qubit());
        assert_ne!(logical, physical);
    }

    #[test]
    fn concrete_qubit_references_use_canonical_qubit_module_types() {
        let logical_id = QubitId::new(7);
        let physical_id = PhysicalQubitId::new(11);

        let logical = IrType::qubit_ref(logical_id);
        let physical = IrType::physical_qubit_ref(physical_id);

        assert!(logical.is_logical_qubit());
        assert!(physical.is_physical_qubit());
    }

    #[test]
    fn dimensions_are_not_hardware_limits() {
        let dimension = Dimension::Static(u64::MAX);

        assert_eq!(dimension.static_size(), Some(u64::MAX));

        let ty = IrType::qubit_array(dimension);

        assert!(ty.is_valid());
        assert_eq!(ty.array_dimension().and_then(Dimension::static_size), Some(u64::MAX));
    }

    #[test]
    fn symbolic_dimension_is_supported() {
        let symbol = TypeId::new(42);
        let dimension = Dimension::symbol(symbol);

        assert!(dimension.is_symbolic());
        assert_eq!(dimension.static_size(), None);
    }

    #[test]
    fn dynamic_dimension_is_supported() {
        let dimension = Dimension::dynamic();

        let ty = IrType::array(IrType::Bool, dimension);

        assert!(ty.is_valid());
        assert!(ty.array_dimension().is_some());
    }

    #[test]
    fn tuple_supports_arbitrary_finite_arity() {
        let elements = vec![IrType::Bool; 1024];
        let tuple = TupleType::new(elements);

        assert_eq!(tuple.len(), 1024);
        assert!(!tuple.is_empty());
    }

    #[test]
    fn struct_rejects_duplicate_fields() {
        let first = StructField::new("a", IrType::Bool)
            .expect("valid field");

        let second = StructField::new("a", IrType::I64())
            .expect("valid field");

        let result = StructType::new(vec![first, second]);

        assert!(matches!(
            result,
            Err(TypeError::DuplicateFieldName(_))
        ));
    }

    #[test]
    fn struct_rejects_invalid_names() {
        assert!(StructField::new("1invalid", IrType::Bool).is_err());
        assert!(StructField::new("", IrType::Bool).is_err());
    }

    #[test]
    fn nested_types_validate() {
        let inner = IrType::array(
            IrType::tuple(vec![
                IrType::Bool,
                IrType::SignedInteger(SignedIntegerType::I64),
                IrType::Qubit,
            ]),
            Dimension::Dynamic,
        );

        let outer = IrType::option(inner);

        assert!(outer.validate().is_ok());
    }

    #[test]
    fn function_types_are_structural() {
        let left = IrType::function(
            vec![IrType::Qubit, IrType::Angle(AngleType::Exact)],
            vec![IrType::Bit(BitType::Bit)],
        );

        let right = IrType::function(
            vec![IrType::Qubit, IrType::Angle(AngleType::Exact)],
            vec![IrType::Bit(BitType::Bit)],
        );

        assert_eq!(left, right);
        assert!(left.is_function());
    }

    #[test]
    fn exact_identity_is_assignable_to_general_resource_type() {
        let concrete = IrType::qubit_ref(QubitId::new(3));
        let general = IrType::Qubit;

        assert!(concrete.is_assignable_to(&general));
        assert!(!general.is_assignable_to(&concrete));
    }

    #[test]
    fn physical_identity_is_assignable_to_physical_resource_type() {
        let concrete =
            IrType::physical_qubit_ref(PhysicalQubitId::new(5));
        let general = IrType::PhysicalQubit;

        assert!(concrete.is_assignable_to(&general));
        assert!(!general.is_assignable_to(&concrete));
    }

    #[test]
    fn numeric_explicit_conversion_is_conservative() {
        let integer = IrType::SignedInteger(SignedIntegerType::I64);
        let float = IrType::Float(FloatType::F64);

        assert!(integer.can_explicitly_convert_to(&float));
        assert!(!IrType::Bool.can_explicitly_convert_to(&float));
    }

    #[test]
    fn canonical_names_are_deterministic() {
        let ty = IrType::array(
            IrType::Qubit,
            Dimension::Static(128),
        );

        assert_eq!(ty.canonical_name(), "[qubit; 128]");
        assert_eq!(ty.canonical_name(), ty.to_string());
    }

    #[test]
    fn categories_are_correct() {
        assert_eq!(IrType::Bool.category(), TypeCategory::Scalar);
        assert_eq!(IrType::Qubit.category(), TypeCategory::Quantum);
        assert_eq!(
            IrType::array(IrType::Bool, Dimension::Dynamic).category(),
            TypeCategory::Container
        );
        assert_eq!(
            IrType::function(vec![], vec![]).category(),
            TypeCategory::Function
        );
        assert_eq!(
            IrType::named(TypeId::new(1)).category(),
            TypeCategory::Extensible
        );
    }

    // Small convenience extension used only by this test module.
    trait TestTypeExt {
        fn I64() -> IrType;
    }

    impl TestTypeExt for IrType {
        #[allow(non_snake_case)]
        fn I64() -> IrType {
            IrType::SignedInteger(SignedIntegerType::I64)
        }
    }
}