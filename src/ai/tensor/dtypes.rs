
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Data Types
//!
//! Path: `src/ai/tensor/dtypes.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Define target-independent AI tensor element data types and their
//! mathematical and representation contracts.
//!
//! This module is independent of tensor storage, hardware discovery,
//! allocation, scheduling, execution, and backend implementation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI tensor element type descriptors;
//! - numerical semantic categories;
//! - integer, floating-point, complex, and boolean descriptors;
//! - quantized numerical type descriptions;
//! - custom extensible type descriptors;
//! - representation metadata;
//! - checked representation-size calculations;
//! - type compatibility classification.
//!
//! ## Does not own
//!
//! - the canonical Zamani semantic type system;
//! - source-language type syntax;
//! - tensor shapes or dimensions;
//! - tensor storage;
//! - numerical kernels;
//! - quantization algorithms;
//! - hardware capabilities;
//! - device-specific representations;
//! - allocation or execution.
//!
//! ## Integration
//!
//! Source syntax:
//! `grammar/types/`
//!
//! Tensor grammar:
//! `grammar/ai/tensors.g4`
//!
//! AI semantic authority:
//! `frontend::semantic::domains::ai`
//!
//! Tensor shapes:
//! `crate::ai::tensor::shapes`
//!
//! AI error contract:
//! `crate::ai::error`
//!
//! Canonical tensor storage:
//! `crate::classical::collections::tensor`
//!
//! Universal tensor IR:
//! `crate::zuir::dialects::tensor`
//!
//! Hardware-specific representation selection remains downstream.
//!
//! ## Scalability
//!
//! No maximum tensor rank, tensor count, parameter count, model size,
//! accelerator count, memory size, or hardware capacity is introduced.
//!
//! Representation widths are described using `u128`.
//!
//! This is a descriptor representation, not a promise that every possible
//! mathematical width can be allocated or executed by every target.
//!
//! Actual execution must establish that the requested representation is
//! supported and that the required resources are available.
//!
//! ## Safety
//!
//! Safe Rust only.

use core::fmt;

// ============================================================================
// NUMERICAL SEMANTICS
// ============================================================================

/// The mathematical or computational category of an element type.
///
/// This is semantic information, not a target-specific storage format.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum NumericCategory {
    /// Boolean truth values.
    Boolean,

    /// Signed integer values.
    SignedInteger,

    /// Unsigned integer values.
    UnsignedInteger,

    /// Real floating-point values.
    RealFloatingPoint,

    /// Complex floating-point values.
    ComplexFloatingPoint,

    /// Quantized integer values.
    QuantizedInteger,

    /// Quantized floating-point values.
    QuantizedFloatingPoint,

    /// A custom numerical category supplied by an extension.
    Custom,
}

// ============================================================================
// BYTE ORDER
// ============================================================================

/// Byte order for a concrete multi-byte representation.
///
/// Byte order does not change the mathematical meaning of a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ByteOrder {
    /// Least significant byte first.
    LittleEndian,

    /// Most significant byte first.
    BigEndian,

    /// Byte order is not applicable or is not specified.
    NotApplicable,
}

// ============================================================================
// REPRESENTATION
// ============================================================================

/// Describes the representation properties of a tensor element type.
///
/// This descriptor does not claim that a particular machine supports
/// the representation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Representation {
    /// Number of bits occupied by the representation.
    ///
    /// This describes the representation, not an allocation size.
    pub bit_width: u128,

    /// Required alignment in bits, when specified.
    ///
    /// `None` means that alignment is determined by the eventual
    /// representation provider.
    pub alignment_bits: Option<u128>,

    /// Byte order for the representation.
    pub byte_order: ByteOrder,

    /// Whether the representation has a fixed width.
    pub fixed_width: bool,
}

impl Representation {
    /// Creates a fixed-width representation.
    ///
    /// The width must be nonzero.
    pub fn fixed(
        bit_width: u128,
        alignment_bits: Option<u128>,
        byte_order: ByteOrder,
    ) -> Result<Self, DTypeError> {
        if bit_width == 0 {
            return Err(DTypeError::ZeroBitWidth);
        }

        if let Some(alignment) = alignment_bits {
            if alignment == 0 {
                return Err(DTypeError::ZeroAlignment);
            }
        }

        Ok(Self {
            bit_width,
            alignment_bits,
            byte_order,
            fixed_width: true,
        })
    }

    /// Returns the representation width in bits.
    #[must_use]
    pub const fn bit_width(&self) -> u128 {
        self.bit_width
    }

    /// Returns the representation width in bytes if it is byte-aligned.
    ///
    /// This method does not round up.
    #[must_use]
    pub const fn byte_width(&self) -> Option<u128> {
        if self.bit_width % 8 == 0 {
            Some(self.bit_width / 8)
        } else {
            None
        }
    }

    /// Returns the number of bytes required to contain this representation.
    ///
    /// This computes the minimum number of bytes needed to hold the
    /// representation. It does not include padding or alignment.
    #[must_use]
    pub const fn minimum_storage_bytes(&self) -> Option<u128> {
        self.bit_width.checked_add(7).map(|bits| bits / 8)
    }
}

// ============================================================================
// INTEGER SEMANTICS
// ============================================================================

/// Signedness of an integer representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerSignedness {
    /// Signed integer.
    Signed,

    /// Unsigned integer.
    Unsigned,
}

/// Integer numerical semantics.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IntegerSemantics {
    /// Signedness.
    pub signedness: IntegerSignedness,

    /// Mathematical width of the integer value.
    ///
    /// A target may use a different physical representation if the
    /// conversion preserves the declared semantics.
    pub value_bits: u128,
}

impl IntegerSemantics {
    /// Creates integer semantics.
    pub fn new(
        signedness: IntegerSignedness,
        value_bits: u128,
    ) -> Result<Self, DTypeError> {
        if value_bits == 0 {
            return Err(DTypeError::ZeroBitWidth);
        }

        Ok(Self {
            signedness,
            value_bits,
        })
    }
}

// ============================================================================
// FLOATING-POINT SEMANTICS
// ============================================================================

/// Floating-point encoding family.
///
/// The descriptor identifies a format family without assuming that
/// every target has a native instruction for it.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum FloatEncoding {
    /// IEEE-style binary encoding.
    Binary,

    /// IEEE-style decimal encoding.
    Decimal,

    /// BFloat-style encoding.
    BrainFloatingPoint,

    /// A custom floating-point encoding.
    Custom(String),
}

/// Floating-point semantic descriptor.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FloatSemantics {
    /// Encoding family.
    pub encoding: FloatEncoding,

    /// Exponent field width.
    pub exponent_bits: u128,

    /// Fraction field width.
    pub fraction_bits: u128,

    /// Whether the format has a sign bit.
    pub has_sign_bit: bool,

    /// Whether the format supports infinity.
    pub supports_infinity: bool,

    /// Whether the format supports NaN.
    pub supports_nan: bool,
}

impl FloatSemantics {
    /// Creates floating-point semantics.
    pub fn new(
        encoding: FloatEncoding,
        exponent_bits: u128,
        fraction_bits: u128,
        has_sign_bit: bool,
        supports_infinity: bool,
        supports_nan: bool,
    ) -> Result<Self, DTypeError> {
        if exponent_bits == 0 {
            return Err(DTypeError::ZeroExponentWidth);
        }

        if fraction_bits == 0 {
            return Err(DTypeError::ZeroFractionWidth);
        }

        Ok(Self {
            encoding,
            exponent_bits,
            fraction_bits,
            has_sign_bit,
            supports_infinity,
            supports_nan,
        })
    }

    /// Returns the mathematical encoding width.
    ///
    /// Returns `None` if the width cannot be represented by `u128`.
    #[must_use]
    pub const fn bit_width(&self) -> Option<u128> {
        let sign_width = if self.has_sign_bit { 1 } else { 0 };

        match self.exponent_bits.checked_add(self.fraction_bits) {
            Some(width) => width.checked_add(sign_width),
            None => None,
        }
    }
}

// ============================================================================
// COMPLEX SEMANTICS
// ============================================================================

/// Complex numerical semantics.
///
/// Complex values contain real and imaginary components with the
/// same component semantics.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ComplexSemantics {
    /// Component floating-point semantics.
    pub component: FloatSemantics,
}

impl ComplexSemantics {
    /// Returns the total representation width.
    #[must_use]
    pub const fn bit_width(&self) -> Option<u128> {
        match self.component.bit_width() {
            Some(width) => width.checked_mul(2),
            None => None,
        }
    }
}

// ============================================================================
// QUANTIZATION
// ============================================================================

/// Quantization scale semantics.
///
/// This describes a quantization contract. It does not perform
/// quantization or choose a target representation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum QuantizationScale {
    /// A single scale applies to all values.
    PerTensor,

    /// A scale applies independently to values along an axis.
    PerAxis {
        /// Axis identity is semantic metadata.
        axis: String,
    },

    /// A custom scale policy.
    Custom {
        /// Stable extension identifier.
        extension: String,
    },
}

/// Quantization zero-point semantics.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum QuantizationZeroPoint {
    /// The quantized representation is symmetric around zero.
    Symmetric,

    /// The quantized representation uses an explicit zero point.
    Explicit,

    /// A custom zero-point policy.
    Custom {
        /// Stable extension identifier.
        extension: String,
    },
}

/// Quantized numerical semantics.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QuantizedSemantics {
    /// Underlying integer signedness.
    pub signedness: IntegerSignedness,

    /// Number of quantized value bits.
    pub value_bits: u128,

    /// Scale policy.
    pub scale: QuantizationScale,

    /// Zero-point policy.
    pub zero_point: QuantizationZeroPoint,
}

impl QuantizedSemantics {
    /// Creates quantized numerical semantics.
    pub fn new(
        signedness: IntegerSignedness,
        value_bits: u128,
        scale: QuantizationScale,
        zero_point: QuantizationZeroPoint,
    ) -> Result<Self, DTypeError> {
        if value_bits == 0 {
            return Err(DTypeError::ZeroBitWidth);
        }

        Ok(Self {
            signedness,
            value_bits,
            scale,
            zero_point,
        })
    }
}

// ============================================================================
// CUSTOM TYPES
// ============================================================================

/// Extension-defined tensor element type.
///
/// Extension identifiers are stable names owned by the registering
/// extension. This module does not resolve or execute extensions.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CustomDType {
    /// Stable extension namespace.
    pub namespace: String,

    /// Stable extension-defined type name.
    pub name: String,

    /// Numerical category.
    pub category: NumericCategory,

    /// Optional representation descriptor.
    ///
    /// `None` means the representation is determined by the extension
    /// or the selected execution provider.
    pub representation: Option<Representation>,
}

impl CustomDType {
    /// Creates a custom type descriptor.
    pub fn new(
        namespace: String,
        name: String,
        category: NumericCategory,
        representation: Option<Representation>,
    ) -> Result<Self, DTypeError> {
        if namespace.trim().is_empty() {
            return Err(DTypeError::EmptyCustomNamespace);
        }

        if name.trim().is_empty() {
            return Err(DTypeError::EmptyCustomName);
        }

        Ok(Self {
            namespace,
            name,
            category,
            representation,
        })
    }
}

// ============================================================================
// DTYPE
// ============================================================================

/// Target-independent AI tensor element type.
///
/// This type describes numerical meaning and representation requirements.
/// It does not guarantee that any particular target supports the type.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum DType {
    /// Boolean value.
    Bool,

    /// Integer value.
    Integer(IntegerSemantics),

    /// Floating-point value.
    Float(FloatSemantics),

    /// Complex value.
    Complex(ComplexSemantics),

    /// Quantized numerical value.
    Quantized(QuantizedSemantics),

    /// Extension-defined type.
    Custom(CustomDType),
}

impl DType {
    /// Returns the numerical category.
    #[must_use]
    pub const fn category(&self) -> NumericCategory {
        match self {
            Self::Bool => NumericCategory::Boolean,

            Self::Integer(IntegerSemantics {
                signedness: IntegerSignedness::Signed,
                ..
            }) => NumericCategory::SignedInteger,

            Self::Integer(IntegerSemantics {
                signedness: IntegerSignedness::Unsigned,
                ..
            }) => NumericCategory::UnsignedInteger,

            Self::Float(_) => NumericCategory::RealFloatingPoint,

            Self::Complex(_) => NumericCategory::ComplexFloatingPoint,

            Self::Quantized(QuantizedSemantics {
                signedness: IntegerSignedness::Signed,
                ..
            }) => NumericCategory::QuantizedInteger,

            Self::Quantized(QuantizedSemantics {
                signedness: IntegerSignedness::Unsigned,
                ..
            }) => NumericCategory::QuantizedInteger,

            Self::Custom(custom) => custom.category,
        }
    }

    /// Returns the fixed representation width if it is known.
    ///
    /// Custom types without a representation return `None`.
    ///
    /// This is a semantic representation query, not a target support query.
    #[must_use]
    pub const fn bit_width(&self) -> Option<u128> {
        match self {
            Self::Bool => Some(1),

            Self::Integer(integer) => Some(integer.value_bits),

            Self::Float(float) => float.bit_width(),

            Self::Complex(complex) => complex.bit_width(),

            Self::Quantized(quantized) => Some(quantized.value_bits),

            Self::Custom(custom) => match &custom.representation {
                Some(representation) => Some(representation.bit_width),
                None => None,
            },
        }
    }

    /// Returns the minimum byte count for the representation, if known.
    ///
    /// This rounds up to contain a partial byte.
    #[must_use]
    pub const fn minimum_storage_bytes(&self) -> Option<u128> {
        match self.bit_width() {
            Some(bits) => bits.checked_add(7).map(|value| value / 8),
            None => None,
        }
    }

    /// Returns whether the type is an integer type.
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns whether the type is a floating-point type.
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns whether the type is a complex type.
    #[must_use]
    pub const fn is_complex(&self) -> bool {
        matches!(self, Self::Complex(_))
    }

    /// Returns whether the type is quantized.
    #[must_use]
    pub const fn is_quantized(&self) -> bool {
        matches!(self, Self::Quantized(_))
    }

    /// Returns whether the type is a custom extension type.
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

// ============================================================================
// BUILT-IN TYPE CONSTRUCTORS
// ============================================================================

/// Creates a signed integer type.
pub fn signed_integer(bits: u128) -> Result<DType, DTypeError> {
    Ok(DType::Integer(IntegerSemantics::new(
        IntegerSignedness::Signed,
        bits,
    )?))
}

/// Creates an unsigned integer type.
pub fn unsigned_integer(bits: u128) -> Result<DType, DTypeError> {
    Ok(DType::Integer(IntegerSemantics::new(
        IntegerSignedness::Unsigned,
        bits,
    )?))
}

/// Creates a binary floating-point type.
pub fn binary_float(
    exponent_bits: u128,
    fraction_bits: u128,
) -> Result<DType, DTypeError> {
    Ok(DType::Float(FloatSemantics::new(
        FloatEncoding::Binary,
        exponent_bits,
        fraction_bits,
        true,
        true,
        true,
    )?))
}

/// Creates a brain floating-point type.
pub fn brain_float(
    exponent_bits: u128,
    fraction_bits: u128,
) -> Result<DType, DTypeError> {
    Ok(DType::Float(FloatSemantics::new(
        FloatEncoding::BrainFloatingPoint,
        exponent_bits,
        fraction_bits,
        true,
        true,
        true,
    )?))
}

/// Creates a complex floating-point type.
pub fn complex_float(
    component: FloatSemantics,
) -> DType {
    DType::Complex(ComplexSemantics { component })
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced while constructing or validating a dtype descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DTypeError {
    /// A representation width must not be zero.
    ZeroBitWidth,

    /// An alignment must not be zero.
    ZeroAlignment,

    /// A floating-point exponent field must not be zero.
    ZeroExponentWidth,

    /// A floating-point fraction field must not be zero.
    ZeroFractionWidth,

    /// A custom type namespace must not be empty.
    EmptyCustomNamespace,

    /// A custom type name must not be empty.
    EmptyCustomName,

    /// A representation width overflowed during calculation.
    WidthOverflow,
}

impl fmt::Display for DTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroBitWidth => {
                f.write_str("data type bit width must be nonzero")
            }

            Self::ZeroAlignment => {
                f.write_str("data type alignment must be nonzero")
            }

            Self::ZeroExponentWidth => {
                f.write_str("floating-point exponent width must be nonzero")
            }

            Self::ZeroFractionWidth => {
                f.write_str("floating-point fraction width must be nonzero")
            }

            Self::EmptyCustomNamespace => {
                f.write_str("custom data type namespace must not be empty")
            }

            Self::EmptyCustomName => {
                f.write_str("custom data type name must not be empty")
            }

            Self::WidthOverflow => {
                f.write_str("data type representation width overflow")
            }
        }
    }
}

impl std::error::Error for DTypeError {}

// ============================================================================
// COMPATIBILITY
// ============================================================================

/// Result of a dtype compatibility check.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DTypeCompatibility {
    /// Types have identical declared semantics.
    Identical,

    /// Types differ but a conversion may preserve the required semantics.
    Convertible,

    /// Types are incompatible under the declared contract.
    Incompatible,

    /// Compatibility depends on an extension or target-specific provider.
    RequiresProvider,
}

/// Determines compatibility between two dtype descriptors.
///
/// This is deliberately conservative. It does not silently infer a
/// precision-losing conversion or claim target support.
#[must_use]
pub fn compatibility(
    source: &DType,
    destination: &DType,
) -> DTypeCompatibility {
    if source == destination {
        return DTypeCompatibility::Identical;
    }

    match (source, destination) {
        (DType::Custom(_), _) | (_, DType::Custom(_)) => {
            DTypeCompatibility::RequiresProvider
        }

        (DType::Integer(left), DType::Integer(right))
            if left.signedness == right.signedness
                && left.value_bits <= right.value_bits =>
        {
            DTypeCompatibility::Convertible
        }

        (DType::Float(left), DType::Float(right))
            if left.encoding == right.encoding
                && left.exponent_bits <= right.exponent_bits
                && left.fraction_bits <= right.fraction_bits =>
        {
            DTypeCompatibility::Convertible
        }

        _ => DTypeCompatibility::Incompatible,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_width_is_preserved() {
        let dtype = signed_integer(257).expect("valid integer width");

        assert_eq!(dtype.bit_width(), Some(257));
        assert_eq!(dtype.minimum_storage_bytes(), Some(33));
    }

    #[test]
    fn zero_width_is_rejected() {
        assert_eq!(
            signed_integer(0),
            Err(DTypeError::ZeroBitWidth)
        );
    }

    #[test]
    fn floating_point_width_is_checked() {
        let dtype = binary_float(8, 23).expect("valid float");

        assert_eq!(dtype.bit_width(), Some(32));
    }

    #[test]
    fn complex_width_is_checked() {
        let component = FloatSemantics::new(
            FloatEncoding::Binary,
            8,
            23,
            true,
            true,
            true,
        )
        .expect("valid component");

        let dtype = complex_float(component);

        assert_eq!(dtype.bit_width(), Some(64));
    }

    #[test]
    fn custom_type_can_have_unknown_representation() {
        let dtype = DType::Custom(
            CustomDType::new(
                "example.extension".to_string(),
                "custom_number".to_string(),
                NumericCategory::Custom,
                None,
            )
            .expect("valid custom type"),
        );

        assert_eq!(dtype.bit_width(), None);
        assert_eq!(dtype.category(), NumericCategory::Custom);
    }

    #[test]
    fn identical_types_are_compatible() {
        let dtype = unsigned_integer(32).expect("valid integer");

        assert_eq!(
            compatibility(&dtype, &dtype),
            DTypeCompatibility::Identical
        );
    }

    #[test]
    fn narrower_same_signedness_integer_is_convertible() {
        let source = signed_integer(8).expect("valid source");
        let destination = signed_integer(32).expect("valid destination");

        assert_eq!(
            compatibility(&source, &destination),
            DTypeCompatibility::Convertible
        );
    }

    #[test]
    fn signedness_change_is_not_assumed_safe() {
        let source = signed_integer(32).expect("valid source");
        let destination = unsigned_integer(32).expect("valid destination");

        assert_eq!(
            compatibility(&source, &destination),
            DTypeCompatibility::Incompatible
        );
    }

    #[test]
    fn custom_type_requires_provider_compatibility() {
        let custom = DType::Custom(
            CustomDType::new(
                "example.extension".to_string(),
                "custom_number".to_string(),
                NumericCategory::Custom,
                None,
            )
            .expect("valid custom type"),
        );

        let standard = signed_integer(32).expect("valid integer");

        assert_eq!(
            compatibility(&custom, &standard),
            DTypeCompatibility::RequiresProvider
        );
    }

    #[test]
    fn representation_byte_width_is_checked() {
        let representation = Representation::fixed(
            12,
            Some(4),
            ByteOrder::LittleEndian,
        )
        .expect("valid representation");

        assert_eq!(representation.byte_width(), None);
        assert_eq!(
            representation.minimum_storage_bytes(),
            Some(2)
        );
    }
}
