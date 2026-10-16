
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical scalar type descriptors.
//!
//! Path: src/classical/types/scalar.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Describes scalar type properties at the classical subsystem boundary.
//! This module does not implement arithmetic, store scalar values, parse
//! source syntax, resolve names, or define a competing AST or IR.
//!
//! # Ownership
//!
//! This module owns:
//! - ScalarKind, ScalarWidth, and ScalarTypeDescriptor.
//! - Descriptor invariants and local validation errors.
//! - Queries for broad scalar categories.
//!
//! It does not own:
//! - Source-level type expressions or their resolution.
//! - Canonical classical values or numeric arithmetic.
//! - Integer and floating-point value representations.
//! - Generic constraints, aggregate types, or memory layouts.
//! - Hardware discovery, target lowering, or runtime execution.
//!
//! # POCO-REAF
//!
//! No universal hardware width, device count, or scalar-count ceiling is
//! imposed here. A fixed width describes a requested semantic property;
//! it does not assert that every target can realize that property.
//!
//! Symbolic widths remain symbolic. This module does not resolve them,
//! evaluate expressions, or silently substitute a host-machine width.
//!
//! Rust's usize is used only for an explicitly fixed, host-representable
//! width. A symbolic width is not converted to usize.
//!
//! # Integration
//!
//! - types/mod.rs must declare `pub mod scalar;`.
//! - classical/mod.rs must expose the types module.
//! - Frontend semantic analysis remains responsible for mapping canonical
//!   source type expressions to validated descriptors.
//! - Canonical classical IR modules remain the authorities for values and
//!   numeric representations.
//! - Generic constraints and layout decisions remain with their own modules.
//! - A target backend must check actual capability and representation
//!   constraints before lowering a descriptor.
//!
//! # Compatibility
//!
//! The descriptor types are a new classical-facing API. They must not be
//! treated as serialized ABI or persistent type identities without a
//! separately specified versioning contract.

use core::fmt;
use core::num::NonZeroUsize;

/// Broad semantic category of a scalar.
///
/// `Extension` allows an independently specified dialect or language
/// extension to introduce a category without changing this enum for every
/// new extension. The extension name is an identifier, not executable code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScalarKind {
    /// Logical truth value.
    Boolean,

    /// One logical classical bit.
    Bit,

    /// Signed integer.
    SignedInteger,

    /// Unsigned integer.
    UnsignedInteger,

    /// Binary floating-point number.
    BinaryFloat,

    /// Decimal numeric type.
    Decimal,

    /// Exact rational numeric type.
    Rational,

    /// Complex numeric type.
    Complex,

    /// Character scalar.
    Character,

    /// Extension-defined scalar category.
    Extension(String),
}

impl ScalarKind {
    /// Returns whether the category represents an integer.
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::SignedInteger | Self::UnsignedInteger)
    }

    /// Returns whether the category is conventionally numeric.
    ///
    /// This is category metadata, not proof that every operation is valid.
    /// Operation legality remains the responsibility of semantic analysis.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::SignedInteger
                | Self::UnsignedInteger
                | Self::BinaryFloat
                | Self::Decimal
                | Self::Rational
                | Self::Complex
        )
    }

    /// Returns whether the category is defined by a bit width.
    ///
    /// For built-in categories, width is meaningful for bits, integers,
    /// and binary floating-point formats. Extension-defined categories
    /// may also declare widths according to their own contracts.
    #[must_use]
    pub const fn supports_width(&self) -> bool {
        matches!(
            self,
            Self::Bit
                | Self::SignedInteger
                | Self::UnsignedInteger
                | Self::BinaryFloat
                | Self::Extension(_)
        )
    }

    /// Returns the stable built-in category name.
    ///
    /// Extension names are returned by [`ScalarTypeDescriptor::kind_name`].
    #[must_use]
    pub const fn built_in_name(&self) -> Option<&'static str> {
        match self {
            Self::Boolean => Some("bool"),
            Self::Bit => Some("bit"),
            Self::SignedInteger => Some("signed_integer"),
            Self::UnsignedInteger => Some("unsigned_integer"),
            Self::BinaryFloat => Some("binary_float"),
            Self::Decimal => Some("decimal"),
            Self::Rational => Some("rational"),
            Self::Complex => Some("complex"),
            Self::Character => Some("character"),
            Self::Extension(_) => None,
        }
    }
}

/// Width specification for a scalar type.
///
/// `Fixed` describes an explicitly known, positive width representable by
/// this Rust host. `Symbolic` preserves an unresolved width expression as
/// supplied by the owning semantic layer. `Arbitrary` means the descriptor
/// does not prescribe a fixed semantic bit width.
///
/// `Unspecified` is distinct from `Arbitrary`: the former means the type
/// declaration has not supplied width information; the latter is an
/// intentional declaration that this descriptor does not prescribe one.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScalarWidth {
    /// No width information has been supplied.
    Unspecified,

    /// A positive, host-representable width.
    Fixed(NonZeroUsize),

    /// Width is not prescribed as one fixed number.
    Arbitrary,

    /// A symbolic width expression awaiting semantic resolution.
    ///
    /// The string is opaque to this module. The canonical semantic analyzer
    /// is responsible for parsing, validating, and evaluating the expression.
    Symbolic(String),
}

impl Default for ScalarWidth {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl ScalarWidth {
    /// Creates a fixed width, rejecting zero.
    ///
    /// This constructor does not impose an architectural width ceiling.
    /// The host's `usize` representation is the only limit of this API.
    pub fn fixed(width: usize) -> Result<Self, ScalarTypeError> {
        NonZeroUsize::new(width)
            .map(Self::Fixed)
            .ok_or(ScalarTypeError::ZeroWidth)
    }

    /// Creates a symbolic width from a non-empty expression.
    ///
    /// This validates only that an expression was supplied. It does not
    /// claim that the expression is syntactically or semantically valid.
    pub fn symbolic(expression: impl Into<String>) -> Result<Self, ScalarTypeError> {
        let expression = expression.into();

        if expression.trim().is_empty() {
            return Err(ScalarTypeError::EmptySymbolicWidth);
        }

        Ok(Self::Symbolic(expression))
    }

    /// Returns the fixed width, if one is explicitly available.
    #[must_use]
    pub const fn as_fixed(&self) -> Option<usize> {
        match self {
            Self::Fixed(width) => Some(width.get()),
            Self::Unspecified | Self::Arbitrary | Self::Symbolic(_) => None,
        }
    }

    /// Returns whether the width is awaiting semantic resolution.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self, Self::Symbolic(_))
    }
}

/// Validated classical scalar type descriptor.
///
/// This is descriptive metadata, not a replacement for the canonical
/// frontend semantic type or canonical IR type. Its fields are private so
/// callers cannot construct a descriptor that bypasses local validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScalarTypeDescriptor {
    kind: ScalarKind,
    width: ScalarWidth,
}

impl ScalarTypeDescriptor {
    /// Constructs and validates a scalar type descriptor.
    ///
    /// Width applicability is checked for built-in categories. Extension
    /// categories may define their own width interpretation.
    ///
    /// # Errors
    ///
    /// Returns an error when the descriptor contradicts the built-in
    /// category's width contract or contains an invalid extension name.
    pub fn new(kind: ScalarKind, width: ScalarWidth) -> Result<Self, ScalarTypeError> {
        if let ScalarKind::Extension(name) = &kind {
            if name.trim().is_empty() {
                return Err(ScalarTypeError::EmptyExtensionName);
            }
        }

        match (&kind, &width) {
            (ScalarKind::Boolean | ScalarKind::Character, ScalarWidth::Fixed(_))
            | (ScalarKind::Boolean | ScalarKind::Character, ScalarWidth::Symbolic(_))
            | (ScalarKind::Boolean | ScalarKind::Character, ScalarWidth::Arbitrary)
            | (ScalarKind::Decimal | ScalarKind::Rational | ScalarKind::Complex,
               ScalarWidth::Fixed(_))
            | (ScalarKind::Decimal | ScalarKind::Rational | ScalarKind::Complex,
               ScalarWidth::Symbolic(_))
            | (ScalarKind::Decimal | ScalarKind::Rational | ScalarKind::Complex,
               ScalarWidth::Arbitrary) => {
                return Err(ScalarTypeError::WidthNotApplicable {
                    kind: kind.clone(),
                });
            }
            (ScalarKind::Bit, ScalarWidth::Fixed(width)) if width.get() != 1 => {
                return Err(ScalarTypeError::BitWidthMustBeOne {
                    actual: width.get(),
                });
            }
            _ => {}
        }

        if let ScalarWidth::Symbolic(expression) = &width {
            if expression.trim().is_empty() {
                return Err(ScalarTypeError::EmptySymbolicWidth);
            }
        }

        Ok(Self { kind, width })
    }

    /// Constructs a scalar descriptor without an explicit width.
    pub fn unspecified(kind: ScalarKind) -> Result<Self, ScalarTypeError> {
        Self::new(kind, ScalarWidth::Unspecified)
    }

    /// Constructs an integer descriptor with an explicitly fixed width.
    pub fn fixed_integer(
        signed: bool,
        width: usize,
    ) -> Result<Self, ScalarTypeError> {
        let kind = if signed {
            ScalarKind::SignedInteger
        } else {
            ScalarKind::UnsignedInteger
        };

        Self::new(kind, ScalarWidth::fixed(width)?)
    }

    /// Constructs a binary floating-point descriptor with a fixed width.
    ///
    /// The width is metadata only. This function does not assert that the
    /// runtime or any target supports that floating-point format.
    pub fn fixed_binary_float(width: usize) -> Result<Self, ScalarTypeError> {
        Self::new(
            ScalarKind::BinaryFloat,
            ScalarWidth::fixed(width)?,
        )
    }

    /// Constructs a scalar descriptor with an unresolved symbolic width.
    pub fn symbolic_width(
        kind: ScalarKind,
        expression: impl Into<String>,
    ) -> Result<Self, ScalarTypeError> {
        Self::new(kind, ScalarWidth::symbolic(expression)?)
    }

    /// Returns the scalar category.
    #[must_use]
    pub const fn kind(&self) -> &ScalarKind {
        &self.kind
    }

    /// Returns the width specification.
    #[must_use]
    pub const fn width(&self) -> &ScalarWidth {
        &self.width
    }

    /// Returns whether the category is numeric.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        self.kind.is_numeric()
    }

    /// Returns whether the category is an integer.
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        self.kind.is_integer()
    }

    /// Returns the category name.
    ///
    /// Built-in names are stable labels. Extension names are supplied by
    /// the extension author and are not normalized by this module.
    #[must_use]
    pub fn kind_name(&self) -> &str {
        match &self.kind {
            ScalarKind::Extension(name) => name.as_str(),
            _ => self.kind.built_in_name().unwrap_or("unknown"),
        }
    }

    /// Revalidates the descriptor's local invariants.
    ///
    /// This is useful at trust boundaries after external deserialization
    /// or conversion from another representation. It does not replace
    /// canonical semantic type checking.
    pub fn validate(&self) -> Result<(), ScalarTypeError> {
        Self::new(self.kind.clone(), self.width.clone()).map(|_| ())
    }
}

/// Errors raised while constructing or validating scalar descriptors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarTypeError {
    /// Fixed width zero is invalid.
    ZeroWidth,

    /// A symbolic width expression was empty.
    EmptySymbolicWidth,

    /// An extension category was given an empty name.
    EmptyExtensionName,

    /// The selected category does not accept the supplied width form.
    WidthNotApplicable {
        /// Category that rejected the width.
        kind: ScalarKind,
    },

    /// A logical bit was declared with a fixed width other than one.
    BitWidthMustBeOne {
        /// Supplied width.
        actual: usize,
    },
}

impl fmt::Display for ScalarTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => f.write_str("scalar width must be greater than zero"),
            Self::EmptySymbolicWidth => {
                f.write_str("symbolic scalar width expression must not be empty")
            }
            Self::EmptyExtensionName => {
                f.write_str("scalar extension category name must not be empty")
            }
            Self::WidthNotApplicable { kind } => {
                write!(f, "width is not applicable to scalar category {kind:?}")
            }
            Self::BitWidthMustBeOne { actual } => {
                write!(f, "a logical bit must have width 1, not {actual}")
            }
        }
    }
}

impl std::error::Error for ScalarTypeError {}

#[cfg(test)]
mod tests {
    use super::{ScalarKind, ScalarTypeDescriptor, ScalarTypeError, ScalarWidth};

    #[test]
    fn fixed_integer_width_is_explicit() {
        let ty = ScalarTypeDescriptor::fixed_integer(true, 128)
            .expect("positive fixed width should be valid");

        assert!(ty.is_integer());
        assert!(ty.is_numeric());
        assert_eq!(ty.width().as_fixed(), Some(128));
    }

    #[test]
    fn zero_width_is_rejected() {
        assert_eq!(
            ScalarTypeDescriptor::fixed_integer(false, 0),
            Err(ScalarTypeError::ZeroWidth)
        );
    }

    #[test]
    fn symbolic_width_is_preserved_without_evaluation() {
        let ty = ScalarTypeDescriptor::symbolic_width(
            ScalarKind::UnsignedInteger,
            "required_bits",
        )
        .expect("non-empty symbolic width should be preserved");

        assert!(ty.width().is_symbolic());
        assert_eq!(
            ty.width(),
            &ScalarWidth::Symbolic("required_bits".to_owned())
        );
    }

    #[test]
    fn arbitrary_width_does_not_claim_infinite_resources() {
        let ty = ScalarTypeDescriptor::new(
            ScalarKind::SignedInteger,
            ScalarWidth::Arbitrary,
        )
        .expect("arbitrary width is valid metadata");

        assert_eq!(ty.width(), &ScalarWidth::Arbitrary);
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn bit_width_must_be_one_when_fixed() {
        assert_eq!(
            ScalarTypeDescriptor::new(
                ScalarKind::Bit,
                ScalarWidth::fixed(8).expect("8 is a positive width"),
            ),
            Err(ScalarTypeError::BitWidthMustBeOne { actual: 8 })
        );
    }

    #[test]
    fn boolean_rejects_numeric_width_metadata() {
        assert!(matches!(
            ScalarTypeDescriptor::new(
                ScalarKind::Boolean,
                ScalarWidth::fixed(1).expect("positive width"),
            ),
            Err(ScalarTypeError::WidthNotApplicable {
                kind: ScalarKind::Boolean
            })
        ));
    }

    #[test]
    fn extension_categories_are_open_ended() {
        let ty = ScalarTypeDescriptor::unspecified(
            ScalarKind::Extension("dialect.example.scalar".to_owned()),
        )
        .expect("a named extension category should be accepted");

        assert_eq!(ty.kind_name(), "dialect.example.scalar");
    }

    #[test]
    fn empty_extension_names_are_rejected() {
        assert_eq!(
            ScalarTypeDescriptor::unspecified(
                ScalarKind::Extension("  ".to_owned()),
            ),
            Err(ScalarTypeError::EmptyExtensionName)
        );
    }

    #[test]
    fn descriptor_equality_is_deterministic() {
        let left = ScalarTypeDescriptor::fixed_integer(false, 256)
            .expect("positive width");
        let right = ScalarTypeDescriptor::fixed_integer(false, 256)
            .expect("positive width");

        assert_eq!(left, right);
    }

    #[test]
    fn binary_float_width_is_metadata_not_target_support() {
        let ty = ScalarTypeDescriptor::fixed_binary_float(128)
            .expect("positive format width");

        assert_eq!(ty.kind(), &ScalarKind::BinaryFloat);
        assert_eq!(ty.width().as_fixed(), Some(128));
    }
}
