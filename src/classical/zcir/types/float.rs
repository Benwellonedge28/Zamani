
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR floating-point type descriptors
//!
//! Path: `src/classical/zcir/types/float.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Provides a validated classical-facing descriptor for floating-point
//! semantic types. The descriptor adapts the canonical ZUIR `TypeKind::Float`
//! representation without creating a competing type system.
//!
//! A floating-point format is identified by a canonical `QualifiedName`.
//! This module does not assume that the set of valid formats is closed or
//! that a particular format is supported by every execution target.
//!
//! ## Authority and ownership
//!
//! Canonical authority:
//! - `crate::zuir::types::TypeKind` owns the semantic type representation.
//! - `crate::zuir::schema::types::QualifiedName` owns qualified-name
//!   representation and local name validation.
//! - The Zamani type-system specification defines numerical semantics.
//! - The frontend semantic layer resolves source syntax and determines
//!   the meaning of the selected format.
//!
//! This file owns:
//! - A small, validated adapter for canonical floating-point types.
//! - Local validation of the format name.
//! - Explicit conversion to and from the canonical ZUIR type.
//! - Errors for invalid descriptors and non-floating-point types.
//!
//! This file does not own:
//! - Source syntax, parsing, type inference, or name resolution.
//! - A closed registry of floating-point formats.
//! - Precision, exponent, rounding, NaN, infinity, or comparison semantics.
//! - Floating-point values, arithmetic, constant folding, or evaluation.
//! - Physical representation, ABI, register width, or memory layout.
//! - Target capability discovery, resource allocation, or lowering.
//! - A second canonical type identity, type registry, or IR.
//!
//! ## POCO-REAF and scalability
//!
//! Format names remain extensible. No maximum floating-point width, precision,
//! exponent range, number of formats, or target count is imposed here.
//!
//! A format identifier does not imply that its precision or range fits a
//! native Rust floating-point type. Such a restriction belongs to the
//! implementation that realizes the requested semantics.
//!
//! Symbolic or extension-defined formats are preserved by name. This module
//! does not invent their semantics or silently replace them with a familiar
//! native format.
//!
//! Finite memory, execution time, representation limits, and allocation
//! failures remain practical implementation concerns. This descriptor does
//! not claim that physical resources are infinite.
//!
//! ## Numerical correctness
//!
//! A valid format name is not sufficient evidence that a numerical operation
//! is valid or portable. The owning semantic and numerical contracts must
//! define the relevant precision, exponent/range, rounding, NaN, infinity,
//! comparison, conversion, and reproducibility behavior.
//!
//! Backends must not use this descriptor alone as proof that a target can
//! implement a format. Unsupported or lossy lowering must be reported by the
//! responsible verification or lowering layer.
//!
//! ## Integration contract
//!
//! - `src/classical/zcir/types/mod.rs` declares `pub mod float;` and may
//!   re-export `FloatTypeDescriptor` and `FloatTypeError`.
//! - `src/zuir/types/type_def.rs` remains the authority for `TypeKind::Float`.
//! - Frontend semantic analysis supplies an already-resolved format name.
//! - ZCIR type verification invokes `validate()` at relevant trust boundaries.
//! - `types/layout.rs` handles logical layout without assuming native support.
//! - `values/float.rs` owns floating-point value representations and constants.
//! - Numeric operations must preserve the specified format semantics.
//! - `lowering/to_zuir.rs` can use `into_type_kind()` without inventing a
//!   second representation.
//! - Target lowering separately checks capabilities and numerical fidelity.
//! - Serialization must validate decoded descriptors before trusting them.
//!
//! No dependency on a planned sibling module is introduced here.
//!
//! ## Compatibility and freeze criteria
//!
//! This descriptor is an adapter API, not a serialized ABI or persistent type
//! identity. Serialized format naming and compatibility remain governed by
//! the canonical ZUIR schema/versioning contracts.
//!
//! Freeze this file only after its imports compile against the actual crate,
//! its parent module is registered, and unit plus integration tests pass.

use core::fmt;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::TypeKind;

/// Validated descriptor for a canonical ZUIR floating-point type.
///
/// The format is represented by a qualified semantic name rather than a
/// closed enum. This allows new formats to be introduced without editing
/// this adapter every time the language or an extension adds a format.
///
/// Construction validates the qualified name's local invariants. It does
/// not establish that the format has been semantically defined, registered,
/// or implemented by a target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FloatTypeDescriptor {
    format: QualifiedName,
}

impl FloatTypeDescriptor {
    /// Constructs a floating-point descriptor from a qualified format name.
    ///
    /// The name is preserved exactly; it is not normalized or replaced with
    /// a native floating-point format.
    ///
    /// # Errors
    ///
    /// Returns [`FloatTypeError::InvalidFormatName`] if the name violates
    /// the canonical qualified-name invariants.
    pub fn new(format: QualifiedName) -> Result<Self, FloatTypeError> {
        format
            .validate()
            .map_err(|error| FloatTypeError::InvalidFormatName(error.to_string()))?;

        Ok(Self { format })
    }

    /// Parses and validates a qualified format name.
    ///
    /// Names use the canonical `::` qualified-name separator. The name's
    /// existence and numerical semantics must be resolved elsewhere.
    ///
    /// # Errors
    ///
    /// Returns [`FloatTypeError::InvalidFormatName`] for an empty,
    /// malformed, or otherwise invalid qualified name.
    pub fn from_format_name(name: &str) -> Result<Self, FloatTypeError> {
        let format = QualifiedName::parse(name)
            .map_err(|error| FloatTypeError::InvalidFormatName(error.to_string()))?;

        Self::new(format)
    }

    /// Adapts a canonical ZUIR type into a floating-point descriptor.
    ///
    /// The canonical type is borrowed, and its format name is preserved.
    /// Other type kinds are rejected instead of being coerced.
    ///
    /// # Errors
    ///
    /// Returns [`FloatTypeError::NotFloat`] if `kind` is not a floating-point
    /// type, or [`FloatTypeError::InvalidFormatName`] if its format name
    /// violates local qualified-name invariants.
    pub fn try_from_kind(kind: &TypeKind) -> Result<Self, FloatTypeError> {
        match kind {
            TypeKind::Float { format } => Self::new(format.clone()),
            _ => Err(FloatTypeError::NotFloat),
        }
    }

    /// Returns the canonical qualified format name.
    #[must_use]
    pub const fn format(&self) -> &QualifiedName {
        &self.format
    }

    /// Returns the format's canonical textual representation.
    ///
    /// This allocates a `String`; callers that only need to inspect the
    /// qualified name should use [`Self::format`] instead.
    #[must_use]
    pub fn canonical_format_name(&self) -> String {
        self.format.canonical_string()
    }

    /// Revalidates local descriptor invariants.
    ///
    /// Call this at trust boundaries when a descriptor may have originated
    /// from deserialization or an external adapter. This does not establish
    /// format registration, semantic correctness, or target support.
    ///
    /// # Errors
    ///
    /// Returns [`FloatTypeError::InvalidFormatName`] if the qualified name
    /// is invalid.
    pub fn validate(&self) -> Result<(), FloatTypeError> {
        self.format
            .validate()
            .map_err(|error| FloatTypeError::InvalidFormatName(error.to_string()))
    }

    /// Converts this descriptor into the canonical ZUIR float type.
    ///
    /// This preserves the format identifier and does not imply that the
    /// format is supported by any particular target.
    #[must_use]
    pub fn into_type_kind(self) -> TypeKind {
        TypeKind::Float {
            format: self.format,
        }
    }

    /// Returns an owned canonical ZUIR float type without consuming `self`.
    #[must_use]
    pub fn to_type_kind(&self) -> TypeKind {
        TypeKind::Float {
            format: self.format.clone(),
        }
    }

    /// Returns whether a canonical ZUIR type is a floating-point type.
    ///
    /// This is a structural classification only; it does not validate the
    /// format name or establish numerical semantics.
    #[must_use]
    pub fn is_float_kind(kind: &TypeKind) -> bool {
        matches!(kind, TypeKind::Float { .. })
    }
}

impl TryFrom<&TypeKind> for FloatTypeDescriptor {
    type Error = FloatTypeError;

    fn try_from(kind: &TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(kind)
    }
}

impl TryFrom<TypeKind> for FloatTypeDescriptor {
    type Error = FloatTypeError;

    fn try_from(kind: TypeKind) -> Result<Self, Self::Error> {
        match kind {
            TypeKind::Float { format } => Self::new(format),
            _ => Err(FloatTypeError::NotFloat),
        }
    }
}

/// Errors raised while constructing or validating a floating-point
/// descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FloatTypeError {
    /// The supplied format name is invalid.
    ///
    /// The message comes from canonical qualified-name validation. It is
    /// diagnostic text, not a stable machine-readable error code.
    InvalidFormatName(String),

    /// The supplied canonical type is not a floating-point type.
    NotFloat,
}

impl fmt::Display for FloatTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormatName(message) => {
                write!(f, "invalid floating-point format name: {message}")
            }
            Self::NotFloat => {
                f.write_str("canonical type is not a floating-point type")
            }
        }
    }
}

impl std::error::Error for FloatTypeError {}

#[cfg(test)]
mod tests {
    use crate::zuir::schema::types::QualifiedName;
    use crate::zuir::types::TypeKind;

    use super::{FloatTypeDescriptor, FloatTypeError};

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test format name should be valid")
    }

    #[test]
    fn constructs_a_descriptor_from_a_qualified_format_name() {
        let descriptor = FloatTypeDescriptor::new(name("core::binary32"))
            .expect("valid qualified format name should be accepted");

        assert_eq!(descriptor.format(), &name("core::binary32"));
        assert_eq!(
            descriptor.canonical_format_name(),
            "core::binary32".to_owned()
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn parses_a_qualified_format_name() {
        let descriptor = FloatTypeDescriptor::from_format_name("vendor::extended_float")
            .expect("a valid extension format name should be accepted");

        assert_eq!(
            descriptor.format(),
            &name("vendor::extended_float")
        );
    }

    #[test]
    fn rejects_an_empty_format_name() {
        assert!(matches!(
            FloatTypeDescriptor::from_format_name(""),
            Err(FloatTypeError::InvalidFormatName(_))
        ));
    }

    #[test]
    fn rejects_malformed_qualified_format_names() {
        for malformed in ["::binary32", "core::", "core::::binary32"] {
            assert!(
                FloatTypeDescriptor::from_format_name(malformed).is_err(),
                "expected malformed format name to be rejected: {malformed:?}"
            );
        }
    }

    #[test]
    fn adapts_a_canonical_zuiR_float_type() {
        let kind = TypeKind::Float {
            format: name("core::binary64"),
        };

        let descriptor = FloatTypeDescriptor::try_from_kind(&kind)
            .expect("canonical float type should adapt successfully");

        assert_eq!(descriptor.format(), &name("core::binary64"));
    }

    #[test]
    fn try_from_borrowed_type_preserves_the_canonical_format() {
        let kind = TypeKind::Float {
            format: name("scientific::extended_precision"),
        };

        let descriptor = FloatTypeDescriptor::try_from(&kind)
            .expect("borrowed canonical float type should adapt successfully");

        assert_eq!(
            descriptor.canonical_format_name(),
            "scientific::extended_precision"
        );
    }

    #[test]
    fn try_from_owned_type_preserves_the_canonical_format() {
        let kind = TypeKind::Float {
            format: name("core::binary32"),
        };

        let descriptor = FloatTypeDescriptor::try_from(kind)
            .expect("owned canonical float type should adapt successfully");

        assert_eq!(descriptor.format(), &name("core::binary32"));
    }

    #[test]
    fn rejects_non_float_types_without_coercion() {
        assert_eq!(
            FloatTypeDescriptor::try_from_kind(&TypeKind::Boolean),
            Err(FloatTypeError::NotFloat)
        );
    }

    #[test]
    fn classification_is_structural_and_borrows_the_type() {
        let float = TypeKind::Float {
            format: name("core::binary32"),
        };

        assert!(FloatTypeDescriptor::is_float_kind(&float));
        assert!(!FloatTypeDescriptor::is_float_kind(&TypeKind::Boolean));

        // Classification does not consume the canonical type.
        assert!(matches!(float, TypeKind::Float { .. }));
    }

    #[test]
    fn round_trip_preserves_the_canonical_float_format() {
        let original = TypeKind::Float {
            format: name("vendor::custom_float"),
        };

        let descriptor = FloatTypeDescriptor::try_from_kind(&original)
            .expect("canonical float should adapt");

        assert_eq!(descriptor.to_type_kind(), original);
        assert_eq!(descriptor.into_type_kind(), original);
    }

    #[test]
    fn format_names_are_not_restricted_to_a_closed_builtin_list() {
        let descriptor = FloatTypeDescriptor::from_format_name(
            "future_vendor::specialized_float_format",
        )
        .expect("well-formed extension format names must remain extensible");

        assert_eq!(
            descriptor.canonical_format_name(),
            "future_vendor::specialized_float_format"
        );
    }

    #[test]
    fn repeated_validation_is_deterministic() {
        let descriptor = FloatTypeDescriptor::from_format_name("core::binary64")
            .expect("valid format should construct");

        assert_eq!(descriptor.validate(), descriptor.validate());
    }
}
