//! Zamani Quantum IR — Canonical Type System
//!
//! This module defines the canonical semantic type system used by the
//! hardware-independent Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `types.rs` answers:
//!
//! > "What type of value is this IR entity?"
//!
//! It owns:
//!
//! - scalar types;
//! - integer types;
//! - floating-point types;
//! - complex types;
//! - boolean types;
//! - quantum-bit types;
//! - classical-bit types;
//! - logical-qubit and physical-qubit types;
//! - angle, duration, frequency, amplitude and phase types;
//! - arrays;
//! - tuples;
//! - structs;
//! - option types;
//! - result types;
//! - function types;
//! - unit type;
//! - never type;
//! - named/opaque/extensible types;
//! - type qualifiers;
//! - type classification;
//! - structural type validation;
//! - deterministic type identity/fingerprinting helpers.
//!
//! It does NOT own:
//!
//! - runtime values;
//! - expression evaluation;
//! - parameter expressions;
//! - gate semantics;
//! - measurement semantics;
//! - control-flow semantics;
//! - pulse generation;
//! - hardware capabilities;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - backend execution;
//! - simulation;
//! - frontend syntax.
//!
//! Those responsibilities belong to their respective IR or downstream
//! modules.
//!
//! # Dependency boundary
//!
//! ```text
//! quantum::ir::identity ───────┐
//!                             │
//! quantum::ir::qubit ─────────┤
//!                             ▼
//!                        types.rs
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!           value.rs      operation.rs    region.rs
//!              │              │              │
//!              └──────────────┼──────────────┘
//!                             ▼
//!                         program.rs
//! ```
//!
//! `types.rs` intentionally remains below values, operations, regions and
//! programs.
//!
//! # Universal quantum-program principle
//!
//! Zamani quantum source is hardware-independent.
//!
//! A type such as:
//!
//! ```text
//! qubit
//! ```
//!
//! means a logical quantum resource in the semantic IR. It does not imply a
//! particular physical technology.
//!
//! Likewise:
//!
//! ```text
//! duration
//! frequency
//! amplitude
//! phase
//! ```
//!
//! describe semantic quantities. Their hardware realization is determined
//! downstream by target and hardware layers.
//!
//! # Scalability
//!
//! There is no architectural fixed limit on:
//!
//! - number of qubits;
//! - number of classical bits;
//! - array length;
//! - tuple arity;
//! - struct field count;
//! - nesting depth;
//! - number of type declarations;
//! - number of program regions;
//! - number of machines.
//!
//! Concrete resource/security limits are external policy.
//!
//! This module therefore does not introduce:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as quantum-machine limits.
//!
//! Large homogeneous arrays can be represented symbolically through
//! `ArrayType::Fixed` without materializing all elements.
//!
//! # Quantum identity boundary
//!
//! Canonical quantum identities remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No duplicate qubit identifier types are defined here.
//!
//! # Type identity boundary
//!
//! `TypeId` remains owned by:
//!
//! ```text
//! quantum::ir::identity::TypeId
//! ```
//!
//! A `TypeId` identifies a declared/extensible type. Structural anonymous
//! types are represented directly by `IrType`.
//!
//! # Type system design
//!
//! The canonical type system distinguishes:
//!
//! ```text
//! semantic type
//!      │
//!      ├── scalar
//!      ├── quantum
//!      ├── classical
//!      ├── container
//!      ├── function
//!      └── extensible
//! ```
//!
//! A type does not contain a runtime value.
//!
//! # Hardware independence
//!
//! These types are deliberately semantic:
//!
//! ```text
//! Qubit
//! PhysicalQubit
//! Duration
//! Frequency
//! Amplitude
//! Phase
//! Angle
//! ```
//!
//! For example, `PhysicalQubit` means that a later compilation stage is
//! intentionally dealing with a physical-qubit identity. It does NOT mean
//! that this module knows the device topology, calibration, frequency,
//! control channel, or physical technology.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `identity.rs` supplies [`TypeId`].
//!
//! `qubit.rs` supplies [`QubitId`] and [`PhysicalQubitId`].
//!
//! `value.rs` stores runtime/IR values whose semantic type can be represented
//! by [`IrType`].
//!
//! `parameter.rs` may use [`IrType`] to describe parameter domains without
//! importing higher-level program structures.
//!
//! `operation.rs` uses [`IrType`] for operands and results.
//!
//! `gate.rs` can use quantum and numeric types.
//!
//! `measurement.rs` can use bit and measurement-result types.
//!
//! `pulse.rs` can use [`IrType::Amplitude`], [`IrType::Duration`],
//! [`IrType::Frequency`] and [`IrType::Phase`].
//!
//! `control_flow.rs` can use [`IrType::Bool`] and classical types.
//!
//! `region.rs` can use function, tuple, array, option and result types.
//!
//! `program.rs` can use named type declarations.
//!
//! `serialization.rs` can serialize the complete structural representation.
//!
//! `validation.rs` can validate type structure and compatibility.
//!
//! `analysis.rs` can classify types without evaluating values.
//!
//! No hardware module is required by this file.
//!
//! # Important compatibility rule
//!
//! This file is intentionally independent from `value.rs`.
//!
//! In particular, it does NOT import `Value` and does not depend on the
//! concrete representation of runtime values. This prevents a type/value
//! dependency cycle and allows the type contract to be frozen independently.
//!
//! ```text
//! types.rs ───────► value.rs
//! types.rs ───────► operation.rs
//! types.rs ───────► region.rs
//!
//! value.rs ───────X──► types.rs
//! ```
//!
//! Higher-level modules may consume both independently.

#![forbid(unsafe_code)]

use std::fmt;

use super::identity::TypeId;
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Type errors
// =============================================================================

/// Errors produced by checked type construction and structural validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    /// An array length expression is structurally invalid.
    InvalidArrayLength,

    /// A type contains an invalid recursive structure.
    InvalidStructure,

    /// A required name is empty.
    EmptyName,

    /// A name contains an invalid character.
    InvalidName,

    /// Two declarations use the same field name.
    DuplicateFieldName(String),

    /// A field has an invalid type.
    InvalidFieldType,

    /// A type operation encountered incompatible types.
    IncompatibleTypes {
        /// Left-hand type.
        left: Box<IrType>,

        /// Right-hand type.
        right: Box<IrType>,
    },

    /// A conversion cannot be performed without losing semantic information.
    InvalidConversion {
        /// Source type.
        from: Box<IrType>,

        /// Destination type.
        to: Box<IrType>,
    },

    /// A function contains an invalid signature.
    InvalidFunctionSignature,

    /// A named type reference is invalid.
    InvalidTypeReference(TypeId),
}

impl fmt::Display for TypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArrayLength => {
                formatter.write_str("array length is invalid")
            }

            Self::InvalidStructure => {
                formatter.write_str("type structure is invalid")
            }

            Self::EmptyName => {
                formatter.write_str("type name must not be empty")
            }

            Self::InvalidName => {
                formatter.write_str("type name contains invalid characters")
            }

            Self::DuplicateFieldName(name) => {
                write!(formatter, "duplicate struct field name: {name}")
            }

            Self::InvalidFieldType => {
                formatter.write_str("struct field type is invalid")
            }

            Self::IncompatibleTypes { left, right } => {
                write!(
                    formatter,
                    "incompatible types: {left} and {right}"
                )
            }

            Self::InvalidConversion { from, to } => {
                write!(
                    formatter,
                    "invalid type conversion from {from} to {to}"
                )
            }

            Self::InvalidFunctionSignature => {
                formatter.write_str("function type signature is invalid")
            }

            Self::InvalidTypeReference(id) => {
                write!(formatter, "invalid type reference: {id}")
            }
        }
    }
}

impl std::error::Error for TypeError {}

// =============================================================================
// Integer types
// =============================================================================

/// Signed integer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SignedIntegerType {
    /// Implementation-sized signed integer.
    ///
    /// This follows the host/compiler representation and is not a quantum
    /// machine-size limit.
    Size,

    /// Explicit 8-bit signed integer.
    I8,

    /// Explicit 16-bit signed integer.
    I16,

    /// Explicit 32-bit signed integer.
    I32,

    /// Explicit 64-bit signed integer.
    I64,

    /// Explicit 128-bit signed integer.
    I128,

    /// Arbitrary-width signed integer.
    ///
    /// The width is semantic and may be constrained by an external resource
    /// policy.
    Arbitrary(u64),
}

impl fmt::Display for SignedIntegerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Size => formatter.write_str("isize"),
            Self::I8 => formatter.write_str("i8"),
            Self::I16 => formatter.write_str("i16"),
            Self::I32 => formatter.write_str("i32"),
            Self::I64 => formatter.write_str("i64"),
            Self::I128 => formatter.write_str("i128"),
            Self::Arbitrary(width) => write!(formatter, "i{width}"),
        }
    }
}

/// Unsigned integer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnsignedIntegerType {
    /// Implementation-sized unsigned integer.
    Size,

    /// Explicit 8-bit unsigned integer.
    U8,

    /// Explicit 16-bit unsigned integer.
    U16,

    /// Explicit 32-bit unsigned integer.
    U32,

    /// Explicit 64-bit unsigned integer.
    U64,

    /// Explicit 128-bit unsigned integer.
    U128,

    /// Arbitrary-width unsigned integer.
    ///
    /// The width is semantic and may be constrained by an external resource
    /// policy.
    Arbitrary(u64),
}

impl fmt::Display for UnsignedIntegerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Size => formatter.write_str("usize"),
            Self::U8 => formatter.write_str("u8"),
            Self::U16 => formatter.write_str("u16"),
            Self::U32 => formatter.write_str("u32"),
            Self::U64 => formatter.write_str("u64"),
            Self::U128 => formatter.write_str("u128"),
            Self::Arbitrary(width) => write!(formatter, "u{width}"),
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

    /// IEEE-754 binary128 semantic type.
    F128,

    /// Arbitrary semantic floating-point precision.
    ///
    /// Actual implementation support is determined by the compiler/backend.
    Arbitrary(u64),
}

impl fmt::Display for FloatType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F16 => formatter.write_str("f16"),
            Self::F32 => formatter.write_str("f32"),
            Self::F64 => formatter.write_str("f64"),
            Self::F128 => formatter.write_str("f128"),
            Self::Arbitrary(bits) => write!(formatter, "f{bits}"),
        }
    }
}

// =============================================================================
// Complex types
// =============================================================================

/// Complex-number representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComplexType {
    /// Complex number whose components use the specified float type.
    Float(FloatType),

    /// Arbitrary component precision.
    Arbitrary(u64),
}

impl fmt::Display for ComplexType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(float) => write!(formatter, "complex<{float}>"),
            Self::Arbitrary(bits) => {
                write!(formatter, "complex<{bits}>")
            }
        }
    }
}

// =============================================================================
// Bit types
// =============================================================================

/// Classical bit width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BitType {
    /// Single classical bit.
    Bit,

    /// Explicit fixed-width classical bit vector.
    Vector(u64),

    /// Arbitrary-width classical bit vector.
    ///
    /// `u64` describes the semantic width, not a machine-size limit.
    Arbitrary(u64),
}

impl BitType {
    /// Returns whether this type represents exactly one classical bit.
    #[must_use]
    pub const fn is_single(self) -> bool {
        matches!(self, Self::Bit)
    }

    /// Returns the statically known width when one exists.
    #[must_use]
    pub const fn width(self) -> Option<u64> {
        match self {
            Self::Bit => Some(1),
            Self::Vector(width) | Self::Arbitrary(width) => Some(width),
        }
    }
}

impl fmt::Display for BitType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bit => formatter.write_str("bit"),
            Self::Vector(width) => write!(formatter, "bit[{width}]"),
            Self::Arbitrary(width) => {
                write!(formatter, "bit[arbitrary:{width}]")
            }
        }
    }
}

// =============================================================================
// Array type
// =============================================================================

/// Array cardinality.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrayLength {
    /// Statically known finite number of elements.
    Fixed(u64),

    /// Length is determined by a runtime/compiler parameter.
    ///
    /// The parameter identity belongs to the parameter subsystem.
    Symbolic(TypeId),

    /// Dynamically sized array.
    Dynamic,
}

impl ArrayLength {
    /// Creates a fixed array length.
    #[must_use]
    pub const fn fixed(length: u64) -> Self {
        Self::Fixed(length)
    }

    /// Returns a fixed length when statically known.
    #[must_use]
    pub const fn as_fixed(&self) -> Option<u64> {
        match self {
            Self::Fixed(length) => Some(*length),
            Self::Symbolic(_) | Self::Dynamic => None,
        }
    }

    /// Returns whether this is dynamically sized.
    #[must_use]
    pub const fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic)
    }
}

impl fmt::Display for ArrayLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed(length) => write!(formatter, "{length}"),
            Self::Symbolic(id) => write!(formatter, "${id}"),
            Self::Dynamic => formatter.write_str("*"),
        }
    }
}

// =============================================================================
// Struct field
// =============================================================================

/// A field in a canonical IR struct type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructField {
    name: String,
    ty: IrType,
}

impl StructField {
    /// Creates a validated struct field.
    pub fn new<N: Into<String>>(
        name: N,
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
    pub const fn ty(&self) -> &IrType {
        &self.ty
    }
}

// =============================================================================
// Function type
// =============================================================================

/// Canonical IR function signature.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionType {
    parameters: Vec<IrType>,
    results: Vec<IrType>,
}

impl FunctionType {
    /// Creates a function signature.
    ///
    /// The vectors are owned by the type and may contain arbitrarily many
    /// entries subject only to external resource policies and host memory.
    #[must_use]
    pub fn new(
        parameters: Vec<IrType>,
        results: Vec<IrType>,
    ) -> Self {
        Self {
            parameters,
            results,
        }
    }

    /// Returns the parameter types.
    #[must_use]
    pub fn parameters(&self) -> &[IrType] {
        &self.parameters
    }

    /// Returns the result types.
    #[must_use]
    pub fn results(&self) -> &[IrType] {
        &self.results
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns the number of results.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the function returns no values.
    #[must_use]
    pub fn returns_unit(&self) -> bool {
        self.results.is_empty()
    }
}

// =============================================================================
// Type qualifiers
// =============================================================================

/// Semantic qualifiers that can be attached to a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeQualifier {
    /// Value is immutable.
    Const,

    /// Value may be mutated.
    Mutable,

    /// Value is only valid for a particular execution region.
    RegionLocal,

    /// Value is intended to be supplied at runtime.
    Runtime,

    /// Value is compile-time known.
    CompileTime,

    /// Value participates in quantum ownership semantics.
    QuantumOwned,

    /// Value is a borrowed/reference-like semantic entity.
    Borrowed,
}

impl fmt::Display for TypeQualifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const => formatter.write_str("const"),
            Self::Mutable => formatter.write_str("mutable"),
            Self::RegionLocal => formatter.write_str("region_local"),
            Self::Runtime => formatter.write_str("runtime"),
            Self::CompileTime => formatter.write_str("compile_time"),
            Self::QuantumOwned => formatter.write_str("quantum_owned"),
            Self::Borrowed => formatter.write_str("borrowed"),
        }
    }
}

// =============================================================================
// Type category
// =============================================================================

/// Broad semantic category of an [`IrType`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeCategory {
    /// No meaningful value.
    Unit,

    /// No value can exist.
    Never,

    /// Classical scalar or scalar-like value.
    Scalar,

    /// Quantum resource/reference.
    Quantum,

    /// Classical information resource.
    Classical,

    /// Collection type.
    Aggregate,

    /// Function/callable type.
    Function,

    /// Named or extension-defined type.
    Named,

    /// Optional/result algebraic type.
    Algebraic,
}

impl fmt::Display for TypeCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => formatter.write_str("unit"),
            Self::Never => formatter.write_str("never"),
            Self::Scalar => formatter.write_str("scalar"),
            Self::Quantum => formatter.write_str("quantum"),
            Self::Classical => formatter.write_str("classical"),
            Self::Aggregate => formatter.write_str("aggregate"),
            Self::Function => formatter.write_str("function"),
            Self::Named => formatter.write_str("named"),
            Self::Algebraic => formatter.write_str("algebraic"),
        }
    }
}

// =============================================================================
// Canonical IR type
// =============================================================================

/// Canonical semantic type in Zamani Quantum IR.
///
/// `IrType` is the central type representation consumed by values,
/// operations, regions, programs and validation.
///
/// It intentionally contains semantic information only.
///
/// No hardware topology, calibration, backend instruction, routing decision,
/// scheduling decision, or simulator state can be represented by this type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrType {
    // -------------------------------------------------------------------------
    // Fundamental
    // -------------------------------------------------------------------------

    /// No meaningful result.
    Unit,

    /// Uninhabited type.
    Never,

    /// Boolean.
    Bool,

    // -------------------------------------------------------------------------
    // Integer
    // -------------------------------------------------------------------------

    /// Signed integer.
    SignedInteger(SignedIntegerType),

    /// Unsigned integer.
    UnsignedInteger(UnsignedIntegerType),

    // -------------------------------------------------------------------------
    // Floating/complex
    // -------------------------------------------------------------------------

    /// Floating-point scalar.
    Float(FloatType),

    /// Complex scalar.
    Complex(ComplexType),

    // -------------------------------------------------------------------------
    // Quantum-semantic scalar types
    // -------------------------------------------------------------------------

    /// Angle measured semantically in radians.
    Angle,

    /// Time duration.
    Duration,

    /// Frequency.
    Frequency,

    /// Hardware-independent amplitude.
    Amplitude,

    /// Phase measured semantically in radians.
    Phase,

    // -------------------------------------------------------------------------
    // Quantum resources
    // -------------------------------------------------------------------------

    /// Logical quantum bit.
    Qubit,

    /// Explicit physical quantum bit identity.
    ///
    /// The type does not imply that the physical qubit exists or is available.
    PhysicalQubit,

    /// Explicit reference to a logical qubit identity.
    ///
    /// This is useful where a value carries a canonical `QubitId`.
    LogicalQubitRef(QubitId),

    /// Explicit reference to a physical qubit identity.
    ///
    /// This does not establish hardware availability.
    PhysicalQubitRef(PhysicalQubitId),

    // -------------------------------------------------------------------------
    // Classical resources
    // -------------------------------------------------------------------------

    /// Single classical bit.
    Bit,

    /// Classical bit vector.
    Bits(BitType),

    // -------------------------------------------------------------------------
    // Aggregates
    // -------------------------------------------------------------------------

    /// Homogeneous array.
    Array {
        /// Element type.
        element: Box<IrType>,

        /// Array cardinality.
        length: ArrayLength,
    },

    /// Ordered heterogeneous tuple.
    Tuple(Vec<IrType>),

    /// Named heterogeneous struct.
    Struct(Vec<StructField>),

    // -------------------------------------------------------------------------
    // Algebraic types
    // -------------------------------------------------------------------------

    /// Optional value.
    Option(Box<IrType>),

    /// Result with success and error types.
    Result {
        /// Success type.
        ok: Box<IrType>,

        /// Error type.
        err: Box<IrType>,
    },

    // -------------------------------------------------------------------------
    // Callable
    // -------------------------------------------------------------------------

    /// Function signature.
    Function(FunctionType),

    // -------------------------------------------------------------------------
    // Named/extension types
    // -------------------------------------------------------------------------

    /// Reference to a declared canonical IR type.
    Named(TypeId),
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

    /// Returns boolean type.
    #[must_use]
    pub const fn bool() -> Self {
        Self::Bool
    }

    /// Returns an `isize`-semantic integer type.
    #[must_use]
    pub const fn isize() -> Self {
        Self::SignedInteger(SignedIntegerType::Size)
    }

    /// Returns a `usize`-semantic integer type.
    #[must_use]
    pub const fn usize() -> Self {
        Self::UnsignedInteger(UnsignedIntegerType::Size)
    }

    /// Returns signed integer type.
    #[must_use]
    pub const fn signed(kind: SignedIntegerType) -> Self {
        Self::SignedInteger(kind)
    }

    /// Returns unsigned integer type.
    #[must_use]
    pub const fn unsigned(kind: UnsignedIntegerType) -> Self {
        Self::UnsignedInteger(kind)
    }

    /// Returns a floating-point type.
    #[must_use]
    pub const fn float(kind: FloatType) -> Self {
        Self::Float(kind)
    }

    /// Returns a complex type.
    #[must_use]
    pub const fn complex(kind: ComplexType) -> Self {
        Self::Complex(kind)
    }

    /// Returns angle type.
    #[must_use]
    pub const fn angle() -> Self {
        Self::Angle
    }

    /// Returns duration type.
    #[must_use]
    pub const fn duration() -> Self {
        Self::Duration
    }

    /// Returns frequency type.
    #[must_use]
    pub const fn frequency() -> Self {
        Self::Frequency
    }

    /// Returns amplitude type.
    #[must_use]
    pub const fn amplitude() -> Self {
        Self::Amplitude
    }

    /// Returns phase type.
    #[must_use]
    pub const fn phase() -> Self {
        Self::Phase
    }

    /// Returns logical-qubit type.
    #[must_use]
    pub const fn qubit() -> Self {
        Self::Qubit
    }

    /// Returns physical-qubit type.
    #[must_use]
    pub const fn physical_qubit() -> Self {
        Self::PhysicalQubit
    }

    /// Creates an explicit logical-qubit reference type.
    #[must_use]
    pub const fn logical_qubit_ref(id: QubitId) -> Self {
        Self::LogicalQubitRef(id)
    }

    /// Creates an explicit physical-qubit reference type.
    #[must_use]
    pub const fn physical_qubit_ref(id: PhysicalQubitId) -> Self {
        Self::PhysicalQubitRef(id)
    }

    /// Returns single classical-bit type.
    #[must_use]
    pub const fn bit() -> Self {
        Self::Bit
    }

    /// Returns classical bit-vector type.
    #[must_use]
    pub const fn bits(kind: BitType) -> Self {
        Self::Bits(kind)
    }

    /// Creates a fixed-size array type.
    #[must_use]
    pub fn array(element: IrType, length: u64) -> Self {
        Self::Array {
            element: Box::new(element),
            length: ArrayLength::Fixed(length),
        }
    }

    /// Creates a dynamically sized array type.
    #[must_use]
    pub fn dynamic_array(element: IrType) -> Self {
        Self::Array {
            element: Box::new(element),
            length: ArrayLength::Dynamic,
        }
    }

    /// Creates an array with symbolic cardinality.
    #[must_use]
    pub fn symbolic_array(
        element: IrType,
        length: TypeId,
    ) -> Self {
        Self::Array {
            element: Box::new(element),
            length: ArrayLength::Symbolic(length),
        }
    }

    /// Creates a tuple type.
    #[must_use]
    pub fn tuple(elements: Vec<IrType>) -> Self {
        Self::Tuple(elements)
    }

    /// Creates a struct type after validating its field names.
    pub fn structure(
        fields: Vec<StructField>,
    ) -> Result<Self, TypeError> {
        validate_struct_fields(&fields)?;
        Ok(Self::Struct(fields))
    }

    /// Creates an option type.
    #[must_use]
    pub fn option(inner: IrType) -> Self {
        Self::Option(Box::new(inner))
    }

    /// Creates a result type.
    #[must_use]
    pub fn result(
        ok: IrType,
        err: IrType,
    ) -> Self {
        Self::Result {
            ok: Box::new(ok),
            err: Box::new(err),
        }
    }

    /// Creates a function type.
    #[must_use]
    pub fn function(
        parameters: Vec<IrType>,
        results: Vec<IrType>,
    ) -> Self {
        Self::Function(FunctionType::new(parameters, results))
    }

    /// Creates a named type reference.
    #[must_use]
    pub const fn named(id: TypeId) -> Self {
        Self::Named(id)
    }

    // =========================================================================
    // Classification
    // =========================================================================

    /// Returns the broad semantic category.
    #[must_use]
    pub const fn category(&self) -> TypeCategory {
        match self {
            Self::Unit => TypeCategory::Unit,
            Self::Never => TypeCategory::Never,

            Self::Bool
            | Self::SignedInteger(_)
            | Self::UnsignedInteger(_)
            | Self::Float(_)
            | Self::Complex(_)
            | Self::Angle
            | Self::Duration
            | Self::Frequency
            | Self::Amplitude
            | Self::Phase => TypeCategory::Scalar,

            Self::Qubit
            | Self::PhysicalQubit
            | Self::LogicalQubitRef(_)
            | Self::PhysicalQubitRef(_) => TypeCategory::Quantum,

            Self::Bit | Self::Bits(_) => TypeCategory::Classical,

            Self::Array { .. }
            | Self::Tuple(_)
            | Self::Struct(_) => TypeCategory::Aggregate,

            Self::Function(_) => TypeCategory::Function,

            Self::Named(_) => TypeCategory::Named,

            Self::Option(_) | Self::Result { .. } => TypeCategory::Algebraic,
        }
    }

    /// Returns whether the type is a scalar.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(self.category(), TypeCategory::Scalar)
    }

    /// Returns whether the type is quantum.
    #[must_use]
    pub const fn is_quantum(&self) -> bool {
        matches!(self.category(), TypeCategory::Quantum)
    }

    /// Returns whether the type is classical.
    #[must_use]
    pub const fn is_classical(&self) -> bool {
        matches!(self.category(), TypeCategory::Classical)
    }

    /// Returns whether the type is an aggregate.
    #[must_use]
    pub const fn is_aggregate(&self) -> bool {
        matches!(self.category(), TypeCategory::Aggregate)
    }

    /// Returns whether the type is callable.
    #[must_use]
    pub const fn is_function(&self) -> bool {
        matches!(self.category(), TypeCategory::Function)
    }

    /// Returns whether this type is algebraic.
    #[must_use]
    pub const fn is_algebraic(&self) -> bool {
        matches!(self.category(), TypeCategory::Algebraic)
    }

    /// Returns whether this is unit.
    #[must_use]
    pub const fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    /// Returns whether this is never.
    #[must_use]
    pub const fn is_never(&self) -> bool {
        matches!(self, Self::Never)
    }

    /// Returns whether this type contains a quantum resource anywhere in its
    /// structure.
    #[must_use]
    pub fn contains_quantum(&self) -> bool {
        match self {
            Self::Qubit
            | Self::PhysicalQubit
            | Self::LogicalQubitRef(_)
            | Self::PhysicalQubitRef(_) => true,

            Self::Array { element, .. } => element.contains_quantum(),

            Self::Tuple(elements) => {
                elements.iter().any(Self::contains_quantum)
            }

            Self::Struct(fields) => {
                fields.iter().any(|field| field.ty.contains_quantum())
            }

            Self::Option(inner) => inner.contains_quantum(),

            Self::Result { ok, err } => {
                ok.contains_quantum() || err.contains_quantum()
            }

            Self::Function(function) => {
                function.parameters.iter().any(Self::contains_quantum)
                    || function.results.iter().any(Self::contains_quantum)
            }

            Self::Unit
            | Self::Never
            | Self::Bool
            | Self::SignedInteger(_)
            | Self::UnsignedInteger(_)
            | Self::Float(_)
            | Self::Complex(_)
            | Self::Angle
            | Self::Duration
            | Self::Frequency
            | Self::Amplitude
            | Self::Phase
            | Self::Bit
            | Self::Bits(_)
            | Self::Named(_) => false,
        }
    }

    /// Returns whether this type directly represents a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(&self) -> bool {
        matches!(
            self,
            Self::Qubit | Self::LogicalQubitRef(_)
        )
    }

    /// Returns whether this type directly represents a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(&self) -> bool {
        matches!(
            self,
            Self::PhysicalQubit | Self::PhysicalQubitRef(_)
        )
    }

    /// Returns whether this is a classical boolean-like type.
    #[must_use]
    pub const fn is_boolean_like(&self) -> bool {
        matches!(self, Self::Bool | Self::Bit)
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the array element type if this is an array.
    #[must_use]
    pub fn array_element(&self) -> Option<&IrType> {
        match self {
            Self::Array { element, .. } => Some(element),
            _ => None,
        }
    }

    /// Returns the array length if this is an array.
    #[must_use]
    pub fn array_length(&self) -> Option<&ArrayLength> {
        match self {
            Self::Array { length, .. } => Some(length),
            _ => None,
        }
    }

    /// Returns tuple elements if this is a tuple.
    #[must_use]
    pub fn tuple_elements(&self) -> Option<&[IrType]> {
        match self {
            Self::Tuple(elements) => Some(elements),
            _ => None,
        }
    }

    /// Returns struct fields if this is a struct.
    #[must_use]
    pub fn struct_fields(&self) -> Option<&[StructField]> {
        match self {
            Self::Struct(fields) => Some(fields),
            _ => None,
        }
    }

    /// Returns function signature if this is a function.
    #[must_use]
    pub fn function_signature(&self) -> Option<&FunctionType> {
        match self {
            Self::Function(function) => Some(function),
            _ => None,
        }
    }

    /// Returns the inner option type if this is an option.
    #[must_use]
    pub fn option_inner(&self) -> Option<&IrType> {
        match self {
            Self::Option(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns result components if this is a result.
    #[must_use]
    pub fn result_types(&self) -> Option<(&IrType, &IrType)> {
        match self {
            Self::Result { ok, err } => Some((ok, err)),
            _ => None,
        }
    }

    /// Returns a named type identity if this is a named type.
    #[must_use]
    pub const fn named_id(&self) -> Option<TypeId> {
        match self {
            Self::Named(id) => Some(*id),
            _ => None,
        }
    }

    // =========================================================================
    // Structural validation
    // =========================================================================

    /// Validates the structural invariants of this type.
    ///
    /// This validates the type representation itself.
    ///
    /// It does not:
    ///
    /// - check target hardware;
    /// - allocate resources;
    /// - check compiler resource budgets;
    /// - resolve named types;
    /// - execute expressions.
    pub fn validate(&self) -> Result<(), TypeError> {
        match self {
            Self::Array { element, .. } => {
                element.validate()?;
            }

            Self::Tuple(elements) => {
                for element in elements {
                    element.validate()?;
                }
            }

            Self::Struct(fields) => {
                validate_struct_fields(fields)?;
            }

            Self::Option(inner) => {
                inner.validate()?;
            }

            Self::Result { ok, err } => {
                ok.validate()?;
                err.validate()?;
            }

            Self::Function(function) => {
                for parameter in &function.parameters {
                    parameter.validate()?;
                }

                for result in &function.results {
                    result.validate()?;
                }
            }

            Self::Unit
            | Self::Never
            | Self::Bool
            | Self::SignedInteger(_)
            | Self::UnsignedInteger(_)
            | Self::Float(_)
            | Self::Complex(_)
            | Self::Angle
            | Self::Duration
            | Self::Frequency
            | Self::Amplitude
            | Self::Phase
            | Self::Qubit
            | Self::PhysicalQubit
            | Self::LogicalQubitRef(_)
            | Self::PhysicalQubitRef(_)
            | Self::Bit
            | Self::Bits(_)
            | Self::Named(_) => {}
        }

        Ok(())
    }

    // =========================================================================
    // Compatibility
    // =========================================================================

    /// Returns whether two types are structurally identical.
    #[must_use]
    pub fn is_exactly(&self, other: &Self) -> bool {
        self == other
    }

    /// Returns whether this type can be used where `other` is expected under
    /// the conservative canonical IR compatibility rules.
    ///
    /// This method intentionally does not perform implicit numeric promotion.
    /// Such promotion policy belongs to the compiler/type-checking layer.
    #[must_use]
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self == other
    }

    /// Checks exact compatibility and returns a typed error on failure.
    pub fn require_compatible(
        &self,
        other: &Self,
    ) -> Result<(), TypeError> {
        if self.is_compatible_with(other) {
            Ok(())
        } else {
            Err(TypeError::IncompatibleTypes {
                left: Box::new(self.clone()),
                right: Box::new(other.clone()),
            })
        }
    }

    // =========================================================================
    // Semantic properties
    // =========================================================================

    /// Returns whether values of this type are copy-like at the semantic IR
    /// level.
    ///
    /// Quantum resources are deliberately excluded.
    #[must_use]
    pub fn is_copyable(&self) -> bool {
        match self {
            Self::Qubit
            | Self::PhysicalQubit
            | Self::LogicalQubitRef(_)
            | Self::PhysicalQubitRef(_) => false,

            Self::Array { element, .. } => element.is_copyable(),

            Self::Tuple(elements) => {
                elements.iter().all(Self::is_copyable)
            }

            Self::Struct(fields) => {
                fields.iter().all(|field| field.ty.is_copyable())
            }

            Self::Option(inner) => inner.is_copyable(),

            Self::Result { ok, err } => {
                ok.is_copyable() && err.is_copyable()
            }

            Self::Function(_) => false,

            Self::Unit
            | Self::Never
            | Self::Bool
            | Self::SignedInteger(_)
            | Self::UnsignedInteger(_)
            | Self::Float(_)
            | Self::Complex(_)
            | Self::Angle
            | Self::Duration
            | Self::Frequency
            | Self::Amplitude
            | Self::Phase
            | Self::Bit
            | Self::Bits(_)
            | Self::Named(_) => true,
        }
    }

    /// Returns whether the type is a zero-sized semantic type.
    #[must_use]
    pub const fn is_zero_sized(&self) -> bool {
        matches!(self, Self::Unit | Self::Never)
    }

    /// Returns whether the type represents a numeric value.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::SignedInteger(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Complex(_)
                | Self::Angle
                | Self::Duration
                | Self::Frequency
                | Self::Amplitude
                | Self::Phase
        )
    }

    /// Returns whether the type represents a temporal quantity.
    #[must_use]
    pub const fn is_temporal(&self) -> bool {
        matches!(self, Self::Duration)
    }

    /// Returns whether the type represents a frequency quantity.
    #[must_use]
    pub const fn is_frequency(&self) -> bool {
        matches!(self, Self::Frequency)
    }

    /// Returns whether the type represents an angle quantity.
    #[must_use]
    pub const fn is_angle(&self) -> bool {
        matches!(self, Self::Angle | Self::Phase)
    }

    /// Returns whether the type is a parameter/control quantity commonly used
    /// by pulse-level semantics.
    #[must_use]
    pub const fn is_pulse_scalar(&self) -> bool {
        matches!(
            self,
            Self::Amplitude
                | Self::Duration
                | Self::Frequency
                | Self::Phase
        )
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for IrType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => formatter.write_str("unit"),
            Self::Never => formatter.write_str("never"),
            Self::Bool => formatter.write_str("bool"),

            Self::SignedInteger(kind) => write!(formatter, "{kind}"),
            Self::UnsignedInteger(kind) => write!(formatter, "{kind}"),

            Self::Float(kind) => write!(formatter, "{kind}"),
            Self::Complex(kind) => write!(formatter, "{kind}"),

            Self::Angle => formatter.write_str("angle"),
            Self::Duration => formatter.write_str("duration"),
            Self::Frequency => formatter.write_str("frequency"),
            Self::Amplitude => formatter.write_str("amplitude"),
            Self::Phase => formatter.write_str("phase"),

            Self::Qubit => formatter.write_str("qubit"),
            Self::PhysicalQubit => formatter.write_str("physical_qubit"),

            Self::LogicalQubitRef(id) => {
                write!(formatter, "qubit<{id}>")
            }

            Self::PhysicalQubitRef(id) => {
                write!(formatter, "physical_qubit<{id}>")
            }

            Self::Bit => formatter.write_str("bit"),
            Self::Bits(kind) => write!(formatter, "{kind}"),

            Self::Array { element, length } => {
                write!(formatter, "[{element}; {length}]")
            }

            Self::Tuple(elements) => {
                formatter.write_str("(")?;

                for (index, element) in elements.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{element}")?;
                }

                formatter.write_str(")")
            }

            Self::Struct(fields) => {
                formatter.write_str("struct {")?;

                for (index, field) in fields.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{}: {}", field.name, field.ty)?;
                }

                formatter.write_str("}")
            }

            Self::Option(inner) => {
                write!(formatter, "option<{inner}>")
            }

            Self::Result { ok, err } => {
                write!(formatter, "result<{ok}, {err}>")
            }

            Self::Function(function) => {
                formatter.write_str("fn(")?;

                for (index, parameter) in
                    function.parameters.iter().enumerate()
                {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{parameter}")?;
                }

                formatter.write_str(")")?;

                if function.results.len() == 1 {
                    write!(formatter, " -> {}", function.results[0])
                } else if function.results.is_empty() {
                    Ok(())
                } else {
                    formatter.write_str(" -> (")?;

                    for (index, result) in
                        function.results.iter().enumerate()
                    {
                        if index != 0 {
                            formatter.write_str(", ")?;
                        }

                        write!(formatter, "{result}")?;
                    }

                    formatter.write_str(")")
                }
            }

            Self::Named(id) => write!(formatter, "type<{id}>"),
        }
    }
}

// =============================================================================
// Identifier validation
// =============================================================================

/// Validates an IR identifier/name.
///
/// The type system deliberately uses a language-neutral identifier contract:
///
/// - first character: ASCII letter or `_`;
/// - remaining characters: ASCII letters, digits or `_`;
/// - non-empty.
///
/// Frontend-specific Unicode/name policies remain frontend concerns.
fn validate_identifier(name: &str) -> Result<(), TypeError> {
    let mut characters = name.chars();

    let first = characters.next().ok_or(TypeError::EmptyName)?;

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(TypeError::InvalidName);
    }

    for character in characters {
        if !(character == '_' || character.is_ascii_alphanumeric()) {
            return Err(TypeError::InvalidName);
        }
    }

    Ok(())
}

/// Validates struct fields.
fn validate_struct_fields(
    fields: &[StructField],
) -> Result<(), TypeError> {
    for field in fields {
        field.ty.validate()?;
    }

    for left_index in 0..fields.len() {
        for right_index in (left_index + 1)..fields.len() {
            if fields[left_index].name == fields[right_index].name {
                return Err(TypeError::DuplicateFieldName(
                    fields[left_index].name.clone(),
                ));
            }
        }
    }

    Ok(())
}

// =============================================================================
// Type helpers
// =============================================================================

/// Returns whether two types are exactly equal.
#[must_use]
pub fn types_equal(
    left: &IrType,
    right: &IrType,
) -> bool {
    left == right
}

/// Requires two types to be exactly compatible.
pub fn require_same_type(
    left: &IrType,
    right: &IrType,
) -> Result<(), TypeError> {
    left.require_compatible(right)
}

/// Returns whether a type is a logical qubit type.
#[must_use]
pub fn is_qubit_type(ty: &IrType) -> bool {
    ty.is_logical_qubit()
}

/// Returns whether a type is a physical-qubit type.
#[must_use]
pub fn is_physical_qubit_type(ty: &IrType) -> bool {
    ty.is_physical_qubit()
}

/// Returns whether a type is a classical bit-like type.
#[must_use]
pub fn is_bit_type(ty: &IrType) -> bool {
    matches!(ty, IrType::Bit | IrType::Bits(_))
}

/// Returns whether a type is a pulse-control scalar.
#[must_use]
pub fn is_pulse_scalar_type(ty: &IrType) -> bool {
    ty.is_pulse_scalar()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_types_are_classified_correctly() {
        assert!(IrType::Bool.is_scalar());
        assert!(IrType::float(FloatType::F64).is_scalar());
        assert!(IrType::angle().is_scalar());
        assert!(IrType::duration().is_scalar());
        assert!(IrType::frequency().is_scalar());
        assert!(IrType::amplitude().is_scalar());
        assert!(IrType::phase().is_scalar());
    }

    #[test]
    fn qubit_types_use_canonical_qubit_identity_types() {
        let logical = IrType::logical_qubit_ref(QubitId::new(0));
        let physical =
            IrType::physical_qubit_ref(PhysicalQubitId::new(0));

        assert!(logical.is_quantum());
        assert!(physical.is_quantum());
        assert!(logical.is_logical_qubit());
        assert!(physical.is_physical_qubit());
    }

    #[test]
    fn quantum_resources_are_not_copyable() {
        assert!(!IrType::Qubit.is_copyable());
        assert!(!IrType::PhysicalQubit.is_copyable());
    }

    #[test]
    fn quantum_containment_is_recursive() {
        let ty = IrType::array(
            IrType::tuple(vec![
                IrType::Bool,
                IrType::Qubit,
            ]),
            1_000_000_000,
        );

        assert!(ty.contains_quantum());
    }

    #[test]
    fn fixed_large_array_does_not_materialize_elements() {
        let ty = IrType::array(
            IrType::Qubit,
            u64::MAX,
        );

        assert_eq!(
            ty.array_length()
                .and_then(ArrayLength::as_fixed),
            Some(u64::MAX)
        );
    }

    #[test]
    fn dynamic_array_is_supported() {
        let ty = IrType::dynamic_array(IrType::Float(
            FloatType::F64,
        ));

        assert!(ty.array_length().is_some());
        assert!(
            ty.array_length()
                .is_some_and(ArrayLength::is_dynamic)
        );
    }

    #[test]
    fn struct_fields_are_validated() {
        let first = StructField::new(
            "q",
            IrType::Qubit,
        )
        .expect("valid field");

        let second = StructField::new(
            "result",
            IrType::Bit,
        )
        .expect("valid field");

        let ty = IrType::structure(vec![
            first,
            second,
        ])
        .expect("valid structure");

        assert!(ty.validate().is_ok());
    }

    #[test]
    fn duplicate_struct_fields_are_rejected() {
        let first = StructField::new(
            "value",
            IrType::Bool,
        )
        .expect("valid field");

        let second = StructField::new(
            "value",
            IrType::Bit,
        )
        .expect("valid field");

        let result = IrType::structure(vec![
            first,
            second,
        ]);

        assert!(matches!(
            result,
            Err(TypeError::DuplicateFieldName(_))
        ));
    }

    #[test]
    fn function_types_are_structural() {
        let function = IrType::function(
            vec![
                IrType::Qubit,
                IrType::Angle,
            ],
            vec![
                IrType::Bit,
            ],
        );

        assert!(function.is_function());

        let signature = function
            .function_signature()
            .expect("function signature");

        assert_eq!(
            signature.parameter_count(),
            2
        );

        assert_eq!(
            signature.result_count(),
            1
        );
    }

    #[test]
    fn result_and_option_types_are_supported() {
        let optional = IrType::option(
            IrType::Qubit,
        );

        let result = IrType::result(
            IrType::Bit,
            IrType::Bool,
        );

        assert!(optional.is_algebraic());
        assert!(result.is_algebraic());
        assert!(optional.contains_quantum());
        assert!(!result.contains_quantum());
    }

    #[test]
    fn pulse_scalar_classification_is_correct() {
        assert!(IrType::Amplitude.is_pulse_scalar());
        assert!(IrType::Duration.is_pulse_scalar());
        assert!(IrType::Frequency.is_pulse_scalar());
        assert!(IrType::Phase.is_pulse_scalar());

        assert!(!IrType::Bool.is_pulse_scalar());
        assert!(!IrType::Qubit.is_pulse_scalar());
    }

    #[test]
    fn exact_compatibility_is_conservative() {
        assert!(
            IrType::Bool.is_compatible_with(
                &IrType::Bool
            )
        );

        assert!(
            !IrType::Bool.is_compatible_with(
                &IrType::Bit
            )
        );

        assert!(
            !IrType::Float(FloatType::F32)
                .is_compatible_with(
                    &IrType::Float(FloatType::F64)
                )
        );
    }

    #[test]
    fn display_is_deterministic() {
        let ty = IrType::array(
            IrType::Qubit,
            128,
        );

        assert_eq!(
            ty.to_string(),
            "[qubit; 128]"
        );
    }

    #[test]
    fn no_machine_size_limit_is_encoded() {
        let ty = IrType::array(
            IrType::Qubit,
            u64::MAX,
        );

        assert!(ty.validate().is_ok());
    }

    #[test]
    fn nested_quantum_function_is_detected() {
        let ty = IrType::function(
            vec![
                IrType::array(
                    IrType::Qubit,
                    u64::MAX,
                ),
            ],
            vec![
                IrType::Bit,
            ],
        );

        assert!(ty.contains_quantum());
    }
}