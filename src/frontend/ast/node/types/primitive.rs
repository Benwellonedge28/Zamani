//! Primitive type expressions for the Zamani frontend AST.
//!
//! # Architectural role
//!
//! This module owns the source-level representation of primitive types.
//!
//! It deliberately does **not** own:
//!
//! - semantic type resolution;
//! - target ABI selection;
//! - machine-word sizing;
//! - register allocation;
//! - quantum hardware representation;
//! - backend instruction selection;
//! - LLVM/QIR/MLIR types;
//! - vendor-specific types;
//! - physical resource allocation.
//!
//! The dependency direction is:
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! frontend::ast::node::types::primitive
//!     │
//!     ▼
//! semantic type resolution
//!     │
//!     ▼
//! compiler / semantic type
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! # Scalability
//!
//! Primitive types must not encode a machine size or hardware limitation.
//!
//! A primitive such as `usize` expresses source-language intent. Its actual
//! representation is selected by a later semantic/target phase.
//!
//! This module therefore never contains:
//!
//! ```text
//! MAX_BITS
//! MAX_INTEGER_WIDTH
//! MAX_REGISTER_SIZE
//! MAX_QUBITS
//! MAX_RESOURCES
//! ```
//!
//! Nor does it use target-specific Rust machine types to model the source
//! primitive itself.
//!
//! # Compatibility
//!
//! The legacy AST currently contains `IntWidth` and `FloatWidth` alongside
//! `TypeExpr`. Those legacy definitions must eventually lower into the types
//! defined here rather than being imported by this module.
//!
//! # Rust
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! # Safety
//!
//! This module uses no `unsafe` code.

use core::fmt;

/// A source-level primitive type.
///
/// `PrimitiveType` describes the primitive type requested by the programmer.
/// It does not determine its physical representation.
///
/// In particular, `usize` and `isize` are intentionally represented as
/// semantic source types rather than being converted to the host compiler's
/// `usize`/`isize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrimitiveType {
    /// Unit / no meaningful value.
    Unit,

    /// Boolean value.
    Bool,

    /// Signed integer.
    Int(IntegerType),

    /// Unsigned integer.
    UInt(IntegerType),

    /// Floating-point value.
    Float(FloatType),

    /// Unicode scalar value.
    Char,

    /// Borrowed string/text slice.
    Str,

    /// Owned string/text value.
    String,
}

impl PrimitiveType {
    /// Returns the canonical Zamani spelling of this primitive.
    ///
    /// This is source-language spelling, not a target ABI spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::Bool => "bool",
            Self::Int(width) => width.signed_name(),
            Self::UInt(width) => width.unsigned_name(),
            Self::Float(width) => width.as_str(),
            Self::Char => "char",
            Self::Str => "str",
            Self::String => "String",
        }
    }

    /// Returns whether this primitive is numeric.
    #[must_use]
    pub const fn is_numeric(self) -> bool {
        matches!(self, Self::Int(_) | Self::UInt(_) | Self::Float(_))
    }

    /// Returns whether this primitive is an integer.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        matches!(self, Self::Int(_) | Self::UInt(_))
    }

    /// Returns whether this primitive is signed.
    #[must_use]
    pub const fn is_signed(self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Returns whether this primitive is unsigned.
    #[must_use]
    pub const fn is_unsigned(self) -> bool {
        matches!(self, Self::UInt(_))
    }

    /// Returns whether this primitive is floating-point.
    #[must_use]
    pub const fn is_float(self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns whether this primitive represents textual data.
    #[must_use]
    pub const fn is_text(self) -> bool {
        matches!(self, Self::Str | Self::String | Self::Char)
    }

    /// Returns the integer width when this is an integer primitive.
    #[must_use]
    pub const fn integer_width(self) -> Option<IntegerType> {
        match self {
            Self::Int(width) | Self::UInt(width) => Some(width),
            _ => None,
        }
    }

    /// Returns the floating-point width when this is a floating primitive.
    #[must_use]
    pub const fn float_width(self) -> Option<FloatType> {
        match self {
            Self::Float(width) => Some(width),
            _ => None,
        }
    }

    /// Parses a canonical Zamani primitive spelling.
    ///
    /// This method intentionally recognizes only the primitive vocabulary
    /// owned by this module. Named types, generic types, aliases, extensions,
    /// domain-specific types and user-defined types belong to other AST
    /// modules.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "unit" | "Unit" => Some(Self::Unit),
            "bool" | "Bool" => Some(Self::Bool),

            "i8" => Some(Self::Int(IntegerType::Bits8)),
            "i16" => Some(Self::Int(IntegerType::Bits16)),
            "i32" => Some(Self::Int(IntegerType::Bits32)),
            "i64" => Some(Self::Int(IntegerType::Bits64)),
            "i128" => Some(Self::Int(IntegerType::Bits128)),
            "isize" => Some(Self::Int(IntegerType::Size)),

            "u8" => Some(Self::UInt(IntegerType::Bits8)),
            "u16" => Some(Self::UInt(IntegerType::Bits16)),
            "u32" => Some(Self::UInt(IntegerType::Bits32)),
            "u64" => Some(Self::UInt(IntegerType::Bits64)),
            "u128" => Some(Self::UInt(IntegerType::Bits128)),
            "usize" => Some(Self::UInt(IntegerType::Size)),

            "f32" => Some(Self::Float(FloatType::Bits32)),
            "f64" => Some(Self::Float(FloatType::Bits64)),
            "f128" => Some(Self::Float(FloatType::Bits128)),

            "char" | "Char" => Some(Self::Char),
            "str" => Some(Self::Str),
            "String" | "string" => Some(Self::String),

            _ => None,
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Source-level integer width.
///
/// `Size` means the language-defined machine-dependent integer size
/// (`isize`/`usize`). It deliberately does not contain the host Rust
/// `usize::BITS` value.
///
/// The semantic/backend layer determines the concrete representation for a
/// compilation target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntegerType {
    /// Exactly 8 bits.
    Bits8,

    /// Exactly 16 bits.
    Bits16,

    /// Exactly 32 bits.
    Bits32,

    /// Exactly 64 bits.
    Bits64,

    /// Exactly 128 bits.
    Bits128,

    /// Target-dependent language size (`isize` / `usize`).
    Size,
}

impl IntegerType {
    /// Returns the canonical signed spelling.
    #[must_use]
    pub const fn signed_name(self) -> &'static str {
        match self {
            Self::Bits8 => "i8",
            Self::Bits16 => "i16",
            Self::Bits32 => "i32",
            Self::Bits64 => "i64",
            Self::Bits128 => "i128",
            Self::Size => "isize",
        }
    }

    /// Returns the canonical unsigned spelling.
    #[must_use]
    pub const fn unsigned_name(self) -> &'static str {
        match self {
            Self::Bits8 => "u8",
            Self::Bits16 => "u16",
            Self::Bits32 => "u32",
            Self::Bits64 => "u64",
            Self::Bits128 => "u128",
            Self::Size => "usize",
        }
    }

    /// Returns the exact bit width when the width is fixed.
    ///
    /// `None` is returned for `Size`, because `isize` and `usize` are
    /// intentionally target-dependent.
    #[must_use]
    pub const fn fixed_bits(self) -> Option<u16> {
        match self {
            Self::Bits8 => Some(8),
            Self::Bits16 => Some(16),
            Self::Bits32 => Some(32),
            Self::Bits64 => Some(64),
            Self::Bits128 => Some(128),
            Self::Size => None,
        }
    }

    /// Returns whether this width is target-dependent.
    #[must_use]
    pub const fn is_target_dependent(self) -> bool {
        matches!(self, Self::Size)
    }

    /// Returns whether this width is fixed independently of the target.
    #[must_use]
    pub const fn is_fixed(self) -> bool {
        !self.is_target_dependent()
    }

    /// Returns the corresponding signed integer primitive.
    #[must_use]
    pub const fn signed(self) -> PrimitiveType {
        PrimitiveType::Int(self)
    }

    /// Returns the corresponding unsigned integer primitive.
    #[must_use]
    pub const fn unsigned(self) -> PrimitiveType {
        PrimitiveType::UInt(self)
    }

    /// Parses a fixed or target-dependent integer width.
    ///
    /// The signedness is deliberately not encoded here because signedness
    /// belongs to `PrimitiveType::Int` versus `PrimitiveType::UInt`.
    #[must_use]
    pub fn from_signed_name(name: &str) -> Option<Self> {
        match name {
            "i8" => Some(Self::Bits8),
            "i16" => Some(Self::Bits16),
            "i32" => Some(Self::Bits32),
            "i64" => Some(Self::Bits64),
            "i128" => Some(Self::Bits128),
            "isize" => Some(Self::Size),
            _ => None,
        }
    }

    /// Parses an unsigned integer width.
    #[must_use]
    pub fn from_unsigned_name(name: &str) -> Option<Self> {
        match name {
            "u8" => Some(Self::Bits8),
            "u16" => Some(Self::Bits16),
            "u32" => Some(Self::Bits32),
            "u64" => Some(Self::Bits64),
            "u128" => Some(Self::Bits128),
            "usize" => Some(Self::Size),
            _ => None,
        }
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.signed_name())
    }
}

/// Source-level floating-point width.
///
/// This enum describes language-level floating-point formats. It does not
/// imply that the selected target has native hardware support for every
/// format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FloatType {
    /// IEEE-style binary32 / `f32`.
    Bits32,

    /// IEEE-style binary64 / `f64`.
    Bits64,

    /// Extended language floating-point type / `f128`.
    ///
    /// Whether the selected backend implements this natively, emulates it,
    /// lowers it to another representation, or rejects it is a downstream
    /// semantic/target decision.
    Bits128,
}

impl FloatType {
    /// Returns the canonical Zamani spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bits32 => "f32",
            Self::Bits64 => "f64",
            Self::Bits128 => "f128",
        }
    }

    /// Returns the fixed format width in bits.
    #[must_use]
    pub const fn bits(self) -> u16 {
        match self {
            Self::Bits32 => 32,
            Self::Bits64 => 64,
            Self::Bits128 => 128,
        }
    }

    /// Parses a floating-point primitive name.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "f32" => Some(Self::Bits32),
            "f64" => Some(Self::Bits64),
            "f128" => Some(Self::Bits128),
            _ => None,
        }
    }

    /// Returns the corresponding primitive type.
    #[must_use]
    pub const fn primitive(self) -> PrimitiveType {
        PrimitiveType::Float(self)
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Returns the canonical primitive type for a source name.
///
/// This is intentionally a small convenience wrapper around
/// [`PrimitiveType::from_name`].
#[must_use]
pub fn parse_primitive(name: &str) -> Option<PrimitiveType> {
    PrimitiveType::from_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_integer_names_are_stable() {
        assert_eq!(PrimitiveType::Int(IntegerType::Bits8).as_str(), "i8");
        assert_eq!(PrimitiveType::Int(IntegerType::Bits16).as_str(), "i16");
        assert_eq!(PrimitiveType::Int(IntegerType::Bits32).as_str(), "i32");
        assert_eq!(PrimitiveType::Int(IntegerType::Bits64).as_str(), "i64");
        assert_eq!(PrimitiveType::Int(IntegerType::Bits128).as_str(), "i128");
        assert_eq!(PrimitiveType::Int(IntegerType::Size).as_str(), "isize");

        assert_eq!(PrimitiveType::UInt(IntegerType::Bits8).as_str(), "u8");
        assert_eq!(PrimitiveType::UInt(IntegerType::Bits16).as_str(), "u16");
        assert_eq!(PrimitiveType::UInt(IntegerType::Bits32).as_str(), "u32");
        assert_eq!(PrimitiveType::UInt(IntegerType::Bits64).as_str(), "u64");
        assert_eq!(PrimitiveType::UInt(IntegerType::Bits128).as_str(), "u128");
        assert_eq!(PrimitiveType::UInt(IntegerType::Size).as_str(), "usize");
    }

    #[test]
    fn canonical_float_names_are_stable() {
        assert_eq!(FloatType::Bits32.as_str(), "f32");
        assert_eq!(FloatType::Bits64.as_str(), "f64");
        assert_eq!(FloatType::Bits128.as_str(), "f128");
    }

    #[test]
    fn primitive_names_round_trip() {
        let names = [
            "unit", "bool", "i8", "i16", "i32", "i64", "i128", "isize", "u8",
            "u16", "u32", "u64", "u128", "usize", "f32", "f64", "f128", "char",
            "str", "String",
        ];

        for name in names {
            let primitive = PrimitiveType::from_name(name)
                .expect("every canonical primitive name must parse");

            assert_eq!(primitive.as_str(), name);
        }
    }

    #[test]
    fn unknown_names_are_not_primitives() {
        assert_eq!(PrimitiveType::from_name("Vec"), None);
        assert_eq!(PrimitiveType::from_name("Qubit"), None);
        assert_eq!(PrimitiveType::from_name("Quantum"), None);
        assert_eq!(PrimitiveType::from_name("MyType"), None);
    }

    #[test]
    fn integer_width_never_uses_host_word_size() {
        assert_eq!(IntegerType::Bits8.fixed_bits(), Some(8));
        assert_eq!(IntegerType::Bits16.fixed_bits(), Some(16));
        assert_eq!(IntegerType::Bits32.fixed_bits(), Some(32));
        assert_eq!(IntegerType::Bits64.fixed_bits(), Some(64));
        assert_eq!(IntegerType::Bits128.fixed_bits(), Some(128));

        assert_eq!(IntegerType::Size.fixed_bits(), None);
        assert!(IntegerType::Size.is_target_dependent());
    }

    #[test]
    fn classification_is_correct() {
        assert!(PrimitiveType::Int(IntegerType::Bits64).is_numeric());
        assert!(PrimitiveType::UInt(IntegerType::Bits64).is_numeric());
        assert!(PrimitiveType::Float(FloatType::Bits64).is_numeric());

        assert!(PrimitiveType::Int(IntegerType::Bits64).is_integer());
        assert!(PrimitiveType::UInt(IntegerType::Bits64).is_integer());
        assert!(!PrimitiveType::Float(FloatType::Bits64).is_integer());

        assert!(PrimitiveType::Int(IntegerType::Bits64).is_signed());
        assert!(!PrimitiveType::UInt(IntegerType::Bits64).is_signed());

        assert!(PrimitiveType::UInt(IntegerType::Bits64).is_unsigned());
        assert!(!PrimitiveType::Int(IntegerType::Bits64).is_unsigned());

        assert!(PrimitiveType::Float(FloatType::Bits64).is_float());
    }

    #[test]
    fn display_is_canonical() {
        assert_eq!(
            PrimitiveType::Int(IntegerType::Bits64).to_string(),
            "i64"
        );
        assert_eq!(
            PrimitiveType::UInt(IntegerType::Bits64).to_string(),
            "u64"
        );
        assert_eq!(
            PrimitiveType::Float(FloatType::Bits64).to_string(),
            "f64"
        );
        assert_eq!(PrimitiveType::Bool.to_string(), "bool");
    }

    #[test]
    fn width_conversion_is_lossless() {
        let widths = [
            IntegerType::Bits8,
            IntegerType::Bits16,
            IntegerType::Bits32,
            IntegerType::Bits64,
            IntegerType::Bits128,
            IntegerType::Size,
        ];

        for width in widths {
            assert_eq!(width.signed().integer_width(), Some(width));
            assert_eq!(width.unsigned().integer_width(), Some(width));
        }
    }

    #[test]
    fn floating_width_conversion_is_lossless() {
        let widths = [
            FloatType::Bits32,
            FloatType::Bits64,
            FloatType::Bits128,
        ];

        for width in widths {
            assert_eq!(width.primitive().float_width(), Some(width));
        }
    }
}