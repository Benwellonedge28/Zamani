
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR integer type descriptor
//!
//! Path: `src/classical/zcir/types/integer.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide a validated classical-facing descriptor for integer semantic
//! types while preserving the canonical ZUIR type vocabulary.
//!
//! This module describes integer *types*. It does not store integer values,
//! implement arithmetic, evaluate symbolic widths, infer source types, or
//! decide how an integer is represented on a particular target.
//!
//! ## Authority
//!
//! - `crate::zuir::types::IntegerSignedness` owns signedness vocabulary.
//! - `crate::zuir::types::IntegerWidth` owns width vocabulary.
//! - `crate::zuir::types::SymbolicExpression` owns symbolic expressions.
//! - `crate::zuir::types::TypeKind` owns the canonical semantic type.
//! - The language semantic model owns the meaning of integer operations.
//! - Value and arithmetic subsystems own integer values and overflow rules.
//! - Layout and lowering subsystems own target-specific realization.
//!
//! ## Owns
//!
//! - `IntegerTypeDescriptor`, a validated adapter over canonical integer
//!   signedness and width.
//! - Validation of locally enforceable integer-type invariants.
//! - Structured errors for invalid integer descriptors.
//!
//! ## Does not own
//!
//! - A competing signedness or width enum.
//! - Integer values, arbitrary-precision arithmetic, or numeric conversions.
//! - Source parsing, name resolution, type inference, or constraint solving.
//! - Type identity allocation, interning, or canonical registries.
//! - Logical/physical layout, ABI, registers, or memory allocation.
//! - Backend capability discovery, instruction selection, or runtime policy.
//!
//! ## Scalability and POCO-REAF
//!
//! No arbitrary maximum integer width is imposed by this module.
//!
//! `IntegerWidth::Unbounded` retains mathematical integer semantics.
//! `IntegerWidth::Symbolic` retains the canonical symbolic expression.
//! `IntegerWidth::Fixed` preserves its canonical `u128` width without
//! converting it to the host's `usize`.
//!
//! A fixed width of zero is rejected as an invalid integer type. This is a
//! semantic invariant, not a machine-capacity ceiling.
//!
//! The canonical `Fixed(u128)` representation is itself finite. This module
//! does not claim that it can encode every conceivable fixed width. Widths
//! that cannot be represented by the canonical form must not be truncated;
//! they require a separately specified canonical representation or must
//! remain symbolic until the owning semantic system can resolve them.
//!
//! This module does not promise that every target can realize every valid
//! integer type. Target feasibility belongs to later verification/lowering.
//!
//! ## Integration contract
//!
//! - `src/classical/zcir/types/mod.rs` declares `pub mod integer;`.
//! - `crate::zuir::types` must expose the canonical types imported here.
//! - Frontend semantic analysis constructs canonical `TypeKind::Integer`
//!   values after resolving source-level type meaning.
//! - ZCIR consumers use this descriptor for integer-specific validation.
//! - `types/scalar.rs` may delegate integer validation to
//!   `validate_integer_type`; it must not duplicate different rules.
//! - `values/integer.rs` owns integer values and numeric operations.
//! - `types/layout.rs` or its canonical counterpart owns logical layout.
//! - Target lowering checks actual target capabilities and representation.
//! - No conversion to a separate classical integer enum is implicit.
//!
//! ## Trust boundaries
//!
//! Public canonical enums can be constructed directly and may be deserialized
//! by other modules. Call `validate()` or `validate_integer_type()` at trust
//! boundaries. Validation here checks only local invariants; it does not
//! establish whole-program type correctness or target feasibility.

use core::fmt;

use crate::zuir::types::{
    IntegerSignedness, IntegerWidth, SymbolicExpression, TypeKind,
};

/// A validated description of an integer semantic type.
///
/// This descriptor reuses ZUIR's canonical signedness and width types. It
/// does not allocate a separate type ID and is not a serialized type identity.
///
/// Use [`IntegerTypeDescriptor::new`] for direct construction or
/// [`IntegerTypeDescriptor::try_from_kind`] to extract an integer from a
/// canonical `TypeKind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerTypeDescriptor {
    signedness: IntegerSignedness,
    width: IntegerWidth,
}

impl IntegerTypeDescriptor {
    /// Construct and validate an integer descriptor.
    ///
    /// The width remains exactly as supplied: fixed, symbolic, or unbounded.
    ///
    /// # Errors
    ///
    /// Returns an error when the fixed width is zero or a symbolic width
    /// contains an empty expression.
    pub fn new(
        signedness: IntegerSignedness,
        width: IntegerWidth,
    ) -> Result<Self, IntegerTypeError> {
        validate_integer_width(&width)?;

        Ok(Self { signedness, width })
    }

    /// Extract an integer descriptor from the canonical ZUIR type.
    ///
    /// The descriptor clones the canonical signedness and width without
    /// narrowing, evaluating, or normalizing them.
    ///
    /// # Errors
    ///
    /// Returns `NotInteger` if `kind` is not an integer type.
    /// Returns a width validation error if its integer width is invalid.
    pub fn try_from_kind(kind: &TypeKind) -> Result<Self, IntegerTypeError> {
        match kind {
            TypeKind::Integer { signedness, width } => {
                Self::new(*signedness, width.clone())
            }
            _ => Err(IntegerTypeError::NotInteger),
        }
    }

    /// Return the canonical signedness.
    #[must_use]
    pub const fn signedness(&self) -> IntegerSignedness {
        self.signedness
    }

    /// Return the canonical width.
    ///
    /// This may be fixed, symbolic, or unbounded. It is deliberately not
    /// converted to `usize` or to a target register width.
    #[must_use]
    pub const fn width(&self) -> &IntegerWidth {
        &self.width
    }

    /// Return the fixed width if and only if the width is explicitly fixed.
    ///
    /// `None` means the width is symbolic or unbounded. It does not mean
    /// that the integer is zero-width or that a default width should be used.
    #[must_use]
    pub const fn fixed_width(&self) -> Option<u128> {
        match &self.width {
            IntegerWidth::Fixed(width) => Some(*width),
            IntegerWidth::Symbolic(_) | IntegerWidth::Unbounded => None,
        }
    }

    /// Return the symbolic width expression, if present.
    #[must_use]
    pub const fn symbolic_width(&self) -> Option<&SymbolicExpression> {
        match &self.width {
            IntegerWidth::Symbolic(expression) => Some(expression),
            IntegerWidth::Fixed(_) | IntegerWidth::Unbounded => None,
        }
    }

    /// Return whether the width has mathematical unbounded-integer semantics.
    ///
    /// This is a semantic classification, not a promise of infinite physical
    /// memory, finite execution time, or native backend support.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        matches!(self.width, IntegerWidth::Unbounded)
    }

    /// Return whether the width must be resolved by the owning semantic or
    /// constraint system before a stage requiring a concrete width can run.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self.width, IntegerWidth::Symbolic(_))
    }

    /// Revalidate the descriptor.
    ///
    /// Call this at trust boundaries when the descriptor may have originated
    /// from unchecked construction or deserialization in another layer.
    pub fn validate(&self) -> Result<(), IntegerTypeError> {
        validate_integer_width(&self.width)
    }

    /// Reconstruct the canonical ZUIR integer type.
    ///
    /// This operation is lossless. It does not create a new type identity.
    #[must_use]
    pub fn into_type_kind(self) -> TypeKind {
        TypeKind::Integer {
            signedness: self.signedness,
            width: self.width,
        }
    }

    /// Borrow the descriptor as a canonical ZUIR integer type.
    ///
    /// The returned value is owned because `TypeKind` is the canonical
    /// container for the integer variant.
    #[must_use]
    pub fn to_type_kind(&self) -> TypeKind {
        TypeKind::Integer {
            signedness: self.signedness,
            width: self.width.clone(),
        }
    }
}

/// Validate the local invariants of a canonical integer type.
///
/// Use this function from other ZCIR type adapters so that zero-width and
/// empty-symbolic-width checks have a single implementation.
///
/// This does not evaluate symbolic expressions or establish target support.
pub fn validate_integer_type(kind: &TypeKind) -> Result<(), IntegerTypeError> {
    match kind {
        TypeKind::Integer { width, .. } => validate_integer_width(width),
        _ => Err(IntegerTypeError::NotInteger),
    }
}

/// Validate a canonical integer width.
///
/// This function is public so adjacent type adapters can share the same
/// invariant checks without defining competing integer-width semantics.
pub fn validate_integer_width(
    width: &IntegerWidth,
) -> Result<(), IntegerTypeError> {
    match width {
        IntegerWidth::Unbounded => Ok(()),

        IntegerWidth::Fixed(0) => Err(IntegerTypeError::ZeroWidth),

        IntegerWidth::Fixed(_) => Ok(()),

        IntegerWidth::Symbolic(expression) => {
            validate_symbolic_width(expression)
        }
    }
}

fn validate_symbolic_width(
    expression: &SymbolicExpression,
) -> Result<(), IntegerTypeError> {
    if expression.expression.trim().is_empty() {
        return Err(IntegerTypeError::EmptySymbolicWidth);
    }

    // Parsing, name resolution, constraint solving, and evaluation belong
    // to their owning subsystems. A non-empty expression is not proof that
    // the expression is semantically valid.
    Ok(())
}

/// Errors raised by integer descriptor validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerTypeError {
    /// The supplied canonical type is not an integer.
    NotInteger,

    /// A fixed integer width of zero was supplied.
    ZeroWidth,

    /// A symbolic width contains no expression.
    EmptySymbolicWidth,
}

impl fmt::Display for IntegerTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInteger => {
                f.write_str("canonical type is not an integer type")
            }
            Self::ZeroWidth => {
                f.write_str("fixed integer width must be greater than zero")
            }
            Self::EmptySymbolicWidth => {
                f.write_str("symbolic integer width expression must not be empty")
            }
        }
    }
}

impl std::error::Error for IntegerTypeError {}

impl TryFrom<&TypeKind> for IntegerTypeDescriptor {
    type Error = IntegerTypeError;

    fn try_from(kind: &TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_positive_fixed_width_without_narrowing() {
        let width = u128::MAX;
        let descriptor = IntegerTypeDescriptor::new(
            IntegerSignedness::Unsigned,
            IntegerWidth::Fixed(width),
        )
        .expect("a positive canonical fixed width should be accepted");

        assert_eq!(descriptor.fixed_width(), Some(width));
        assert_eq!(descriptor.signedness(), IntegerSignedness::Unsigned);
        assert!(!descriptor.is_symbolic());
        assert!(!descriptor.is_unbounded());
    }

    #[test]
    fn rejects_zero_fixed_width() {
        let result = IntegerTypeDescriptor::new(
            IntegerSignedness::Signed,
            IntegerWidth::Fixed(0),
        );

        assert_eq!(result, Err(IntegerTypeError::ZeroWidth));
    }

    #[test]
    fn accepts_unbounded_integer_semantics() {
        let descriptor = IntegerTypeDescriptor::new(
            IntegerSignedness::Signless,
            IntegerWidth::Unbounded,
        )
        .expect("unbounded mathematical integer semantics should be valid");

        assert!(descriptor.is_unbounded());
        assert_eq!(descriptor.fixed_width(), None);
        assert_eq!(descriptor.symbolic_width(), None);
    }

    #[test]
    fn preserves_symbolic_width_without_evaluating_it() {
        let expression = SymbolicExpression {
            expression: "N + OFFSET".to_owned(),
        };

        let descriptor = IntegerTypeDescriptor::new(
            IntegerSignedness::Unsigned,
            IntegerWidth::Symbolic(expression.clone()),
        )
        .expect("a non-empty symbolic width should be preserved");

        assert!(descriptor.is_symbolic());
        assert_eq!(descriptor.fixed_width(), None);
        assert_eq!(descriptor.symbolic_width(), Some(&expression));
    }

    #[test]
    fn rejects_empty_symbolic_width() {
        let result = IntegerTypeDescriptor::new(
            IntegerSignedness::Unsigned,
            IntegerWidth::Symbolic(SymbolicExpression {
                expression: " \t\n ".to_owned(),
            }),
        );

        assert_eq!(result, Err(IntegerTypeError::EmptySymbolicWidth));
    }

    #[test]
    fn extracts_integer_from_canonical_type_kind() {
        let kind = TypeKind::Integer {
            signedness: IntegerSignedness::Signed,
            width: IntegerWidth::Fixed(257),
        };

        let descriptor = IntegerTypeDescriptor::try_from_kind(&kind)
            .expect("canonical integer type should be accepted");

        assert_eq!(descriptor.signedness(), IntegerSignedness::Signed);
        assert_eq!(descriptor.fixed_width(), Some(257));
    }

    #[test]
    fn rejects_non_integer_canonical_types() {
        let result = IntegerTypeDescriptor::try_from_kind(&TypeKind::Boolean);

        assert_eq!(result, Err(IntegerTypeError::NotInteger));
    }

    #[test]
    fn canonical_round_trip_preserves_symbolic_width() {
        let original = TypeKind::Integer {
            signedness: IntegerSignedness::Unsigned,
            width: IntegerWidth::Symbolic(SymbolicExpression {
                expression: "problem_size * 3".to_owned(),
            }),
        };

        let descriptor = IntegerTypeDescriptor::try_from_kind(&original)
            .expect("valid canonical integer type");

        assert_eq!(descriptor.to_type_kind(), original);
        assert_eq!(descriptor.into_type_kind(), original);
    }

    #[test]
    fn shared_validator_rejects_non_integer_types() {
        assert_eq!(
            validate_integer_type(&TypeKind::String),
            Err(IntegerTypeError::NotInteger)
        );
    }

    #[test]
    fn repeated_validation_is_deterministic() {
        let descriptor = IntegerTypeDescriptor::new(
            IntegerSignedness::Signed,
            IntegerWidth::Fixed(64),
        )
        .expect("valid integer descriptor");

        assert_eq!(descriptor.validate(), Ok(()));
        assert_eq!(descriptor.validate(), Ok(()));
    }
}
